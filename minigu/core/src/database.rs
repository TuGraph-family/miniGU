use std::path::Path;
use std::sync::Arc;

use minigu_catalog::memory::MemoryCatalog;
use minigu_catalog::memory::directory::MemoryDirectoryCatalog;
use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_catalog::provider::{CatalogProvider, DirectoryOrSchema, SchemaRef};
use minigu_catalog::txn::manager::CatalogTxnManager;
use minigu_common::constants::DEFAULT_SCHEMA_NAME;
use minigu_context::database::DatabaseContext;
use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};
use rayon::ThreadPoolBuilder;

use crate::error::Result;
use crate::procedures::build_predefined_procedures;
use crate::session::Session;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub num_threads: usize,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self { num_threads: 1 }
    }
}

pub struct Database {
    context: Arc<DatabaseContext>,
    default_schema: Arc<MemorySchemaCatalog>,
}

impl Database {
    pub fn open<P: AsRef<Path>>(_path: P, _config: &DatabaseConfig) -> Result<Self> {
        todo!("on-disk database is not implemented yet")
    }

    pub fn open_in_memory(config: &DatabaseConfig) -> Result<Self> {
        let (catalog, default_schema) = init_memory_catalog()?;
        let runtime = ThreadPoolBuilder::new()
            .num_threads(config.num_threads)
            .build()?;
        let context = Arc::new(DatabaseContext::new(catalog, runtime));
        Ok(Self {
            context,
            default_schema,
        })
    }

    pub fn session(&self) -> Result<Session> {
        Session::new(self.context.clone(), self.default_schema().clone())
    }

    fn default_schema(&self) -> &Arc<MemorySchemaCatalog> {
        &self.default_schema
    }
}

fn init_memory_catalog() -> Result<(MemoryCatalog, Arc<MemorySchemaCatalog>)> {
    let root = Arc::new(MemoryDirectoryCatalog::new(None));
    let parent = Arc::downgrade(&root);
    let default_schema = Arc::new(MemorySchemaCatalog::new(Some(parent)));
    // Set default procedures and default schema node in a single transaction
    let mgr = CatalogTxnManager::new();
    let txn = mgr
        .begin_transaction(IsolationLevel::Snapshot)
        .map_err(|e| {
            crate::error::Error::Catalog(minigu_catalog::error::CatalogError::External(Box::new(e)))
        })?;
    for (name, procedure) in build_predefined_procedures() {
        default_schema
            .add_procedure_txn(name, Arc::new(procedure), txn.as_ref())
            .map_err(|e| {
                crate::error::Error::Catalog(minigu_catalog::error::CatalogError::External(
                    Box::new(e),
                ))
            })?;
    }
    root.as_ref()
        .add_child(
            DEFAULT_SCHEMA_NAME.into(),
            DirectoryOrSchema::Schema(default_schema.clone()),
            txn.as_ref(),
        )
        .map_err(|e| {
            crate::error::Error::Catalog(minigu_catalog::error::CatalogError::External(Box::new(e)))
        })?;
    let _ = txn.commit();
    let catalog = MemoryCatalog::new(DirectoryOrSchema::Directory(root));
    Ok((catalog, default_schema))
}
