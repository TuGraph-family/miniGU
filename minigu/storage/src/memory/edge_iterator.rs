use std::sync::Arc;

use common::datatype::types::EdgeId;
use dashmap::iter::Iter;

use crate::error::StorageResult;
use crate::iterators::{ChunkData, EdgeIteratorTrait};
use crate::memory::memory_graph::{MemTransaction, VersionedEdge};
use crate::model::edge::Edge;

/// An edge iterator that supports filtering.
pub struct EdgeIterator<'a> {
    inner: Iter<'a, EdgeId, VersionedEdge>, // Native DashMap iterator
    txn: &'a MemTransaction,                // Reference to the transaction
    filters: Vec<Box<dyn Fn(&Edge) -> bool + 'a>>, // List of filtering predicates
    current_edge: Option<Edge>,             // Currently iterated edge
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = StorageResult<Edge>;

    /// Retrieves the next visible edge that satisfies all filters.
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.inner.next() {
            let eid = *entry.key();
            let versioned_edge = entry.value();

            // Perform MVCC visibility check
            let visible_edge =
                match versioned_edge.get_visible(self.txn.start_ts(), self.txn.tx_id()) {
                    Some(e) if !e.is_tombstone() => e, // Skip logically deleted edges
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
    fn advance(&mut self, id: EdgeId) -> StorageResult<bool> {
        while let Some(result) = self.next() {
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
impl<'a> MemTransaction {
    /// Returns an iterator over all edges in the graph.
    /// Filtering conditions can be applied using the `filter` method.
    pub fn iter_edges(&self) -> EdgeIterator<'_> {
        EdgeIterator {
            inner: self.graph().edges().iter(),
            txn: self,
            filters: Vec::new(), // Initialize with an empty filter list
            current_edge: None,  // No edge selected initially
        }
    }
}
