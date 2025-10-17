// Copyright (c) Microsoft Corporation.  All rights reserved.
// Copyright (c) 2025 MiniGU. All rights reserved.
//
// Licensed under the MIT License. See diskann-rs/LICENSE for license information.
//
// Modifications for miniGU integration:
// - Added memory-based index building (build_from_memory, insert_from_memory)
// - Enhanced search function: added distances output, pre-filter and post-filter support
//   * Pre-filter: filter during graph traversal (via search_with_l_override)
//   * Post-filter: filter in result candidates before returning top-K
// - Added get_aligned_vector_data for zero-copy vector access

use std::cmp;
use std::sync::RwLock;
use std::time::Duration;

use hashbrown::HashSet;
use hashbrown::hash_set::Entry::*;
use vector::FullPrecisionDistance;

use crate::common::{ANNError, ANNResult, FilterIndex};
use crate::index::ANNInmemIndex;
use crate::model::graph::AdjacencyList;
use crate::model::{
    ArcConcurrentBoxedQueue, InMemQueryScratch, InMemoryGraph, IndexConfiguration, InmemDataset,
    Neighbor, ScratchStoreManager, Vertex,
};
use crate::utils::rayon_util::execute_with_rayon;
use crate::utils::{Timer, set_rayon_num_threads};

/// In-memory Index
pub struct InmemIndex<T, const N: usize>
where
    [T; N]: FullPrecisionDistance<T, N>,
{
    /// Dataset
    pub dataset: InmemDataset<T, N>,

    /// Graph
    pub final_graph: InMemoryGraph,

    /// Index configuration
    pub configuration: IndexConfiguration,

    /// Start point of the search. When _num_frozen_pts is greater than zero,
    /// this is the location of the first frozen point. Otherwise, this is a
    /// location of one of the points in index.
    pub start: u32,

    /// Max observed out degree
    pub max_observed_degree: u32,

    /// Number of active points i.e. existing in the graph
    pub num_active_pts: usize,

    /// query scratch queue.
    query_scratch_queue: ArcConcurrentBoxedQueue<InMemQueryScratch<T, N>>,

    pub delete_set: RwLock<HashSet<u32>>,
}

