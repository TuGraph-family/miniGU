use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use minigu_catalog::memory::MemoryCatalog;
use minigu_catalog::memory::directory::MemoryDirectoryCatalog;
use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_catalog::provider::{CatalogProvider, DirectoryOrSchema, SchemaRef};
use minigu_common::constants::DEFAULT_SCHEMA_NAME;
pub use minigu_context::database::DatabaseConfig;
use minigu_context::database::DatabaseContext;
use rayon::ThreadPoolBuilder;

use crate::error::Result;
use crate::procedures::build_predefined_procedures;
use crate::session::Session;

pub struct Database {
    context: Arc<DatabaseContext>,
    default_schema: Arc<MemorySchemaCatalog>,
}

impl Database {
    pub fn open<P: AsRef<Path>>(_path: P, _config: DatabaseConfig) -> Result<Self> {
        todo!("on-disk database is not implemented yet")
    }

    pub fn open_in_memory(config: DatabaseConfig) -> Result<Self> {
        let (catalog, default_schema) = init_memory_catalog()?;
        let runtime = ThreadPoolBuilder::new()
            .num_threads(config.num_threads)
            .build()?;
        let context = Arc::new(DatabaseContext::new(catalog, runtime, config));
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
    for (name, procedure) in build_predefined_procedures() {
        #[allow(deprecated)]
        default_schema.add_procedure(name, Arc::new(procedure));
    }
    #[allow(deprecated)]
    root.add_child(
        DEFAULT_SCHEMA_NAME.into(),
        DirectoryOrSchema::Schema(default_schema.clone()),
    );
    let catalog = MemoryCatalog::new(DirectoryOrSchema::Directory(root));
    Ok((catalog, default_schema))
}
