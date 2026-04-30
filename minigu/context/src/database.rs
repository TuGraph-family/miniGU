use std::path::PathBuf;

use minigu_catalog::memory::MemoryCatalog;

use crate::runtime::DatabaseRuntime;

/// Storage-related configuration.
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// Number of WAL entries before triggering auto checkpoint. 0 = disabled.
    pub wal_threshold: usize,
    /// Batch size for TP adjacency list iteration.
    pub batch_size: usize,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            wal_threshold: 1000,
            batch_size: 64,
        }
    }
}

/// Execution-related configuration.
#[derive(Debug, Clone)]
pub struct ExecutionConfig {
    /// Batch size for vertex scan source.
    pub vertex_scan_batch_size: usize,
    /// Batch size for expand source (neighbor iteration).
    pub expand_batch_size: usize,
    /// Chunk size for sort operator.
    pub sort_chunk_size: usize,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            vertex_scan_batch_size: 1024,
            expand_batch_size: 64,
            sort_chunk_size: 2048,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub num_threads: usize,
    pub db_path: Option<PathBuf>,
    pub storage: StorageConfig,
    pub execution: ExecutionConfig,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            num_threads: 1,
            db_path: None,
            storage: StorageConfig::default(),
            execution: ExecutionConfig::default(),
        }
    }
}

#[derive(Debug)]
pub struct DatabaseContext {
    catalog: MemoryCatalog,
    runtime: DatabaseRuntime,
    config: DatabaseConfig,
}

impl DatabaseContext {
    pub fn new(catalog: MemoryCatalog, runtime: DatabaseRuntime, config: DatabaseConfig) -> Self {
        Self {
            catalog,
            runtime,
            config,
        }
    }

    #[inline]
    pub fn catalog(&self) -> &MemoryCatalog {
        &self.catalog
    }

    #[inline]
    pub fn runtime(&self) -> &DatabaseRuntime {
        &self.runtime
    }

    #[inline]
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }
}