impl<T, const N: usize> InmemIndex<T, N>
where
    T: Default + Copy + Sync + Send + Into<f32>,
    [T; N]: FullPrecisionDistance<T, N>,
{
    /// Create Index obj based on configuration
    pub fn new(mut config: IndexConfiguration) -> ANNResult<Self> {
        // Sanity check. While logically it is correct, max_points = 0 causes
        // downstream problems.
        if config.max_points == 0 {
            config.max_points = 1;
        }

        let total_internal_points = config.max_points + config.num_frozen_pts;

        if config.use_pq_dist {
            // TODO: pq
            todo!("PQ is not supported now");
        }

        let start = config.max_points.try_into()?;

        let query_scratch_queue = ArcConcurrentBoxedQueue::<InMemQueryScratch<T, N>>::new();
        let delete_set = RwLock::new(HashSet::<u32>::new());

        Ok(Self {
            dataset: InmemDataset::<T, N>::new(total_internal_points, config.growth_potential)?,
            final_graph: InMemoryGraph::new(
                total_internal_points,
                config.index_write_parameter.max_degree,
            ),
            configuration: config,
            start,
            max_observed_degree: 0,
            num_active_pts: 0,
            query_scratch_queue,
            delete_set,
        })
    }

    /// Get distance between two vertices.
    pub fn get_distance(&self, id1: u32, id2: u32) -> ANNResult<f32> {
        self.dataset
            .get_distance(id1, id2, self.configuration.dist_metric)
    }

    fn build_with_data_populated(&mut self) -> ANNResult<()> {
        println!(
            "Starting index build with {} points...",
            self.num_active_pts
        );

        if self.num_active_pts < 1 {
            return Err(ANNError::log_index_error(
                "Error: Trying to build an index with 0 points.".to_string(),
            ));
        }

        if self.query_scratch_queue.size()? == 0 {
            self.initialize_query_scratch(
                5 + self.configuration.index_write_parameter.num_threads,
                self.configuration.index_write_parameter.search_list_size,
            )?;
        }

        // TODO: generate_frozen_point()

        self.link()?;

        self.print_stats()?;

        Ok(())
    }

    fn link(&mut self) -> ANNResult<()> {
        // visit_order is a vector that is initialized to the entire graph
        let mut visit_order =
            Vec::with_capacity(self.num_active_pts + self.configuration.num_frozen_pts);
        for i in 0..self.num_active_pts {
            visit_order.push(i as u32);
        }

        // If there are any frozen points, add them all.
        for frozen in self.configuration.max_points
            ..(self.configuration.max_points + self.configuration.num_frozen_pts)
        {
            visit_order.push(frozen as u32);
        }

        // if there are frozen points, the first such one is set to be the _start
        if self.configuration.num_frozen_pts > 0 {
            self.start = self.configuration.max_points as u32;
        } else {
            self.start = self.dataset.calculate_medoid_point_id()?;
        }

        let timer = Timer::new();

        let range = visit_order.len();

        execute_with_rayon(
            0..range,
            self.configuration.index_write_parameter.num_threads,
            |idx| {
                self.insert_vertex_id(visit_order[idx])?;

                Ok(())
            },
        )?;

        self.cleanup_graph(&visit_order)?;

        if self.num_active_pts > 0 {
            println!("{}", timer.elapsed_seconds_for_step("Link time: "));
        }

        Ok(())
    }

    fn insert_vertex_id(&self, vertex_id: u32) -> ANNResult<()> {
        let mut scratch_manager =
            ScratchStoreManager::new(self.query_scratch_queue.clone(), Duration::from_millis(10))?;
        let scratch = scratch_manager.scratch_space().ok_or_else(|| {
            ANNError::log_index_error(
                "ScratchStoreManager doesn't have InMemQueryScratch instance available".to_string(),
            )
        })?;

        let new_neighbors = self.search_for_point_and_prune(scratch, vertex_id)?;
        self.update_vertex_with_neighbors(vertex_id, new_neighbors)?;
        self.update_neighbors_of_vertex(vertex_id, scratch)?;

        Ok(())
    }

    fn update_neighbors_of_vertex(
        &self,
        vertex_id: u32,
        scratch: &mut InMemQueryScratch<T, N>,
    ) -> Result<(), ANNError> {
        let vertex = self.final_graph.read_vertex_and_neighbors(vertex_id)?;
        assert!(vertex.size() <= self.configuration.index_write_parameter.max_degree as usize);
        self.inter_insert(
            vertex_id,
            vertex.get_neighbors(),
            self.configuration.index_write_parameter.max_degree,
            scratch,
        )?;
        Ok(())
    }

    fn update_vertex_with_neighbors(
        &self,
        vertex_id: u32,
        new_neighbors: AdjacencyList,
    ) -> Result<(), ANNError> {
        let vertex = &mut self.final_graph.write_vertex_and_neighbors(vertex_id)?;
        vertex.set_neighbors(new_neighbors);
        assert!(vertex.size() <= self.configuration.index_write_parameter.max_degree as usize);
        Ok(())
    }

    fn search_for_point_and_prune(
        &self,
        scratch: &mut InMemQueryScratch<T, N>,
        vertex_id: u32,
    ) -> ANNResult<AdjacencyList> {
        let mut pruned_list =
            AdjacencyList::for_range(self.configuration.index_write_parameter.max_degree as usize);
        let vertex = self.dataset.get_vertex(vertex_id)?;
        let mut visited_nodes = self.search_for_point(&vertex, scratch)?;

        self.prune_neighbors(vertex_id, &mut visited_nodes, &mut pruned_list, scratch)?;

        if pruned_list.is_empty() {
            return Err(ANNError::log_index_error(
                "pruned_list is empty.".to_string(),
            ));
        }

        if self.final_graph.size()
            != self.configuration.max_points + self.configuration.num_frozen_pts
        {
            return Err(ANNError::log_index_error(format!(
                "final_graph has {} vertices instead of {}",
                self.final_graph.size(),
                self.configuration.max_points + self.configuration.num_frozen_pts,
            )));
        }

        Ok(pruned_list)
    }

    #[allow(clippy::too_many_arguments)]
    fn search(
        &self,
        query: &Vertex<T, N>,
        k_value: usize,
        l_value: u32,
        indices: &mut [u32],
        distances: &mut [f32],
        filter_mask: Option<&dyn FilterIndex>,
        should_pre: bool,
    ) -> ANNResult<u32> {
        if k_value > l_value as usize {
            return Err(ANNError::log_index_error(format!(
                "Set L: {l_value} to a value of at least K: {k_value}"
            )));
        }

        let mut scratch_manager =
            ScratchStoreManager::new(self.query_scratch_queue.clone(), Duration::from_millis(10))?;

        let scratch = scratch_manager.scratch_space().ok_or_else(|| {
            ANNError::log_index_error(
                "ScratchStoreManager doesn't have InMemQueryScratch instance available".to_string(),
            )
        })?;

        if l_value > scratch.candidate_size {
            println!(
                "Attempting to expand query scratch_space. Was created with Lsize: {} but search L is: {}",
                scratch.candidate_size, l_value
            );
            scratch.resize_for_new_candidate_size(l_value);
            println!(
                "Resize completed. New scratch size is: {}",
                scratch.candidate_size
            );
        }

        let cmp =
            self.search_with_l_override(query, scratch, l_value as usize, filter_mask, should_pre)?;
        let mut pos = 0;

        for i in 0..scratch.best_candidates.size() {
            if scratch.best_candidates[i].id < self.configuration.max_points as u32 {
                // Filter out the deleted points.
                if let Ok(delete_set_guard) = self.delete_set.read() {
                    if !delete_set_guard.contains(&scratch.best_candidates[i].id) {
                        // no filter || post-filter as long as filter_mask is not None
                        if filter_mask.is_none()
                            || filter_mask
                                .as_ref()
                                .is_some_and(|m| m.contains_vector(scratch.best_candidates[i].id))
                        {
                            indices[pos] = scratch.best_candidates[i].id;
                            distances[pos] = scratch.best_candidates[i].distance;
                            pos += 1;
                        }
                    }
                } else {
                    return Err(ANNError::log_lock_poison_error(
                        "failed to acquire the lock for delete_set.".to_string(),
                    ));
                }
            }

            if pos == k_value {
                break;
            }
        }

        if pos < k_value {
            eprintln!("Found fewer than K elements for query! Found: {pos} but K: {k_value}");
        }

        Ok(cmp)
    }

    fn cleanup_graph(&mut self, visit_order: &[u32]) -> ANNResult<()> {
        if self.num_active_pts > 0 {
            println!("Starting final cleanup..");
        }

        execute_with_rayon(
            0..visit_order.len(),
            self.configuration.index_write_parameter.num_threads,
            |idx| {
                let vertex_id = visit_order[idx];
                let num_nbrs = self.get_neighbor_count(vertex_id)?;

                if num_nbrs <= self.configuration.index_write_parameter.max_degree as usize {
                    // Neighbor list is already small enough.
                    return Ok(());
                }

                let mut scratch_manager = ScratchStoreManager::new(
                    self.query_scratch_queue.clone(),
                    Duration::from_millis(10),
                )?;
                let scratch = scratch_manager.scratch_space().ok_or_else(|| {
                    ANNError::log_index_error(
                        "ScratchStoreManager doesn't have InMemQueryScratch instance available"
                            .to_string(),
                    )
                })?;

                let mut dummy_pool = self.get_neighbors_for_vertex(vertex_id)?;

                let mut new_out_neighbors = AdjacencyList::for_range(
                    self.configuration.index_write_parameter.max_degree as usize,
                );
                self.prune_neighbors(vertex_id, &mut dummy_pool, &mut new_out_neighbors, scratch)?;

                self.final_graph
                    .write_vertex_and_neighbors(vertex_id)?
                    .set_neighbors(new_out_neighbors);

                Ok(())
            },
        )
    }

    /// Get the unique neighbors for a vertex.
    ///
    /// This code feels out of place here. This should have nothing to do with whether this
    /// is in memory index?
    /// # Errors
    ///
    /// This function will return an error if we are not able to get the read lock.
    fn get_neighbors_for_vertex(&self, vertex_id: u32) -> ANNResult<Vec<Neighbor>> {
        let binding = self.final_graph.read_vertex_and_neighbors(vertex_id)?;
        let neighbors = binding.get_neighbors();
        let dummy_pool = self.get_unique_neighbors(neighbors, vertex_id)?;

        Ok(dummy_pool)
    }

    /// Returns a vector of unique neighbors for the given vertex, along with their distances.
    ///
    /// # Arguments
    ///
    /// * `neighbors` - A vector of neighbor id index for the given vertex.
    /// * `vertex_id` - The given vertex id.
    ///
    /// # Errors
    ///
    /// Returns an `ANNError` if there is an error retrieving the vertex or one of its neighbors.
    pub fn get_unique_neighbors(
        &self,
        neighbors: &[u32],
        vertex_id: u32,
    ) -> Result<Vec<Neighbor>, ANNError> {
        let vertex = self.dataset.get_vertex(vertex_id)?;

        let len = neighbors.len();
        if len == 0 {
            return Ok(Vec::new());
        }

        self.dataset.prefetch_vector(neighbors[0]);

        let mut dummy_visited: HashSet<u32> = HashSet::with_capacity(len);
        let mut dummy_pool: Vec<Neighbor> = Vec::with_capacity(len);

        // let slice = ['w', 'i', 'n', 'd', 'o', 'w', 's'];
        // for window in slice.windows(2) {
        //   &println!{"[{}, {}]", window[0], window[1]};
        // }
        // prints: [w, i] -> [i, n] -> [n, d] -> [d, o] -> [o, w] -> [w, s]
        for current in neighbors.windows(2) {
            // Prefetch the next item.
            self.dataset.prefetch_vector(current[1]);
            let current = current[0];

            self.insert_neighbor_if_unique(
                &mut dummy_visited,
                current,
                vertex_id,
                &vertex,
                &mut dummy_pool,
            )?;
        }

        // Insert the last neighbor
        #[allow(clippy::unwrap_used)]
        self.insert_neighbor_if_unique(
            &mut dummy_visited,
            *neighbors.last().unwrap(), // we know len != 0, so this is safe.
            vertex_id,
            &vertex,
            &mut dummy_pool,
        )?;

        Ok(dummy_pool)
    }

    fn insert_neighbor_if_unique(
        &self,
        dummy_visited: &mut HashSet<u32>,
        current: u32,
        vertex_id: u32,
        vertex: &Vertex<'_, T, N>,
        dummy_pool: &mut Vec<Neighbor>,
    ) -> Result<(), ANNError> {
        if current != vertex_id {
            if let Vacant(entry) = dummy_visited.entry(current) {
                let cur_nbr_vertex = self.dataset.get_vertex(current)?;
                let dist = vertex.compare(&cur_nbr_vertex, self.configuration.dist_metric);
                dummy_pool.push(Neighbor::new(current, dist));
                entry.insert();
            }
        }

        Ok(())
    }

    /// Get count of neighbors for a given vertex.
    ///
    /// # Errors
    ///
    /// This function will return an error if we can't get a lock.
    fn get_neighbor_count(&self, vertex_id: u32) -> ANNResult<usize> {
        let num_nbrs = self
            .final_graph
            .read_vertex_and_neighbors(vertex_id)?
            .size();
        Ok(num_nbrs)
    }

    fn soft_delete_vertex(&self, vertex_id_to_delete: u32) -> ANNResult<()> {
        if vertex_id_to_delete as usize > self.num_active_pts {
            return Err(ANNError::log_index_error(format!(
                "vertex_id_to_delete: {} is greater than the number of active points in the graph: {}",
                vertex_id_to_delete, self.num_active_pts
            )));
        }

        let mut delete_set_guard = match self.delete_set.write() {
            Ok(guard) => guard,
            Err(_) => {
                return Err(ANNError::log_index_error(format!(
                    "Failed to acquire delete_set lock, cannot delete vertex {vertex_id_to_delete}"
                )));
            }
        };

        delete_set_guard.insert(vertex_id_to_delete);
        Ok(())
    }

    fn initialize_query_scratch(
        &mut self,
        num_threads: u32,
        search_candidate_size: u32,
    ) -> ANNResult<()> {
        self.query_scratch_queue.reserve(num_threads as usize)?;
        for _ in 0..num_threads {
            let scratch = Box::new(InMemQueryScratch::<T, N>::new(
                search_candidate_size,
                &self.configuration.index_write_parameter,
                false,
            )?);

            self.query_scratch_queue.push(scratch)?;
        }

        Ok(())
    }

    fn print_stats(&mut self) -> ANNResult<()> {
        let mut max = 0;
        let mut min = usize::MAX;
        let mut total = 0;
        let mut cnt = 0;

        for i in 0..self.num_active_pts {
            let vertex_id = i.try_into()?;
            let pool_size = self
                .final_graph
                .read_vertex_and_neighbors(vertex_id)?
                .size();
            max = cmp::max(max, pool_size);
            min = cmp::min(min, pool_size);
            total += pool_size;
            if pool_size < 2 {
                cnt += 1;
            }
        }

        println!(
            "Index built with degree: max: {} avg: {} min: {} count(deg<2): {}",
            max,
            (total as f32) / ((self.num_active_pts + self.configuration.num_frozen_pts) as f32),
            min,
            cnt
        );

        match self.delete_set.read() {
            Ok(guard) => {
                println!(
                    "Number of soft deleted vertices {}, soft deleted percentage: {}",
                    guard.len(),
                    (guard.len() as f32)
                        / ((self.num_active_pts + self.configuration.num_frozen_pts) as f32),
                );
            }
            Err(_) => {
                return Err(ANNError::log_lock_poison_error(
                    "Failed to acquire delete_set lock, cannot get the number of deleted vertices"
                        .to_string(),
                ));
            }
        };

        self.max_observed_degree = cmp::max(max as u32, self.max_observed_degree);

        Ok(())
    }
}

