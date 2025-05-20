use std::collections::HashMap;

use minigu_common::types::PropertyId;

use crate::error::CatalogResult;
use crate::label_set::LabelSet;
use crate::provider::{PropertyRef, PropertySetProvider, VertexTypeProvider};
use crate::types::VertexTypeId;

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
