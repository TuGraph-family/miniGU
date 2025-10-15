// Copyright (c) Microsoft Corporation.  All rights reserved.
// Copyright (c) 2025 MiniGU. All rights reserved.
//
// Licensed under the MIT License. See diskann-rs/LICENSE for license information.
//
// Modifications:
// - Added memory-based interface methods for miniGU integration: `build_from_memory`,
//   `insert_from_memory`, and `get_aligned_vector_data`.
// - Extended the `search` method to include additional parameters: `distances: &mut [f32]`,
//   `filter_mask: Option<&dyn FilterIndex>`, and `should_pre: bool`, allowing for more flexible
//   search results with filtering and distance tracking.
#![warn(missing_docs)]

//! ANN in-memory index abstraction

use vector::FullPrecisionDistance;

use super::InmemIndex;
use crate::common::{ANNError, ANNResult, FilterIndex};
use crate::model::IndexConfiguration;
use crate::model::vertex::{DIM_104, DIM_128, DIM_256};

/// ANN inmem-index abstraction for custom <T, N>
#[allow(clippy::upper_case_acronyms)]
pub trait ANNInmemIndex<T>: Sync + Send
where
    T: Default + Copy + Sync + Send + Into<f32>,
{
    /// Search the index for K nearest neighbors of query using given L value, for benchmarking
    /// purposes
    #[allow(clippy::too_many_arguments)]
    fn search(
        &self,
        query: &[T],
        k_value: usize,
        l_value: u32,
        indices: &mut [u32],
        distances: &mut [f32],
        filter_mask: Option<&dyn FilterIndex>,
        should_pre: bool,
    ) -> ANNResult<u32>;

    /// Soft deletes the nodes with the ids in the given array.
    fn soft_delete(
        &mut self,
        vertex_ids_to_delete: Vec<u32>,
        num_points_to_delete: usize,
    ) -> ANNResult<()>;

    // Memory-based interface methods (added for miniGU integration)

    /// Build index from memory vectors
    /// Default implementation returns not supported error for backward compatibility
    fn build_from_memory(&mut self, _vectors: &[&[T]]) -> ANNResult<()> {
        Err(ANNError::log_index_error(
            "build_from_memory not implemented".to_string(),
        ))
    }

    /// Insert vectors from memory
    /// Default implementation returns not supported error for backward compatibility
    fn insert_from_memory(&mut self, _vectors: &[&[T]]) -> ANNResult<()> {
        Err(ANNError::log_index_error(
            "insert_from_memory not implemented".to_string(),
        ))
    }

    /// Get 64-byte aligned vector reference by vector ID for zero-copy SIMD operations
    /// Default implementation returns not supported error for backward compatibility
    fn get_aligned_vector_data(&self, _vector_id: u32) -> ANNResult<&[T]> {
        Err(ANNError::log_index_error(
            "get_aligned_vector_data not implemented".to_string(),
        ))
    }
}

/// Create Index<T, N> based on configuration
pub fn create_inmem_index<'a, T>(
    config: IndexConfiguration,
) -> ANNResult<Box<dyn ANNInmemIndex<T> + 'a>>
where
    T: Default + Copy + Sync + Send + Into<f32> + 'a,
    [T; DIM_104]: FullPrecisionDistance<T, DIM_104>,
    [T; DIM_128]: FullPrecisionDistance<T, DIM_128>,
    [T; DIM_256]: FullPrecisionDistance<T, DIM_256>,
{
    match config.aligned_dim {
        DIM_104 => {
            let index = Box::new(InmemIndex::<T, DIM_104>::new(config)?);
            Ok(index as Box<dyn ANNInmemIndex<T>>)
        }
        DIM_128 => {
            let index = Box::new(InmemIndex::<T, DIM_128>::new(config)?);
            Ok(index as Box<dyn ANNInmemIndex<T>>)
        }
        DIM_256 => {
            let index = Box::new(InmemIndex::<T, DIM_256>::new(config)?);
            Ok(index as Box<dyn ANNInmemIndex<T>>)
        }
        _ => Err(ANNError::IndexError {
            err: format!("Invalid dimension: {}", config.aligned_dim),
        }),
    }
}

#[cfg(test)]
mod dataset_test {
    use vector::Metric;

    use super::*;
    use crate::model::configuration::index_write_parameters::IndexWriteParametersBuilder;

    // Tests for memory-based interface methods

    #[test]
    fn test_create_index_memory_interface() {
        let index_write_parameters = IndexWriteParametersBuilder::new(50, 4)
            .with_alpha(1.2)
            .with_saturate_graph(false)
            .with_num_threads(1)
            .build();

        let config = IndexConfiguration::new(
            Metric::L2,
            128, // 128 dimensions to match DIM_128 const generic
            128, // aligned dimension
            100,
            false,
            0,
            false,
            0,
            1f32,
            index_write_parameters,
        );

        let mut index = create_inmem_index::<f32>(config).unwrap();

        // Create test vectors (128 dimensions)
        let mut vector1 = vec![0.0f32; 128];
        let mut vector2 = vec![0.0f32; 128];
        let mut vector3 = vec![0.0f32; 128];

        // Initialize with distinct values for each vector
        for i in 0..128 {
            vector1[i] = (i + 1) as f32 / 10.0; // [0.1, 0.2, ..., 12.8]
            vector2[i] = (i + 129) as f32 / 10.0; // [12.9, 13.0, ..., 25.6]
            vector3[i] = (i + 257) as f32 / 10.0; // [25.7, 25.8, ..., 38.4]
        }

        let vectors: Vec<&[f32]> = vec![&vector1, &vector2, &vector3];

        // Test build_from_memory
        let result = index.build_from_memory(&vectors);
        assert!(
            result.is_ok(),
            "build_from_memory should succeed: {:?}",
            result.err()
        );

        // Test search functionality
        let mut indices = vec![0u32; 2];
        let mut distances = vec![0.0f32; 2];
        let search_result =
            index.search(&vector1, 2, 50, &mut indices, &mut distances, None, false);
        assert!(
            search_result.is_ok(),
            "Search should succeed after build_from_memory"
        );

        // The most similar vector should be the query itself (vector1 at index 0)
        assert_eq!(
            indices[0], 0,
            "First result should be the query vector itself"
        );
    }

