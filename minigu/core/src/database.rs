use std::path::{Path, PathBuf};
use std::sync::Arc;

use minigu_catalog::memory::MemoryCatalog;
use minigu_catalog::memory::directory::MemoryDirectoryCatalog;
use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_catalog::provider::{CatalogProvider, DirectoryOrSchema, SchemaRef};
use minigu_common::constants::DEFAULT_SCHEMA_NAME;
pub use minigu_context::database::DatabaseConfig;
use minigu_context::database::DatabaseContext;
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_storage::tp::MemoryGraph;
use rayon::ThreadPoolBuilder;

use crate::catalog_persistence::{
    catalog_entry_to_graph_type, graph_data_path, load_catalog,
};
use crate::error::Result;
use crate::procedures::build_predefined_procedures;
use crate::session::Session;

pub struct Database {
    context: Arc<DatabaseContext>,
    default_schema: Arc<MemorySchemaCatalog>,
}

impl Database {
    /// Open (or create) an on-disk database at the given directory path.
    ///
    /// The directory will be created if it does not exist. On subsequent opens,
    /// the catalog and graph data are restored from disk.
    pub fn open<P: AsRef<Path>>(path: P, config: DatabaseConfig) -> Result<Self> {
        let db_path = path.as_ref().to_path_buf();

        // Create the database directory if it doesn't exist
        if !db_path.exists() {
            std::fs::create_dir_all(&db_path)?;
        }

        let (catalog, default_schema) = init_memory_catalog()?;

        // Load catalog from disk and restore graphs
        if let Some(db_catalog) = load_catalog(&db_path)? {
            for (graph_name, entry) in &db_catalog.graphs {
                let data_path = graph_data_path(&db_path, graph_name);
                let graph = MemoryGraph::with_db_file(&data_path)?;
                let graph_type = catalog_entry_to_graph_type(entry);
                let container = Arc::new(GraphContainer::new(
                    graph_type,
                    GraphStorage::Memory(graph),
                ));
                default_schema.add_graph(graph_name.clone(), container);
            }
        }

        let config = DatabaseConfig {
            db_path: Some(db_path),
            ..config
        };
        let runtime = ThreadPoolBuilder::new()
            .num_threads(config.num_threads)
            .build()?;
        let context = Arc::new(DatabaseContext::new(catalog, runtime, config));
        Ok(Self {
            context,
            default_schema,
        })
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
        default_schema.add_procedure(name, Arc::new(procedure));
    }
    root.add_child(
        DEFAULT_SCHEMA_NAME.into(),
        DirectoryOrSchema::Schema(default_schema.clone()),
    );
    let catalog = MemoryCatalog::new(DirectoryOrSchema::Directory(root));
    Ok((catalog, default_schema))
}
