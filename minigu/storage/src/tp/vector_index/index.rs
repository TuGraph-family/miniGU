use crate::error::StorageResult;

/// Parameters for building a vector index
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BuildParams {
    /// The maximum degree of the graph
    pub ef_construction: usize,
    /// The maximum degree of the graph
    pub max_degree: usize,
    /// The alpha parameter for pruning
    pub alpha: f32,
}

impl Default for BuildParams {
    fn default() -> Self {
        Self {
            ef_construction: 200,
            max_degree: 64,
            alpha: 1.2,
        }
    }
}

/// Parameters for searching a vector index
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchParams {
    /// The size of the dynamic search list
    pub ef_search: u32,
}

impl Default for SearchParams {
    fn default() -> Self {
        Self {
            ef_search: 100,
        }
    }
}

/// Parameters for loading a vector index
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoadParams {
    /// Expected number of points in the index
    pub expected_num_points: usize,
    /// Whether to use mmap for loading
    pub use_mmap: bool,
}

impl Default for LoadParams {
    fn default() -> Self {
        Self {
            expected_num_points: 1000,
            use_mmap: false,
        }
    }
}

pub trait VectorIndex: Send + Sync {
    /// Build the index from vectors with their associated node IDs
    fn build(&mut self, vectors: &[(u32, Vec<f32>)], params: BuildParams) -> StorageResult<()>;
    
    /// Search for k nearest neighbors, returning node IDs
    fn search(&self, query: &[f32], k: usize, params: SearchParams) -> StorageResult<Vec<u32>>;
    
    /// Insert a vector with its node ID (for dynamic updates)
    fn insert(&mut self, node_id: u32, vector: Vec<f32>) -> StorageResult<()>;
    
    /// Delete vectors by their node IDs
    fn delete(&mut self, node_ids: &[u32]) -> StorageResult<()>;
    
    /// Save the index to a file
    fn save(&mut self, path: &str) -> StorageResult<()>;
    
    /// Load the index from a file
    fn load(&mut self, path: &str, params: LoadParams) -> StorageResult<()>;
    
    /// Get the dimension of vectors in this index
    fn get_dimension(&self) -> usize;
    
    /// Get the number of vectors in this index
    fn size(&self) -> usize;
}