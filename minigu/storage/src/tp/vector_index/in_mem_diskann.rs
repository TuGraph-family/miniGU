use std::collections::BinaryHeap;
use std::sync::atomic::{AtomicU32, Ordering};

use dashmap::DashMap;
use diskann::common::{AlignedBoxWithSlice, FilterIndex as DiskANNFilterMask};
use diskann::index::{ANNInmemIndex, create_inmem_index};
use diskann::model::IndexConfiguration;
use diskann::model::configuration::index_write_parameters::IndexWriteParametersBuilder;
use diskann::model::vertex::{DIM_104, DIM_128, DIM_256};
use ordered_float::OrderedFloat;
use vector::{Metric, distance_l2_vector_f32};

use super::filter::{DenseFilterMask, FilterMask, SELECTIVITY_THRESHOLD};
use super::index::VectorIndex;
use crate::error::{StorageError, StorageResult, VectorIndexError};

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
    vector_to_node: DashMap<u32, u64>,
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

        Ok(Self {
            inner,
            dimension, // raw dimension not aligned
            node_to_vector: DashMap::new(),
            vector_to_node: DashMap::new(),
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
    /// performance and to avoid undefined behavior. Uses DiskANN-rs AlignedBoxWithSlice for
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
        filter_mask: &dyn FilterMask,
    ) -> StorageResult<Vec<u64>> {
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

        let node_ids: Vec<u64> = results
            .into_iter()
            .filter_map(|(_, vector_id)| {
                self.vector_to_node.get(&vector_id).map(|node_id| *node_id)
            })
            .collect();

        Ok(node_ids)
    }

    /// filter search: DiskANN search with FilterMask filtering
    /// Used for larger candidate sets where diskann index search is more efficient
    fn filter_search(
        &self,
        query: &[f32],
        k: usize,
        l_value: u32,
        filter_mask: &dyn FilterMask,
        should_pre: bool,
    ) -> StorageResult<Vec<u64>> {
        // Convert miniGU FilterMask to DiskANN FilterMask for pre-filtering
        let diskann_filter = {
            if let Some(dense) = filter_mask.as_any().downcast_ref::<DenseFilterMask>() {
                dense as &dyn DiskANNFilterMask
            } else {
                return Err(StorageError::VectorIndex(VectorIndexError::FilterError(
                    "Unsupported FilterMask type".to_string(),
                )));
            }
        };
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
    fn build(&mut self, vectors: &[(u64, Vec<f32>)]) -> StorageResult<()> {
        if vectors.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::EmptyDataset));
        }

        self.clear_mappings();

        let mut sorted_vectors = vectors.to_vec();
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
        let mut vector_data = Vec::with_capacity(sorted_vectors.len());
        let mut seen_nodes = std::collections::HashSet::new();

        for (array_index, (node_id, vector)) in sorted_vectors.iter().enumerate() {
            if !seen_nodes.insert(*node_id) {
                self.clear_mappings();
                return Err(StorageError::VectorIndex(
                    VectorIndexError::DuplicateNodeId { node_id: *node_id },
                ));
            }
            // Establish ID mapping - DiskANN will assign vector_id = array_index
            let vector_id = array_index as u32;
            self.node_to_vector.insert(*node_id, vector_id);
            self.vector_to_node.insert(vector_id, *node_id);

            vector_data.push(vector.as_slice());
        }

        match self.inner.build_from_memory(&vector_data) {
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
    ) -> StorageResult<Vec<u64>> {
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
        let actual_count = self
            .inner
            .search(
                query,
                effective_k,
                l_value,
                &mut vector_ids,
                filter_mask,
                should_pre,
            )
            .map_err(|e| StorageError::VectorIndex(VectorIndexError::SearchError(e.to_string())))?;
        let mut node_ids = Vec::with_capacity(actual_count as usize);
        for &vector_id in vector_ids.iter().take(actual_count as usize) {
            if let Some(entry) = self.vector_to_node.get(&vector_id) {
                let node_id = *entry;
                // DiskANN-rs already filters deleted vectors in its search method
                node_ids.push(node_id);
            } else {
                return Err(StorageError::VectorIndex(
                    VectorIndexError::VectorIdNotFound { vector_id },
                ));
            }
        }

        Ok(node_ids)
    }

    fn search(
        &self,
        query: &[f32],
        k: usize,
        l_value: u32,
        filter_mask: Option<&dyn FilterMask>,
        should_pre: bool,
    ) -> StorageResult<Vec<u64>> {
        // No filter provided, DiskANN search without filter
        let Some(mask) = filter_mask else {
            return self.ann_search(query, k, l_value, None, false);
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

    fn insert(&mut self, vectors: &[(u64, Vec<f32>)]) -> StorageResult<()> {
        if vectors.is_empty() {
            return Ok(());
        }
        if self.node_to_vector.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::IndexNotBuilt));
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
            self.vector_to_node.insert(vector_id, *node_id);

            // Track for potential rollback
            inserted_mappings.push((*node_id, vector_id));
        }

        let vector_data: Vec<&[f32]> = vectors
            .iter()
            .map(|(_, vector)| vector.as_slice())
            .collect();

        match self.inner.insert_from_memory(&vector_data) {
            Ok(()) => Ok(()),
            Err(e) => {
                for (node_id, vector_id) in inserted_mappings {
                    self.node_to_vector.remove(&node_id);
                    self.vector_to_node.remove(&vector_id);
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
                if self.vector_to_node.contains_key(&*vector_id) {
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
                // DiskANN soft deletion successful, now clean up our mappings
                for &node_id in node_ids {
                    if let Some((_, vector_id)) = self.node_to_vector.remove(&node_id) {
                        // Remove both directions of the mapping
                        self.vector_to_node.remove(&vector_id);
                    }
                }
            }
            Err(e) => {
                // DiskANN soft deletion failed, don't modify our mappings
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
        aligned_dim: dimension, // keeps the raw dimension. No round_up(dim, 8)
        max_points: calculated_max_points,
        num_frozen_pts: 0,
        use_pq_dist: false,
        num_pq_chunks: 0,
        use_opq: false,
        growth_potential: 2.0, // Pre-allocation capacity in InmemDataset of DiskANN to insert
    }
}
