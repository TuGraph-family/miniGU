use crate::error::{StorageResult, StorageError, VectorIndexError};
use super::index::{VectorIndex, BuildParams, SearchParams, LoadParams};
use diskann::{
    index::{ANNInmemIndex, create_inmem_index},
    model::IndexConfiguration,
};
use dashmap::DashMap;
use std::time::Instant;
use tempfile::NamedTempFile;

/// Index statistics and performance metrics
#[derive(Debug, Clone, Default)]
pub struct IndexStats {
    /// Number of vectors in the index
    pub vector_count: usize,
    /// Index size in bytes
    pub memory_usage: usize,
    /// Build time in milliseconds
    pub build_time_ms: Option<u64>,
    /// Average search time in microseconds
    pub avg_search_time_us: Option<f64>,
    /// Number of searches performed
    pub search_count: u64,
}

impl IndexStats {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn update_after_build(&mut self, vector_count: usize, build_time_ms: u64, memory_usage: usize) {
        self.vector_count = vector_count;
        self.build_time_ms = Some(build_time_ms);
        self.memory_usage = memory_usage;
    }
}


pub struct InMemDiskANNAdapter {
    inner: Box<dyn ANNInmemIndex<f32>>,
    dimension: usize,
    /// Mapping from node ID to vector ID (assigned by DiskANN)
    node_to_vector_id: DashMap<u32, u32>,
    /// Mapping from vector ID to node ID
    vector_to_node_id: DashMap<u32, u32>,
    /// Index statistics and performance metrics
    stats: std::sync::RwLock<IndexStats>,
}

impl InMemDiskANNAdapter {
    pub fn new(config: IndexConfiguration) -> StorageResult<Self> {
        let dimension = config.dim;
        let inner = create_inmem_index::<f32>(config)
            .map_err(|e| StorageError::VectorIndex(VectorIndexError::DiskANN(e)))?;
        
        Ok(Self {
            inner,
            dimension,
            node_to_vector_id: DashMap::new(),
            vector_to_node_id: DashMap::new(),
            stats: std::sync::RwLock::new(IndexStats::new()),
        })
    }
    
    pub fn stats(&self) -> IndexStats {
        self.stats.read().unwrap().clone()
    }
    
    /// Get the number of ID mappings currently stored
    pub fn mapping_count(&self) -> usize {
        self.node_to_vector_id.len()
    }
    
    /// Clear all ID mappings (useful for rebuilding)
    pub fn clear_mappings(&mut self) {
        self.node_to_vector_id.clear();
        self.vector_to_node_id.clear();
    }
    
    /// Write vectors to diskann binary format using a temporary file
    fn write_vectors_to_temp_file(&self, vectors: &[Vec<f32>]) -> StorageResult<NamedTempFile> {
        use std::io::Write;
        
        let mut temp_file = NamedTempFile::new()
            .map_err(|e| StorageError::VectorIndex(VectorIndexError::InvalidInput(format!("Failed to create temp file: {}", e))))?;
        
        // Write number of vectors and dimension
        let num_vectors = vectors.len() as u32;
        let dimension = self.dimension as u32;
        
        temp_file.write_all(&num_vectors.to_le_bytes())
            .map_err(|e| StorageError::VectorIndex(VectorIndexError::InvalidInput(format!("Write error: {}", e))))?;
        temp_file.write_all(&dimension.to_le_bytes())
            .map_err(|e| StorageError::VectorIndex(VectorIndexError::InvalidInput(format!("Write error: {}", e))))?;
        
        // Write vector data
        for vector in vectors {
            for &value in vector {
                temp_file.write_all(&value.to_le_bytes())
                    .map_err(|e| StorageError::VectorIndex(VectorIndexError::InvalidInput(format!("Write error: {}", e))))?;
            }
        }
        
        temp_file.flush()
            .map_err(|e| StorageError::VectorIndex(VectorIndexError::InvalidInput(format!("Flush error: {}", e))))?;
        
        Ok(temp_file)
    }
}

