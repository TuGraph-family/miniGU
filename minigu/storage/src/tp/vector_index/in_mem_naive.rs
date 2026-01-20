use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use ordered_float::OrderedFloat;

use super::config::VectorIndexConfig;
use super::filter::{FilterMask, SELECTIVITY_THRESHOLD};
use super::index::{FilterIndex, VectorIndex};
use crate::error::{StorageError, StorageResult, VectorIndexError};

#[allow(clippy::upper_case_acronyms)]
pub struct InMemANNAdapter {
    dimension: usize,
    node_to_vector: HashMap<u64, u32>,
    vector_to_node: Vec<Option<u64>>,
    vectors: Vec<Vec<f32>>,
}

impl InMemANNAdapter {
    pub fn new(config: VectorIndexConfig) -> StorageResult<Self> {
        if config.dimension == 0 {
            return Err(StorageError::VectorIndex(VectorIndexError::InvalidInput(
                "dimension must be > 0".to_string(),
            )));
        }

        Ok(Self {
            dimension: config.dimension,
            node_to_vector: HashMap::new(),
            vector_to_node: Vec::new(),
            vectors: Vec::new(),
        })
    }

    #[inline]
    fn l2_squared(a: &[f32], b: &[f32]) -> f32 {
        let mut sum = 0.0f32;
        for i in 0..a.len() {
            let diff = a[i] - b[i];
            sum += diff * diff;
        }
        sum
    }

    fn validate_vector_dim(&self, vector: &[f32]) -> StorageResult<()> {
        if vector.len() != self.dimension {
            return Err(StorageError::VectorIndex(
                VectorIndexError::InvalidDimension {
                    expected: self.dimension,
                    actual: vector.len(),
                },
            ));
        }
        Ok(())
    }

    fn search_over_iter(
        &self,
        query: &[f32],
        k: usize,
        candidates: impl Iterator<Item = u32>,
    ) -> StorageResult<Vec<(u64, f32)>> {
        self.validate_vector_dim(query)?;

        let k = std::cmp::min(k, self.size());
        if k == 0 {
            return Ok(Vec::new());
        }

        let mut heap: BinaryHeap<(OrderedFloat<f32>, u32)> = BinaryHeap::with_capacity(k);

        for vector_id in candidates {
            let Some(Some(_)) = self.vector_to_node.get(vector_id as usize) else {
                continue;
            };
            let Some(stored) = self.vectors.get(vector_id as usize) else {
                continue;
            };
            if stored.len() != self.dimension {
                continue;
            }

            let dist = Self::l2_squared(query, stored);

            if heap.len() < k {
                heap.push((OrderedFloat(dist), vector_id));
                continue;
            }

            if let Some((max_dist, _)) = heap.peek() {
                if OrderedFloat(dist).cmp(max_dist) == Ordering::Less {
                    heap.pop();
                    heap.push((OrderedFloat(dist), vector_id));
                }
            }
        }

        let mut results = Vec::with_capacity(heap.len());
        for (dist, vector_id) in heap.into_sorted_vec().into_iter() {
            let Some(Some(node_id)) = self.vector_to_node.get(vector_id as usize) else {
                continue;
            };
            results.push((*node_id, dist.0));
        }
        Ok(results)
    }
}

