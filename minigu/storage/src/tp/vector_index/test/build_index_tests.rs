//! 构建索引功能测试
//! 
//! 测试 InMemDiskANNAdapter 的构建索引功能，包括：
//! - 基本构建功能
//! - ID映射机制
//! - 参数验证
//! - 错误处理
//! - 边界情况

use super::*;
use super::test_utils::*;
use crate::{assert_vector_index_error, assert_build_params_error};

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试基本的索引构建功能
    #[test]
    fn test_basic_index_build() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(5, TEST_DIMENSION);
        let build_params = create_test_build_params();

        // 构建索引
        let result = adapter.build(&test_vectors, build_params);
        assert!(result.is_ok(), "Index build should succeed: {:?}", result);

        // 验证索引状态
        verify_id_mapping(&adapter, &test_vectors);
        verify_index_stats(&adapter, test_vectors.len());
    }

    /// 测试较大规模的索引构建
    #[test]
    fn test_medium_scale_build() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(50, TEST_DIMENSION);
        let build_params = create_test_build_params();

        let result = adapter.build(&test_vectors, build_params);
        assert!(result.is_ok(), "Medium scale build should succeed: {:?}", result);

        verify_id_mapping(&adapter, &test_vectors);
        verify_index_stats(&adapter, test_vectors.len());
    }

    /// 测试随机向量数据的构建
    #[test]
    fn test_random_vectors_build() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_random_test_vectors(20, TEST_DIMENSION, 12345);
        let build_params = create_test_build_params();

        let result = adapter.build(&test_vectors, build_params);
        assert!(result.is_ok(), "Random vectors build should succeed: {:?}", result);

        verify_id_mapping(&adapter, &test_vectors);
        verify_index_stats(&adapter, test_vectors.len());
    }

    /// 测试单个向量的构建（边界情况）
    #[test]
    fn test_single_vector_build() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(1, TEST_DIMENSION);
        let build_params = create_test_build_params();

        let result = adapter.build(&test_vectors, build_params);
        // DiskANN无法处理单个向量，应该失败
        assert!(result.is_err(), "Single vector build should fail with DiskANN");
        
        // 验证错误类型
        match result {
            Err(StorageError::VectorIndex(VectorIndexError::BuildError(_))) => {
                // 预期的错误类型
            }
            other => panic!("Expected BuildError, got: {:?}", other),
        }
    }

    /// 测试索引重建功能
    #[test]
    fn test_index_rebuild() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let build_params = create_test_build_params();

        // 第一次构建
        let first_vectors = generate_simple_test_vectors(3, TEST_DIMENSION);
        adapter.build(&first_vectors, build_params.clone()).expect("First build should succeed");
        verify_id_mapping(&adapter, &first_vectors);

        // 重建索引（应该清除之前的映射）
        let second_vectors = generate_simple_test_vectors(5, TEST_DIMENSION);
        adapter.build(&second_vectors, build_params).expect("Rebuild should succeed");
        verify_id_mapping(&adapter, &second_vectors);
        
        // 验证新的索引大小
        assert_eq!(adapter.size(), second_vectors.len());
    }

    // =================================
    // 参数验证测试
    // =================================

    /// 测试无效的BuildParams参数
    #[test]
    fn test_invalid_build_params() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(3, TEST_DIMENSION);
        let invalid_params_list = create_invalid_build_params();

        for invalid_params in invalid_params_list {
            let result = adapter.build(&test_vectors, invalid_params);
            assert_build_params_error!(result);
        }
    }

    /// 测试有效边界参数
    #[test]
    fn test_valid_boundary_params() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(3, TEST_DIMENSION);

        // 测试最小有效参数
        let minimal_params = BuildParams {
            ef_construction: 1,
            max_degree: 1,
            alpha: 0.1,
        };

        let result = adapter.build(&test_vectors, minimal_params);
        assert!(result.is_ok(), "Minimal valid params should work: {:?}", result);
    }

    // =================================
    // 数据验证测试
    // =================================

    /// 测试空向量集合
    #[test]
    fn test_empty_vectors() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let empty_vectors: Vec<(u32, Vec<f32>)> = vec![];
        let build_params = create_test_build_params();

        let result = adapter.build(&empty_vectors, build_params);
        assert_vector_index_error!(result, VectorIndexError::EmptyDataset);
    }

    /// 测试重复节点ID
    #[test]
    fn test_duplicate_node_ids() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let duplicate_vectors = generate_duplicate_node_id_vectors();
        let build_params = create_test_build_params();

        let result = adapter.build(&duplicate_vectors, build_params);
        assert_vector_index_error!(result, VectorIndexError::DuplicateNodeId { node_id: _ });
    }

    /// 测试维度不匹配
    #[test]
    fn test_dimension_mismatch() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let mismatched_vectors = generate_mismatched_dimension_vectors();
        let build_params = create_test_build_params();

        let result = adapter.build(&mismatched_vectors, build_params);
        assert_vector_index_error!(result, VectorIndexError::InvalidDimension { expected: _, actual: _ });
    }

    /// 测试非有限值（NaN、无穷大）
    #[test]
    fn test_non_finite_values() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let non_finite_vectors = generate_non_finite_vectors();
        let build_params = create_test_build_params();

        let result = adapter.build(&non_finite_vectors, build_params);
        assert_vector_index_error!(result, VectorIndexError::DataConversion(_));
    }

    // =================================
    // 边界情况和压力测试
    // =================================

    /// 测试不同维度的向量
    #[test]
    fn test_different_dimensions() {
        // 只测试我们知道有效的维度
        let dimensions = vec![128, 256];
        
        for dim in dimensions {
            let mut adapter = create_test_adapter(dim).expect("Failed to create adapter");
            let test_vectors = generate_simple_test_vectors(5, dim);
            let build_params = create_test_build_params();

            let result = adapter.build(&test_vectors, build_params);
            assert!(result.is_ok(), "Build with dimension {} should succeed: {:?}", dim, result);
            
            assert_eq!(adapter.get_dimension(), dim);
            verify_id_mapping(&adapter, &test_vectors);
        }
    }

    /// 测试大量向量的构建（压力测试）
    #[test]
    fn test_large_scale_build() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_random_test_vectors(1000, TEST_DIMENSION, 54321);
        let build_params = create_test_build_params();

        let result = adapter.build(&test_vectors, build_params);
        assert!(result.is_ok(), "Large scale build should succeed: {:?}", result);

        verify_id_mapping(&adapter, &test_vectors);
        verify_index_stats(&adapter, test_vectors.len());
        
        // 验证构建时间被记录
        let stats = adapter.stats();
        assert!(stats.build_time_ms.unwrap() > 0);
    }

    /// 测试极端节点ID值
    #[test]
    fn test_extreme_node_ids() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        
        // 使用极端的节点ID值
        let extreme_vectors = vec![
            (0, vec![1.0; TEST_DIMENSION]),              // 最小值
            (u32::MAX, vec![2.0; TEST_DIMENSION]),       // 最大值
            (u32::MAX / 2, vec![3.0; TEST_DIMENSION]),   // 中间值
        ];
        
        let build_params = create_test_build_params();
        let result = adapter.build(&extreme_vectors, build_params);
        
        assert!(result.is_ok(), "Extreme node IDs should be handled: {:?}", result);
        verify_id_mapping(&adapter, &extreme_vectors);
    }

    // =================================
    // 索引状态和统计测试
    // =================================

    /// 测试索引统计信息的正确性
    #[test]
    fn test_index_statistics() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(10, TEST_DIMENSION);
        let build_params = create_test_build_params();

        // 构建前的状态
        assert_eq!(adapter.size(), 0);
        assert_eq!(adapter.mapping_count(), 0);

        // 构建索引
        adapter.build(&test_vectors, build_params).expect("Build should succeed");

        // 构建后的状态
        assert_eq!(adapter.size(), test_vectors.len());
        assert_eq!(adapter.mapping_count(), test_vectors.len());
        assert_eq!(adapter.get_dimension(), TEST_DIMENSION);

        // 验证统计信息
        let stats = adapter.stats();
        assert_eq!(stats.vector_count, test_vectors.len());
        assert!(stats.build_time_ms.is_some());
        // 构建时间可能为0（很快的构建）
    }

    /// 测试清理映射功能
    #[test]
    fn test_mapping_cleanup() {
        let mut adapter = create_test_adapter(TEST_DIMENSION).expect("Failed to create adapter");
        let test_vectors = generate_simple_test_vectors(5, TEST_DIMENSION);
        let build_params = create_test_build_params();

        // 第一次构建
        adapter.build(&test_vectors, build_params.clone()).expect("First build should succeed");
        assert_eq!(adapter.mapping_count(), 5);

        // 第二次构建应该清理之前的映射
        let new_vectors = generate_simple_test_vectors(3, TEST_DIMENSION);
        adapter.build(&new_vectors, build_params).expect("Second build should succeed");
        assert_eq!(adapter.mapping_count(), 3);
    }
}