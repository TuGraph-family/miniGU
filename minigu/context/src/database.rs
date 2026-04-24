use std::path::PathBuf;

use minigu_catalog::memory::MemoryCatalog;
use minigu_transaction::TxnOptions;

use crate::runtime::DatabaseRuntime;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub num_threads: usize,
    pub db_path: Option<PathBuf>,
    pub txn_options: TxnOptions,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            num_threads: 1,
            db_path: None,
            txn_options: TxnOptions::default(),
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
