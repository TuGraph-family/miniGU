use std::sync::Arc;

use dashmap::iter::Iter;
use minigu_common::types::EdgeId;

use crate::common::iterators::{ChunkData, EdgeIteratorTrait};
use crate::common::model::edge::Edge;
use crate::error::StorageResult;
use crate::tp::memory_graph::VersionedEdge;
use crate::tp::transaction::MemTransaction;

type EdgeFilter<'a> = Box<dyn Fn(&Edge) -> bool + 'a>;

/// An edge iterator that supports filtering.
pub struct EdgeIterator<'a> {
    inner: Iter<'a, EdgeId, VersionedEdge>, // Native DashMap iterator
    #[allow(dead_code)]
    txn: &'a MemTransaction, // Reference to the transaction
    filters: Vec<EdgeFilter<'a>>,           // List of filtering predicates
    current_edge: Option<Edge>,             // Currently iterated edge
}

impl Iterator for EdgeIterator<'_> {
    type Item = StorageResult<Edge>;

    /// Retrieves the next visible edge that satisfies all filters.
    fn next(&mut self) -> Option<Self::Item> {
        for entry in self.inner.by_ref() {
            let edge = entry.value().data();

            if edge.is_tombstone() {
                continue;
            }

            // Apply all filtering conditions
            if self.filters.iter().all(|f| f(&edge)) {
                // Record the edge read in the transaction
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
        EdgeIterator {
            inner: self.graph().edges().iter(),
            txn: self,
            filters: Vec::new(), // Initialize with an empty filter list
            current_edge: None,  // No edge selected initially
        }
    }
}
