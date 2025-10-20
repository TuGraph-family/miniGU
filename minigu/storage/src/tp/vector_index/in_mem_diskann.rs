use std::collections::BinaryHeap;
use std::sync::atomic::{AtomicU32, Ordering};

use dashmap::DashMap;
use diskann::common::{AlignedBoxWithSlice, FilterIndex as DiskANNFilterMask};
use diskann::index::{ANNInmemIndex, create_inmem_index};
use diskann::model::IndexConfiguration;
use diskann::model::configuration::index_write_parameters::IndexWriteParametersBuilder;
use diskann::model::vertex::{DIM_104, DIM_128, DIM_256};
use ordered_float::OrderedFloat;
use parking_lot::RwLock;
use vector::{Metric, distance_l2_vector_f32};

use super::filter::{FilterMask, SELECTIVITY_THRESHOLD};
use super::index::VectorIndex;
use crate::error::{StorageError, StorageResult, VectorIndexError};

/// Sharded vector-to-node mapping
pub struct ShardedVectorMap {
    shards: Vec<RwLock<Vec<Option<u64>>>>,
    shard_bits: u32, // log2(shard_count)
}

impl ShardedVectorMap {
    /// Create a new sharded vector map with bit-striping sharding
    /// shard_count_log2: log2 of shard count
    pub fn new(shard_count_log2: u32) -> StorageResult<Self> {
        if shard_count_log2 > 16 {
            return Err(StorageError::VectorIndex(
                VectorIndexError::UnsupportedOperation(
                    "Shard count too large (max 2^16 = 65536 shards)".to_string(),
                ),
            ));
        }

        let shard_count = 1usize << shard_count_log2;
        let shards = (0..shard_count).map(|_| RwLock::new(Vec::new())).collect();
        Ok(Self {
            shards,
            shard_bits: shard_count_log2,
        })
    }

    /// Get the shard and local index for a given vector_id using bit-striping sharding
    #[inline]
    fn get_shard_and_index(&self, vector_id: u32) -> (usize, usize) {
        let shard_mask = (1u32 << self.shard_bits) - 1;
        let shard_idx = (vector_id & shard_mask) as usize;
        let local_idx = (vector_id >> self.shard_bits) as usize;
        (shard_idx, local_idx)
    }

    /// Get node_id for a given vector_id
    pub fn get(&self, vector_id: u32) -> Option<u64> {
        let (shard_idx, local_idx) = self.get_shard_and_index(vector_id);
        if shard_idx >= self.shards.len() {
            return None;
        }
        let shard = &self.shards[shard_idx];
        let vec = shard.read();
        vec.get(local_idx).and_then(|opt| *opt)
    }

    /// Set node_id for a given vector_id (auto-expanding)
    pub fn set(&self, vector_id: u32, node_id: u64) -> StorageResult<()> {
        let (shard_idx, local_idx) = self.get_shard_and_index(vector_id);
        if shard_idx >= self.shards.len() {
            return Err(StorageError::VectorIndex(
                VectorIndexError::UnsupportedOperation(format!(
                    "Vector ID {} maps to invalid shard {} (max: {})",
                    vector_id,
                    shard_idx,
                    self.shards.len() - 1
                )),
            ));
        }
        let shard = &self.shards[shard_idx];
        let mut vec = shard.write();

        // Expand the vector if necessary
        if local_idx >= vec.len() {
            vec.resize(local_idx + 1, None); // None for unset values
        }
        vec[local_idx] = Some(node_id);
        Ok(())
    }

    /// Get total number of shards
    pub fn shard_count(&self) -> usize {
        self.shards.len()
    }

    /// Clear all mappings across all shards
    pub fn clear(&self) {
        for shard in &self.shards {
            let mut vec = shard.write();
            vec.clear();
        }
    }

    /// Check if all shards are empty
    pub fn is_empty(&self) -> bool {
        for shard in &self.shards {
            let vec = shard.read();
            if !vec.is_empty() {
                return false;
            }
        }
        true
    }