impl VectorIndex for InMemDiskANNAdapter {
    fn build(&mut self, vectors: &[(u32, Vec<f32>)], params: BuildParams) -> StorageResult<()> {
        let start = Instant::now();
        
        // Validate input
        if vectors.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::EmptyDataset));
        }
        
        // Validate build parameters
        if params.ef_construction == 0 {
            return Err(StorageError::VectorIndex(VectorIndexError::InvalidBuildParams(
                "ef_construction must be greater than 0".to_string()
            )));
        }
        if params.max_degree == 0 {
            return Err(StorageError::VectorIndex(VectorIndexError::InvalidBuildParams(
                "max_degree must be greater than 0".to_string()
            )));
        }
        if params.alpha <= 0.0 {
            return Err(StorageError::VectorIndex(VectorIndexError::InvalidBuildParams(
                "alpha must be greater than 0".to_string()
            )));
        }
        
        // Clear existing mappings
        self.clear_mappings();
        
        // Validate dimension consistency and build ID mappings
        let expected_dim = self.dimension;
        let mut vector_data = Vec::with_capacity(vectors.len());
        let mut seen_nodes = std::collections::HashSet::new();
        
        for (vector_id, (node_id, vector)) in vectors.iter().enumerate() {
            // Check for duplicate node IDs
            if !seen_nodes.insert(*node_id) {
                return Err(StorageError::VectorIndex(VectorIndexError::DuplicateNodeId { node_id: *node_id }));
            }
            
            // Validate vector dimension
            if vector.len() != expected_dim {
                return Err(StorageError::VectorIndex(VectorIndexError::InvalidDimension {
                    expected: expected_dim,
                    actual: vector.len()
                }));
            }
            
            // Validate vector values (no NaN or infinity)
            for (i, &value) in vector.iter().enumerate() {
                if !value.is_finite() {
                    return Err(StorageError::VectorIndex(VectorIndexError::DataConversion(
                        format!("Vector for node {} contains non-finite value at index {}: {}", node_id, i, value)
                    )));
                }
            }
            
            let vector_id = vector_id as u32;
            
            // Build bidirectional ID mapping
            self.node_to_vector_id.insert(*node_id, vector_id);
            self.vector_to_node_id.insert(vector_id, *node_id);
            
            // Collect vector data for DiskANN
            vector_data.push(vector.clone());
        }
        
        // Write vectors to temporary file
        let temp_file = self.write_vectors_to_temp_file(&vector_data)
            .map_err(|e| StorageError::VectorIndex(VectorIndexError::TempFileError(e.to_string())))?;
        
        // Build the index
        self.inner.build(&temp_file.path().to_string_lossy(), vectors.len())
            .map_err(|e| StorageError::VectorIndex(VectorIndexError::BuildError(e.to_string())))?;
        
        // Update statistics
        let build_time = start.elapsed().as_millis() as u64;
        {
            let mut stats = self.stats.write().unwrap();
            stats.update_after_build(vectors.len(), build_time, 0); // Memory usage estimation would need diskann API
        }
        
        // temp_file is automatically cleaned up when dropped
        Ok(())
    }
    
    fn search(&self, query: &[f32], k: usize, params: SearchParams) -> StorageResult<Vec<u32>> {
        // Validate search parameters
        if k == 0 {
            return Err(StorageError::VectorIndex(VectorIndexError::InvalidSearchParams(
                "k must be greater than 0".to_string()
            )));
        }
        if params.ef_search == 0 {
            return Err(StorageError::VectorIndex(VectorIndexError::InvalidSearchParams(
                "ef_search must be greater than 0".to_string()
            )));
        }
        
        // Validate query vector
        if query.len() != self.dimension {
            return Err(StorageError::VectorIndex(VectorIndexError::InvalidDimension {
                expected: self.dimension,
                actual: query.len()
            }));
        }
        
        // Validate query values (no NaN or infinity)
        for (i, &value) in query.iter().enumerate() {
            if !value.is_finite() {
                return Err(StorageError::VectorIndex(VectorIndexError::DataConversion(
                    format!("Query vector contains non-finite value at index {}: {}", i, value)
                )));
            }
        }
        
        // Check if index is built
        if self.vector_to_node_id.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::IndexNotBuilt));
        }
        
        // Prepare result buffers
        let effective_k = std::cmp::min(k, self.size());
        let mut vector_ids = vec![0u32; effective_k];
        
        let actual_count = self.inner.search(query, effective_k, params.ef_search, &mut vector_ids)
            .map_err(|e| StorageError::VectorIndex(VectorIndexError::SearchError(e.to_string())))?;
        
        // Convert vector IDs to node IDs using the mapping
        let mut node_ids = Vec::with_capacity(actual_count as usize);
        for &vector_id in vector_ids.iter().take(actual_count as usize) {
            if let Some(node_id) = self.vector_to_node_id.get(&vector_id) {
                node_ids.push(*node_id);
            } else {
                return Err(StorageError::VectorIndex(VectorIndexError::VectorIdNotFound { vector_id }));
            }
        }
        
        // Update search statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.search_count += 1;
        }
        
        Ok(node_ids)
    }
    
    fn get_dimension(&self) -> usize {
        self.dimension
    }
    
    fn size(&self) -> usize {
        // Return the number of vectors in the index
        self.mapping_count()
    }
    
    fn insert(&mut self, _node_id: u32, _vector: Vec<f32>) -> StorageResult<()> {
        // DiskANN doesn't support dynamic insertion in current implementation
        Err(StorageError::VectorIndex(VectorIndexError::InvalidInput("Dynamic insertion not supported by DiskANN".to_string())))
    }
    
    fn delete(&mut self, node_ids: &[u32]) -> StorageResult<()> {
        if node_ids.is_empty() {
            return Ok(());
        }
        
        // Check if index is built
        if self.node_to_vector_id.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::IndexNotBuilt));
        }
        
        // Convert node IDs to vector IDs and validate they exist
        let mut vector_ids = Vec::with_capacity(node_ids.len());
        for &node_id in node_ids {
            if let Some(vector_id) = self.node_to_vector_id.get(&node_id) {
                vector_ids.push(*vector_id);
            } else {
                return Err(StorageError::VectorIndex(VectorIndexError::NodeIdNotFound { node_id }));
            }
        }
        
        // TODO: Implement soft deletion when diskann-rs supports it
        // self.inner.soft_delete(&vector_ids, vector_ids.len())
        //     .map_err(|e| StorageError::VectorIndex(VectorIndexError::DiskANN(e)))?;
        // 
        // // Remove from mappings
        // for &node_id in node_ids {
        //     if let Some(vector_id) = self.node_to_vector_id.remove(&node_id) {
        //         self.vector_to_node_id.remove(&vector_id.1);
        //     }
        // }
        
        Err(StorageError::VectorIndex(VectorIndexError::UnsupportedOperation(
            "Deletion not yet implemented in DiskANN".to_string()
        )))
    }
    
    fn save(&mut self, path: &str) -> StorageResult<()> {
        self.inner.save(path)
            .map_err(|e| StorageError::VectorIndex(VectorIndexError::DiskANN(e)))?;
        Ok(())
    }
    
    fn load(&mut self, path: &str, params: LoadParams) -> StorageResult<()> {
        // For now, loading requires recreating the index
        // In the future, we could implement proper loading by:
        // 1. Loading the index from file
        // 2. Reconstructing the ID mappings from metadata
        
        // TODO: Implement proper loading when diskann-rs supports it
        // self.inner.load(path, params.expected_num_points)
        //     .map_err(|e| StorageError::VectorIndex(VectorIndexError::DiskANN(e)))?;
        
        Err(StorageError::VectorIndex(VectorIndexError::InvalidInput(
            format!("Loading not yet implemented. Expected {} points from {}", params.expected_num_points, path)
        )))
    }
}