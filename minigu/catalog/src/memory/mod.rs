mod directory;
mod graph;
mod graph_type;
mod schema;

use std::collections::HashMap;
use std::num::NonZero;
use std::sync::Arc;

use directory::MemoryDirectoryCatalog;

use crate::error::CatalogResult;
use crate::provider::{CatalogProvider, DirectoryOrSchema, DirectoryProvider};
use crate::types::SchemaId;

pub const ROOT_DIRECTORY_ID: u32 = 1;

#[derive(Debug)]
pub struct MemoryCatalog {
    root: Arc<MemoryDirectoryCatalog>,
    directory_schema_map: HashMap<SchemaId, DirectoryOrSchema>,
}

impl Default for MemoryCatalog {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryCatalog {
    #[inline]
    pub fn new() -> Self {
        let root = Arc::new(MemoryDirectoryCatalog::new(
            NonZero::new(ROOT_DIRECTORY_ID).expect("ROOT_DIRECTORY_ID must be non-zero"),
            None,
        ));
        let mut directory_schema_map = HashMap::new();
        directory_schema_map.insert(root.id(), DirectoryOrSchema::Directory(root.clone()));
        Self {
            root,
            directory_schema_map,
        }
    }
}

impl CatalogProvider for MemoryCatalog {
    #[inline]
    fn get_root(&self) -> CatalogResult<DirectoryOrSchema> {
        Ok(DirectoryOrSchema::Directory(self.root.clone()))
    }

    #[inline]
    fn get_directory_or_schema_by_id(
        &self,
        id: SchemaId,
    ) -> CatalogResult<Option<DirectoryOrSchema>> {
        Ok(self.directory_schema_map.get(&id).cloned())
    }
}
