use bitvec::prelude::*;
use diskann::common::FilterIndex as DiskANNFilterMask;

/// Selectivity threshold for choosing between search strategies.
/// Below this threshold (e.g., < 10% selectivity), use brute force search.
/// Above this threshold (e.g. >= 10% selectivity), use ann search.
pub const SELECTIVITY_THRESHOLD: f32 = 0.1;

/// Filter mask using BitVec for vector index filtering
#[derive(Debug, Clone)]
pub struct FilterMask {
    bitmap: BitVec,
    candidate_count: usize,
}

impl FilterMask {
    pub fn new(candidates: Vec<u32>, total_vector_num: usize) -> Self {
        let mut bitmap = bitvec![0; total_vector_num];
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
        }
    }

    pub fn bitmap(&self) -> &BitVec {
        &self.bitmap
    }
}

impl FilterMask {
    pub fn contains_vector(&self, vector_id: u32) -> bool {
        self.bitmap
            .get(vector_id as usize)
            .map(|bit| *bit)
            .unwrap_or(false)
    }

    pub fn selectivity(&self) -> f32 {
        self.candidate_count as f32 / self.bitmap.len().max(1) as f32
    }

    pub fn candidate_count(&self) -> usize {
        self.candidate_count
    }

    pub fn total_vector_num(&self) -> usize {
        self.bitmap.len()
    }

    pub fn iter_candidates(&self) -> impl Iterator<Item = u32> + '_ {
        self.bitmap.iter_ones().map(|i| i as u32)
    }
}

/// Implement DiskANN FilterMask trait for FilterMask
impl DiskANNFilterMask for FilterMask {
    fn contains_vector(&self, vector_id: u32) -> bool {
        self.contains_vector(vector_id)
    }
}

/// Factory function to create FilterMask
pub fn create_filter_mask(candidates: Vec<u32>, total_vector_num: usize) -> FilterMask {
    FilterMask::new(candidates, total_vector_num)
}

#[cfg(test)]
mod tests {
    use diskann::common::FilterIndex as DiskANNFilterMask;

    use super::*;

    #[test]
    fn test_filter_mask_creation() {
        let candidates = vec![1, 3, 5, 7, 9];
        let total_vector_num = 100;

        let mask = FilterMask::new(candidates.clone(), total_vector_num);

        assert_eq!(mask.candidate_count(), 5);
        assert_eq!(mask.total_vector_num(), total_vector_num);
        assert_eq!(mask.selectivity(), 5.0 / 100.0);

        // Verify all candidates are present
        for &candidate in &candidates {
            assert!(mask.contains_vector(candidate));
        }
    }

    #[test]
    fn test_filter_mask_contains_vector() {
        let candidates = vec![0, 5, 10, 15];
        let total_vector_num = 20;
        let mask = FilterMask::new(candidates, total_vector_num);

        // Test positive cases
        assert!(mask.contains_vector(0));
        assert!(mask.contains_vector(15));

        // Test negative cases
        assert!(!mask.contains_vector(1));
        assert!(!mask.contains_vector(19));
    }

    #[test]
    fn test_filter_mask_selectivity() {
        let candidates = vec![1, 2, 3, 4, 5];
        let total_vector_num = 10;
        let mask = FilterMask::new(candidates, total_vector_num);

        assert_eq!(mask.selectivity(), 0.5);

        // Test with different ratios
        let mask2 = FilterMask::new(vec![0, 1], 100);
        assert_eq!(mask2.selectivity(), 0.02);

        let mask3 = FilterMask::new(vec![], 50);
        assert_eq!(mask3.selectivity(), 0.0);
    }

    #[test]
    fn test_filter_mask_candidate_count() {
        let mask1 = FilterMask::new(vec![1, 2, 3], 100);
        assert_eq!(mask1.candidate_count(), 3);

        let mask2 = FilterMask::new(vec![], 100);
        assert_eq!(mask2.candidate_count(), 0);
    }