impl<T, const N: usize> ANNInmemIndex<T> for InmemIndex<T, N>
where
    T: Default + Copy + Sync + Send + Into<f32>,
    [T; N]: FullPrecisionDistance<T, N>,
{
    fn search(
        &self,
        query: &[T],
        k_value: usize,
        l_value: u32,
        indices: &mut [u32],
        distances: &mut [f32],
        filter_mask: Option<&dyn FilterIndex>,
        should_pre: bool,
    ) -> ANNResult<u32> {
        let query_vector = Vertex::new(<&[T; N]>::try_from(query)?, 0);
        InmemIndex::search(
            self,
            &query_vector,
            k_value,
            l_value,
            indices,
            distances,
            filter_mask,
            should_pre,
        )
    }

    fn soft_delete(
        &mut self,
        vertex_ids_to_delete: Vec<u32>,
        num_points_to_delete: usize,
    ) -> ANNResult<()> {
        println!("Deleting {num_points_to_delete} vectors from file.");

        let timer = Timer::new();

        execute_with_rayon(
            0..num_points_to_delete,
            self.configuration.index_write_parameter.num_threads,
            |idx: usize| {
                self.soft_delete_vertex(vertex_ids_to_delete[idx])?;

                Ok(())
            },
        )?;

        println!("{}", timer.elapsed_seconds_for_step("Delete time: "));
        self.print_stats()?;

        Ok(())
    }

    // Memory-based interface implementation

    fn build_from_memory(&mut self, vectors: &[&[T]]) -> ANNResult<()> {
        if vectors.is_empty() {
            return Err(ANNError::log_index_error(
                "ERROR: Cannot build index with 0 vectors.".to_string(),
            ));
        }

        let num_points = vectors.len();

        if num_points > self.configuration.max_points {
            return Err(ANNError::log_index_error(format!(
                "ERROR: Cannot load {} vectors, index can support only {} points as specified in configuration.",
                num_points, self.configuration.max_points
            )));
        }

        for (i, vector) in vectors.iter().enumerate() {
            if vector.len() != self.configuration.dim {
                return Err(ANNError::log_index_error(format!(
                    "ERROR: Vector {} has dimension {} but configuration expects {} dimension.",
                    i,
                    vector.len(),
                    self.configuration.dim
                )));
            }
        }

        if self.configuration.use_pq_dist {
            // TODO: PQ
            todo!("PQ is not supported now");
        }

        if self.configuration.index_write_parameter.num_threads > 0 {
            set_rayon_num_threads(self.configuration.index_write_parameter.num_threads);
        }

        // Use dataset's new memory interface
        self.dataset
            .build_from_memory(vectors, num_points, self.configuration.dim)?;

        println!("Using {num_points} vectors from memory.");

        // TODO: tag_lock

        self.num_active_pts = num_points;
        self.build_with_data_populated()?;

        Ok(())
    }

    fn insert_from_memory(&mut self, vectors: &[&[T]]) -> ANNResult<()> {
        if vectors.is_empty() {
            return Ok(()); // Nothing to insert
        }

        let num_points = vectors.len();

        // Validate all vectors have the same dimension as configured
        for (i, vector) in vectors.iter().enumerate() {
            if vector.len() != self.configuration.dim {
                return Err(ANNError::log_index_error(format!(
                    "ERROR: Vector {} has dimension {} but configuration expects {} dimension.",
                    i,
                    vector.len(),
                    self.configuration.dim
                )));
            }
        }

        if self.configuration.use_pq_dist {
            // TODO: PQ
            todo!("PQ is not supported now");
        }

        if self.query_scratch_queue.size()? == 0 {
            self.initialize_query_scratch(
                5 + self.configuration.index_write_parameter.num_threads,
                self.configuration.index_write_parameter.search_list_size,
            )?;
        }

        if self.configuration.index_write_parameter.num_threads > 0 {
            set_rayon_num_threads(self.configuration.index_write_parameter.num_threads);
        }

        // Use dataset's memory append functionality
        self.dataset
            .append_from_memory(vectors, num_points, self.configuration.dim)?;

        self.final_graph.extend(
            num_points,
            self.configuration.index_write_parameter.max_degree,
        );

        // TODO: this should not consider frozen points
        let previous_last_pt = self.num_active_pts;
        self.num_active_pts += num_points;
        self.configuration.max_points += num_points;

        println!("Inserting {num_points} vectors from memory.");

        // TODO: tag_lock
        let timer = Timer::new();
        execute_with_rayon(
            previous_last_pt..self.num_active_pts,
            self.configuration.index_write_parameter.num_threads,
            |idx| {
                self.insert_vertex_id(idx as u32)?;

                Ok(())
            },
        )?;

        let mut visit_order =
            Vec::with_capacity(self.num_active_pts + self.configuration.num_frozen_pts);
        for i in 0..self.num_active_pts {
            visit_order.push(i as u32);
        }

        self.cleanup_graph(&visit_order)?;
        println!(
            "{}",
            timer.elapsed_seconds_for_step("Insert from memory time: ")
        );

        self.print_stats()?;

        Ok(())
    }

    fn get_aligned_vector_data(&self, vector_id: u32) -> ANNResult<&[T]> {
        // Calculate the start and end positions in the aligned dataset
        let start = (vector_id as usize) * N;
        let end = start + N;

        // Validate bounds
        if end > self.dataset.data.len() {
            return Err(ANNError::log_index_error(format!(
                "Invalid vector id {vector_id}."
            )));
        }

        // Return direct slice reference from aligned dataset (zero-copy!)
        Ok(&self.dataset.data[start..end])
    }
}

