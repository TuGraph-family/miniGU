use std::sync::Arc;

use dashmap::iter::Iter;
use minigu_common::types::EdgeId;
use minigu_transaction::LockStrategy;

use crate::common::iterators::{ChunkData, EdgeIteratorTrait};
use crate::common::model::edge::Edge;
use crate::error::StorageResult;
use crate::tp::memory_graph::VersionedEdge;
use crate::tp::transaction::{MemTransaction, WriteKind};

type EdgeFilter<'a> = Box<dyn Fn(&Edge) -> bool + 'a>;

/// An edge iterator that supports filtering.
pub struct EdgeIterator<'a> {
    inner: Iter<'a, EdgeId, VersionedEdge>, // Native DashMap iterator
    txn: &'a MemTransaction,                // Reference to the transaction
    filters: Vec<EdgeFilter<'a>>,           // List of filtering predicates
    current_edge: Option<Edge>,             // Currently iterated edge
    pending_inserts: Vec<Edge>,             // OCC-only edges not yet in graph
    pending_index: usize,                   // Iterator index for pending inserts
}

impl Iterator for EdgeIterator<'_> {
    type Item = StorageResult<Edge>;

    /// Retrieves the next visible edge that satisfies all filters.
    fn next(&mut self) -> Option<Self::Item> {
        for entry in self.inner.by_ref() {
            let eid = *entry.key();
            let versioned_edge = entry.value();

            if self.txn.lock_strategy() == LockStrategy::Optimistic
                && let Some(intent) = self.txn.lookup_edge_write(eid)
            {
                match intent.kind {
                    WriteKind::InsertEdge(ref e) | WriteKind::UpdateEdge { after: ref e, .. } => {
                        if e.is_tombstone() {
                            continue;
                        }
                        if self.filters.iter().all(|f| f(e)) {
                            self.current_edge = Some(e.clone());
                            return Some(Ok(e.clone()));
                        }
                        continue;
                    }
                    WriteKind::DeleteEdge { .. } => {
                        continue;
                    }
                    _ => {}
                }
            }

            // Perform MVCC visibility check
            let visible_edge = match versioned_edge.get_visible(self.txn) {
                Ok(e) => e, // Skip logically deleted edges
                _ => continue,
            };

            // Apply all filtering conditions
            if self.filters.iter().all(|f| f(&visible_edge)) {
                // Record the edge read in the transaction
                self.txn.edge_reads().insert(eid);
                self.current_edge = Some(visible_edge.clone());
                return Some(Ok(visible_edge));
            }
        }

        while self.pending_index < self.pending_inserts.len() {
            let edge = self.pending_inserts[self.pending_index].clone();
            self.pending_index += 1;
            if self.filters.iter().all(|f| f(&edge)) {
                self.current_edge = Some(edge.clone());
                return Some(Ok(edge));
            }
        }

        self.current_edge = None; // Reset when iteration ends
        None
    }
}

impl<'a> EdgeIteratorTrait<'a> for EdgeIterator<'a> {
    /// Adds a filtering predicate to the iterator (supports method chaining).
    fn filter<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&Edge) -> bool + 'a,
    {
        self.filters.push(Box::new(predicate));
        self
    }

    /// Advances the iterator to the edge with the specified ID or the next greater edge.
    /// Returns `Ok(true)` if the exact edge is found, `Ok(false)` otherwise.
    fn seek(&mut self, id: EdgeId) -> StorageResult<bool> {
        for result in self.by_ref() {
            match result {
                Ok(edge) if edge.eid() == id => return Ok(true),
                Ok(edge) if edge.eid() > id => return Ok(false),
                _ => continue,
            }
        }
        Ok(false)
    }

    /// Returns a reference to the currently iterated edge.
    fn edge(&self) -> Option<&Edge> {
        self.current_edge.as_ref()
    }

    /// Retrieves the properties of the currently iterated edge.
    fn properties(&self) -> ChunkData {
        if let Some(edge) = &self.current_edge {
            vec![Arc::new(edge.properties().clone())]
        } else {
            ChunkData::new()
        }
    }
}

/// Implementation for `MemTransaction`
impl MemTransaction {
    /// Returns an iterator over all edges in the graph.
    /// Filtering conditions can be applied using the `filter` method.
    pub fn iter_edges(&self) -> EdgeIterator<'_> {
        let mut pending_inserts = Vec::new();
        if self.lock_strategy() == LockStrategy::Optimistic {
            let writes = self.edge_writes.read().unwrap();
            for (eid, intent) in writes.iter() {
                if let WriteKind::InsertEdge(edge) = &intent.kind
                    && self.graph().edges().get(eid).is_none()
                {
                    pending_inserts.push(edge.clone());
                }
            }
        }
        EdgeIterator {
            inner: self.graph().edges().iter(),
            txn: self,
            filters: Vec::new(), // Initialize with an empty filter list
            current_edge: None,  // No edge selected initially
            pending_inserts,
            pending_index: 0,
        }
    }
}
