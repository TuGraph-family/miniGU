use common::datatype::types::EdgeId;

use crate::error::StorageResult;
use crate::model::edge::EdgeUid;

/// Trait defining the behavior of an adjacency iterator.
pub trait AdjacencyIteratorTrait<'a>: Iterator<Item = StorageResult<EdgeUid>> {
    /// Adds a filtering predicate to the iterator (supports method chaining).
    fn filter<F>(self, predicate: F) -> Self
    where
        F: Fn(&EdgeUid) -> bool + 'a,
        Self: Sized;

    /// Advances the iterator to the edge with the specified ID or the next greater edge.
    /// Returns `Ok(true)` if the exact edge is found, `Ok(false)` otherwise.
    fn advance(&mut self, id: EdgeId) -> StorageResult<bool>;

    /// Returns a reference to the currently iterated adjacency entry.
    fn current_entry(&self) -> Option<&EdgeUid>;
}
