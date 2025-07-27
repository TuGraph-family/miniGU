use crate::error::{StorageResult, StorageError, VectorIndexError};
use super::index::VectorIndex;
use diskann::{
    index::{create_inmem_index, ANNInmemIndex},
    model::IndexConfiguration,
};
use dashmap::DashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;

/// Index statistics and performance metrics
#[derive(Debug, Clone, Default)]
pub struct IndexStats {
    pub vector_count: usize,
    pub memory_usage: usize,
    pub build_time_ms: Option<u64>,
    pub avg_search_time_us: Option<f64>,
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
    inner: Box<dyn ANNInmemIndex<f32> + 'static>,
    dimension: usize,

    node_to_vector: DashMap<u32, u32>,
    /// Mapping from vector ID to node_id
    vector_to_node: DashMap<u32, u32>,
    next_vector_id: AtomicU32,                   // Next vector ID to be allocated
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
            node_to_vector: DashMap::new(),
            vector_to_node: DashMap::new(),
            next_vector_id: AtomicU32::new(0),
            stats: std::sync::RwLock::new(IndexStats::new()),
        })
    }
    
    pub fn stats(&self) -> IndexStats {
        self.stats.read().unwrap().clone()
    }

    pub fn mapping_count(&self) -> usize {
        self.node_to_vector.len()
    }

    fn clear_mappings(&mut self) {
        self.node_to_vector.clear();
        self.vector_to_node.clear();
        self.next_vector_id.store(0, Ordering::Relaxed);
        // *self.stats.write().unwrap() = IndexStats::new();
    }
    
}

