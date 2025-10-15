// Copyright (c) Microsoft Corporation.  All rights reserved.
// Copyright (c) 2025 MiniGU. All rights reserved.
//
// Licensed under the MIT License. See diskann-rs/LICENSE for license information.
//
// Modifications for miniGU integration:
// - Added FilterIndex support for pre-filter and post-filter search
// - Enhanced search_with_l_override and greedy_search to support filtered search
// - Modified get_init_ids to find alternative start points when pre-filtering
// - Added early termination logic for filtered neighbor expansion

#![warn(missing_debug_implementations, missing_docs)]

//! Search algorithm for index construction and query

use hashbrown::hash_set::Entry::*;
use vector::FullPrecisionDistance;

use crate::common::{ANNError, ANNResult, FilterIndex};
use crate::index::InmemIndex;
use crate::model::scratch::InMemQueryScratch;
use crate::model::{Neighbor, Vertex};

impl<T, const N: usize> InmemIndex<T, N>
where
    T: Default + Copy + Sync + Send + Into<f32>,
    [T; N]: FullPrecisionDistance<T, N>,
{
    /// Search for query using given L value, for benchmarking purposes
    /// # Arguments
    /// * `query` - query vertex
    /// * `scratch` - in-memory query scratch
    /// * `search_list_size` - search list size to use for the benchmark
    pub fn search_with_l_override(
        &self,
        query: &Vertex<T, N>,
        scratch: &mut InMemQueryScratch<T, N>,
        search_list_size: usize,
        filter_mask: Option<&dyn FilterIndex>,
        should_pre: bool,
    ) -> ANNResult<u32> {
        let init_ids = self.get_init_ids(filter_mask, should_pre)?;
        self.init_graph_for_point(query, init_ids, scratch)?;
        // Scratch is created using largest L val from search_memory_index, so we artificially make
        // it smaller here This allows us to use the same scratch for all L values without
        // having to rebuild the query scratch
        scratch.best_candidates.set_capacity(search_list_size);
        let (_, cmp) = self.greedy_search(query, scratch, filter_mask, should_pre)?;

        Ok(cmp)
    }

    /// search for point
    /// # Arguments
    /// * `query` - query vertex
    /// * `scratch` - in-memory query scratch TODO: use_filter, filteredLindex
    pub fn search_for_point(
        &self,
        query: &Vertex<T, N>,
        scratch: &mut InMemQueryScratch<T, N>,
    ) -> ANNResult<Vec<Neighbor>> {
        let init_ids = self.get_init_ids(None, false)?;
        self.init_graph_for_point(query, init_ids, scratch)?;
        let (mut visited_nodes, _) = self.greedy_search(query, scratch, None, false)?;

        visited_nodes.retain(|&element| element.id != query.vertex_id());
        Ok(visited_nodes)
    }

    /// Returns the locations of start point and frozen points suitable for use with
    /// iterate_to_fixed_point.
    fn get_init_ids(
        &self,
        filter_mask: Option<&dyn FilterIndex>,
        should_pre: bool,
    ) -> ANNResult<Vec<u32>> {
        let mut init_ids = Vec::with_capacity(1 + self.configuration.num_frozen_pts);
        init_ids.push(self.start);

        // If pre-filtering is enabled and self.start doesn't satisfy filter condition,
        // add the first point that satisfies the filter condition
        if let (Some(filter), true) = (filter_mask, should_pre) {
            if !filter.contains_vector(self.start) {
                // Find first point that satisfies filter condition
                for id in 0..self.configuration.max_points as u32 {
                    if filter.contains_vector(id) {
                        init_ids.push(id);
                        break;
                    }
                }
            }
        }

        for frozen in self.configuration.max_points
            ..(self.configuration.max_points + self.configuration.num_frozen_pts)
        {
            let frozen_u32 = frozen.try_into()?;
            if frozen_u32 != self.start {
                init_ids.push(frozen_u32);
            }
        }

        Ok(init_ids)
    }

    /// Initialize graph for point
    /// # Arguments
    /// * `query` - query vertex
    /// * `init_ids` - initial nodes from which search starts
    /// * `scratch` - in-memory query scratch
    /// * `search_list_size_override` - override for search list size in index config
    fn init_graph_for_point(
        &self,
        query: &Vertex<T, N>,
        init_ids: Vec<u32>,
        scratch: &mut InMemQueryScratch<T, N>,
    ) -> ANNResult<()> {
        scratch
            .best_candidates
            .reserve(self.configuration.index_write_parameter.search_list_size as usize);
        scratch.query.memcpy(query.vector())?;

        if !scratch.id_scratch.is_empty() {
            return Err(ANNError::log_index_error(
                "id_scratch is not empty.".to_string(),
            ));
        }

        let query_vertex = Vertex::<T, N>::try_from((&scratch.query[..], query.vertex_id()))
            .map_err(|err| {
                ANNError::log_index_error(format!(
                    "TryFromSliceError: failed to get Vertex for query, err={err}"
                ))
            })?;

        for id in init_ids {
            if (id as usize) >= self.configuration.max_points + self.configuration.num_frozen_pts {
                return Err(ANNError::log_index_error(format!(
                    "vertex_id {} is out of valid range of points {}",
                    id,
                    self.configuration.max_points + self.configuration.num_frozen_pts
                )));
            }

            if let Vacant(entry) = scratch.node_visited_robinset.entry(id) {
                entry.insert();

                let vertex = self.dataset.get_vertex(id)?;

                let distance = vertex.compare(&query_vertex, self.configuration.dist_metric);
                let neighbor = Neighbor::new(id, distance);
                scratch.best_candidates.insert(neighbor);
            }
        }

        Ok(())
    }

    /// GreedySearch against query node
    /// Returns visited nodes
    /// # Arguments
    /// * `query` - query vertex
    /// * `scratch` - in-memory query scratch TODO: use_filter, filter_label, search_invocation
    fn greedy_search(
        &self,
        query: &Vertex<T, N>,
        scratch: &mut InMemQueryScratch<T, N>,
        filter_mask: Option<&dyn FilterIndex>,
        should_pre: bool,
    ) -> ANNResult<(Vec<Neighbor>, u32)> {
        let mut visited_nodes =
            Vec::with_capacity((3 * scratch.candidate_size + scratch.max_degree) as usize);

        // TODO: uncomment hops?
        // let mut hops: u32 = 0;
        let mut cmps: u32 = 0;

        let query_vertex = Vertex::<T, N>::try_from((&scratch.query[..], query.vertex_id()))
            .map_err(|err| {
                ANNError::log_index_error(format!(
                    "TryFromSliceError: failed to get Vertex for query, err={err}"
                ))
            })?;

        while scratch.best_candidates.has_notvisited_node() {
            let closest_node = scratch.best_candidates.closest_notvisited();

            // Add node to visited nodes to create pool for prune later
            // TODO: search_invocation and use_filter
            visited_nodes.push(closest_node);

            // Find which of the nodes in des have not been visited before
            scratch.id_scratch.clear();

            let max_vertex_id = self.configuration.max_points + self.configuration.num_frozen_pts;

            for id in self
                .final_graph
                .read_vertex_and_neighbors(closest_node.id)?
                .get_neighbors()
            {
                let current_vertex_id = *id;
                debug_assert!(
                    (current_vertex_id as usize) < max_vertex_id,
                    "current_vertex_id {current_vertex_id} is out of valid range of points {max_vertex_id}"
                );
                if current_vertex_id as usize >= max_vertex_id {
                    continue;
                }

                // quickly de-dup. Remember, we are in a read lock
                // we want to exit out of it quickly
                if scratch.node_visited_robinset.insert(current_vertex_id) {
                    // Apply pre-filtering if enabled
                    if let (Some(filter), true) = (filter_mask, should_pre) {
                        if filter.contains_vector(current_vertex_id) {
                            scratch.id_scratch.push(current_vertex_id);
                        }
                    } else {
                        // No pre-filtering, add all unvisited neighbors
                        scratch.id_scratch.push(current_vertex_id);
                    }
                }
            }

            let len = scratch.id_scratch.len();
            for (m, &id) in scratch.id_scratch.iter().enumerate() {
                if m + 1 < len {
                    let next_node = unsafe { *scratch.id_scratch.get_unchecked(m + 1) };
                    self.dataset.prefetch_vector(next_node);
                }

                let vertex = self.dataset.get_vertex(id)?;
                let distance = query_vertex.compare(&vertex, self.configuration.dist_metric);

                // Insert <id, dist> pairs into the pool of candidates
                scratch.best_candidates.insert(Neighbor::new(id, distance));
            }

            cmps += len as u32;
        }

        Ok((visited_nodes, cmps))
    }
}
