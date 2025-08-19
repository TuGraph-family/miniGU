use std::any::Any;

use bitvec::prelude::*;
use diskann::common::FilterIndex as DiskANNFilterMask;

/// Selectivity threshold for choosing between sparse/dense representations and search strategies.
/// Below this threshold (e.g., < 10% selectivity), use sparse representation and brute force
/// search. Above this threshold (e.g. >= 10% selectivity), use dense representation and index-based
/// search.
pub const SELECTIVITY_THRESHOLD: f32 = 0.1;

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
    if selectivity < SELECTIVITY_THRESHOLD {
        Box::new(SparseFilterMask::new(
            selectivity,
            candidates,
            total_vectors,
        ))
    } else {
        Box::new(DenseFilterMask::new(selectivity, candidates, total_vectors))
    }
}

#[cfg(test)]
mod tests {
    use diskann::common::FilterIndex as DiskANNFilterMask;

    use super::*;

    #[test]
    fn test_sparse_filter_mask_creation() {
        let candidates = vec![1, 3, 5, 7, 9];
        let total_vectors = 100;
        let selectivity = 0.05;

        let mask = SparseFilterMask::new(selectivity, candidates.clone(), total_vectors);

        assert_eq!(mask.selectivity(), selectivity);
        assert_eq!(mask.candidate_count(), 5);
        assert_eq!(mask.total_vectors(), total_vectors);
        assert_eq!(mask.candidates(), &[1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_sparse_filter_mask_deduplication_and_sorting() {
        // Test with unsorted candidates and duplicates
        let candidates = vec![9, 1, 5, 3, 7, 1, 5];
        let total_vectors = 100;
        let selectivity = 0.05;

        let mask = SparseFilterMask::new(selectivity, candidates, total_vectors);

        // Should be deduplicated and sorted
        assert_eq!(mask.candidates(), &[1, 3, 5, 7, 9]);
        assert_eq!(mask.candidate_count(), 5);
    }

    #[test]
    fn test_sparse_filter_mask_contains() {
        let candidates = vec![1, 3, 5, 7, 9];
        let mask = SparseFilterMask::new(0.05, candidates, 100);

        // Test positive cases
        assert!(FilterMask::contains_vector(&mask, 1));
        assert!(FilterMask::contains_vector(&mask, 3));
        assert!(FilterMask::contains_vector(&mask, 5));
        assert!(FilterMask::contains_vector(&mask, 7));
        assert!(FilterMask::contains_vector(&mask, 9));

        // Test negative cases
        assert!(!FilterMask::contains_vector(&mask, 0));
        assert!(!FilterMask::contains_vector(&mask, 2));
        assert!(!FilterMask::contains_vector(&mask, 4));
        assert!(!FilterMask::contains_vector(&mask, 10));
        assert!(!FilterMask::contains_vector(&mask, 99));
    }

    #[test]
    fn test_sparse_filter_mask_iteration() {
        let candidates = vec![1, 3, 5, 7, 9];
        let mask = SparseFilterMask::new(0.05, candidates, 100);

        let iterated: Vec<u32> = mask.iter_candidates().collect();
        assert_eq!(iterated, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_sparse_filter_mask_diskann_compatibility() {
        let candidates = vec![1, 3, 5];
        let mask = SparseFilterMask::new(0.03, candidates, 100);

        // Test DiskANN FilterMask trait implementation
        assert!(DiskANNFilterMask::contains_vector(&mask, 1));
        assert!(DiskANNFilterMask::contains_vector(&mask, 3));
        assert!(!DiskANNFilterMask::contains_vector(&mask, 2));
    }

    #[test]
    fn test_dense_filter_mask_creation_from_candidates() {
        let candidates = vec![1, 3, 5, 7, 9];
        let total_vectors = 10;
        let selectivity = 0.5;

        let mask = DenseFilterMask::new(selectivity, candidates, total_vectors);

        assert_eq!(mask.selectivity(), selectivity);
        assert_eq!(mask.candidate_count(), 5);
        assert_eq!(mask.total_vectors(), total_vectors);
    }

    #[test]
    fn test_dense_filter_mask_creation_from_bitmap() {
        let mut bitmap = bitvec![0; 10];
        bitmap.set(1, true);
        bitmap.set(3, true);
        bitmap.set(5, true);

        let mask = DenseFilterMask::from_bitmap(bitmap);

        assert_eq!(mask.candidate_count(), 3);
        assert_eq!(mask.total_vectors(), 10);
        assert_eq!(mask.selectivity(), 0.3);
    }

    #[test]
    fn test_dense_filter_mask_contains() {
        let candidates = vec![1, 3, 5, 7, 9];
        let mask = DenseFilterMask::new(0.5, candidates, 10);

        // Test positive cases
        assert!(FilterMask::contains_vector(&mask, 1));
        assert!(FilterMask::contains_vector(&mask, 3));
        assert!(FilterMask::contains_vector(&mask, 5));
        assert!(FilterMask::contains_vector(&mask, 7));
        assert!(FilterMask::contains_vector(&mask, 9));

        // Test negative cases
        assert!(!FilterMask::contains_vector(&mask, 0));
        assert!(!FilterMask::contains_vector(&mask, 2));
        assert!(!FilterMask::contains_vector(&mask, 4));
        assert!(!FilterMask::contains_vector(&mask, 6));
        assert!(!FilterMask::contains_vector(&mask, 8));
    }

    #[test]
    fn test_dense_filter_mask_iteration() {
        let candidates = vec![1, 3, 5, 7, 9];
        let mask = DenseFilterMask::new(0.5, candidates, 10);

        let iterated: Vec<u32> = mask.iter_candidates().collect();
        assert_eq!(iterated, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_dense_filter_mask_deduplication() {
        // Test with duplicate candidates
        let candidates = vec![1, 3, 5, 3, 1];
        let mask = DenseFilterMask::new(0.3, candidates, 10);

        // Should only count unique candidates
        assert_eq!(mask.candidate_count(), 3);

        let iterated: Vec<u32> = mask.iter_candidates().collect();
        assert_eq!(iterated, vec![1, 3, 5]);
    }

    #[test]
    fn test_dense_filter_mask_out_of_bounds() {
        let candidates = vec![1, 3, 15]; // 15 is out of bounds for size 10
        let mask = DenseFilterMask::new(0.3, candidates, 10);

        // Should only count valid candidates
        assert_eq!(mask.candidate_count(), 2);
        assert!(FilterMask::contains_vector(&mask, 1));
        assert!(FilterMask::contains_vector(&mask, 3));
        assert!(!FilterMask::contains_vector(&mask, 15));
    }

    #[test]
    fn test_dense_filter_mask_diskann_compatibility() {
        let candidates = vec![1, 3, 5];
        let mask = DenseFilterMask::new(0.3, candidates, 10);

        // Test DiskANN FilterMask trait implementation
        assert!(DiskANNFilterMask::contains_vector(&mask, 1));
        assert!(DiskANNFilterMask::contains_vector(&mask, 3));
        assert!(!DiskANNFilterMask::contains_vector(&mask, 2));
    }

    #[test]
    fn test_create_filter_mask_low_selectivity() {
        let candidates = vec![1, 3, 5]; // 3/100 = 0.03 < 0.1
        let total_vectors = 100;

        let mask = create_filter_mask(candidates, total_vectors);

        // Should create SparseFilterMask for low selectivity
        assert!(mask.as_any().downcast_ref::<SparseFilterMask>().is_some());
        assert_eq!(mask.selectivity(), 0.03);
        assert_eq!(mask.candidate_count(), 3);
    }

    #[test]
    fn test_create_filter_mask_high_selectivity() {
        let candidates = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // 10/50 = 0.2 > 0.1
        let total_vectors = 50;

        let mask = create_filter_mask(candidates, total_vectors);

        // Should create DenseFilterMask for high selectivity
        assert!(mask.as_any().downcast_ref::<DenseFilterMask>().is_some());
        assert_eq!(mask.selectivity(), 0.2);
        assert_eq!(mask.candidate_count(), 10);
    }

    #[test]
    fn test_create_filter_mask_boundary_case() {
        let candidates = vec![1, 2, 3, 4, 5]; // 5/50 = SELECTIVITY_THRESHOLD (exactly at threshold)
        let total_vectors = 50;

        let mask = create_filter_mask(candidates, total_vectors);

        // At exactly SELECTIVITY_THRESHOLD, should use DenseFilterMask (>= SELECTIVITY_THRESHOLD)
        assert!(mask.as_any().downcast_ref::<DenseFilterMask>().is_some());
        assert_eq!(mask.selectivity(), SELECTIVITY_THRESHOLD);
    }

    #[test]
    fn test_empty_candidates() {
        let candidates = vec![];
        let total_vectors = 100;

        let mask = create_filter_mask(candidates, total_vectors);

        assert_eq!(mask.candidate_count(), 0);
        assert_eq!(mask.selectivity(), 0.0);
        assert!(!FilterMask::contains_vector(mask.as_ref(), 0));
        assert!(!FilterMask::contains_vector(mask.as_ref(), 50));

        let iterated: Vec<u32> = mask.iter_candidates().collect();
        assert!(iterated.is_empty());
    }

    #[test]
    fn test_zero_total_vectors() {
        let candidates = vec![];
        let total_vectors = 0;

        let mask = create_filter_mask(candidates, total_vectors);

        assert_eq!(mask.candidate_count(), 0);
        assert_eq!(mask.total_vectors(), 0);
        // Should handle division by zero gracefully
        assert_eq!(mask.selectivity(), 0.0);
    }

    #[test]
    fn test_all_vectors_selected() {
        let candidates = (0..10).collect::<Vec<u32>>();
        let total_vectors = 10;

        let mask = create_filter_mask(candidates, total_vectors);

        assert_eq!(mask.candidate_count(), 10);
        assert_eq!(mask.selectivity(), 1.0);

        // All vectors should be included
        for i in 0..10 {
            assert!(FilterMask::contains_vector(mask.as_ref(), i));
        }
    }

    #[test]
    fn test_filter_mask_trait_object_behavior() {
        let sparse_mask: Box<dyn FilterMask> =
            Box::new(SparseFilterMask::new(0.05, vec![1, 3, 5], 100));

        let dense_mask: Box<dyn FilterMask> =
            Box::new(DenseFilterMask::new(0.3, vec![1, 3, 5], 10));

        // Both should work through trait object
        assert!(FilterMask::contains_vector(sparse_mask.as_ref(), 1));
        assert!(!FilterMask::contains_vector(sparse_mask.as_ref(), 2));
        assert_eq!(sparse_mask.candidate_count(), 3);

        assert!(FilterMask::contains_vector(dense_mask.as_ref(), 1));
        assert!(!FilterMask::contains_vector(dense_mask.as_ref(), 2));
        assert_eq!(dense_mask.candidate_count(), 3);
    }
}
