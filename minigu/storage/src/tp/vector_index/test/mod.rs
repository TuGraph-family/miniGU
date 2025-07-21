//! 测试模块：向量索引功能测试
//! 
//! 本模块包含对向量索引集成功能的全面测试，包括：
//! - 构建索引功能测试
//! - ID映射机制测试  
//! - 参数验证测试
//! - 错误处理测试
//! - 边界情况测试
//! - 与diskann-rs集成测试

pub mod test_utils;
pub mod build_index_tests;
pub mod search_index_tests;

// 公共导入，供所有测试模块使用
pub use super::{VectorIndex, InMemDiskANNAdapter, BuildParams, SearchParams, LoadParams};
pub use crate::error::{StorageResult, StorageError, VectorIndexError};
pub use diskann::model::IndexConfiguration;

// 测试常量 - diskann可能需要更大的维度
pub const TEST_DIMENSION: usize = 128;
pub const DEFAULT_MAX_DEGREE: usize = 32;
pub const DEFAULT_EF_CONSTRUCTION: usize = 50;
pub const DEFAULT_ALPHA: f32 = 1.2;
pub const DEFAULT_EF_SEARCH: u32 = 50;

// 测试辅助宏
#[macro_export]
macro_rules! assert_vector_index_error {
    ($result:expr, $expected_error:pat) => {
        match $result {
            Err(StorageError::VectorIndex($expected_error)) => {
                // 测试通过
            }
            other => panic!("Expected VectorIndexError::{}, got: {:?}", stringify!($expected_error), other),
        }
    };
}

#[macro_export]
macro_rules! assert_build_params_error {
    ($result:expr) => {
        match $result {
            Err(StorageError::VectorIndex(VectorIndexError::InvalidBuildParams(_))) => {
                // 测试通过
            }
            other => panic!("Expected InvalidBuildParams error, got: {:?}", other),
        }
    };
}