// Copyright (c) Microsoft Corporation.  All rights reserved.
// Copyright (c) 2025 MiniGU. All rights reserved.
//
// Licensed under the MIT License. See diskann-rs/LICENSE for license information.
//
// Modifications for miniGU integration:
// - Added build_from_memory and append_from_memory for memory-based dataset construction
// - Added copy_aligned_data_from_memory for zero-copy vector loading
#![warn(missing_debug_implementations, missing_docs)]

//! In-memory Dataset

use std::mem;

use rayon::prelude::*;
use vector::{FullPrecisionDistance, Metric};

use crate::common::{ANNError, ANNResult, AlignedBoxWithSlice};
use crate::model::Vertex;

/// Dataset of all in-memory FP points
#[derive(Debug)]
pub struct InmemDataset<T, const N: usize>
where
    [T; N]: FullPrecisionDistance<T, N>,
{
    /// All in-memory points
    pub data: AlignedBoxWithSlice<T>,

    /// Number of points we anticipate to have
    pub num_points: usize,

    /// Number of active points i.e. existing in the graph
    pub num_active_pts: usize,

    /// Capacity of the dataset
    pub capacity: usize,
}

impl<'a, T, const N: usize> InmemDataset<T, N>
where
    T: Default + Copy + Sync + Send + Into<f32>,
    [T; N]: FullPrecisionDistance<T, N>,
{
    /// Create the dataset with size num_points and growth factor.
    /// growth factor=1 means no growth (provision 100% space of num_points)
    /// growth factor=1.2 means provision 120% space of num_points (20% extra space)
    pub fn new(num_points: usize, index_growth_factor: f32) -> ANNResult<Self> {
        let capacity = (((num_points * N) as f32) * index_growth_factor) as usize;

        Ok(Self {
            data: AlignedBoxWithSlice::new(capacity, mem::size_of::<T>() * 16)?,
            num_points,
            num_active_pts: num_points,
            capacity,
        })
    }

    /// get immutable data slice
    pub fn get_data(&self) -> &[T] {
        &self.data
    }

    /// Build the dataset from memory vectors
    pub fn build_from_memory(
        &mut self,
        vectors: &[&[T]],
        num_points_to_load: usize,
        actual_dim: usize,
    ) -> ANNResult<()> {
        println!("Loading {num_points_to_load} vectors from memory into dataset...");

        if num_points_to_load > vectors.len() {
            return Err(ANNError::log_index_error(format!(
                "Requested to load {} points but only {} vectors provided",
                num_points_to_load,
                vectors.len()
            )));
        }

        if num_points_to_load > self.capacity / N {
            return Err(ANNError::log_index_error(format!(
                "Cannot load {} points to dataset of capacity {}",
                num_points_to_load,
                self.capacity / N
            )));
        }

        self.num_active_pts = num_points_to_load;
        self.num_points = num_points_to_load;

        self.copy_aligned_data_from_memory(&vectors[..num_points_to_load], 0, actual_dim)?;

        println!("Dataset loaded from memory.");
        Ok(())
    }

    /// Append the dataset from memory vectors
    pub fn append_from_memory(
        &mut self,
        vectors: &[&[T]],
        num_points_to_append: usize,
        actual_dim: usize,
    ) -> ANNResult<()> {
        println!("Appending {num_points_to_append} vectors from memory into dataset...");

        if num_points_to_append > vectors.len() {
            return Err(ANNError::log_index_error(format!(
                "Requested to append {} points but only {} vectors provided",
                num_points_to_append,
                vectors.len()
            )));
        }

        if self.num_active_pts + num_points_to_append > self.capacity / N {
            return Err(ANNError::log_index_error(format!(
                "Cannot append {} points to dataset of capacity {}",
                num_points_to_append,
                self.capacity / N
            )));
        }

        let pts_offset = self.num_active_pts;
        self.copy_aligned_data_from_memory(
            &vectors[..num_points_to_append],
            pts_offset,
            actual_dim,
        )?;

        self.num_active_pts += num_points_to_append;
        self.num_points += num_points_to_append;

        println!("Dataset appended from memory.");
        Ok(())
    }

    /// Copy data from memory vectors into aligned storage
    /// Similar to copy_aligned_data_from_file but operates on in-memory vectors
    fn copy_aligned_data_from_memory(
        &mut self,
        vectors: &[&[T]],
        pts_offset: usize,
        actual_dim: usize,
    ) -> ANNResult<()> {
        let offset = pts_offset * N;

        for (i, vector) in vectors.iter().enumerate() {
            // Validate vector dimension matches actual dimension (not aligned dimension)
            if vector.len() != actual_dim {
                return Err(ANNError::log_index_error(format!(
                    "Vector {} has dimension {} but expected {}",
                    i,
                    vector.len(),
                    actual_dim
                )));
            }

            // Calculate target slice position for this vector
            let start = offset + i * N;
            let data_end = start + actual_dim;
            let aligned_end = start + N;

            if aligned_end > self.data.len() {
                return Err(ANNError::log_index_error(format!(
                    "Cannot copy vector {i}: storage capacity exceeded"
                )));
            }

            // Copy actual vector data to aligned storage (only actual_dim elements)
            self.data[start..data_end].copy_from_slice(vector);

            // Fill padding area with default values (from actual_dim to N)
            if actual_dim < N {
                for j in data_end..aligned_end {
                    self.data[j] = T::default();
                }
            }
        }

        Ok(())
    }

    /// Get vertex by id
    pub fn get_vertex(&'a self, id: u32) -> ANNResult<Vertex<'a, T, N>> {
        let start = id as usize * N;
        let end = start + N;

        if end <= self.data.len() {
            let val = <&[T; N]>::try_from(&self.data[start..end]).map_err(|err| {
                ANNError::log_index_error(format!("Failed to get vertex {id}, err={err}"))
            })?;
            Ok(Vertex::new(val, id))
        } else {
            Err(ANNError::log_index_error(format!(
                "Invalid vertex id {id}."
            )))
        }
    }

    /// Get full precision distance between two nodes
    pub fn get_distance(&self, id1: u32, id2: u32, metric: Metric) -> ANNResult<f32> {
        let vertex1 = self.get_vertex(id1)?;
        let vertex2 = self.get_vertex(id2)?;

        Ok(vertex1.compare(&vertex2, metric))
    }

    /// find out the medoid, the vertex in the dataset that is closest to the centroid
    pub fn calculate_medoid_point_id(&self) -> ANNResult<u32> {
        Ok(self.find_nearest_point_id(self.calculate_centroid_point()?))
    }

    /// calculate centroid, average of all vertices in the dataset
    fn calculate_centroid_point(&self) -> ANNResult<[f32; N]> {
        // Allocate and initialize the centroid vector
        let mut center: [f32; N] = [0.0; N];

        // Sum the data points' components
        for i in 0..self.num_active_pts {
            let vertex = self.get_vertex(i as u32)?;
            let vertex_slice = vertex.vector();
            for j in 0..N {
                center[j] += vertex_slice[j].into();
            }
        }

        // Divide by the number of points to calculate the centroid
        let capacity = self.num_active_pts as f32;
        for item in center.iter_mut().take(N) {
            *item /= capacity;
        }

        Ok(center)
    }

    /// find out the vertex closest to the given point
    fn find_nearest_point_id(&self, point: [f32; N]) -> u32 {
        // compute all to one distance
        let mut distances = vec![0f32; self.num_active_pts];
        let slice = &self.data[..];
        distances.par_iter_mut().enumerate().for_each(|(i, dist)| {
            let start = i * N;
            for j in 0..N {
                let diff: f32 = (point.as_slice()[j] - slice[start + j].into())
                    * (point.as_slice()[j] - slice[start + j].into());
                *dist += diff;
            }
        });

        let mut min_idx = 0;
        let mut min_dist = f32::MAX;
        for (i, distance) in distances.iter().enumerate().take(self.num_active_pts) {
            if *distance < min_dist {
                min_idx = i;
                min_dist = *distance;
            }
        }
        min_idx as u32
    }

    /// Prefetch vertex data in the memory hierarchy
    /// NOTE: good efficiency when total_vec_size is integral multiple of 64
    #[inline]
    pub fn prefetch_vector(&self, id: u32) {
        let start = id as usize * N;
        let end = start + N;

        if end <= self.data.len() {
            let vec = &self.data[start..end];
            vector::prefetch_vector(vec);
        }
    }

    /// Convert into dto object
    pub fn as_dto(&mut self) -> DatasetDto<T> {
        DatasetDto {
            data: &mut self.data,
            rounded_dim: N,
        }
    }
}

/// Dataset dto used for other layer, such as storage
/// N is the aligned dimension
#[derive(Debug)]
pub struct DatasetDto<'a, T> {
    /// data slice borrow from dataset
    pub data: &'a mut [T],

    /// rounded dimension
    pub rounded_dim: usize,
}