    /// Check if a vector_id has a mapping
    pub fn contains_key(&self, vector_id: u32) -> bool {
        let (shard_idx, local_idx) = self.get_shard_and_index(vector_id);
        if shard_idx >= self.shards.len() {
            return false;
        }

        let shard = &self.shards[shard_idx];
        let vec = shard.read();
        local_idx < vec.len() && vec[local_idx].is_some()
    }

    /// Remove a mapping (used only for error rollback, NOT for soft delete)
    pub fn remove(&self, vector_id: u32) -> Option<u64> {
        let (shard_idx, local_idx) = self.get_shard_and_index(vector_id);
        if shard_idx >= self.shards.len() {
            return None;
        }
        let shard = &self.shards[shard_idx];
        let mut vec = shard.write();

        if local_idx < vec.len() {
            vec[local_idx].take()
        } else {
            None
        }
    }

    /// Soft delete a mapping: sets to None
    pub fn soft_delete(&self, vector_id: u32) -> StorageResult<Option<u64>> {
        let (shard_idx, local_idx) = self.get_shard_and_index(vector_id);
        if shard_idx >= self.shards.len() {
            return Ok(None);
        }
        let shard = &self.shards[shard_idx];
        let mut vec = shard.write();

        if local_idx < vec.len() {
            Ok(vec[local_idx].take())
        } else {
            Ok(None)
        }
    }

    /// Batch soft delete for multiple vector_ids
    pub fn batch_soft_delete(&self, vector_ids: &[u32]) -> StorageResult<Vec<u64>> {
        use std::collections::HashMap;

        // Group by shard to minimize lock acquisitions
        let mut shard_groups: HashMap<usize, Vec<(usize, u32)>> = HashMap::new();

        for &vector_id in vector_ids {
            let (shard_idx, local_idx) = self.get_shard_and_index(vector_id);
            if shard_idx < self.shards.len() {
                shard_groups
                    .entry(shard_idx)
                    .or_default()
                    .push((local_idx, vector_id));
            }
        }

        let mut deleted_nodes = Vec::new();
        for (shard_idx, indices) in shard_groups {
            let shard = &self.shards[shard_idx];
            let mut vec = shard.write();

            for (local_idx, _vector_id) in indices {
                if local_idx < vec.len() {
                    if let Some(node_id) = vec[local_idx].take() {
                        deleted_nodes.push(node_id);
                    }
                }
            }
        }

        Ok(deleted_nodes)
    }
}

/// Aligned query buffer that maintains 64-byte alignment guarantee
enum AlignedQueryBuffer<'a> {
    Borrowed(&'a [f32]),
    Owned(AlignedBoxWithSlice<f32>),
}

