use std::sync::Arc;

use crossbeam_skiplist::SkipSet;
use minigu_common::types::{EdgeId, VertexId};
use minigu_transaction::LockStrategy;

use crate::common::iterators::{AdjacencyIteratorTrait, Direction};
use crate::common::model::edge::Neighbor;
use crate::error::StorageResult;
use crate::tp::transaction::{MemTransaction, WriteKind};

type AdjFilter<'a> = Box<dyn Fn(&Neighbor) -> bool + 'a>;

const BATCH_SIZE: usize = 64;

/// An adjacency list iterator that supports filtering (for iterating over a single vertex's
/// adjacency list).
pub struct AdjacencyIterator<'a> {
    adj_list: Option<Arc<SkipSet<Neighbor>>>, // The adjacency list for the vertex
    current_entries: Vec<Neighbor>,           // Store current batch of entries
    current_index: usize,                     // Current index in the batch
    txn: &'a MemTransaction,                  // Reference to the transaction
    filters: Vec<AdjFilter<'a>>,              // List of filtering predicates
    current_adj: Option<Neighbor>,            // Current adjacency entry
}

impl Iterator for AdjacencyIterator<'_> {
    type Item = StorageResult<Neighbor>;

    /// Retrieves the next visible adjacency entry that satisfies all filters.
    fn next(&mut self) -> Option<Self::Item> {
        // If current batch is processed, get a new batch
        if self.current_index >= self.current_entries.len() {
            self.load_next_batch()?;
        }

        // Process entries in current batch
        while self.current_index < self.current_entries.len() {
            let entry = &self.current_entries[self.current_index];
            self.current_index += 1;

            let eid = entry.eid();

            // Perform MVCC visibility check.
            //
            // In OCC/Optimistic mode, newly inserted edges live only in the transaction's
            // write set before commit. To preserve "read-your-writes" semantics, consult
            // the write intent first.
            let is_visible = if matches!(self.txn.lock_strategy(), LockStrategy::Optimistic)
                && let Some(intent) = self.txn.lookup_edge_write(eid)
            {
                match intent.kind {
                    WriteKind::InsertEdge(ref e) | WriteKind::UpdateEdge { after: ref e, .. } => {
                        !e.is_tombstone()
                    }
                    WriteKind::DeleteEdge { .. } => false,
                    _ => self
                        .txn
                        .graph()
                        .edges
                        .get(&eid)
                        .map(|edge| edge.is_visible(self.txn))
                        .unwrap_or(false),
                }
            } else {
                self.txn
                    .graph()
                    .edges
                    .get(&eid)
                    .map(|edge| edge.is_visible(self.txn))
                    .unwrap_or(false)
            };

            if is_visible && self.filters.iter().all(|f| f(entry)) {
                let adj = *entry;
                self.current_adj = Some(adj);
                return Some(Ok(adj));
            }
        }

        // If current batch is processed but no match found, try loading next batch
        self.load_next_batch()?;
        self.next()
    }
}

impl<'a> AdjacencyIterator<'a> {
    fn load_next_batch(&mut self) -> Option<()> {
        if let Some(adj_list) = &self.adj_list {
            let mut current = if let Some(e) = self.current_entries.last() {
                // If there is a last entry, get the next entry from the adjacency list
                adj_list.get(e)?.next()?
            } else {
                // If there is no last entry, get the first entry from the adjacency list
                adj_list.front()?
            };
            // Clear current entry batch
            self.current_entries.clear();
            self.current_index = 0;

            // Load the next batch of entries
            self.current_entries.push(*current.value());
            for _ in 0..BATCH_SIZE {
                if let Some(entry) = current.next() {
                    self.current_entries.push(*entry.value());
                    current = entry;
                } else {
                    break;
                }
            }

            if !self.current_entries.is_empty() {
                return Some(());
            }
        }
        None
    }