#[cfg(test)]
mod dataset_test {
    use super::*;
    use crate::model::vertex::DIM_128;

    #[test]
    fn get_vertex_within_range() {
        let num_points = 1_000_000;
        let id = 999_999;
        let dataset = InmemDataset::<f32, DIM_128>::new(num_points, 1f32).unwrap();

        let vertex = dataset.get_vertex(999_999).unwrap();

        assert_eq!(vertex.vertex_id(), id);
        assert_eq!(vertex.vector().len(), DIM_128);
        assert_eq!(vertex.vector().as_ptr(), unsafe {
            dataset.data.as_ptr().add((id as usize) * DIM_128)
        });
    }

    #[test]
    fn get_vertex_out_of_range() {
        let num_points = 1_000_000;
        let invalid_id = 1_000_000;
        let dataset = InmemDataset::<f32, DIM_128>::new(num_points, 1f32).unwrap();

        if dataset.get_vertex(invalid_id).is_ok() {
            panic!("id ({invalid_id}) should be out of range")
        };
    }

    // Tests for memory-based interfaces

    #[test]
    fn test_build_from_memory_basic() {
        let mut dataset = InmemDataset::<f32, 8>::new(3, 1f32).unwrap();

        // Create test vectors
        let vector1: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let vector2: [f32; 8] = [9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
        let vector3: [f32; 8] = [17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0];

        let vectors: Vec<&[f32]> = vec![&vector1, &vector2, &vector3];

        // Build from memory
        let result = dataset.build_from_memory(&vectors, 3, 8);
        assert!(result.is_ok(), "build_from_memory should succeed");

        // Verify data was loaded correctly
        assert_eq!(dataset.num_active_pts, 3);

        let vertex0 = dataset.get_vertex(0).unwrap();
        let vertex1 = dataset.get_vertex(1).unwrap();
        let vertex2 = dataset.get_vertex(2).unwrap();

        assert_eq!(*vertex0.vector(), vector1);
        assert_eq!(*vertex1.vector(), vector2);
        assert_eq!(*vertex2.vector(), vector3);
    }

    #[test]
    fn test_append_from_memory() {
        let mut dataset = InmemDataset::<f32, 8>::new(5, 1f32).unwrap();

        // Initial build
        let vector1: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let vector2: [f32; 8] = [9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
        let initial_vectors: Vec<&[f32]> = vec![&vector1, &vector2];

        dataset.build_from_memory(&initial_vectors, 2, 8).unwrap();
        assert_eq!(dataset.num_active_pts, 2);

        // Append more vectors
        let vector3: [f32; 8] = [17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0];
        let vector4: [f32; 8] = [25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0, 32.0];
        let append_vectors: Vec<&[f32]> = vec![&vector3, &vector4];

        let result = dataset.append_from_memory(&append_vectors, 2, 8);
        assert!(result.is_ok(), "append_from_memory should succeed");

        // Verify total count
        assert_eq!(dataset.num_active_pts, 4);
        assert_eq!(dataset.num_points, 4);

        // Verify all vectors are present
        let vertex0 = dataset.get_vertex(0).unwrap();
        let vertex1 = dataset.get_vertex(1).unwrap();
        let vertex2 = dataset.get_vertex(2).unwrap();
        let vertex3 = dataset.get_vertex(3).unwrap();

        assert_eq!(*vertex0.vector(), vector1);
        assert_eq!(*vertex1.vector(), vector2);
        assert_eq!(*vertex2.vector(), vector3);
        assert_eq!(*vertex3.vector(), vector4);
    }

    #[test]
    fn test_build_from_memory_dimension_mismatch() {
        let mut dataset = InmemDataset::<f32, 8>::new(2, 1f32).unwrap();

        // Create vectors with wrong dimensions
        let vector_good: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let vector_bad: [f32; 4] = [9.0, 10.0, 11.0, 12.0]; // Wrong dimension

        let vectors: Vec<&[f32]> = vec![&vector_good, &vector_bad];

        let result = dataset.build_from_memory(&vectors, 2, 8);
        assert!(
            result.is_err(),
            "build_from_memory should fail with dimension mismatch"
        );

        // Verify error message contains dimension information
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("dimension"),
            "Error should mention dimension mismatch"
        );
        assert!(
            error_msg.contains("8"),
            "Error should mention expected dimension"
        );
        assert!(
            error_msg.contains("4"),
            "Error should mention actual dimension"
        );
    }

    #[test]
    fn test_build_from_memory_empty_vectors() {
        let mut dataset = InmemDataset::<f32, 8>::new(1, 1f32).unwrap();

        let empty_vectors: Vec<&[f32]> = vec![];

        let result = dataset.build_from_memory(&empty_vectors, 0, 8);
        assert!(
            result.is_ok(),
            "build_from_memory should handle empty vector set"
        );

        assert_eq!(dataset.num_active_pts, 0);
    }

    #[test]
    fn test_build_from_memory_capacity_exceeded() {
        let mut dataset = InmemDataset::<f32, 8>::new(2, 1f32).unwrap(); // Capacity for 2 vectors

        // Try to load 3 vectors (exceeds capacity)
        let vector1: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let vector2: [f32; 8] = [9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
        let vector3: [f32; 8] = [17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0];

        let vectors: Vec<&[f32]> = vec![&vector1, &vector2, &vector3];

        let result = dataset.build_from_memory(&vectors, 3, 8);
        assert!(
            result.is_err(),
            "build_from_memory should fail when capacity is exceeded"
        );

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("capacity"),
            "Error should mention capacity limitation"
        );
    }

    #[test]
    fn test_append_from_memory_capacity_exceeded() {
        let mut dataset = InmemDataset::<f32, 8>::new(3, 1f32).unwrap(); // Capacity for 3 vectors

        // First, build with 2 vectors
        let vector1: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let vector2: [f32; 8] = [9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
        let initial_vectors: Vec<&[f32]> = vec![&vector1, &vector2];

        dataset.build_from_memory(&initial_vectors, 2, 8).unwrap();

        // Now try to append 2 more vectors (would exceed capacity of 3)
        let vector3: [f32; 8] = [17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0];
        let vector4: [f32; 8] = [25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0, 32.0];
        let append_vectors: Vec<&[f32]> = vec![&vector3, &vector4];

        let result = dataset.append_from_memory(&append_vectors, 2, 8);
        assert!(
            result.is_err(),
            "append_from_memory should fail when capacity would be exceeded"
        );

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("capacity"),
            "Error should mention capacity limitation"
        );
    }

    #[test]
    fn test_build_from_memory_vector_count_mismatch() {
        let mut dataset = InmemDataset::<f32, 8>::new(5, 1f32).unwrap();

        let vector1: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let vector2: [f32; 8] = [9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
        let vectors: Vec<&[f32]> = vec![&vector1, &vector2];

        // Request to load more vectors than provided
        let result = dataset.build_from_memory(&vectors, 3, 8);
        assert!(
            result.is_err(),
            "build_from_memory should fail when requesting more vectors than provided"
        );

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("3"),
            "Error should mention requested count"
        );
        assert!(error_msg.contains("2"), "Error should mention actual count");
    }
}