impl AlignedQueryBuffer<'_> {
    fn as_slice(&self) -> &[f32] {
        match self {
            Self::Borrowed(slice) => slice,
            Self::Owned(aligned) => aligned.as_slice(),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
pub struct InMemANNAdapter {
    inner: Box<dyn ANNInmemIndex<f32> + 'static>,
    dimension: usize,

    node_to_vector: DashMap<u64, u32>,
    vector_to_node: ShardedVectorMap,
    next_vector_id: AtomicU32, // Next vector ID to be allocated
}

impl InMemANNAdapter {
    pub fn new(config: IndexConfiguration) -> StorageResult<Self> {
        // Validate distance metric type: only L2 distance is supported
        if config.dist_metric != Metric::L2 {
            return Err(StorageError::VectorIndex(
                VectorIndexError::UnsupportedOperation(format!(
                    "Unsupported metric type: {:?}. Only L2 distance is supported.",
                    config.dist_metric
                )),
            ));
        }

        let dimension = config.dim;
        let inner = create_inmem_index::<f32>(config)
            .map_err(|e| StorageError::VectorIndex(VectorIndexError::DiskANN(e)))?;

        // Configure sharded vector map for optimal concurrent performance
        const SHARD_BITS: u32 = 4; // 2^4 = 16 shards for parallelism

        Ok(Self {
            inner,
            dimension, // raw dimension not aligned
            node_to_vector: DashMap::new(),
            vector_to_node: ShardedVectorMap::new(SHARD_BITS)?,
            next_vector_id: AtomicU32::new(0),
        })
    }

    pub fn mapping_count(&self) -> usize {
        self.node_to_vector.len()
    }

    // Private implementation methods for InMemANNAdapter
    fn clear_mappings(&mut self) {
        self.node_to_vector.clear();
        self.vector_to_node.clear();
        self.next_vector_id.store(0, Ordering::Relaxed);
    }

    /// Create aligned query vector for optimal SIMD performance.
    /// DiskANN uses AVX-512 SIMD instructions, which require 64-byte alignment for optimal
    /// performance and to avoid undefined behavior. Uses diskann-rs AlignedBoxWithSlice for
    /// memory-safe 64-byte alignment.
    /// Returns AlignedQueryBuffer to maintain alignment guarantee.
    fn ensure_query_aligned(query: &[f32]) -> StorageResult<AlignedQueryBuffer<'_>> {
        if query.as_ptr().align_offset(64) == 0 {
            Ok(AlignedQueryBuffer::Borrowed(query))
        } else {
            let mut aligned = AlignedBoxWithSlice::<f32>::new(query.len(), 64)
                .map_err(|e| StorageError::VectorIndex(VectorIndexError::DiskANN(e)))?;
            aligned.as_mut_slice().copy_from_slice(query);
            Ok(AlignedQueryBuffer::Owned(aligned))
        }
    }

    /// Brute force search with SIMD-optimized distance computation
    /// Direct iteration over candidate vectors for optimal low selectivity performance
    fn brute_force_search(
        &self,
        query: &[f32],
        k: usize,
        filter_mask: &FilterMask,
    ) -> StorageResult<Vec<(u64, f32)>> {
        if k == 0 {
            return Ok(Vec::new());
        }

        // Ensure query vector is 64-byte aligned for SIMD requirements
        let aligned_query = Self::ensure_query_aligned(query)?;

        let mut heap = BinaryHeap::<(OrderedFloat<f32>, u32)>::with_capacity(k);

        for vector_id in filter_mask.iter_candidates() {
            // Get 64-byte aligned vector data from DiskANN (zero-copy access)
            let stored_vector = self
                .inner
                .get_aligned_vector_data(vector_id)
                .map_err(|e| StorageError::VectorIndex(VectorIndexError::DiskANN(e)))?;
            let distance = Self::compute_l2_distance(aligned_query.as_slice(), stored_vector)?;

            if heap.len() < k {
                heap.push((OrderedFloat(distance), vector_id));
            } else if let Some((max_distance, _)) = heap.peek() {
                if OrderedFloat(distance) < *max_distance {
                    heap.pop();
                    heap.push((OrderedFloat(distance), vector_id));
                }
            }
        }
        let results: Vec<_> = heap.into_sorted_vec();

        let results_with_distances: Vec<(u64, f32)> = results
            .into_iter()
            .filter_map(|(distance, vector_id)| {
                self.vector_to_node
                    .get(vector_id)
                    .map(|node_id| (node_id, distance.0))
            })
            .collect();

        Ok(results_with_distances)
    }

    /// filter search: DiskANN search with FilterMask filtering
    /// Used for larger candidate sets where diskann index search is more efficient
    fn filter_search(
        &self,
        query: &[f32],
        k: usize,
        l_value: u32,
        filter_mask: &FilterMask,
        should_pre: bool,
    ) -> StorageResult<Vec<(u64, f32)>> {
        // Convert miniGU FilterMask to DiskANN FilterMask
        let diskann_filter = filter_mask as &dyn DiskANNFilterMask;
        let filtered_results =
            self.ann_search(query, k, l_value, Some(diskann_filter), should_pre)?;

        Ok(filtered_results)
    }

    /// Compute L2 squared distance between query vector and stored vector
    /// Returns squared distance (without sqrt) for consistency with DiskANN SIMD implementation
    #[inline]
    fn compute_l2_distance(query: &[f32], stored: &[f32]) -> StorageResult<f32> {
        if query.len() != stored.len() {
            return Err(StorageError::VectorIndex(
                VectorIndexError::InvalidDimension {
                    expected: stored.len(),
                    actual: query.len(),
                },
            ));
        }

        let dimension = query.len();

        macro_rules! simd_distance {
            ($const_dim:expr) => {{
                // Verify exact dimension match at runtime
                if query.len() != $const_dim || stored.len() != $const_dim {
                    return Err(StorageError::VectorIndex(
                        VectorIndexError::InvalidDimension {
                            expected: $const_dim,
                            actual: query.len(),
                        },
                    ));
                }

                // Enforce 64-byte alignment for AVX-512 optimizations (matches DiskANN standard)
                if query.as_ptr().align_offset(64) != 0 {
                    return Err(StorageError::VectorIndex(
                        VectorIndexError::UnsupportedOperation(
                            "Query vector not 64-byte aligned for optimal SIMD performance"
                                .to_string(),
                        ),
                    ));
                }
                if stored.as_ptr().align_offset(64) != 0 {
                    return Err(StorageError::VectorIndex(
                        VectorIndexError::UnsupportedOperation(
                            "Stored vector not 64-byte aligned for optimal SIMD performance"
                                .to_string(),
                        ),
                    ));
                }

                // Safety: Verified exact length and 64-byte alignment
                unsafe {
                    let query_array = &*(query.as_ptr() as *const [f32; $const_dim]);
                    let stored_array = &*(stored.as_ptr() as *const [f32; $const_dim]);
                    distance_l2_vector_f32::<$const_dim>(query_array, stored_array)
                }
            }};
        }

        let distance = match dimension {
            DIM_104 => simd_distance!(DIM_104),
            DIM_128 => simd_distance!(DIM_128),
            DIM_256 => simd_distance!(DIM_256),
            _ => {
                return Err(StorageError::VectorIndex(
                    VectorIndexError::InvalidDimension {
                        expected: stored.len(),
                        actual: dimension,
                    },
                ));
            }
        };

        Ok(distance)
    }
}

impl VectorIndex for InMemANNAdapter {
    fn build(&mut self, vectors: &[(u64, &[f32])]) -> StorageResult<()> {
        if vectors.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::EmptyDataset));
        }

        self.clear_mappings();

        // Verify dimension consistency with index configuration
        // Note: Upper layer should ensure all vectors have consistent dimensions
        if let Some((_, first_vector)) = vectors.first() {
            if first_vector.len() != self.dimension {
                return Err(StorageError::VectorIndex(
                    VectorIndexError::InvalidDimension {
                        expected: self.dimension,
                        actual: first_vector.len(),
                    },
                ));
            }
        }

        let mut sorted_vectors: Vec<(u64, &[f32])> = vectors.to_vec();
        sorted_vectors.sort_by_key(|(node_id, _)| *node_id);

        // Basic boundary check: ensure vector count fits in u32 for DiskANN compatibility
        if sorted_vectors.len() > u32::MAX as usize {
            self.clear_mappings();
            return Err(StorageError::VectorIndex(
                VectorIndexError::UnsupportedOperation(format!(
                    "Vector count {} exceeds u32::MAX limit for DiskANN",
                    sorted_vectors.len()
                )),
            ));
        }

        // Note: Removed max_points capacity check to rely on DiskANN's internal capacity management
        //
        // DiskANN Capacity Management:
        // - growth_potential is a PRE-ALLOCATION multiplier, not dynamic expansion
        // - Physical capacity = max_points × growth_potential (set at initialization)
        // - Once physical capacity is reached, no more vectors can be inserted
        // - This is the correct behavior - DiskANN has fixed pre-allocated memory

        // Validate node IDs and establish ID mappings BEFORE calling DiskANN
        let mut seen_nodes = std::collections::HashSet::new();

        for (array_index, (node_id, _)) in sorted_vectors.iter().enumerate() {
            if !seen_nodes.insert(*node_id) {
                self.clear_mappings();
                return Err(StorageError::VectorIndex(
                    VectorIndexError::DuplicateNodeId { node_id: *node_id },
                ));
            }
            // Establish ID mapping - DiskANN will assign vector_id = array_index
            let vector_id = array_index as u32;
            self.node_to_vector.insert(*node_id, vector_id);
            if let Err(e) = self.vector_to_node.set(vector_id, *node_id) {
                self.clear_mappings();
                return Err(e);
            }
        }

        // Extract vector slices directly (no conversion needed)
        let vector_slices: Vec<&[f32]> = sorted_vectors.iter().map(|(_, v)| *v).collect();

        match self.inner.build_from_memory(&vector_slices) {
            Ok(()) => {
                self.next_vector_id
                    .store(sorted_vectors.len() as u32, Ordering::Relaxed);

                Ok(())
            }
            Err(e) => {
                self.clear_mappings();
                Err(StorageError::VectorIndex(VectorIndexError::BuildError(
                    e.to_string(),
                )))
            }
        }
    }

    fn ann_search(
        &self,
        query: &[f32],
        k: usize,
        l_value: u32,
        filter_mask: Option<&dyn DiskANNFilterMask>,
        should_pre: bool,
    ) -> StorageResult<Vec<(u64, f32)>> {
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
        let mut distances = vec![0.0f32; effective_k];
        let actual_count = self
            .inner
            .search(
                query,
                effective_k,
                l_value,
                &mut vector_ids,
                &mut distances,
                filter_mask,
                should_pre,
            )
            .map_err(|e| StorageError::VectorIndex(VectorIndexError::SearchError(e.to_string())))?;
        let mut results = Vec::with_capacity(actual_count as usize);
        for (&vector_id, &distance) in vector_ids
            .iter()
            .zip(distances.iter())
            .take(actual_count as usize)
        {
            if let Some(node_id) = self.vector_to_node.get(vector_id) {
                // Verify the node is still active (not soft-deleted)
                if self.node_to_vector.contains_key(&node_id) {
                    results.push((node_id, distance));
                }
            }
        }

        Ok(results)
    }

    fn search(
        &self,
        query: &[f32],
        k: usize,
        l_value: u32,
        filter_mask: Option<&FilterMask>,
        should_pre: bool,
    ) -> StorageResult<Vec<(u64, f32)>> {
        // No filter provided, DiskANN search without filter
        let Some(mask) = filter_mask else {
            return self.ann_search(query, k, l_value, None, should_pre);
        };

        if self.vector_to_node.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::IndexNotBuilt));
        }
        if mask.candidate_count() == 0 {
            return Ok(Vec::new());
        }

        let selectivity = mask.selectivity();
        if selectivity < SELECTIVITY_THRESHOLD {
            self.brute_force_search(query, k, mask)
        } else {
            self.filter_search(query, k, l_value, mask, should_pre)
        }
    }

    fn get_dimension(&self) -> usize {
        self.dimension
    }

    fn size(&self) -> usize {
        // Return the actual number of active vectors based on our mappings
        // This correctly excludes deleted vectors, unlike get_num_active_pts()
        self.node_to_vector.len()
    }

    fn node_to_vector_id(&self, node_id: u64) -> Option<u32> {
        self.node_to_vector.get(&node_id).map(|entry| *entry)
    }

    fn insert(&mut self, vectors: &[(u64, &[f32])]) -> StorageResult<()> {
        if vectors.is_empty() {
            return Ok(());
        }
        if self.node_to_vector.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::IndexNotBuilt));
        }

        // Verify dimension consistency with index configuration
        // Note: Upper layer should ensure all vectors have consistent dimensions
        for (_, vector) in vectors.iter() {
            if vector.len() != self.dimension {
                return Err(StorageError::VectorIndex(
                    VectorIndexError::InvalidDimension {
                        expected: self.dimension,
                        actual: vector.len(),
                    },
                ));
            }
        }

        // Check for duplicate node IDs
        for (node_id, _) in vectors {
            if self.node_to_vector.contains_key(node_id) {
                return Err(StorageError::VectorIndex(
                    VectorIndexError::DuplicateNodeId { node_id: *node_id },
                ));
            }
        }

        // Note: Removed capacity check to rely on DiskANN's internal capacity management
        //
        // DiskANN Insert Capacity:
        // - Uses the same pre-allocated memory pool as build()
        // - Physical capacity = max_points × growth_potential (fixed at initialization)
        // - DiskANN will return error if insertion would exceed pre-allocated capacity
        // - This is expected behavior for memory-based indices with fixed allocation

        // Safe atomic ID allocation (max_points ≤ u32::MAX guaranteed by build())
        let base_vector_id = self
            .next_vector_id
            .fetch_add(vectors.len() as u32, Ordering::Relaxed);

        let mut inserted_mappings = Vec::new();
        for (array_index, (node_id, _)) in vectors.iter().enumerate() {
            let vector_id = base_vector_id + array_index as u32;

            self.node_to_vector.insert(*node_id, vector_id);
            if let Err(e) = self.vector_to_node.set(vector_id, *node_id) {
                // Rollback already inserted mappings
                for (prev_node_id, prev_vector_id) in inserted_mappings {
                    self.node_to_vector.remove(&prev_node_id);
                    self.vector_to_node.remove(prev_vector_id);
                }
                self.next_vector_id
                    .fetch_sub((array_index + 1) as u32, Ordering::Relaxed);
                return Err(e);
            }

            // Track for potential rollback
            inserted_mappings.push((*node_id, vector_id));
        }

        // Extract vector slices directly (no conversion needed)
        let vector_data: Vec<&[f32]> = vectors.iter().map(|(_, v)| *v).collect();

        match self.inner.insert_from_memory(&vector_data) {
            Ok(()) => Ok(()),
            Err(e) => {
                for (node_id, vector_id) in inserted_mappings {
                    self.node_to_vector.remove(&node_id);
                    self.vector_to_node.remove(vector_id);
                }

                self.next_vector_id
                    .fetch_sub(vectors.len() as u32, Ordering::Relaxed);

                Err(StorageError::VectorIndex(VectorIndexError::BuildError(
                    e.to_string(),
                )))
            }
        }
    }

    fn soft_delete(&mut self, node_ids: &[u64]) -> StorageResult<()> {
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
                // Check if mapping exists in vector_to_node (should always exist if node_to_vector
                // exists)
                if self.vector_to_node.contains_key(*vector_id) {
                    vector_ids_to_delete.push(*vector_id);
                } else {
                    return Err(StorageError::VectorIndex(
                        VectorIndexError::NodeIdNotFound { node_id },
                    ));
                }
            } else {
                return Err(StorageError::VectorIndex(
                    VectorIndexError::NodeIdNotFound { node_id },
                ));
            }
        }

        match self
            .inner
            .soft_delete(vector_ids_to_delete.clone(), vector_ids_to_delete.len())
        {
            Ok(()) => {
                // DiskANN soft deletion successful
                // Update both mappings to maintain consistency
                let deleted_nodes = self
                    .vector_to_node
                    .batch_soft_delete(&vector_ids_to_delete)?;

                // Remove from node_to_vector mapping
                for node_id in deleted_nodes {
                    self.node_to_vector.remove(&node_id);
                }
            }
            Err(e) => {
                return Err(StorageError::VectorIndex(VectorIndexError::DiskANN(e)));
            }
        }

        Ok(())
    }

    fn save(&mut self, _path: &str) -> StorageResult<()> {
        Err(StorageError::VectorIndex(VectorIndexError::NotSupported(
            "save() is not yet implemented".to_string(),
        )))
    }

    fn load(&mut self, _path: &str) -> StorageResult<()> {
        Err(StorageError::VectorIndex(VectorIndexError::NotSupported(
            "load() is not yet implemented for InMemANNAdapter".to_string(),
        )))
    }
}

