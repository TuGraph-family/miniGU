pub mod directory;
pub mod graph_type;
pub mod schema;

use crate::error::CatalogResult;
use crate::provider::{CatalogProvider, DirectoryOrSchema};

#[derive(Debug)]
pub struct MemoryCatalog {
    root: DirectoryOrSchema,
}

impl MemoryCatalog {
    pub fn new(root: DirectoryOrSchema) -> Self {
        Self { root }
    }
}

impl CatalogProvider for MemoryCatalog {
    #[inline]
    fn get_root(&self) -> CatalogResult<DirectoryOrSchema> {
        Ok(self.root.clone())
    }
}
