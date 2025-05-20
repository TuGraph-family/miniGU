use std::collections::HashMap;
use std::sync::Arc;

use minigu_common::types::{LabelId, PropertyId};

use crate::error::CatalogResult;
use crate::label_set::LabelSet;
use crate::provider::{
    EdgeTypeProvider, EdgeTypeRef, GraphTypeProvider, PropertyRef, PropertySetProvider,
    VertexTypeProvider, VertexTypeRef,
};
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

#[derive(Debug)]
pub struct MemoryVertexTypeCatalog {
    id: VertexTypeId,
    label_set: LabelSet,
    property_id_map: HashMap<String, PropertyId>,
    property_map: HashMap<PropertyId, PropertyRef>,
}

impl PropertySetProvider for MemoryVertexTypeCatalog {
    #[inline]
    fn get_property(&self, name: &str) -> CatalogResult<Option<PropertyRef>> {
        Ok(self.property_id_map.get(name).map(|id| {
            self.property_map
                .get(id)
                .expect("property must exist")
                .clone()
        }))
    }

    #[inline]
    fn get_property_by_id(&self, id: PropertyId) -> CatalogResult<Option<PropertyRef>> {
        Ok(self.property_map.get(&id).cloned())
    }
}

impl VertexTypeProvider for MemoryVertexTypeCatalog {
    #[inline]
    fn id(&self) -> VertexTypeId {
        self.id
    }

    #[inline]
    fn label_set(&self) -> &LabelSet {
        &self.label_set
    }
}

#[derive(Debug)]
pub struct MemoryEdgeTypeCatalog {
    id: EdgeTypeId,
    label_set: LabelSet,
    src: VertexTypeRef,
    dst: VertexTypeRef,
    property_id_map: HashMap<String, PropertyId>,
    property_map: HashMap<PropertyId, PropertyRef>,
}

impl PropertySetProvider for MemoryEdgeTypeCatalog {
    #[inline]
    fn get_property(&self, name: &str) -> CatalogResult<Option<PropertyRef>> {
        Ok(self.property_id_map.get(name).map(|id| {
            self.property_map
                .get(id)
                .expect("property must exist")
                .clone()
        }))
    }

    #[inline]
    fn get_property_by_id(&self, id: PropertyId) -> CatalogResult<Option<PropertyRef>> {
        Ok(self.property_map.get(&id).cloned())
    }
}

impl EdgeTypeProvider for MemoryEdgeTypeCatalog {
    #[inline]
    fn id(&self) -> EdgeTypeId {
        self.id
    }

    #[inline]
    fn label_set(&self) -> &LabelSet {
        &self.label_set
    }

    #[inline]
    fn src(&self) -> VertexTypeRef {
        self.src.clone()
    }

    #[inline]
    fn dst(&self) -> VertexTypeRef {
        self.dst.clone()
    }
}
