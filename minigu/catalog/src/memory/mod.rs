pub mod directory;
pub mod graph_type;
pub mod schema;

use std::sync::Arc;

use directory::MemoryDirectoryCatalog;

use crate::error::CatalogResult;
use crate::provider::{CatalogProvider, DirectoryOrSchema};

#[derive(Debug)]
pub struct MemoryCatalog {
    root: Arc<MemoryDirectoryCatalog>,
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
        Self {
            root: Arc::new(MemoryDirectoryCatalog::new(None)),
        }
    }
}

impl CatalogProvider for MemoryCatalog {
    #[inline]
    fn get_root(&self) -> CatalogResult<DirectoryOrSchema> {
        Ok(DirectoryOrSchema::Directory(self.root.clone()))
    }
}
