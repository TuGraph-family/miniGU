use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::Weak;

use crate::error::CatalogResult;
use crate::provider::{DirectoryOrSchema, DirectoryProvider};

#[derive(Debug)]
pub struct MemoryDirectoryCatalog {
    parent: Option<Weak<dyn DirectoryProvider>>,
    children: HashMap<String, DirectoryOrSchema>,
}

impl MemoryDirectoryCatalog {
    #[inline]
    pub fn new(parent: Option<Weak<dyn DirectoryProvider>>) -> Self {
        Self {
            parent,
            children: HashMap::new(),
        }
    }

    #[inline]
    pub fn add_child(&mut self, name: String, child: DirectoryOrSchema) -> bool {
        match self.children.entry(name) {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                e.insert(child);
                true
            }
        }
    }

    #[inline]
    pub fn remove_child(&mut self, name: &str) -> bool {
        self.children.remove(name).is_some()
    }
}

impl DirectoryProvider for MemoryDirectoryCatalog {
    #[inline]
    fn parent(&self) -> Option<Weak<dyn DirectoryProvider>> {
        self.parent.clone()
    }

    #[inline]
    fn get_directory_or_schema(&self, name: &str) -> CatalogResult<Option<DirectoryOrSchema>> {
        Ok(self.children.get(name).cloned())
    }
}