/// Create a vector index configuration with intelligent capacity management
///
/// This function calculates optimal DiskANN configuration parameters based on the actual
/// dataset size, using a headroom ratio to provide growth capacity while maintaining
/// efficiency.
pub fn create_vector_index_config(dimension: usize, vector_count: usize) -> IndexConfiguration {
    let write_params = IndexWriteParametersBuilder::new(100, 64)
        .with_alpha(1.2)
        .with_num_threads(1)
        .build();

    // Set max_points to actual vector count
    let calculated_max_points = vector_count.min(u32::MAX as usize);

    IndexConfiguration {
        index_write_parameter: write_params,
        dist_metric: Metric::L2,
        dim: dimension,
        aligned_dim: dimension,
        max_points: calculated_max_points,
        num_frozen_pts: 0,
        use_pq_dist: false,
        num_pq_chunks: 0,
        use_opq: false,
        growth_potential: 2.0, // Pre-allocation capacity in InmemDataset of DiskANN to insert
    }
}

#[cfg(test)]
mod sharded_vector_map_tests {
    use super::*;

    #[test]
    fn test_basic_operations_and_consistency() -> StorageResult<()> {
        let map = ShardedVectorMap::new(4)?; // 2^4 = 16 shards  
        assert_eq!(map.shard_count(), 16);

        // Test basic set and get operations
        map.set(0, 100)?;
        map.set(15, 115)?;

        assert_eq!(map.get(0), Some(100));
        assert_eq!(map.get(15), Some(115));
        assert_eq!(map.get(999), None); // Non-existent

        // Test consistency between get() and contains_key()
        for i in 16..36u32 {
            // Before setting
            assert_eq!(map.get(i), None);
            assert!(!map.contains_key(i));

            // After setting
            map.set(i, i as u64 + 500)?;
            assert_eq!(map.get(i), Some(i as u64 + 500));
            assert!(map.contains_key(i));
        }

        // Verify initial values still exist
        assert!(map.contains_key(0));
        assert!(map.contains_key(15));
        assert!(!map.contains_key(999));

        Ok(())
    }

