use std::collections::HashMap;

use minigu_common::types::PropertyId;

use crate::error::CatalogResult;
use crate::label_set::LabelSet;
use crate::provider::{EdgeTypeProvider, PropertyRef, PropertySetProvider, VertexTypeRef};
use crate::types::EdgeTypeId;

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