    /// Creates a new `AdjacencyIterator` for a given vertex and direction (incoming or outgoing).
    pub fn new(txn: &'a MemTransaction, vid: VertexId, direction: Direction) -> Self {
        let adjacency_entry = txn.graph().adjacency_list.get(&vid);

        // Fast-path: in pessimistic mode, the adjacency lists are already updated in-place and
        // do not require an overlay. Avoid O(degree) copying by reusing the original Arc.
        let needs_overlay = matches!(txn.lock_strategy(), LockStrategy::Optimistic)
            || matches!(direction, Direction::Both);

        let adj_list = if !needs_overlay {
            let base = adjacency_entry.as_ref().and_then(|entry| match direction {
                Direction::Incoming => Some(entry.incoming().clone()),
                Direction::Outgoing => Some(entry.outgoing().clone()),
                Direction::Both => None,
            });
            base.filter(|set| !set.is_empty())
        } else {
            let combined = SkipSet::new();

            if let Some(entry) = adjacency_entry.as_ref() {
                match direction {
                    Direction::Incoming => {
                        for neighbor in entry.incoming().iter() {
                            combined.insert(*neighbor);
                        }
                    }
                    Direction::Outgoing => {
                        for neighbor in entry.outgoing().iter() {
                            combined.insert(*neighbor);
                        }
                    }
                    Direction::Both => {
                        for neighbor in entry.incoming().iter() {
                            combined.insert(*neighbor);
                        }
                        for neighbor in entry.outgoing().iter() {
                            combined.insert(*neighbor);
                        }
                    }
                }
            }

            if matches!(txn.lock_strategy(), LockStrategy::Optimistic) {
                let writes = txn.edge_writes.read().unwrap();
                for intent in writes.values() {
                    match &intent.kind {
                        WriteKind::InsertEdge(edge) | WriteKind::UpdateEdge { after: edge, .. } => {
                            if edge.is_tombstone() {
                                continue;
                            }
                            if matches!(direction, Direction::Outgoing | Direction::Both)
                                && edge.src_id() == vid
                            {
                                combined.insert(Neighbor::new(
                                    edge.label_id(),
                                    edge.dst_id(),
                                    edge.eid(),
                                ));
                            }
                            if matches!(direction, Direction::Incoming | Direction::Both)
                                && edge.dst_id() == vid
                            {
                                combined.insert(Neighbor::new(
                                    edge.label_id(),
                                    edge.src_id(),
                                    edge.eid(),
                                ));
                            }
                        }
                        WriteKind::DeleteEdge { before } => {
                            if matches!(direction, Direction::Outgoing | Direction::Both)
                                && before.src_id() == vid
                            {
                                combined.remove(&Neighbor::new(
                                    before.label_id(),
                                    before.dst_id(),
                                    before.eid(),
                                ));
                            }
                            if matches!(direction, Direction::Incoming | Direction::Both)
                                && before.dst_id() == vid
                            {
                                combined.remove(&Neighbor::new(
                                    before.label_id(),
                                    before.src_id(),
                                    before.eid(),
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            }

            if combined.is_empty() {
                None
            } else {
                Some(Arc::new(combined))
            }
        };

        let mut result = Self {
            adj_list,
            current_entries: Vec::new(),
            current_index: 0,
            txn,
            filters: Vec::new(),
            current_adj: None,
        };

        // Preload the first batch of data
        if result.adj_list.is_some() {
            result.load_next_batch();
        }

        result
    }
}

impl<'a> AdjacencyIteratorTrait<'a> for AdjacencyIterator<'a> {
    /// Adds a filtering predicate to the iterator (supports method chaining).
    fn filter<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&Neighbor) -> bool + 'a,
    {
        self.filters.push(Box::new(predicate));
        self
    }

    /// Advances the iterator to the edge with the specified ID or the next greater edge.
    /// Returns `Ok(true)` if the exact edge is found, `Ok(false)` otherwise.
    fn seek(&mut self, id: EdgeId) -> StorageResult<bool> {
        for result in self.by_ref() {
            match result {
                Ok(entry) if entry.eid() == id => return Ok(true),
                Ok(entry) if entry.eid() > id => return Ok(false),
                _ => continue,
            }
        }
        Ok(false)
    }

    /// Returns a reference to the currently iterated adjacency entry.
    fn current_entry(&self) -> Option<&Neighbor> {
        self.current_adj.as_ref()
    }
}

/// Implementation for `MemTransaction`
impl MemTransaction {
    /// Returns an iterator over the adjacency list of a given vertex.
    /// Filtering conditions can be applied using the `filter` method.
    pub fn iter_adjacency(&self, vid: VertexId) -> AdjacencyIterator<'_> {
        AdjacencyIterator::new(self, vid, Direction::Both)
    }

    #[allow(dead_code)]
    pub fn iter_adjacency_outgoing(&self, vid: VertexId) -> AdjacencyIterator<'_> {
        AdjacencyIterator::new(self, vid, Direction::Outgoing)
    }

    #[allow(dead_code)]
    pub fn iter_adjacency_incoming(&self, vid: VertexId) -> AdjacencyIterator<'_> {
        AdjacencyIterator::new(self, vid, Direction::Incoming)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use minigu_common::types::LabelId;
    use minigu_transaction::{IsolationLevel, LockStrategy};

    use super::AdjacencyIterator;
    use crate::common::iterators::Direction;
    use crate::model::edge::{Edge, Neighbor};
    use crate::model::properties::PropertyRecord;
    use crate::model::vertex::Vertex;
    use crate::tp::memory_graph::MemoryGraph;

    #[test]
    fn adjacency_iterator_pessimistic_outgoing_reuses_arc() {
        let graph = crate::tp::memory_graph::tests::mock_graph();
        let txn = graph
            .txn_manager()
            .begin_transaction_at(
                None,
                None,
                IsolationLevel::Serializable,
                LockStrategy::Pessimistic,
                false,
            )
            .unwrap();

        let vid = 1;
        let expected = graph.adjacency_list.get(&vid).unwrap().outgoing().clone();
        assert!(!expected.is_empty());

        let iter = AdjacencyIterator::new(txn.as_ref(), vid, Direction::Outgoing);
        let actual = iter.adj_list.expect("adj_list should exist");

        assert!(Arc::ptr_eq(&actual, &expected));
    }

    #[test]
    fn adjacency_iterator_pessimistic_incoming_reuses_arc() {
        let graph = crate::tp::memory_graph::tests::mock_graph();
        let txn = graph
            .txn_manager()
            .begin_transaction_at(
                None,
                None,
                IsolationLevel::Serializable,
                LockStrategy::Pessimistic,
                false,
            )
            .unwrap();

        let vid = 1;
        let expected = graph.adjacency_list.get(&vid).unwrap().incoming().clone();
        assert!(!expected.is_empty());

        let iter = AdjacencyIterator::new(txn.as_ref(), vid, Direction::Incoming);
        let actual = iter.adj_list.expect("adj_list should exist");

        assert!(Arc::ptr_eq(&actual, &expected));
    }

    #[test]
    fn adjacency_iterator_optimistic_overlay_includes_inserts() {
        let graph = MemoryGraph::in_memory();
        let txn = graph
            .txn_manager()
            .begin_transaction_at(
                None,
                None,
                IsolationLevel::Serializable,
                LockStrategy::Optimistic,
                false,
            )
            .unwrap();

        let label = LabelId::new(1).unwrap();
        graph
            .create_vertex(&txn, Vertex::new(1, label, PropertyRecord::new(vec![])))
            .unwrap();
        graph
            .create_vertex(&txn, Vertex::new(2, label, PropertyRecord::new(vec![])))
            .unwrap();
        graph
            .create_edge(
                &txn,
                Edge::new(10, 1, 2, label, PropertyRecord::new(vec![])),
            )
            .unwrap();

        let neighbors: Vec<Neighbor> = txn.iter_adjacency_outgoing(1).map(|r| r.unwrap()).collect();

        assert_eq!(neighbors, vec![Neighbor::new(label, 2, 10)]);
    }
}
