use minigu_common::types::VertexId;
use minigu_common::value::ScalarValue;

use crate::error::StorageResult;
use crate::iterators::DEFAULT_BATCH_SIZE;
use crate::model::vertex::Vertex;

/// Trait defining the behavior of a vertex iterator.
///
/// This trait provides methods for iterating over vertices in a graph storage,
/// with support for filtering, seeking, and batch operations.
pub trait VertexIteratorTrait<'a>: Iterator<Item = StorageResult<Vertex>> {
    /// Adds a filtering predicate to the iterator (supports method chaining).
    ///
    /// # Arguments
    /// * `predicate` - A function that takes a vertex reference and returns true if the vertex
    ///   should be included in the iteration
    ///
    /// # Returns
    /// Returns self to support method chaining
    fn filter<F>(self, predicate: F) -> Self
    where
        F: Fn(&Vertex) -> bool + 'a;

    /// Seeks the iterator to the vertex with the specified ID or the next greater vertex.
    ///
    /// This method positions the iterator at the vertex with the given ID if it exists,
    /// or at the next vertex with a greater ID.
    ///
    /// # Arguments
    /// * `id` - The target vertex ID to seek to
    ///
    /// # Returns
    /// * `Ok(true)` if the exact vertex is found
    /// * `Ok(false)` if positioned at next greater vertex or vertex not found
    /// * `Err(StorageError)` if an error occurs during seeking
    fn seek(&mut self, id: VertexId) -> StorageResult<bool>;

    /// Returns a reference to the currently iterated vertex.
    ///
    /// # Returns
    /// * `Some(&Vertex)` if there is a current vertex
    /// * `None` if the iterator is not positioned at a valid vertex
    fn current_vertex(&self) -> Option<&Vertex>;

    /// Returns the properties of the currently iterated vertex.
    ///
    /// # Returns
    /// A vector of scalar values representing the vertex properties,
    /// or an empty vector if no current vertex exists.
    fn current_properties(&self) -> Vec<ScalarValue> {
        self.current_vertex()
            .map(|v| v.properties().clone())
            .unwrap_or_default()
    }

    /// Retrieves the next batch of vertices.
    ///
    /// This method allows for efficient batch processing of vertices,
    /// which can be particularly useful for analytical workloads.
    ///
    /// # Arguments
    /// * `batch_size` - Maximum number of vertices to return in the batch. If None, uses
    ///   DEFAULT_BATCH_SIZE.
    ///
    /// # Returns
    /// * `Ok(Some(vertices))` if vertices are available
    /// * `Ok(None)` if no more vertices are available
    /// * `Err(StorageError)` if an error occurs during batch retrieval
    fn next_batch(&mut self, batch_size: Option<usize>) -> StorageResult<Option<Vec<Vertex>>> {
        let size = batch_size.unwrap_or(DEFAULT_BATCH_SIZE);
        let mut batch = Vec::with_capacity(size);

        for _ in 0..size {
            match self.next() {
                Some(Ok(vertex)) => batch.push(vertex),
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

    /// Collects all remaining vertices into a vector.
    ///
    /// # Returns
    /// * `Ok(vertices)` if all vertices are collected successfully
    /// * `Err(StorageError)` if an error occurs during collection
    fn collect_all(self) -> StorageResult<Vec<Vertex>>
    where
        Self: Sized,
    {
        self.collect()
    }

    /// Counts the remaining vertices in the iterator.
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
}
