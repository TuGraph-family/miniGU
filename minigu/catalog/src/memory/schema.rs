use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::{Arc, RwLock, Weak};

use super::graph_type::MemoryGraphTypeCatalog;
use crate::error::CatalogResult;
use crate::provider::{
    DirectoryProvider, DirectoryRef, GraphRef, GraphTypeRef, ProcedureRef, SchemaProvider,
};

#[derive(Debug)]
pub struct MemorySchemaCatalog {
    parent: Option<Weak<dyn DirectoryProvider>>,
    graph_map: RwLock<HashMap<String, GraphRef>>,
    graph_type_map: RwLock<HashMap<String, Arc<MemoryGraphTypeCatalog>>>,
    procedure_map: RwLock<HashMap<String, ProcedureRef>>,
}

impl MemorySchemaCatalog {
    #[inline]
    pub fn new(parent: Option<Weak<dyn DirectoryProvider>>) -> Self {
        Self {
            parent,
            graph_map: RwLock::new(HashMap::new()),
            graph_type_map: RwLock::new(HashMap::new()),
            procedure_map: RwLock::new(HashMap::new()),
        }
    }

    #[inline]
    pub fn add_graph(&self, name: String, graph: GraphRef) -> bool {
        let mut graph_map = self
            .graph_map
            .write()
            .expect("the write lock should be acquired successfully");
        match graph_map.entry(name) {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                e.insert(graph);
                true
            }
        }
    }

    #[inline]
    pub fn remove_graph(&self, name: &str) -> bool {
        let mut graph_map = self
            .graph_map
            .write()
            .expect("the write lock should be acquired successfully");
        graph_map.remove(name).is_some()
    }

    #[inline]
    pub fn add_graph_type(&self, name: String, graph_type: Arc<MemoryGraphTypeCatalog>) -> bool {
        let mut graph_type_map = self
            .graph_type_map
            .write()
            .expect("the write lock should be acquired successfully");
        match graph_type_map.entry(name) {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                e.insert(graph_type);
                true
            }
        }
    }

    #[inline]
    pub fn remove_graph_type(&self, name: &str) -> bool {
        let mut graph_type_map = self
            .graph_type_map
            .write()
            .expect("the write lock should be acquired successfully");
        graph_type_map.remove(name).is_some()
    }

    #[inline]
    pub fn add_procedure(&self, name: String, procedure: ProcedureRef) -> bool {
        let mut procedure_map = self
            .procedure_map
            .write()
            .expect("the write lock should be acquired successfully");
        match procedure_map.entry(name) {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                e.insert(procedure);
                true
            }
        }
    }

    #[inline]
    pub fn remove_procedure(&self, name: &str) -> bool {
        let mut procedure_map = self
            .procedure_map
            .write()
            .expect("the write lock should be acquired successfully");
        procedure_map.remove(name).is_some()
    }
}

impl SchemaProvider for MemorySchemaCatalog {
    #[inline]
    fn parent(&self) -> Option<DirectoryRef> {
        self.parent.clone().and_then(|p| p.upgrade())
    }

    #[inline]
    fn graph_names(&self) -> Vec<String> {
        self.graph_map
            .read()
            .expect("the read lock should be acquired successfully")
            .keys()
            .cloned()
            .collect()
    }

    #[inline]
    fn get_graph(&self, name: &str) -> CatalogResult<Option<GraphRef>> {
        Ok(self
            .graph_map
            .read()
            .expect("the read lock should be acquired successfully")
            .get(name)
            .map(|g| g.clone() as _))
    }

    #[inline]
    fn graph_type_names(&self) -> Vec<String> {
        self.graph_type_map
            .read()
            .expect("the read lock should be acquired successfully")
            .keys()
            .cloned()
            .collect()
    }

    #[inline]
    fn get_graph_type(&self, name: &str) -> CatalogResult<Option<GraphTypeRef>> {
        Ok(self
            .graph_type_map
            .read()
            .expect("the read lock should be acquired successfully")
            .get(name)
            .map(|g| g.clone() as _))
    }

    #[inline]
    fn procedure_names(&self) -> Vec<String> {
        self.procedure_map
            .read()
            .expect("the read lock should be acquired successfully")
            .keys()
            .cloned()
            .collect()
    }

    #[inline]
    fn get_procedure(&self, name: &str) -> CatalogResult<Option<ProcedureRef>> {
        Ok(self
            .procedure_map
            .read()
            .expect("the read lock should be acquired successfully")
            .get(name)
            .map(|p| p.clone() as _))
    }
}
