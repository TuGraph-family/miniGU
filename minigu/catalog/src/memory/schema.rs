use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::{Arc, Weak};

use super::graph::MemoryGraphCatalog;
use super::graph_type::MemoryGraphTypeCatalog;
use crate::error::CatalogResult;
use crate::provider::{DirectoryProvider, GraphRef, GraphTypeRef, ProcedureRef, SchemaProvider};

#[derive(Debug)]
pub struct MemorySchemaCatalog {
    parent: Option<Weak<dyn DirectoryProvider>>,
    graph_map: HashMap<String, Arc<MemoryGraphCatalog>>,
    graph_type_map: HashMap<String, Arc<MemoryGraphTypeCatalog>>,
    procedure_map: HashMap<String, ProcedureRef>,
}

impl MemorySchemaCatalog {
    #[inline]
    pub fn new(parent: Option<Weak<dyn DirectoryProvider>>) -> Self {
        Self {
            parent,
            graph_map: HashMap::new(),
            graph_type_map: HashMap::new(),
            procedure_map: HashMap::new(),
        }
    }

    #[inline]
    pub fn add_graph(&mut self, name: String, graph: Arc<MemoryGraphCatalog>) -> bool {
        match self.graph_map.entry(name) {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                e.insert(graph);
                true
            }
        }
    }

    #[inline]
    pub fn remove_graph(&mut self, name: &str) -> bool {
        self.graph_map.remove(name).is_some()
    }

    #[inline]
    pub fn add_graph_type(
        &mut self,
        name: String,
        graph_type: Arc<MemoryGraphTypeCatalog>,
    ) -> bool {
        match self.graph_type_map.entry(name) {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                e.insert(graph_type);
                true
            }
        }
    }

    #[inline]
    pub fn remove_graph_type(&mut self, name: &str) -> bool {
        self.graph_type_map.remove(name).is_some()
    }

    #[inline]
    pub fn add_procedure(&mut self, name: String, procedure: ProcedureRef) -> bool {
        match self.procedure_map.entry(name) {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                e.insert(procedure);
                true
            }
        }
    }

    #[inline]
    pub fn remove_procedure(&mut self, name: &str) -> bool {
        self.procedure_map.remove(name).is_some()
    }
}

impl SchemaProvider for MemorySchemaCatalog {
    #[inline]
    fn parent(&self) -> Option<Weak<dyn DirectoryProvider>> {
        self.parent.clone()
    }

    #[inline]
    fn graphs(&self) -> Box<dyn Iterator<Item = (&str, GraphRef)> + '_> {
        Box::new(
            self.graph_map
                .iter()
                .map(|(name, graph)| (name.as_str(), graph.clone() as _)),
        )
    }

    #[inline]
    fn get_graph(&self, name: &str) -> CatalogResult<Option<GraphRef>> {
        Ok(self.graph_map.get(name).map(|g| g.clone() as _))
    }

    fn graph_types(&self) -> Box<dyn Iterator<Item = (&str, GraphTypeRef)> + '_> {
        Box::new(
            self.graph_type_map
                .iter()
                .map(|(name, graph_type)| (name.as_str(), graph_type.clone() as _)),
        )
    }

    #[inline]
    fn get_graph_type(&self, name: &str) -> CatalogResult<Option<GraphTypeRef>> {
        Ok(self.graph_type_map.get(name).map(|g| g.clone() as _))
    }

    #[inline]
    fn procedures(&self) -> Box<dyn Iterator<Item = (&str, ProcedureRef)> + '_> {
        Box::new(
            self.procedure_map
                .iter()
                .map(|(name, procedure)| (name.as_str(), procedure.clone() as _)),
        )
    }

    #[inline]
    fn get_procedure(&self, name: &str) -> CatalogResult<Option<ProcedureRef>> {
        Ok(self.procedure_map.get(name).cloned())
    }
}
