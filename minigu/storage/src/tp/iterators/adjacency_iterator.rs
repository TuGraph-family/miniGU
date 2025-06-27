use std::sync::Arc;

use crossbeam_skiplist::SkipSet;
use minigu_common::types::{EdgeId, VertexId};

use crate::common::iterators::{AdjacencyIteratorTrait, Direction};
use crate::common::model::edge::Neighbor;
use crate::error::StorageResult;
use crate::tp::transaction::MemTransaction;

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

            // Perform MVCC visibility check
            let is_visible = self
                .txn
                .graph()
                .edges
                .get(&eid)
                .map(|edge| edge.is_visible(self.txn))
                .unwrap_or(false);

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
        let adjacency_list = txn.graph().adjacency_list.get(&vid);

        let mut result = Self {
            adj_list: adjacency_list.map(|entry| match direction {
                Direction::Incoming => entry.incoming().clone(),
                Direction::Outgoing => entry.outgoing().clone(),
                Direction::Both => {
                    let combined = SkipSet::new();
                    for neighbor in entry.incoming().iter() {
                        combined.insert(*neighbor);
                    }
                    for neighbor in entry.outgoing().iter() {
                        combined.insert(*neighbor);
                    }
                    Arc::new(combined)
                }
            }),
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
