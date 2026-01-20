use diskann::common::FilterIndex as DiskANNFilterMask;

use super::filter::FilterMask;
use crate::error::StorageResult;

/// Vector index trait for approximate nearest neighbor search
pub trait VectorIndex: Send + Sync {
    /// Build the index from vectors with their associated node IDs
    /// Configuration is provided during adapter creation
    fn build(&mut self, vectors: &[(u64, &[f32])]) -> StorageResult<()>;

    /// Pure DiskANN search for k nearest neighbors without filtering
    /// l_value corresponds to the search list size parameter
    /// Returns a vector of (vertex, distance) tuples
    fn ann_search(
        &self,
        query: &[f32],
        k: usize,
        l_value: u32,
        filter_mask: Option<&dyn DiskANNFilterMask>,
        should_pre: bool,
    ) -> StorageResult<Vec<(u64, f32)>>;

    /// Search for k nearest neighbors with optional filtering
    /// filter_mask: None for no filtering, Some(mask) for filtered search
    /// Automatically selects optimal strategy based on filter characteristics
    /// Returns a vector of (vertex, distance) tuples
    fn search(
        &self,
        query: &[f32],
        k: usize,
        l_value: u32,
        filter_mask: Option<&FilterMask>,
        should_pre: bool,
    ) -> StorageResult<Vec<(u64, f32)>>;

    /// Insert vectors with their node IDs (for dynamic updates)
    fn insert(&mut self, vectors: &[(u64, &[f32])]) -> StorageResult<()>;

    /// Delete vectors by their node IDs
    fn soft_delete(&mut self, node_ids: &[u64]) -> StorageResult<()>;

    /// Save the index to a file
    fn save(&mut self, path: &str) -> StorageResult<()>;

    /// Load the index from a file
    fn load(&mut self, path: &str) -> StorageResult<()>;

    /// Get the dimension of vectors in this index
    fn get_dimension(&self) -> usize;

    /// Get the number of vectors in this index
    fn size(&self) -> usize;

    /// Convert node_id to vector_id, returns None if node_id not found in index
    fn node_to_vector_id(&self, node_id: u64) -> Option<u32>;
}
