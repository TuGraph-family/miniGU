use std::path::PathBuf;

use minigu_catalog::memory::MemoryCatalog;
use rayon::ThreadPool;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub num_threads: usize,
    pub checkpoint_dir: PathBuf,
    pub wal_path: PathBuf,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            num_threads: 1,
            checkpoint_dir: PathBuf::from(".checkpoint"),
            wal_path: PathBuf::from(".wal"),
        }
    }
}

#[derive(Debug)]
pub struct DatabaseContext {
    catalog: MemoryCatalog,
    runtime: ThreadPool,
    config: DatabaseConfig,
}

impl DatabaseContext {
    pub fn new(catalog: MemoryCatalog, runtime: ThreadPool, config: DatabaseConfig) -> Self {
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
    pub fn runtime(&self) -> &ThreadPool {
        &self.runtime
    }

    #[inline]
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }
}
