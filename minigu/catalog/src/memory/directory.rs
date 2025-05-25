use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::{RwLock, Weak};

use crate::error::CatalogResult;
use crate::provider::{DirectoryOrSchema, DirectoryProvider, DirectoryRef};

#[derive(Debug)]
pub struct MemoryDirectoryCatalog {
    parent: Option<Weak<dyn DirectoryProvider>>,
    children: RwLock<HashMap<String, DirectoryOrSchema>>,
}

impl MemoryDirectoryCatalog {
    #[inline]
    pub fn new(parent: Option<Weak<dyn DirectoryProvider>>) -> Self {
        Self {
            parent,
            children: RwLock::new(HashMap::new()),
        }
    }

    #[inline]
    pub fn add_child(&self, name: String, child: DirectoryOrSchema) -> bool {
        match self
            .children
            .write()
            .expect("the write lock should be acquired successfully")
            .entry(name)
        {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                e.insert(child);
                true
            }
        }
    }

    #[inline]
    pub fn remove_child(&self, name: &str) -> bool {
        self.children
            .write()
            .expect("the write lock should be acquired successfully")
            .remove(name)
            .is_some()
    }
}

impl DirectoryProvider for MemoryDirectoryCatalog {
    #[inline]
    fn parent(&self) -> Option<DirectoryRef> {
        self.parent.clone().and_then(|p| p.upgrade())
    }

    #[inline]
    fn get_child(&self, name: &str) -> CatalogResult<Option<DirectoryOrSchema>> {
        Ok(self
            .children
            .read()
            .expect("the read lock should be acquired successfully")
            .get(name)
            .cloned())
    }

    #[inline]
    fn children_names(&self) -> Vec<String> {
        self.children
            .read()
            .expect("the read lock should be acquired successfully")
            .keys()
            .cloned()
            .collect()
    }
}