impl VectorIndex for InMemDiskANNAdapter {
    fn build(&mut self, vectors: &[(u32, Vec<f32>)]) -> StorageResult<()> {
        let start = Instant::now();

        if vectors.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::EmptyDataset));
        }

        self.clear_mappings();

        let mut sorted_vectors = vectors.to_vec();
        sorted_vectors.sort_by_key(|(node_id, _)| *node_id);

        // Validate node IDs and establish ID mappings BEFORE calling DiskANN
        let mut vector_data = Vec::with_capacity(sorted_vectors.len());
        let mut seen_nodes = std::collections::HashSet::new();

        for (array_index, (node_id, vector)) in sorted_vectors.iter().enumerate() {
            // Check for duplicate node IDs (miniGU-specific validation)
            if !seen_nodes.insert(*node_id) {
                self.clear_mappings();
                return Err(StorageError::VectorIndex(VectorIndexError::DuplicateNodeId { node_id: *node_id }));
            }

            // Establish ID mapping - DiskANN will assign vector_id = array_index
            let vector_id = array_index as u32;

            self.node_to_vector.insert(*node_id, vector_id);
            self.vector_to_node.insert(vector_id, *node_id);
            
            vector_data.push(vector.as_slice());
        }

        // Call DiskANN to build the index 
        match self.inner.build_from_memory(&vector_data) {
            Ok(()) => {
                self.next_vector_id.store(sorted_vectors.len() as u32, Ordering::Relaxed);
                
                let build_time = start.elapsed().as_millis() as u64;
                {
                    let mut stats = self.stats.write().unwrap();
                    stats.update_after_build(sorted_vectors.len(), build_time, 0);
                }
                
                Ok(())
            },
            Err(e) => {
                self.clear_mappings();
                Err(StorageError::VectorIndex(VectorIndexError::BuildError(e.to_string())))
            }
        }
    }
    
    fn search(&self, query: &[f32], k: usize, l_value: u32) -> StorageResult<Vec<u32>> {
        // Check if index is built
        if self.vector_to_node.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::IndexNotBuilt));
        }
        
        // Perform DiskANN search
        let effective_k = std::cmp::min(k, self.size());
        if effective_k == 0 {
            return Ok(Vec::new()); // No active vectors
        }
        
        let mut vector_ids = vec![0u32; effective_k];
        let actual_count = self.inner.search(query, effective_k, l_value, &mut vector_ids)
            .map_err(|e| StorageError::VectorIndex(VectorIndexError::SearchError(e.to_string())))?;

        // Filter deleted vectors and convert to node_ids
        let mut node_ids = Vec::with_capacity(actual_count as usize);
        
        for &vector_id in vector_ids.iter().take(actual_count as usize) {
            if let Some(entry) = self.vector_to_node.get(&vector_id) {
                let node_id = *entry;
                // DiskANN-rs already filters deleted vectors in its search method
                // No need for additional filtering here
                node_ids.push(node_id);
            } else {
                // This should not happen if our mapping is consistent
                return Err(StorageError::VectorIndex(VectorIndexError::VectorIdNotFound { vector_id }));
            }
        }

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
        // Return the actual number of active vectors based on our mappings
        // This correctly excludes deleted vectors, unlike get_num_active_pts()
        self.node_to_vector.len()
    }
    
    fn insert(&mut self, vectors: &[(u32, Vec<f32>)]) -> StorageResult<()> {
        if vectors.is_empty() {
            return Ok(());
        }

        if self.node_to_vector.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::IndexNotBuilt));
        }

        // Check for duplicate node IDs
        for (node_id, _) in vectors {
            if self.node_to_vector.contains_key(node_id) {
                return Err(StorageError::VectorIndex(VectorIndexError::DuplicateNodeId { 
                    node_id: *node_id 
                }));
            }
        }

        // atomic ID allocation
        let base_vector_id = self.next_vector_id.fetch_add(
            vectors.len() as u32, 
            Ordering::Relaxed
        );

        let mut inserted_mappings = Vec::new();
        for (array_index, (node_id, _)) in vectors.iter().enumerate() {
            let vector_id = base_vector_id + array_index as u32;

            self.node_to_vector.insert(*node_id, vector_id);
            self.vector_to_node.insert(vector_id, *node_id);
            
            // Track for potential rollback
            inserted_mappings.push((*node_id, vector_id));
        }

        let vector_data: Vec<&[f32]> = vectors.iter()
            .map(|(_, vector)| vector.as_slice())
            .collect();

        // Call DiskANN insert
        match self.inner.insert_from_memory(&vector_data) {
            Ok(()) => {
                Ok(())
            },
            Err(e) => {
                for (node_id, vector_id) in inserted_mappings {
                    self.node_to_vector.remove(&node_id);
                    self.vector_to_node.remove(&vector_id);
                }

                self.next_vector_id.fetch_sub(vectors.len() as u32, Ordering::Relaxed);
                
                Err(StorageError::VectorIndex(VectorIndexError::BuildError(e.to_string())))
            }
        }
    }
    
    fn soft_delete(&mut self, node_ids: &[u32]) -> StorageResult<()> {
        if node_ids.is_empty() {
            return Ok(());
        }

        if self.node_to_vector.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::IndexNotBuilt));
        }

        // Validate all node_ids exist and collect vector_ids to delete
        let mut vector_ids_to_delete = Vec::with_capacity(node_ids.len());
        for &node_id in node_ids {
            if let Some(vector_id) = self.node_to_vector.get(&node_id) {
                // Check if mapping exists in vector_to_node (should always exist if node_to_vector exists)
                if self.vector_to_node.contains_key(&*vector_id) {
                    vector_ids_to_delete.push(*vector_id);
                } else {
                    return Err(StorageError::VectorIndex(VectorIndexError::NodeIdNotFound { node_id }));
                }
            } else {
                return Err(StorageError::VectorIndex(VectorIndexError::NodeIdNotFound { node_id }));
            }
        }

        // Call DiskANN soft deletion
        match self.inner.soft_delete(vector_ids_to_delete.clone(), vector_ids_to_delete.len()) {
            Ok(()) => {
                // DiskANN soft deletion successful, now clean up our mappings
                for &node_id in node_ids {
                    if let Some((_, vector_id)) = self.node_to_vector.remove(&node_id) {
                        // Remove both directions of the mapping
                        self.vector_to_node.remove(&vector_id);
                    }
                }
            },
            Err(e) => {
                // DiskANN soft deletion failed, don't modify our mappings
                return Err(StorageError::VectorIndex(VectorIndexError::DiskANN(e)));
            }
        }
        
        Ok(())
    }
    
    fn save(&mut self, path: &str) -> StorageResult<()> {
        self.inner.save(path)
            .map_err(|e| StorageError::VectorIndex(VectorIndexError::DiskANN(e)))?;
        Ok(())
    }
    
    fn load(&mut self, path: &str, expected_num_points: usize) -> StorageResult<()> {
        // For now, loading requires recreating the index
        // In the future, we could implement proper loading by:
        // 1. Loading the index from file
        // 2. Reconstructing the ID mappings from metadata
        
        // TODO: Implement proper loading when diskann-rs supports it
        // self.inner.load(path, expected_num_points)
        //     .map_err(|e| StorageError::VectorIndex(VectorIndexError::DiskANN(e)))?;
        
        Err(StorageError::VectorIndex(VectorIndexError::InvalidInput(
            format!("Loading not yet implemented. Expected {} points from {}", expected_num_points, path)
        )))
    }
}