impl VectorIndex for InMemANNAdapter {
    fn build(&mut self, vectors: &[(u64, &[f32])]) -> StorageResult<()> {
        if vectors.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::EmptyDataset));
        }

        self.node_to_vector.clear();
        self.vector_to_node.clear();
        self.vectors.clear();

        for (node_id, vector) in vectors {
            self.validate_vector_dim(vector)?;
            if self.node_to_vector.contains_key(node_id) {
                return Err(StorageError::VectorIndex(
                    VectorIndexError::DuplicateNodeId { node_id: *node_id },
                ));
            }

            let vector_id = self.vectors.len();
            if vector_id > u32::MAX as usize {
                return Err(StorageError::VectorIndex(
                    VectorIndexError::VectorIdOverflow {
                        vector_id: vector_id as u64,
                    },
                ));
            }

            self.node_to_vector.insert(*node_id, vector_id as u32);
            self.vector_to_node.push(Some(*node_id));
            self.vectors.push(vector.to_vec());
        }

        Ok(())
    }

    fn ann_search(
        &self,
        query: &[f32],
        k: usize,
        _l_value: u32,
        filter_mask: Option<&dyn FilterIndex>,
        _should_pre: bool,
    ) -> StorageResult<Vec<(u64, f32)>> {
        if self.node_to_vector.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::IndexNotBuilt));
        }

        match filter_mask {
            None => self.search_over_iter(query, k, 0..(self.vector_to_node.len() as u32)),
            Some(filter) => self.search_over_iter(
                query,
                k,
                (0..(self.vector_to_node.len() as u32)).filter(|vid| filter.contains_vector(*vid)),
            ),
        }
    }

    fn search(
        &self,
        query: &[f32],
        k: usize,
        l_value: u32,
        filter_mask: Option<&FilterMask>,
        should_pre: bool,
    ) -> StorageResult<Vec<(u64, f32)>> {
        let Some(mask) = filter_mask else {
            return self.ann_search(query, k, l_value, None, should_pre);
        };

        if self.node_to_vector.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::IndexNotBuilt));
        }

        if mask.candidate_count() == 0 {
            return Ok(Vec::new());
        }

        let selectivity = mask.selectivity();
        if selectivity < SELECTIVITY_THRESHOLD {
            self.search_over_iter(query, k, mask.iter_candidates())
        } else {
            self.search_over_iter(query, k, mask.iter_candidates())
        }
    }

    fn insert(&mut self, vectors: &[(u64, &[f32])]) -> StorageResult<()> {
        if vectors.is_empty() {
            return Ok(());
        }
        if self.node_to_vector.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::IndexNotBuilt));
        }

        for (node_id, vector) in vectors {
            self.validate_vector_dim(vector)?;
            if self.node_to_vector.contains_key(node_id) {
                return Err(StorageError::VectorIndex(
                    VectorIndexError::DuplicateNodeId { node_id: *node_id },
                ));
            }

            let vector_id = self.vectors.len();
            if vector_id > u32::MAX as usize {
                return Err(StorageError::VectorIndex(
                    VectorIndexError::VectorIdOverflow {
                        vector_id: vector_id as u64,
                    },
                ));
            }

            self.node_to_vector.insert(*node_id, vector_id as u32);
            self.vector_to_node.push(Some(*node_id));
            self.vectors.push(vector.to_vec());
        }

        Ok(())
    }

    fn soft_delete(&mut self, node_ids: &[u64]) -> StorageResult<()> {
        if node_ids.is_empty() {
            return Ok(());
        }
        if self.node_to_vector.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::IndexNotBuilt));
        }

        for node_id in node_ids {
            let Some(vector_id) = self.node_to_vector.remove(node_id) else {
                continue;
            };
            if let Some(slot) = self.vector_to_node.get_mut(vector_id as usize) {
                *slot = None;
            }
        }

        Ok(())
    }

    fn save(&mut self, _path: &str) -> StorageResult<()> {
        Err(StorageError::VectorIndex(VectorIndexError::NotSupported(
            "save() is not supported on WASM".to_string(),
        )))
    }

    fn load(&mut self, _path: &str) -> StorageResult<()> {
        Err(StorageError::VectorIndex(VectorIndexError::NotSupported(
            "load() is not supported on WASM".to_string(),
        )))
    }

    fn get_dimension(&self) -> usize {
        self.dimension
    }

    fn size(&self) -> usize {
        self.node_to_vector.len()
    }

    fn node_to_vector_id(&self, node_id: u64) -> Option<u32> {
        self.node_to_vector.get(&node_id).copied()
    }
}
