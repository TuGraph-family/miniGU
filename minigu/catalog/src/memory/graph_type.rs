use std::collections::HashMap;
use std::sync::Arc;

use minigu_common::types::LabelId;

use super::edge::MemoryEdgeTypeCatalog;
use super::vertex::MemoryVertexTypeCatalog;
use crate::error::CatalogResult;
use crate::label_set::LabelSet;
use crate::provider::{EdgeTypeRef, GraphTypeProvider, VertexTypeRef};
use crate::types::{EdgeTypeId, GraphTypeId, VertexTypeId};

#[derive(Debug)]
pub struct MemoryGraphTypeCatalog {
    id: GraphTypeId,
    label_map: HashMap<String, LabelId>,
    vertex_type_id_map: HashMap<LabelSet, VertexTypeId>,
    vertex_type_map: HashMap<VertexTypeId, Arc<MemoryVertexTypeCatalog>>,
    edge_type_id_map: HashMap<LabelSet, EdgeTypeId>,
    edge_type_map: HashMap<EdgeTypeId, Arc<MemoryEdgeTypeCatalog>>,
}

impl GraphTypeProvider for MemoryGraphTypeCatalog {
    #[inline]
    fn id(&self) -> GraphTypeId {
        self.id
    }

    #[inline]
    fn get_label_id(&self, name: &str) -> CatalogResult<Option<LabelId>> {
        Ok(self.label_map.get(name).copied())
    }

    #[inline]
    fn get_vertex_type(&self, key: &LabelSet) -> CatalogResult<Option<VertexTypeRef>> {
        Ok(self.vertex_type_id_map.get(key).map(|id| {
            self.vertex_type_map
                .get(id)
                .expect("vertex type must exist")
                .clone() as _
        }))
    }

    #[inline]
    fn get_vertex_type_by_id(&self, id: VertexTypeId) -> CatalogResult<Option<VertexTypeRef>> {
        Ok(self.vertex_type_map.get(&id).map(|v| v.clone() as _))
    }

    #[inline]
    fn get_edge_type(&self, key: &LabelSet) -> CatalogResult<Option<EdgeTypeRef>> {
        Ok(self.edge_type_id_map.get(key).map(|id| {
            self.edge_type_map
                .get(id)
                .expect("edge type must exist")
                .clone() as _
        }))
    }

    #[inline]
    fn get_edge_type_by_id(&self, id: EdgeTypeId) -> CatalogResult<Option<EdgeTypeRef>> {
        Ok(self.edge_type_map.get(&id).map(|e| e.clone() as _))
    }
}
