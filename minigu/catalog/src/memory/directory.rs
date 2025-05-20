use std::collections::HashMap;

use crate::error::CatalogResult;
use crate::provider::{DirectoryOrSchema, DirectoryProvider};
use crate::types::SchemaId;

#[derive(Debug)]
pub struct MemoryDirectoryCatalog {
    id: SchemaId,
    parent: Option<SchemaId>,
    children: HashMap<String, DirectoryOrSchema>,
}

impl MemoryDirectoryCatalog {
    #[inline]
    pub fn new(id: SchemaId, parent: Option<SchemaId>) -> Self {
        Self {
            id,
            parent,
            children: HashMap::new(),
        }
    }

    // pub fn add_schema(&mut self, name: String, schema: SchemaOrSchema) -> CatalogResult<()> {
    //     todo!()
    // }
}

impl DirectoryProvider for MemoryDirectoryCatalog {
    #[inline]
    fn id(&self) -> SchemaId {
        self.id
    }

    #[inline]
    fn parent(&self) -> Option<SchemaId> {
        self.parent
    }

    #[inline]
    fn get_directory_or_schema(&self, name: &str) -> CatalogResult<Option<DirectoryOrSchema>> {
        Ok(self.children.get(name).cloned())
    }
}
