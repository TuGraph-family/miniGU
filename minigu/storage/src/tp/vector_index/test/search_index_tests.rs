//! 搜索索引功能测试
//! 
//! 测试 InMemDiskANNAdapter 的搜索索引功能，包括：
//! - 基本搜索功能
//! - 搜索参数验证
//! - ID映射正确性
//! - 不同k值的搜索
//! - 错误处理
//! - 边界情况

use super::*;
use super::test_utils::*;
use crate::{assert_vector_index_error};

#[cfg(test)]
mod tests {
    use super::*;

    // =================================
    // 基本搜索功能测试
    // =================================

    /// 测试基本的向量搜索功能
    #[test]
    fn test_basic_search() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(10, TEST_DIMENSION);
        let build_params = create_test_build_params();
        let search_params = create_test_search_params();

        // 构建索引
        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        // 使用第一个向量作为查询
        let query = &test_vectors[0].1;
        let k = 3;

        let results = adapter.search(query, k, search_params).expect("Search should succeed");
        
        // 验证结果
        assert!(!results.is_empty(), "Search should return results");
        assert!(results.len() <= k, "Results should not exceed k");
        assert!(results.len() <= test_vectors.len(), "Results should not exceed total vectors");
        
        // 第一个结果应该是查询向量本身（最相似）
        assert_eq!(results[0], test_vectors[0].0, "First result should be the query vector itself");
    }

    /// 测试不同k值的搜索
    #[test]
    fn test_search_with_different_k() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(10, TEST_DIMENSION);
        let build_params = create_test_build_params();
        let search_params = create_test_search_params();

        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        let query = &test_vectors[0].1;

        // 测试不同的k值
        for k in 1..=5 {
            let results = adapter.search(query, k, search_params.clone()).expect("Search should succeed");
            assert!(results.len() <= k, "Results count should not exceed k={}", k);
            assert!(results.len() <= test_vectors.len(), "Results should not exceed total vectors");
        }
    }

    /// 测试k值大于索引大小的情况
    #[test]
    fn test_search_k_larger_than_index() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(5, TEST_DIMENSION);
        let build_params = create_test_build_params();
        let search_params = create_test_search_params();

        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        let query = &test_vectors[0].1;
        let k = 10; // 大于向量数量

        let results = adapter.search(query, k, search_params).expect("Search should succeed");
        
        // 结果数量应该不超过索引中的向量数量，但由于DiskANN的内部图连接性，可能会少于索引大小
        assert!(results.len() <= test_vectors.len(), "Results should not exceed index size");
        assert!(!results.is_empty(), "Should return at least some results");
    }

    /// 测试随机查询向量的搜索
    #[test]
    fn test_search_random_query() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_random_test_vectors(20, TEST_DIMENSION, 12345);
        let build_params = create_test_build_params();
        let search_params = create_test_search_params();

        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        // 生成随机查询向量
        let query_vectors = generate_random_test_vectors(3, TEST_DIMENSION, 54321);
        
        for (_, query) in query_vectors.iter() {
            let results = adapter.search(query, 5, search_params.clone()).expect("Search should succeed");
            assert!(!results.is_empty(), "Search should return results");
            assert!(results.len() <= 5, "Results should not exceed k=5");
            
            // 验证返回的node IDs都是有效的
            for &node_id in &results {
                assert!(test_vectors.iter().any(|(id, _)| *id == node_id), 
                       "Returned node ID {} should exist in original vectors", node_id);
            }
        }
    }

    /// 测试搜索结果的唯一性
    #[test]
    fn test_search_result_uniqueness() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(10, TEST_DIMENSION);
        let build_params = create_test_build_params();
        let search_params = create_test_search_params();

        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        let query = &test_vectors[0].1;
        let results = adapter.search(query, 5, search_params).expect("Search should succeed");
        
        // 验证结果中没有重复的node ID
        let mut seen_ids = std::collections::HashSet::new();
        for &node_id in &results {
            assert!(seen_ids.insert(node_id), "Duplicate node ID {} in search results", node_id);
        }
    }

    // =================================
    // 搜索参数验证测试
    // =================================

    /// 测试无效的k值
    #[test]
    fn test_invalid_k_values() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(5, TEST_DIMENSION);
        let build_params = create_test_build_params();
        let search_params = create_test_search_params();

        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        let query = &test_vectors[0].1;

        // 测试k=0
        let result = adapter.search(query, 0, search_params.clone());
        assert_vector_index_error!(result, VectorIndexError::InvalidSearchParams(_));
    }

    /// 测试无效的SearchParams参数
    #[test]
    fn test_invalid_search_params() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(5, TEST_DIMENSION);
        let build_params = create_test_build_params();

        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        let query = &test_vectors[0].1;
        let invalid_params_list = create_invalid_search_params();

        for invalid_params in invalid_params_list {
            let result = adapter.search(query, 3, invalid_params);
            assert_vector_index_error!(result, VectorIndexError::InvalidSearchParams(_));
        }
    }

    /// 测试有效边界搜索参数
    #[test]
    fn test_valid_boundary_search_params() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(5, TEST_DIMENSION);
        let build_params = create_test_build_params();

        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        let query = &test_vectors[0].1;

        // 测试最小有效参数 - DiskANN要求ef_search >= k
        let minimal_params = SearchParams {
            ef_search: 3,  // ef_search必须至少等于k
        };

        let result = adapter.search(query, 3, minimal_params);
        assert!(result.is_ok(), "Minimal valid search params should work: {:?}", result);
    }

    // =================================
    // 查询向量验证测试
    // =================================

    /// 测试维度不匹配的查询向量
    #[test]
    fn test_search_dimension_mismatch() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(5, TEST_DIMENSION);
        let build_params = create_test_build_params();
        let search_params = create_test_search_params();

        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        // 错误维度的查询向量
        let wrong_dim_query = vec![1.0; TEST_DIMENSION - 1];
        let result = adapter.search(&wrong_dim_query, 3, search_params);
        
        assert_vector_index_error!(result, VectorIndexError::InvalidDimension { expected: _, actual: _ });
    }

    /// 测试包含非有限值的查询向量
    #[test]
    fn test_search_non_finite_query() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(5, TEST_DIMENSION);
        let build_params = create_test_build_params();
        let search_params = create_test_search_params();

        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        // 包含NaN的查询向量
        let mut nan_query = vec![1.0; TEST_DIMENSION];
        nan_query[0] = f32::NAN;
        let result = adapter.search(&nan_query, 3, search_params.clone());
        assert_vector_index_error!(result, VectorIndexError::DataConversion(_));

        // 包含无穷大的查询向量
        let mut inf_query = vec![1.0; TEST_DIMENSION];
        inf_query[0] = f32::INFINITY;
        let result = adapter.search(&inf_query, 3, search_params);
        assert_vector_index_error!(result, VectorIndexError::DataConversion(_));
    }

    /// 测试空查询向量
    #[test]
    fn test_search_empty_query() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(5, TEST_DIMENSION);
        let build_params = create_test_build_params();
        let search_params = create_test_search_params();

        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        let empty_query: Vec<f32> = vec![];
        let result = adapter.search(&empty_query, 3, search_params);
        
        assert_vector_index_error!(result, VectorIndexError::InvalidDimension { expected: _, actual: _ });
    }

    // =================================
    // 索引状态验证测试
    // =================================

    /// 测试在未构建索引时搜索
    #[test]
    fn test_search_without_building_index() {
        let adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let query = vec![1.0; TEST_DIMENSION];
        let search_params = create_test_search_params();

        let result = adapter.search(&query, 3, search_params);
        assert_vector_index_error!(result, VectorIndexError::IndexNotBuilt);
    }

    /// 测试搜索性能统计
    #[test]
    fn test_search_statistics() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(10, TEST_DIMENSION);
        let build_params = create_test_build_params();
        let search_params = create_test_search_params();

        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        let query = &test_vectors[0].1;
        
        // 执行多次搜索
        for _ in 0..3 {
            adapter.search(query, 5, search_params.clone()).expect("Search should succeed");
        }

        // 验证搜索统计被正确更新
        let stats = adapter.stats();
        assert_eq!(stats.search_count, 3, "Search count should be updated");
    }

    // =================================
    // 边界情况和压力测试
    // =================================

    /// 测试大量向量的搜索性能
    #[test]
    fn test_large_scale_search() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_random_test_vectors(1000, TEST_DIMENSION, 12345);
        let build_params = create_test_build_params();
        let search_params = create_test_search_params();

        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        // 执行多个查询
        let query_vectors = generate_random_test_vectors(10, TEST_DIMENSION, 54321);
        
        for (_, query) in query_vectors.iter() {
            let results = adapter.search(query, 10, search_params.clone()).expect("Search should succeed");
            assert!(!results.is_empty(), "Search should return results");
            assert!(results.len() <= 10, "Results should not exceed k=10");
            
            // 验证所有返回的node ID都存在于原始数据中
            for &node_id in &results {
                assert!(test_vectors.iter().any(|(id, _)| *id == node_id), 
                       "Returned node ID should exist in original data");
            }
        }
    }

    /// 测试极端搜索参数
    #[test]
    fn test_extreme_search_parameters() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(10, TEST_DIMENSION);
        let build_params = create_test_build_params();

        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        let query = &test_vectors[0].1;

        // 测试非常大的ef_search值
        let large_ef_params = SearchParams {
            ef_search: 1000,
        };
        
        let result = adapter.search(query, 5, large_ef_params);
        assert!(result.is_ok(), "Large ef_search should work: {:?}", result);

        // 测试k=1的情况
        let result = adapter.search(query, 1, create_test_search_params());
        assert!(result.is_ok(), "k=1 should work: {:?}", result);
        let results = result.unwrap();
        assert_eq!(results.len(), 1, "Should return exactly one result");
    }

    /// 测试搜索结果的一致性
    #[test]
    fn test_search_consistency() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(10, TEST_DIMENSION);
        let build_params = create_test_build_params();
        let search_params = create_test_search_params();

        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        let query = &test_vectors[0].1;

        // 多次执行相同的搜索，结果应该一致
        let first_results = adapter.search(query, 5, search_params.clone()).expect("Search should succeed");
        
        for _ in 0..3 {
            let results = adapter.search(query, 5, search_params.clone()).expect("Search should succeed");
            assert_eq!(results, first_results, "Search results should be consistent");
        }
    }

    /// 测试不同类型查询向量的搜索
    #[test]
    fn test_search_different_query_types() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(10, TEST_DIMENSION);
        let build_params = create_test_build_params();
        let search_params = create_test_search_params();

        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        // 全零向量
        let zero_query = vec![0.0; TEST_DIMENSION];
        let result = adapter.search(&zero_query, 3, search_params.clone());
        assert!(result.is_ok(), "Zero query should work: {:?}", result);

        // 全一向量
        let ones_query = vec![1.0; TEST_DIMENSION];
        let result = adapter.search(&ones_query, 3, search_params.clone());
        assert!(result.is_ok(), "Ones query should work: {:?}", result);

        // 负值向量
        let negative_query = vec![-1.0; TEST_DIMENSION];
        let result = adapter.search(&negative_query, 3, search_params);
        assert!(result.is_ok(), "Negative query should work: {:?}", result);
    }
}