    #[test]
    fn test_filter_mask_total_vector_num() {
        let mask1 = FilterMask::new(vec![1, 2, 3], 100);
        assert_eq!(mask1.total_vector_num(), 100);

        let mask2 = FilterMask::new(vec![1, 2, 3], 50);
        assert_eq!(mask2.total_vector_num(), 50);
    }

    #[test]
    fn test_filter_mask_iter_candidates() {
        let candidates = vec![1, 3, 5, 7, 9];
        let mask = FilterMask::new(candidates, 100);

        let iterated: Vec<u32> = mask.iter_candidates().collect();
        assert_eq!(iterated, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_filter_mask_iteration_order() {
        let candidates = vec![9, 1, 5, 3, 7]; // Unordered input
        let mask = FilterMask::new(candidates, 100);

        let iterated: Vec<u32> = mask.iter_candidates().collect();
        assert_eq!(iterated, vec![1, 3, 5, 7, 9]); // Should be sorted
    }

    #[test]
    fn test_filter_mask_out_of_bounds() {
        let candidates = vec![1, 3, 15, 150]; // 150 is out of bounds for size 100
        let total_vector_num = 100;
        let mask = FilterMask::new(candidates, total_vector_num);

        // Should only count valid candidates (ignore 150)
        assert_eq!(mask.candidate_count(), 3);
        assert_eq!(mask.total_vector_num(), total_vector_num);

        // Verify valid candidates work
        assert!(mask.contains_vector(1));
        assert!(mask.contains_vector(15));

        // Out of bounds should return false
        assert!(!mask.contains_vector(150));
    }

    #[test]
    fn test_filter_mask_all_vectors_selected() {
        let candidates = (0..10).collect::<Vec<u32>>();
        let total_vector_num = 10;
        let mask = FilterMask::new(candidates, total_vector_num);

        assert_eq!(mask.candidate_count(), 10);
        assert_eq!(mask.total_vector_num(), 10);
        assert_eq!(mask.selectivity(), 1.0);

        // All vectors should be included
        for i in 0..10 {
            assert!(mask.contains_vector(i));
        }

        // Iterator should include all vectors
        let iterated: Vec<u32> = mask.iter_candidates().collect();
        assert_eq!(iterated, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_filter_mask_diskann_trait() {
        let candidates = vec![1, 3, 5];
        let mask = FilterMask::new(candidates, 100);

        // Test DiskANN FilterMask trait implementation
        assert!(DiskANNFilterMask::contains_vector(&mask, 1));
        assert!(DiskANNFilterMask::contains_vector(&mask, 5));
        assert!(!DiskANNFilterMask::contains_vector(&mask, 2));
    }

    #[test]
    fn test_filter_mask_bitmap_access() {
        let candidates = vec![1, 3, 5];
        let mask = FilterMask::new(candidates, 20);

        let bitmap = mask.bitmap();
        assert_eq!(bitmap.len(), 20);
        assert!(bitmap[1]);
        assert!(bitmap[3]);
        assert!(!bitmap[2]);
        assert!(!bitmap[19]);
    }

    #[test]
    fn test_filter_mask_large_scale() {
        // Test with larger dataset to ensure efficiency
        let candidates: Vec<u32> = (0..10000).step_by(10).collect(); // Every 10th vector
        let total_vector_num = 10000;
        let mask = FilterMask::new(candidates, total_vector_num);

        assert_eq!(mask.candidate_count(), 1000);
        assert_eq!(mask.total_vector_num(), 10000);
        assert_eq!(mask.selectivity(), 0.1);

        // Test some random samples
        assert!(mask.contains_vector(0));
        assert!(mask.contains_vector(5000));
        assert!(mask.contains_vector(9990));
        assert!(!mask.contains_vector(1));
        assert!(!mask.contains_vector(5001));
    }
}
