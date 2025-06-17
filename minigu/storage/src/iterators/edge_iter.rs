use minigu_common::types::EdgeId;

use crate::error::StorageResult;
use crate::iterators::ChunkData;
use crate::model::edge::Edge;

/// Trait defining the behavior of an edge iterator.
pub trait EdgeIteratorTrait<'a>: Iterator<Item = StorageResult<Edge>> {
    /// Adds a filtering predicate to the iterator (supports method chaining).
    fn filter<F>(self, predicate: F) -> Self
    where
        F: Fn(&Edge) -> bool + 'a,
        Self: Sized;

    /// Seeks the iterator to the edge with the specified ID or the next greater edge.
    /// Returns `Ok(true)` if the exact edge is found, `Ok(false)` otherwise.
    fn seek(&mut self, id: EdgeId) -> StorageResult<bool>;

    /// Returns a reference to the currently iterated edge.
    fn edge(&self) -> Option<&Edge>;

    /// Retrieves the properties of the currently iterated edge.
    fn properties(&self) -> ChunkData;
}
