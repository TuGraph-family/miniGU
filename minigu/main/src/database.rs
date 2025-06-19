use std::path::Path;
use std::sync::Arc;

use minigu_catalog::memory::MemoryCatalog;
use minigu_catalog::memory::directory::MemoryDirectoryCatalog;
use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_catalog::provider::{CatalogProvider, DirectoryOrSchema, SchemaRef};
use minigu_common::constants::DEFAULT_SCHEMA_NAME;
use minigu_context::database::DatabaseContext;
use rayon::ThreadPoolBuilder;

use crate::error::Result;
use crate::procedures::build_predefined_procedures;
use crate::session::Session;

pub struct Database {
    context: Arc<DatabaseContext>,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        todo!("on-disk database is not implemented yet")
    }

    pub fn open_in_memory() -> Result<Self> {
        let catalog = init_memory_catalog()?;
        let runtime = ThreadPoolBuilder::new().build()?;
        let context = Arc::new(DatabaseContext::new(catalog, runtime));
        Ok(Self { context })
    }

    pub fn session(&self) -> Result<Session> {
        Session::new(self.context.clone(), self.get_default_schema()?)
    }

    fn get_default_schema(&self) -> Result<SchemaRef> {
        let root = self
            .context
            .catalog()
            .get_root()?
            .into_directory()
            .expect("root must be a directory");
        let default_schema = root
            .get_child(DEFAULT_SCHEMA_NAME)?
            .expect("default schema must exist")
            .into_schema()
            .unwrap();
        Ok(default_schema)
    }
}

fn init_memory_catalog() -> Result<MemoryCatalog> {
    let root = Arc::new(MemoryDirectoryCatalog::new(None));
    let parent = Arc::downgrade(&root);
    let default_schema = Arc::new(MemorySchemaCatalog::new(Some(parent)));
    for (name, procedure) in build_predefined_procedures() {
        default_schema.add_procedure(name, Arc::new(procedure));
    }
    root.add_child(
        DEFAULT_SCHEMA_NAME.into(),
        DirectoryOrSchema::Schema(default_schema),
    );
    let catalog = MemoryCatalog::new(DirectoryOrSchema::Directory(root));
    Ok(catalog)
}
