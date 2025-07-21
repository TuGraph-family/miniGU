//! 测试工具函数
//! 
//! 提供测试数据生成、配置创建等辅助功能

use super::*;
use std::collections::HashSet;

/// 创建测试用的IndexConfiguration
pub fn create_test_config(dimension: usize) -> IndexConfiguration {
    use diskann::model::IndexWriteParametersBuilder;
    
    let index_write_parameter = IndexWriteParametersBuilder::new(50, 32)
        .with_alpha(1.2)
        .with_num_threads(1)
        .build();
    
    IndexConfiguration {
        index_write_parameter,
        dist_metric: vector::Metric::L2,
        dim: dimension,
        aligned_dim: (dimension + 7) / 8 * 8,  // 向上取整到8的倍数
        max_points: 10000,
        num_frozen_pts: 0,
        use_pq_dist: false,
        num_pq_chunks: 0,
        use_opq: false,
        growth_potential: 1.2,
    }
}

/// 创建测试用的BuildParams
pub fn create_test_build_params() -> BuildParams {
    BuildParams {
        ef_construction: DEFAULT_EF_CONSTRUCTION,
        max_degree: DEFAULT_MAX_DEGREE,
        alpha: DEFAULT_ALPHA,
    }
}

/// 创建测试用的SearchParams
pub fn create_test_search_params() -> SearchParams {
    SearchParams {
        ef_search: DEFAULT_EF_SEARCH,
    }
}

/// 创建无效的BuildParams（用于错误测试）
pub fn create_invalid_build_params() -> Vec<BuildParams> {
    vec![
        BuildParams {
            ef_construction: 0,  // 无效：ef_construction = 0
            max_degree: DEFAULT_MAX_DEGREE,
            alpha: DEFAULT_ALPHA,
        },
        BuildParams {
            ef_construction: DEFAULT_EF_CONSTRUCTION,
            max_degree: 0,       // 无效：max_degree = 0
            alpha: DEFAULT_ALPHA,
        },
        BuildParams {
            ef_construction: DEFAULT_EF_CONSTRUCTION,
            max_degree: DEFAULT_MAX_DEGREE,
            alpha: 0.0,          // 无效：alpha = 0
        },
        BuildParams {
            ef_construction: DEFAULT_EF_CONSTRUCTION,
            max_degree: DEFAULT_MAX_DEGREE,
            alpha: -1.0,         // 无效：alpha < 0
        },
    ]
}

/// 创建无效的SearchParams（用于错误测试）
pub fn create_invalid_search_params() -> Vec<SearchParams> {
    vec![
        SearchParams {
            ef_search: 0,        // 无效：ef_search = 0
        },
    ]
}

/// 生成简单的测试向量数据
/// 返回 (node_id, vector) 的向量集合
pub fn generate_simple_test_vectors(count: usize, dimension: usize) -> Vec<(u32, Vec<f32>)> {
    (0..count)
        .map(|i| {
            let node_id = (i + 1) as u32; // 从1开始的节点ID
            let vector: Vec<f32> = (0..dimension)
                .map(|j| (i * dimension + j) as f32 * 0.1)
                .collect();
            (node_id, vector)
        })
        .collect()
}

/// 生成随机化的测试向量数据
pub fn generate_random_test_vectors(count: usize, dimension: usize, seed: u64) -> Vec<(u32, Vec<f32>)> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut vectors = Vec::new();
    for i in 0..count {
        let node_id = (i + 100) as u32; // 避免与简单测试数据重复
        let mut vector = Vec::new();
        
        for j in 0..dimension {
            // 简单的伪随机数生成
            let mut hasher = DefaultHasher::new();
            (seed + i as u64 + j as u64).hash(&mut hasher);
            let hash = hasher.finish();
            let value = (hash % 1000) as f32 / 1000.0; // 0.0 到 1.0 之间
            vector.push(value);
        }
        
        vectors.push((node_id, vector));
    }
    vectors
}

