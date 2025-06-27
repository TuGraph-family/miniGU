use std::sync::Arc;

use dashmap::iter::Iter;
use minigu_common::types::VertexId;

use crate::common::iterators::{ChunkData, VertexIteratorTrait};
use crate::common::model::vertex::Vertex;
use crate::error::StorageResult;
use crate::tp::iterators::adjacency_iterator::AdjacencyIterator;
use crate::tp::memory_graph::VersionedVertex;
use crate::tp::transaction::MemTransaction;

type VertexFilter<'a> = Box<dyn Fn(&Vertex) -> bool + 'a>;

/// A vertex iterator that supports filtering.
pub struct VertexIterator<'a> {
    inner: Iter<'a, VertexId, VersionedVertex>, // Native DashMap iterator
    txn: &'a MemTransaction,                    // Reference to the transaction
    filters: Vec<VertexFilter<'a>>,             // List of filtering predicates
    current_vertex: Option<Vertex>,             // Currently iterated vertex
}

impl Iterator for VertexIterator<'_> {
    type Item = StorageResult<Vertex>;

    /// Retrieves the next visible vertex that satisfies all filters.
    fn next(&mut self) -> Option<Self::Item> {
        for entry in self.inner.by_ref() {
            let vid = *entry.key();
            let versioned_vertex = entry.value();

            // Perform MVCC visibility check
            let visible_vertex = match versioned_vertex.get_visible(self.txn) {
                Ok(v) => v,
                _ => continue,
            };

            // Apply all filtering conditions
            if self.filters.iter().all(|f| f(&visible_vertex)) {
                // Record the vertex read in the transaction
                self.txn.vertex_reads().insert(vid);
                self.current_vertex = Some(visible_vertex.clone());
                return Some(Ok(visible_vertex));
            }
        }

        self.current_vertex = None; // Reset when iteration ends
        None
    }
}

impl<'a> VertexIteratorTrait<'a> for VertexIterator<'a> {
    type AdjacencyIterator = AdjacencyIterator<'a>;

    /// Adds a filtering predicate to the iterator (supports method chaining).
    fn filter<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&Vertex) -> bool + 'a,
    {
        self.filters.push(Box::new(predicate));
        self
    }

    /// Advances the iterator to the vertex with the specified ID or the next greater vertex.
    /// Returns `Ok(true)` if the exact vertex is found, `Ok(false)` otherwise.
    fn seek(&mut self, id: VertexId) -> StorageResult<bool> {
        for result in self.by_ref() {
            match result {
                Ok(vertex) if vertex.vid() == id => return Ok(true),
                Ok(vertex) if vertex.vid() > id => return Ok(false),
                _ => continue,
            }
        }
        Ok(false)
    }

    /// Returns a reference to the currently iterated vertex.
    fn vertex(&self) -> Option<&Vertex> {
        self.current_vertex.as_ref()
    }

    /// Retrieves the properties of the currently iterated vertex.
    fn properties(&self) -> ChunkData {
        if let Some(vertex) = &self.current_vertex {
            vec![Arc::new(vertex.properties().clone())]
        } else {
            ChunkData::new()
        }
    }
}

/// Implementation for `MemTransaction`
impl MemTransaction {
    /// Returns an iterator over all vertices in the graph.
    /// Filtering conditions can be applied using the `filter` method.
    pub fn iter_vertices(&self) -> VertexIterator<'_> {
        VertexIterator {
            inner: self.graph().vertices().iter(),
            txn: self,
            filters: Vec::new(), // Initialize with an empty filter list
            current_vertex: None,
        }
    }
}
