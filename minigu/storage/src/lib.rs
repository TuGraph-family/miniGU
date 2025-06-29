#![feature(coroutines)]
#![feature(gen_blocks)]
#![feature(impl_trait_in_assoc_type)]

// Core modules
pub mod ap_storage;
pub mod common;
pub mod error;
pub mod tp_storage;

// Re-export commonly used types for backward compatibility
// Common exports
pub use common::{iterators, model, wal};
pub use error::StorageResult;
// OLTP-specific exports (backward compatible)
pub use tp_storage::{IsolationLevel, MemTransaction, MemoryGraph, TransactionHandle};

// Storage mode configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageMode {
    /// Transaction Processing (OLTP) mode
    OLTP,
    /// Analytical Processing (OLAP) mode  
    OLAP,
    /// Hybrid mode supporting both OLTP and OLAP
    Hybrid,
}

impl Default for StorageMode {
    fn default() -> Self {
        StorageMode::OLTP
    }
}

/// Configuration for storage engine
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// Storage mode
    pub mode: StorageMode,
    /// Enable WAL (Write-Ahead Logging)
    pub enable_wal: bool,
    /// Enable compression for analytical workloads
    pub enable_compression: bool,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            mode: StorageMode::OLTP,
            enable_wal: true,
            enable_compression: false,
        }
    }
}