    #[test]
    fn test_sharding_and_distribution() -> StorageResult<()> {
        let map = ShardedVectorMap::new(4)?; // 16 shards
        for vector_id in 0..64u32 {
            map.set(vector_id, vector_id as u64 + 1000)?;
        }
        let mut shard_counts = [0; 16];
        for vector_id in 0..64u32 {
            let (shard_idx, _) = map.get_shard_and_index(vector_id);
            shard_counts[shard_idx] += 1;
        }

        // Verify bit-striping sharding spreads data evenly
        let min_count = *shard_counts.iter().min().unwrap();
        let max_count = *shard_counts.iter().max().unwrap();
        let range = max_count - min_count;

        assert!(
            range <= 8,
            "Bit-striping sharding should distribute evenly, range: {}",
            range
        );
        assert!(min_count > 0, "All shards should have data");

        // Test shard boundary conditions with specific vector_ids
        let boundary_map = ShardedVectorMap::new(2)?; // 4 shards, mask = 3
        let boundary_ids = vec![0, 1, 2, 3, 4, 7, 8, 11, 12, 15, 16];

        for &vector_id in &boundary_ids {
            boundary_map.set(vector_id, vector_id as u64 + 2000)?;
        }

        // Verify shard calculation consistency
        for &vector_id in &boundary_ids {
            let (shard_idx, local_idx) = boundary_map.get_shard_and_index(vector_id);
            assert!(shard_idx < boundary_map.shard_count());

            // Verify reverse calculation matches
            let reconstructed = (local_idx << 2) | shard_idx;
            assert_eq!(reconstructed, vector_id as usize);

            assert_eq!(boundary_map.get(vector_id), Some(vector_id as u64 + 2000));
            assert!(boundary_map.contains_key(vector_id));
        }

        Ok(())
    }

