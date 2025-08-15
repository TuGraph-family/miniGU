use std::any::Any;

use bitvec::prelude::*;
use diskann::common::FilterIndex as DiskANNFilterMask;

/// Filter mask trait for vector index filtering
/// Provides a unified interface for different filtering strategies
/// Works in vector_id domain (0..N-1) where N is the number of vectors in the index
pub trait FilterMask: Send + Sync {
    /// Check if a vector ID should be included in the search results
    fn contains_vector(&self, vector_id: u32) -> bool;

    /// Get the selectivity (ratio of included vectors to total vectors)
    fn selectivity(&self) -> f32;

    /// Get the number of candidate vectors
    fn candidate_count(&self) -> usize;

    /// Get total vector count for context
    fn total_vectors(&self) -> usize;

    /// Get iterator over candidate vector IDs for efficient traversal
    /// Sparse masks iterate over Vec, Dense masks iterate over set bits
    fn iter_candidates(&self) -> Box<dyn Iterator<Item = u32> + '_>;

    /// Support for downcasting to concrete types
    fn as_any(&self) -> &dyn Any;
}

/// Sparse filter mask using Vec - optimal for low selectivity brute force search
/// Stores sorted list of candidate vector IDs for direct iteration
/// Provides O(log n) lookup and O(1) iteration over candidates
#[derive(Debug, Clone)]
pub struct SparseFilterMask {
    candidates: Vec<u32>,
    total_vectors: usize,
    selectivity: f32,
}

impl SparseFilterMask {
    pub fn new(selectivity: f32, mut candidates: Vec<u32>, total_vectors: usize) -> Self {
        // Remove duplicates and sort for binary search
        candidates.sort_unstable();
        candidates.dedup();

        Self {
            candidates,
            total_vectors,
            selectivity,
        }
    }

    pub fn candidates(&self) -> &[u32] {
        &self.candidates
    }
}

impl FilterMask for SparseFilterMask {
    fn contains_vector(&self, vector_id: u32) -> bool {
        self.candidates.binary_search(&vector_id).is_ok()
    }

    fn selectivity(&self) -> f32 {
        self.selectivity
    }

    fn candidate_count(&self) -> usize {
        self.candidates.len()
    }

    fn total_vectors(&self) -> usize {
        self.total_vectors
    }

    fn iter_candidates(&self) -> Box<dyn Iterator<Item = u32> + '_> {
        Box::new(self.candidates.iter().cloned())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implement DiskANN FilterMask trait for SparseFilterMask
impl DiskANNFilterMask for SparseFilterMask {
    fn contains_vector(&self, vector_id: u32) -> bool {
        FilterMask::contains_vector(self, vector_id)
    }
}

/// Dense filter mask using BitVec - optimal for high selectivity
/// Provides efficient bit-level filtering for post-filter search strategies
#[derive(Debug, Clone)]
pub struct DenseFilterMask {
    bitmap: BitVec,
    candidate_count: usize,
    selectivity: f32,
}

impl DenseFilterMask {
    pub fn new(selectivity: f32, candidates: Vec<u32>, total_vectors: usize) -> Self {
        let mut bitmap = bitvec![0; total_vectors];
        let mut valid_candidates = 0;

        for &vector_id in &candidates {
            if let Some(mut bit) = bitmap.get_mut(vector_id as usize) {
                if !*bit {
                    bit.set(true);
                    valid_candidates += 1;
                }
            }
        }

        Self {
            bitmap,
            candidate_count: valid_candidates,
            selectivity,
        }
    }

    pub fn from_bitmap(bitmap: BitVec) -> Self {
        let candidate_count = bitmap.count_ones();
        let total_vectors = bitmap.len();
        let selectivity = candidate_count as f32 / total_vectors.max(1) as f32;

        Self {
            bitmap,
            candidate_count,
            selectivity,
        }
    }

    pub fn bitmap(&self) -> &BitVec {
        &self.bitmap
    }
}

impl FilterMask for DenseFilterMask {
    fn contains_vector(&self, vector_id: u32) -> bool {
        self.bitmap
            .get(vector_id as usize)
            .map(|bit| *bit)
            .unwrap_or(false)
    }

    fn selectivity(&self) -> f32 {
        self.selectivity
    }

    fn candidate_count(&self) -> usize {
        self.candidate_count
    }

    fn total_vectors(&self) -> usize {
        self.bitmap.len()
    }

    fn iter_candidates(&self) -> Box<dyn Iterator<Item = u32> + '_> {
        Box::new(self.bitmap.iter_ones().map(|i| i as u32))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implement DiskANN FilterMask trait for DenseFilterMask  
impl DiskANNFilterMask for DenseFilterMask {
    fn contains_vector(&self, vector_id: u32) -> bool {
        FilterMask::contains_vector(self, vector_id)
    }
}

/// Factory function to create optimal FilterMask based on selectivity
pub fn create_filter_mask(candidates: Vec<u32>, total_vectors: usize) -> Box<dyn FilterMask> {
    let selectivity = candidates.len() as f32 / total_vectors.max(1) as f32;

    // Adaptive threshold: use sparse representation for low selectivity
    if selectivity < 0.1 {
        Box::new(SparseFilterMask::new(
            selectivity,
            candidates,
            total_vectors,
        ))
    } else {
        Box::new(DenseFilterMask::new(selectivity, candidates, total_vectors))
    }
}