    #[test]
    fn test_build_from_memory_trait_default() {
        // Create a mock implementation that doesn't override build_from_memory
        struct MockIndex;

        impl ANNInmemIndex<f32> for MockIndex {
            fn search(
                &self,
                _query: &[f32],
                _k_value: usize,
                _l_value: u32,
                _indices: &mut [u32],
                _distances: &mut [f32],
                _filter_mask: Option<&dyn FilterIndex>,
                _should_pre: bool,
            ) -> crate::common::ANNResult<u32> {
                Ok(0)
            }

            fn soft_delete(
                &mut self,
                _vertex_ids_to_delete: Vec<u32>,
                _num_points_to_delete: usize,
            ) -> crate::common::ANNResult<()> {
                Ok(())
            }

            // Note: build_from_memory and insert_from_memory will use default implementations
        }

        let mut mock_index = MockIndex;

        // Test that default implementation returns "not implemented" error
        let vector: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
        let vectors: Vec<&[f32]> = vec![&vector];

        let result = mock_index.build_from_memory(&vectors);
        assert!(
            result.is_err(),
            "Default build_from_memory should return error"
        );

        let error_msg = format!("{}", result.unwrap_err());
        assert!(
            error_msg.contains("build_from_memory not implemented"),
            "Error should indicate method not implemented"
        );
    }

    #[test]
    fn test_insert_from_memory_trait_default() {
        // Same mock implementation as above
        struct MockIndex;

        impl ANNInmemIndex<f32> for MockIndex {
            fn search(
                &self,
                _query: &[f32],
                _k_value: usize,
                _l_value: u32,
                _indices: &mut [u32],
                _distances: &mut [f32],
                _filter_mask: Option<&dyn FilterIndex>,
                _should_pre: bool,
            ) -> crate::common::ANNResult<u32> {
                Ok(0)
            }

            fn soft_delete(
                &mut self,
                _vertex_ids_to_delete: Vec<u32>,
                _num_points_to_delete: usize,
            ) -> crate::common::ANNResult<()> {
                Ok(())
            }
        }

        let mut mock_index = MockIndex;

        // Test that default implementation returns "not implemented" error
        let vector: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
        let vectors: Vec<&[f32]> = vec![&vector];

        let result = mock_index.insert_from_memory(&vectors);
        assert!(
            result.is_err(),
            "Default insert_from_memory should return error"
        );

        let error_msg = format!("{}", result.unwrap_err());
        assert!(
            error_msg.contains("insert_from_memory not implemented"),
            "Error should indicate method not implemented"
        );
    }

    #[test]
    fn test_memory_interface_dimension_support() {
        // Test different supported dimensions (104, 128, 256)
        let dimensions = vec![(104, 104), (128, 128), (256, 256)];

        for (dim, aligned_dim) in dimensions {
            let index_write_parameters = IndexWriteParametersBuilder::new(50, 4)
                .with_alpha(1.2)
                .with_saturate_graph(false)
                .with_num_threads(1)
                .build();

            let config = IndexConfiguration::new(
                Metric::L2,
                dim,
                aligned_dim,
                100,
                false,
                0,
                false,
                0,
                1f32,
                index_write_parameters,
            );

            let result = create_inmem_index::<f32>(config);
            assert!(result.is_ok(), "Should create index for dimension {dim}");

            // We can't easily test build_from_memory here due to const generic constraints
            // but we've verified the index can be created
        }
    }

    #[test]
    fn test_memory_interface_invalid_dimension() {
        let index_write_parameters = IndexWriteParametersBuilder::new(50, 4)
            .with_alpha(1.2)
            .with_saturate_graph(false)
            .with_num_threads(1)
            .build();

        // Try to create index with unsupported dimension (300 > 256)
        let config = IndexConfiguration::new(
            Metric::L2,
            300, // Unsupported dimension
            300,
            100,
            false,
            0,
            false,
            0,
            1f32,
            index_write_parameters,
        );

        let result = create_inmem_index::<f32>(config);
        assert!(
            result.is_err(),
            "Should fail to create index for unsupported dimension 300"
        );

        match result {
            Err(error) => {
                let error_msg = format!("{error}");
                assert!(
                    error_msg.contains("Invalid dimension"),
                    "Error should mention invalid dimension"
                );
                assert!(
                    error_msg.contains("300"),
                    "Error should mention the invalid dimension value"
                );
            }
            Ok(_) => panic!("Expected error but got success"),
        }
    }
}