    #[test]
    fn test_soft_delete_comprehensive() -> StorageResult<()> {
        let map = ShardedVectorMap::new(3)?; // 8 shards

        // Set up test data distributed across shards
        let test_data = vec![
            (0, 1000),
            (1, 1001),
            (2, 1002),
            (3, 1003),
            (10, 1010),
            (20, 1020),
            (30, 1030),
            (40, 1040),
            (100, 1100),
            (200, 1200),
            (300, 1300),
            (400, 1400),
        ];

        for (vector_id, node_id) in &test_data {
            map.set(*vector_id, *node_id)?;
        }

        // Verify initial state
        for (vector_id, node_id) in &test_data {
            assert_eq!(map.get(*vector_id), Some(*node_id));
            assert!(map.contains_key(*vector_id));
        }

        // Test single soft delete
        let deleted = map.soft_delete(20)?;
        assert_eq!(deleted, Some(1020));
        assert_eq!(map.get(20), None);
        assert!(!map.contains_key(20));

        // Test double deletion (should return None)
        let deleted_again = map.soft_delete(20)?;
        assert_eq!(deleted_again, None);

        // Test batch soft delete on mixed shards
        let delete_ids = vec![1, 30, 200, 400];
        let deleted_nodes = map.batch_soft_delete(&delete_ids)?;

        assert_eq!(deleted_nodes.len(), 4);
        assert!(deleted_nodes.contains(&1001));
        assert!(deleted_nodes.contains(&1030));
        assert!(deleted_nodes.contains(&1200));
        assert!(deleted_nodes.contains(&1400));

        // Verify batch deletion effects
        for &id in &delete_ids {
            assert_eq!(map.get(id), None);
            assert!(!map.contains_key(id));
        }

        // Test batch delete edge cases
        let empty_deleted = map.batch_soft_delete(&[])?;
        assert!(empty_deleted.is_empty());

        let nonexistent_deleted = map.batch_soft_delete(&[999, 888])?;
        assert!(nonexistent_deleted.is_empty());

        // Verify remaining data intact
        let remaining_ids = vec![0, 2, 3, 10, 40, 100, 300];
        for &id in &remaining_ids {
            assert!(map.contains_key(id));
            assert!(map.get(id).is_some());
        }

        Ok(())
    }