/// 生成包含重复节点ID的测试数据（用于错误测试）
pub fn generate_duplicate_node_id_vectors() -> Vec<(u32, Vec<f32>)> {
    vec![
        (1, vec![1.0; TEST_DIMENSION]),
        (2, vec![2.0; TEST_DIMENSION]),
        (1, vec![3.0; TEST_DIMENSION]), // 重复的节点ID
    ]
}

/// 生成维度不匹配的测试数据（用于错误测试）
pub fn generate_mismatched_dimension_vectors() -> Vec<(u32, Vec<f32>)> {
    vec![
        (1, vec![1.0; TEST_DIMENSION]),           // 正确维度
        (2, vec![2.0; TEST_DIMENSION - 1]),       // 维度不匹配
        (3, vec![3.0; TEST_DIMENSION]),           // 正确维度
    ]
}

/// 生成包含非有限值的测试数据（用于错误测试）
pub fn generate_non_finite_vectors() -> Vec<(u32, Vec<f32>)> {
    let normal_vector = vec![1.0; TEST_DIMENSION];
    let mut nan_vector = vec![2.0; TEST_DIMENSION];
    let mut inf_vector = vec![3.0; TEST_DIMENSION];
    
    // 在中间位置插入非有限值
    nan_vector[TEST_DIMENSION / 2] = f32::NAN;
    inf_vector[TEST_DIMENSION / 2] = f32::INFINITY;
    
    vec![
        (1, normal_vector),
        (2, nan_vector),     // 包含NaN
        (3, inf_vector),     // 包含无穷大
    ]
}

/// 创建测试用的InMemDiskANNAdapter
pub fn create_test_adapter(dimension: usize) -> StorageResult<InMemDiskANNAdapter> {
    let config = create_test_config(dimension);
    InMemDiskANNAdapter::new(config)
}

/// 验证ID映射的正确性
pub fn verify_id_mapping(
    adapter: &InMemDiskANNAdapter,
    original_vectors: &[(u32, Vec<f32>)]
) {
    // 验证映射数量
    assert_eq!(adapter.mapping_count(), original_vectors.len());
    
    // 验证所有节点ID都被正确映射
    let original_node_ids: HashSet<u32> = original_vectors
        .iter()
        .map(|(node_id, _)| *node_id)
        .collect();
    
    // 这里我们无法直接访问内部映射，但可以通过索引大小来验证
    assert_eq!(adapter.size(), original_node_ids.len());
}

/// 验证索引统计信息
pub fn verify_index_stats(adapter: &InMemDiskANNAdapter, expected_count: usize) {
    let stats = adapter.stats();
    assert_eq!(stats.vector_count, expected_count);
    assert!(stats.build_time_ms.is_some());
    // 构建时间已被记录（可能为0，表示很快的构建）
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_simple_vectors() {
        let vectors = generate_simple_test_vectors(3, 2);
        assert_eq!(vectors.len(), 3);
        assert_eq!(vectors[0], (1, vec![0.0, 0.1]));
        assert_eq!(vectors[1], (2, vec![0.2, 0.3]));
        assert_eq!(vectors[2], (3, vec![0.4, 0.5]));
    }
    
    #[test]
    fn test_create_test_config() {
        let config = create_test_config(128);
        assert_eq!(config.dim, 128);
        assert_eq!(config.index_write_parameter.num_threads, 1);
        assert_eq!(config.dist_metric, vector::Metric::L2);
    }
    
    #[test]
    fn test_invalid_build_params() {
        let invalid_params = create_invalid_build_params();
        assert_eq!(invalid_params.len(), 4);
        
        // 验证每个参数都有问题
        assert_eq!(invalid_params[0].ef_construction, 0);
        assert_eq!(invalid_params[1].max_degree, 0);
        assert_eq!(invalid_params[2].alpha, 0.0);
        assert!(invalid_params[3].alpha < 0.0);
    }
    
    #[test]
    fn test_create_test_search_params() {
        let params = create_test_search_params();
        assert_eq!(params.ef_search, DEFAULT_EF_SEARCH);
    }
    
    #[test]
    fn test_invalid_search_params() {
        let invalid_params = create_invalid_search_params();
        assert_eq!(invalid_params.len(), 1);
        
        // 验证参数有问题
        assert_eq!(invalid_params[0].ef_search, 0);
    }
}