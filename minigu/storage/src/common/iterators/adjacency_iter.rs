use minigu_common::types::EdgeId;

use crate::error::StorageResult;
use crate::iterators::DEFAULT_BATCH_SIZE;
use crate::model::edge::Neighbor;

/// Direction for adjacency iteration.
///
/// Specifies whether to iterate over incoming edges, outgoing edges, or both.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Iterate over incoming edges (edges pointing to the vertex)
    Incoming,
    /// Iterate over outgoing edges (edges pointing from the vertex)
    Outgoing,
    /// Iterate over both incoming and outgoing edges
    Both,
}

/// Trait defining the behavior of an adjacency iterator.
///
/// This trait provides methods for iterating over the adjacency list of a vertex,
/// with support for filtering, seeking, and batch operations.
pub trait AdjacencyIteratorTrait<'a>: Iterator<Item = StorageResult<Neighbor>> {
    /// Adds a filtering predicate to the iterator (supports method chaining).
    ///
    /// # Arguments
    /// * `predicate` - A function that takes a neighbor reference and returns true if the neighbor
    ///   should be included in the iteration
    ///
    /// # Returns
    /// Returns self to support method chaining
    fn filter<F>(self, predicate: F) -> Self
    where
        F: Fn(&Neighbor) -> bool + 'a,
        Self: Sized;

    /// Seeks the iterator to the edge with the specified ID or the next greater edge.
    ///
    /// This method positions the iterator at the neighbor with the given edge ID if it exists,
    /// or at the next neighbor with a greater edge ID.
    ///
    /// # Arguments
    /// * `id` - The target edge ID to seek to
    ///
    /// # Returns
    /// * `Ok(true)` if the exact edge is found
    /// * `Ok(false)` if positioned at next greater edge or edge not found
    /// * `Err(StorageError)` if an error occurs during seeking
    fn seek(&mut self, id: EdgeId) -> StorageResult<bool>;

    /// Returns a reference to the currently iterated adjacency entry.
    ///
    /// # Returns
    /// * `Some(&Neighbor)` if there is a current neighbor
    /// * `None` if the iterator is not positioned at a valid neighbor
    fn current_neighbor(&self) -> Option<&Neighbor>;

    /// Retrieves the next batch of neighbors.
    ///
    /// This method allows for efficient batch processing of neighbors,
    /// which can be particularly useful for analytical workloads.
    ///
    /// # Arguments
    /// * `batch_size` - Maximum number of neighbors to return in the batch. If None, uses
    ///   DEFAULT_BATCH_SIZE.
    ///
    /// # Returns
    /// * `Ok(Some(neighbors))` if neighbors are available
    /// * `Ok(None)` if no more neighbors are available
    /// * `Err(StorageError)` if an error occurs during batch retrieval
    fn next_batch(&mut self, batch_size: Option<usize>) -> StorageResult<Option<Vec<Neighbor>>> {
        let size = batch_size.unwrap_or(DEFAULT_BATCH_SIZE);
        let mut batch = Vec::with_capacity(size);

        for _ in 0..size {
            match self.next() {
                Some(Ok(neighbor)) => batch.push(neighbor),
                Some(Err(e)) => return Err(e),
                None => break,
            }
        }

        if batch.is_empty() {
            Ok(None)
        } else {
            Ok(Some(batch))
        }
    }

    /// Collects all remaining neighbors into a vector.
    ///
    /// # Returns
    /// * `Ok(neighbors)` if all neighbors are collected successfully
    /// * `Err(StorageError)` if an error occurs during collection
    fn collect_all(self) -> StorageResult<Vec<Neighbor>>
    where
        Self: Sized,
    {
        self.collect()
    }

    /// Counts the remaining neighbors in the iterator.
    ///
    /// # Returns
    /// * `Ok(count)` if counting is successful
    /// * `Err(StorageError)` if an error occurs during counting
    fn count_remaining(self) -> StorageResult<usize>
    where
        Self: Sized,
    {
        let mut count = 0;
        for result in self {
            result?; // Propagate any errors
            count += 1;
        }
        Ok(count)
    }

    /// Filters neighbors by neighbor vertex ID.
    ///
    /// # Arguments
    /// * `neighbor_id` - The neighbor vertex ID to filter by
    ///
    /// # Returns
    /// Returns self to support method chaining
    fn filter_by_neighbor_id(self, neighbor_id: minigu_common::types::VertexId) -> Self
    where
        Self: Sized,
    {
        AdjacencyIteratorTrait::filter(self, move |neighbor| neighbor.neighbor_id() == neighbor_id)
    }

    /// Filters neighbors by label ID.
    ///
    /// # Arguments
    /// * `label_id` - The label ID to filter by
    ///
    /// # Returns
    /// Returns self to support method chaining
    fn filter_by_label(self, label_id: minigu_common::types::LabelId) -> Self
    where
        Self: Sized,
    {
        AdjacencyIteratorTrait::filter(self, move |neighbor| neighbor.label_id() == label_id)
    }

    /// Filters neighbors by edge ID.
    ///
    /// # Arguments
    /// * `edge_id` - The edge ID to filter by
    ///
    /// # Returns
    /// Returns self to support method chaining
    fn filter_by_edge_id(self, edge_id: EdgeId) -> Self
    where
        Self: Sized,
    {
        AdjacencyIteratorTrait::filter(self, move |neighbor| neighbor.eid() == edge_id)
    }
}