    #[test]
    fn test_dynamic_expansion() -> StorageResult<()> {
        let map = ShardedVectorMap::new(2)?; // 4 shards

        // Test setting vector_ids that require expansion across different shards
        // With 2 shard_bits: shard_idx = vector_id & 3, local_idx = vector_id >> 2

        // Small initial value
        map.set(10, 1010)?;

        // Larger values that will expand different shards
        map.set(1000, 2000)?; // shard 0, local_idx 250
        map.set(10001, 20001)?; // shard 1, local_idx 2500  
        map.set(100002, 200002)?; // shard 2, local_idx 25000
        map.set(1000003, 2000003)?; // shard 3, local_idx 250000

        // Verify all values are stored correctly
        assert_eq!(map.get(10), Some(1010));
        assert_eq!(map.get(1000), Some(2000));
        assert_eq!(map.get(10001), Some(20001));
        assert_eq!(map.get(100002), Some(200002));
        assert_eq!(map.get(1000003), Some(2000003));

        // Test intermediate values are None (sparse storage)
        assert_eq!(map.get(500), None);
        assert_eq!(map.get(5000), None);
        assert_eq!(map.get(50000), None);

        Ok(())
    }

    #[test]
    fn test_parameter_validation() {
        // Test boundary conditions for shard_bits parameter
        assert!(ShardedVectorMap::new(17).is_err()); // > 16 should fail
        assert!(ShardedVectorMap::new(0).is_ok()); // min valid value
        assert!(ShardedVectorMap::new(16).is_ok()); // max valid value

        // Verify shard count calculation: 2^shard_bits
        assert_eq!(ShardedVectorMap::new(2).unwrap().shard_count(), 4);
        assert_eq!(ShardedVectorMap::new(0).unwrap().shard_count(), 1);
    }

    #[test]
    fn test_empty_map_behavior() -> StorageResult<()> {
        let map = ShardedVectorMap::new(2)?;

        // Basic empty map properties
        assert!(map.is_empty());
        assert_eq!(map.get(0), None);
        assert!(!map.contains_key(0));

        // All operations on empty map should return None/empty
        assert_eq!(map.soft_delete(0)?, None);
        assert!(map.batch_soft_delete(&[1, 2, 3])?.is_empty());
        assert_eq!(map.remove(0), None);

        Ok(())
    }

    #[test]
    fn test_removal_operations() -> StorageResult<()> {
        let map = ShardedVectorMap::new(2)?;

        // Setup test data
        map.set(10, 110)?;
        map.set(20, 120)?;

        // Test remove (used for rollback)
        assert_eq!(map.remove(10), Some(110));
        assert_eq!(map.get(10), None);
        assert_eq!(map.remove(10), None); // Already removed

        // Test clear
        assert!(!map.is_empty());
        map.clear();
        assert!(map.is_empty());
        assert_eq!(map.get(20), None);

        Ok(())
    }
}
