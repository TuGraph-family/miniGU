use std::collections::HashMap;
use std::sync::Arc;

use minigu_common::types::GraphId;

use super::graph::MemoryGraphCatalog;
use super::graph_type::MemoryGraphTypeCatalog;
use crate::error::CatalogResult;
use crate::provider::{GraphRef, GraphTypeRef, SchemaProvider};
use crate::types::{GraphTypeId, SchemaId};

#[derive(Debug)]
pub struct MemorySchemaCatalog {
    id: SchemaId,
    parent: Option<SchemaId>,
    graph_id_map: HashMap<String, GraphId>,
    graph_map: HashMap<GraphId, Arc<MemoryGraphCatalog>>,
    graph_type_id_map: HashMap<String, GraphTypeId>,
    graph_type_map: HashMap<GraphTypeId, Arc<MemoryGraphTypeCatalog>>,
}

impl SchemaProvider for MemorySchemaCatalog {
    #[inline]
    fn id(&self) -> SchemaId {
        self.id
    }

    #[inline]
    fn parent(&self) -> Option<SchemaId> {
        self.parent
    }

    #[inline]
    fn get_graph(&self, name: &str) -> CatalogResult<Option<GraphRef>> {
        Ok(self
            .graph_id_map
            .get(name)
            .map(|id| self.graph_map.get(id).expect("graph must exist").clone() as _))
    }

    #[inline]
    fn get_graph_by_id(&self, id: GraphId) -> CatalogResult<Option<GraphRef>> {
        Ok(self.graph_map.get(&id).map(|g| g.clone() as _))
    }

    #[inline]
    fn get_graph_type(&self, name: &str) -> CatalogResult<Option<GraphTypeRef>> {
        Ok(self.graph_type_id_map.get(name).map(|id| {
            self.graph_type_map
                .get(id)
                .expect("graph type must exist")
                .clone() as _
        }))
    }

    #[inline]
    fn get_graph_type_by_id(&self, id: GraphTypeId) -> CatalogResult<Option<GraphTypeRef>> {
        Ok(self.graph_type_map.get(&id).map(|g| g.clone() as _))
    }
}