#[cfg(test)]
mod index_test {
    use vector::Metric;

    use super::*;
    use crate::model::configuration::index_write_parameters::IndexWriteParametersBuilder;
    use crate::model::vertex::DIM_128;
    use crate::utils::round_up;

    const R: u32 = 4;
    const L: u32 = 50;
    const ALPHA: f32 = 1.2;

    // Tests for memory-based interface implementations

    #[test]
    fn test_inmem_index_build_from_memory() {
        let index_write_parameters = IndexWriteParametersBuilder::new(L, R)
            .with_alpha(ALPHA)
            .with_num_threads(1)
            .build();

        let config = IndexConfiguration::new(
            Metric::L2,
            128, // 128 dimensions to use DIM_128
            round_up(128u64, 16u64) as usize,
            100,
            false,
            0,
            false,
            0,
            1.0f32,
            index_write_parameters,
        );

        let mut index: InmemIndex<f32, DIM_128> = InmemIndex::new(config).unwrap();

        // Create test vectors (128 dimensions)
        let mut test_vectors = Vec::new();
        for i in 0..10 {
            let mut vector = vec![0.0f32; 128];
            // Create distinct vectors
            for (j, item) in vector.iter_mut().enumerate() {
                *item = (i * 128 + j) as f32 / 1000.0;
            }
            test_vectors.push(vector);
        }

        // Convert to references
        let vector_refs: Vec<&[f32]> = test_vectors.iter().map(|v| v.as_slice()).collect();

        // Build from memory
        let result = index.build_from_memory(&vector_refs);
        assert!(
            result.is_ok(),
            "build_from_memory should succeed: {:?}",
            result.err()
        );

        // Verify index properties
        assert_eq!(index.num_active_pts, 10, "Should have 10 active points");
        assert!(index.start < 10, "Start point should be valid");

        // Test search functionality
        let query_vertex = index.dataset.get_vertex(0).unwrap();
        let mut indices = vec![0u32; 3];
        let mut distances = vec![0.0f32; 3];
        let search_result = index.search(
            &query_vertex,
            3,
            50,
            &mut indices,
            &mut distances,
            None,
            false,
        );
        assert!(search_result.is_ok(), "Search should succeed");

        // The first result should be the query vector itself (index 0)
        assert_eq!(indices[0], 0, "First result should be the query vector");
    }

