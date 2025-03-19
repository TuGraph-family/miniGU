use common::datatype::types::VertexId;

use crate::error::StorageResult;
use crate::iterators::{AdjacencyIteratorTrait, ChunkData};
use crate::model::vertex::Vertex;
/// A trait that defines a vertex iterator with filtering capabilities.
pub trait VertexIteratorTrait<'a>: Iterator<Item = StorageResult<Vertex>> {
    type AdjacencyIterator: AdjacencyIteratorTrait<'a>;
    /// Adds a filtering predicate to the iterator (supports method chaining).
    fn filter<F>(self, predicate: F) -> Self
    where
        F: Fn(&Vertex) -> bool + 'a;

    /// Advances the iterator to the vertex with the specified ID or the next greater vertex.
    /// Returns `Ok(true)` if the exact vertex is found, `Ok(false)` otherwise.
    fn advance(&mut self, id: VertexId) -> StorageResult<bool>;

    /// Returns a reference to the currently iterated vertex.
    fn vertex(&self) -> Option<&Vertex>;

    /// Retrieves the properties of the currently iterated vertex.
    fn properties(&self) -> ChunkData;

    /// Returns an iterator for the outgoing edges of the current vertex.
    fn get_out_edge_iterator(&self) -> StorageResult<Self::AdjacencyIterator>;

    /// Returns an iterator for the incoming edges of the current vertex.
    fn get_in_edge_iterator(&self) -> StorageResult<Self::AdjacencyIterator>;
}
