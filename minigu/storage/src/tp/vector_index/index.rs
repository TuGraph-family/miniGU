use crate::error::StorageResult;

/// Vector index trait for approximate nearest neighbor search
pub trait VectorIndex: Send + Sync {
    /// Build the index from vectors with their associated node IDs
    /// Configuration is provided during adapter creation
    fn build(&mut self, vectors: &[(u32, Vec<f32>)]) -> StorageResult<()>;
    
    /// Search for k nearest neighbors using diskann-rs l_value parameter
    /// l_value corresponds to the search list size
    fn search(&self, query: &[f32], k: usize, l_value: u32) -> StorageResult<Vec<u32>>;
    
    /// Insert vectors with their node IDs (for dynamic updates)
    fn insert(&mut self, vectors: &[(u32, Vec<f32>)]) -> StorageResult<()>;
    
    /// Delete vectors by their node IDs
    fn delete(&mut self, node_ids: &[u32]) -> StorageResult<()>;
    
    /// Save the index to a file
    fn save(&mut self, path: &str) -> StorageResult<()>;
    
    /// Load the index from a file
    fn load(&mut self, path: &str, expected_num_points: usize) -> StorageResult<()>;
    
    /// Get the dimension of vectors in this index
    fn get_dimension(&self) -> usize;
    
    /// Get the number of vectors in this index
    fn size(&self) -> usize;
}