    #[test]
    fn test_inmem_index_insert_from_memory() {
        let index_write_parameters = IndexWriteParametersBuilder::new(L, R)
            .with_alpha(ALPHA)
            .with_num_threads(1)
            .build();

        let config = IndexConfiguration::new(
            Metric::L2,
            128,
            round_up(128u64, 16u64) as usize,
            20, // Initial capacity
            false,
            0,
            false,
            0,
            2.0f32, // Growth potential for insertion
            index_write_parameters,
        );

        let mut index: InmemIndex<f32, DIM_128> = InmemIndex::new(config).unwrap();

        // Create initial vectors
        let mut initial_vectors = Vec::new();
        for i in 0..5 {
            let mut vector = vec![0.0f32; 128];
            for (j, item) in vector.iter_mut().enumerate() {
                *item = (i * 128 + j) as f32 / 1000.0;
            }
            initial_vectors.push(vector);
        }

        let initial_refs: Vec<&[f32]> = initial_vectors.iter().map(|v| v.as_slice()).collect();

        // Build initial index
        index.build_from_memory(&initial_refs).unwrap();
        assert_eq!(index.num_active_pts, 5);

        // Create additional vectors for insertion
        let mut insert_vectors = Vec::new();
        for i in 5..8 {
            let mut vector = vec![0.0f32; 128];
            for (j, item) in vector.iter_mut().enumerate() {
                *item = (i * 128 + j) as f32 / 1000.0;
            }
            insert_vectors.push(vector);
        }

        let insert_refs: Vec<&[f32]> = insert_vectors.iter().map(|v| v.as_slice()).collect();

        // Insert from memory
        let result = index.insert_from_memory(&insert_refs);
        assert!(
            result.is_ok(),
            "insert_from_memory should succeed: {:?}",
            result.err()
        );

        // Verify total count
        assert_eq!(
            index.num_active_pts, 8,
            "Should have 8 points after insertion"
        );

        // Test search on inserted vectors
        let query_vertex = index.dataset.get_vertex(5).unwrap(); // First inserted vector
        let mut indices = vec![0u32; 3];
        let mut distances = vec![0.0f32; 3];
        let search_result = index.search(
            &query_vertex,
            3,
            50,
            &mut indices,
            &mut distances,
            None,
            false,
        );
        assert!(search_result.is_ok(), "Search should find inserted vectors");

        // The inserted vector should be findable
        assert!(
            indices.contains(&5),
            "Should find the first inserted vector (index 5)"
        );
    }

