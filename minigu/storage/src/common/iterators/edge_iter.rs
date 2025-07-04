use minigu_common::types::EdgeId;
use minigu_common::value::ScalarValue;

use crate::error::StorageResult;
use crate::iterators::DEFAULT_BATCH_SIZE;
use crate::model::edge::Edge;

/// Trait defining the behavior of an edge iterator.
///
/// This trait provides methods for iterating over edges in a graph storage,
/// with support for filtering, seeking, and batch operations.
pub trait EdgeIteratorTrait<'a>: Iterator<Item = StorageResult<Edge>> {
    /// Adds a filtering predicate to the iterator (supports method chaining).
    ///
    /// # Arguments
    /// * `predicate` - A function that takes an edge reference and returns true if the edge should
    ///   be included in the iteration
    ///
    /// # Returns
    /// Returns self to support method chaining
    fn filter<F>(self, predicate: F) -> Self
    where
        F: Fn(&Edge) -> bool + 'a,
        Self: Sized;

    /// Seeks the iterator to the edge with the specified ID or the next greater edge.
    ///
    /// This method positions the iterator at the edge with the given ID if it exists,
    /// or at the next edge with a greater ID.
    ///
    /// # Arguments
    /// * `id` - The target edge ID to seek to
    ///
    /// # Returns
    /// * `Ok(true)` if the exact edge is found
    /// * `Ok(false)` if positioned at next greater edge or edge not found
    /// * `Err(StorageError)` if an error occurs during seeking
    fn seek(&mut self, id: EdgeId) -> StorageResult<bool>;

    /// Returns a reference to the currently iterated edge.
    ///
    /// # Returns
    /// * `Some(&Edge)` if there is a current edge
    /// * `None` if the iterator is not positioned at a valid edge
    fn current_edge(&self) -> Option<&Edge>;

    /// Returns the properties of the currently iterated edge.
    ///
    /// # Returns
    /// A vector of scalar values representing the edge properties,
    /// or an empty vector if no current edge exists.
    fn current_properties(&self) -> Vec<ScalarValue> {
        self.current_edge()
            .map(|e| e.properties().clone())
            .unwrap_or_default()
    }

    /// Retrieves the next batch of edges.
    ///
    /// This method allows for efficient batch processing of edges,
    /// which can be particularly useful for analytical workloads.
    ///
    /// # Arguments
    /// * `batch_size` - Maximum number of edges to return in the batch. If None, uses
    ///   DEFAULT_BATCH_SIZE.
    ///
    /// # Returns
    /// * `Ok(Some(edges))` if edges are available
    /// * `Ok(None)` if no more edges are available
    /// * `Err(StorageError)` if an error occurs during batch retrieval
    fn next_batch(&mut self, batch_size: Option<usize>) -> StorageResult<Option<Vec<Edge>>> {
        let size = batch_size.unwrap_or(DEFAULT_BATCH_SIZE);
        let mut batch = Vec::with_capacity(size);

        for _ in 0..size {
            match self.next() {
                Some(Ok(edge)) => batch.push(edge),
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

    /// Collects all remaining edges into a vector.
    ///
    /// # Returns
    /// * `Ok(edges)` if all edges are collected successfully
    /// * `Err(StorageError)` if an error occurs during collection
    fn collect_all(self) -> StorageResult<Vec<Edge>>
    where
        Self: Sized,
    {
        self.collect()
    }

    /// Counts the remaining edges in the iterator.
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

    /// Filters edges by source vertex ID.
    ///
    /// # Arguments
    /// * `src_id` - The source vertex ID to filter by
    ///
    /// # Returns
    /// Returns self to support method chaining
    fn filter_by_src(self, src_id: minigu_common::types::VertexId) -> Self
    where
        Self: Sized,
    {
        EdgeIteratorTrait::filter(self, move |edge| edge.src_id() == src_id)
    }

    /// Filters edges by destination vertex ID.
    ///
    /// # Arguments
    /// * `dst_id` - The destination vertex ID to filter by
    ///
    /// # Returns
    /// Returns self to support method chaining
    fn filter_by_dst(self, dst_id: minigu_common::types::VertexId) -> Self
    where
        Self: Sized,
    {
        EdgeIteratorTrait::filter(self, move |edge| edge.dst_id() == dst_id)
    }

    /// Filters edges by label ID.
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
        EdgeIteratorTrait::filter(self, move |edge| edge.label_id() == label_id)
    }
}
