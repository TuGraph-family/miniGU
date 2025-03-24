use std::sync::Arc;

use common::datatype::types::VertexId;
use dashmap::iter::Iter;

use crate::error::StorageResult;
use crate::iterators::{ChunkData, VertexIteratorTrait};
use crate::memory::adjacency_iterator::AdjacencyIterator;
use crate::memory::memory_graph::VersionedVertex;
use crate::model::edge::Direction;
use crate::model::vertex::Vertex;

use super::transaction::MemTransaction;

/// A vertex iterator that supports filtering.
pub struct VertexIterator<'a> {
    inner: Iter<'a, VertexId, VersionedVertex>, // Native DashMap iterator
    txn: &'a MemTransaction,                    // Reference to the transaction
    filters: Vec<Box<dyn Fn(&Vertex) -> bool + 'a>>, // List of filtering predicates
    current_vertex: Option<Vertex>,             // Currently iterated vertex
}

impl<'a> Iterator for VertexIterator<'a> {
    type Item = StorageResult<Vertex>;

    /// Retrieves the next visible vertex that satisfies all filters.
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.inner.next() {
            let vid = *entry.key();
            let versioned_vertex = entry.value();

            // Perform MVCC visibility check
            let visible_vertex =
                match versioned_vertex.get_visible(&self.txn) {
                    Ok(v) if !v.is_tombstone() => v, // Skip logically deleted vertices
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
    fn advance(&mut self, id: VertexId) -> StorageResult<bool> {
        while let Some(result) = self.next() {
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

    /// Returns an iterator for the outgoing edges of the current vertex.
    fn get_out_edge_iterator(&self) -> StorageResult<AdjacencyIterator<'a>> {
        let vid = self.current_vertex.as_ref().map(|v| v.vid()).unwrap();
        Ok(AdjacencyIterator::new(self.txn, vid, Direction::Out))
    }

    /// Returns an iterator for the incoming edges of the current vertex.
    fn get_in_edge_iterator(&self) -> StorageResult<AdjacencyIterator<'a>> {
        let vid = self.current_vertex.as_ref().map(|v| v.vid()).unwrap();
        Ok(AdjacencyIterator::new(self.txn, vid, Direction::In))
    }
}

/// Implementation for `MemTransaction`
impl<'a> MemTransaction {
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