    #[test]
    fn test_inmem_index_memory_dimension_validation() {
        let index_write_parameters = IndexWriteParametersBuilder::new(L, R)
            .with_alpha(ALPHA)
            .with_num_threads(1)
            .build();

        let config = IndexConfiguration::new(
            Metric::L2,
            128,
            round_up(128u64, 16u64) as usize,
            10,
            false,
            0,
            false,
            0,
            1.0f32,
            index_write_parameters,
        );

        let mut index: InmemIndex<f32, DIM_128> = InmemIndex::new(config).unwrap();

        // Create vectors with wrong dimensions
        let wrong_vector: Vec<f32> = vec![1.0; 64]; // 64 != 128
        let correct_vector: Vec<f32> = vec![2.0; 128];

        let mixed_vectors: Vec<&[f32]> = vec![correct_vector.as_slice(), wrong_vector.as_slice()];

        // Should fail due to dimension mismatch
        let result = index.build_from_memory(&mixed_vectors);
        assert!(result.is_err(), "Should fail with dimension mismatch");

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("dimension"),
            "Error should mention dimension"
        );
        assert!(
            error_msg.contains("128"),
            "Error should mention expected dimension"
        );
        assert!(
            error_msg.contains("64"),
            "Error should mention actual dimension"
        );
    }

    #[test]
    fn test_inmem_index_memory_empty_vectors() {
        let index_write_parameters = IndexWriteParametersBuilder::new(L, R)
            .with_alpha(ALPHA)
            .with_num_threads(1)
            .build();

        let config = IndexConfiguration::new(
            Metric::L2,
            128,
            round_up(128u64, 16u64) as usize,
            10,
            false,
            0,
            false,
            0,
            1.0f32,
            index_write_parameters,
        );

        let mut index: InmemIndex<f32, DIM_128> = InmemIndex::new(config).unwrap();

        let empty_vectors: Vec<&[f32]> = vec![];

        // Should fail with empty dataset
        let result = index.build_from_memory(&empty_vectors);
        assert!(result.is_err(), "Should fail with empty vectors");

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("0 vectors"),
            "Error should mention empty dataset"
        );
    }

    #[test]
    fn test_inmem_index_memory_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let index_write_parameters = IndexWriteParametersBuilder::new(L, R)
            .with_alpha(ALPHA)
            .with_num_threads(1)
            .build();

        let config = IndexConfiguration::new(
            Metric::L2,
            128,
            round_up(128u64, 16u64) as usize,
            20,
            false,
            0,
            false,
            0,
            1.0f32,
            index_write_parameters,
        );

        let mut index: InmemIndex<f32, DIM_128> = InmemIndex::new(config).unwrap();

        // Create test vectors
        let mut test_vectors = Vec::new();
        for i in 0..10 {
            let mut vector = vec![0.0f32; 128];
            for (j, item) in vector.iter_mut().enumerate() {
                *item = (i * 128 + j) as f32 / 1000.0;
            }
            test_vectors.push(vector);
        }

        let vector_refs: Vec<&[f32]> = test_vectors.iter().map(|v| v.as_slice()).collect();

        // Build index
        index.build_from_memory(&vector_refs).unwrap();

        let index_arc = Arc::new(index);

        // Concurrent search operations
        let handles: Vec<_> = (0..4)
            .map(|i| {
                let index_clone = Arc::clone(&index_arc);
                let query_idx = (i % test_vectors.len()) as u32;

                thread::spawn(move || {
                    let mut indices = vec![0u32; 3];
                    let mut distances = vec![0.0f32; 3];
                    let query_vertex = index_clone.dataset.get_vertex(query_idx).unwrap();
                    let result = index_clone.search(
                        &query_vertex,
                        3,
                        50,
                        &mut indices,
                        &mut distances,
                        None,
                        false,
                    );
                    (result.is_ok(), indices)
                })
            })
            .collect();

        // Collect results
        for handle in handles {
            let (success, _indices) = handle.join().unwrap();
            assert!(success, "Concurrent search should succeed");
        }
    }

    #[test]
    fn test_inmem_index_memory_performance_basic() {
        use std::time::Instant;

        let index_write_parameters = IndexWriteParametersBuilder::new(L, R)
            .with_alpha(ALPHA)
            .with_num_threads(1)
            .build();

        let config = IndexConfiguration::new(
            Metric::L2,
            128,
            round_up(128u64, 16u64) as usize,
            100,
            false,
            0,
            false,
            0,
            1.0f32,
            index_write_parameters,
        );

        // Create test data
        let mut test_vectors = Vec::new();
        for i in 0..50 {
            // 50 vectors for basic performance test
            let mut vector = vec![0.0f32; 128];
            for (j, item) in vector.iter_mut().enumerate() {
                *item = (i * 128 + j) as f32 / 1000.0;
            }
            test_vectors.push(vector);
        }

        let vector_refs: Vec<&[f32]> = test_vectors.iter().map(|v| v.as_slice()).collect();

        // Measure build time
        let mut index: InmemIndex<f32, DIM_128> = InmemIndex::new(config).unwrap();

        let start = Instant::now();
        let result = index.build_from_memory(&vector_refs);
        let build_time = start.elapsed();

        assert!(result.is_ok(), "Build should succeed");

        // Measure search time
        let query_vertex = index.dataset.get_vertex(0).unwrap();
        let mut indices = vec![0u32; 5];
        let mut distances = vec![0.0f32; 5];

        let start = Instant::now();
        let search_result = index.search(
            &query_vertex,
            5,
            50,
            &mut indices,
            &mut distances,
            None,
            false,
        );
        let search_time = start.elapsed();

        assert!(search_result.is_ok(), "Search should succeed");

        // Basic performance sanity checks (not precise benchmarks)
        assert!(
            build_time.as_millis() < 5000,
            "Build should complete in reasonable time"
        );
        assert!(search_time.as_millis() < 100, "Search should be fast");

        println!("Memory interface build time: {build_time:?}, search time: {search_time:?}");
    }
}
