use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::Arc;

use minigu_common::types::LabelId;

use crate::error::CatalogResult;
use crate::label_set::LabelSet;
use crate::provider::{
    EdgeTypeProvider, EdgeTypeRef, GraphTypeProvider, PropertyRef, PropertySetProvider,
    VertexTypeProvider, VertexTypeRef,
};

#[derive(Debug)]
pub struct MemoryGraphTypeCatalog {
    next_label_id: LabelId,
    label_map: HashMap<String, LabelId>,
    vertex_type_map: HashMap<LabelSet, Arc<MemoryVertexTypeCatalog>>,
    edge_type_map: HashMap<LabelSet, Arc<MemoryEdgeTypeCatalog>>,
}

impl MemoryGraphTypeCatalog {
    #[inline]
    pub fn new() -> Self {
        Self {
            next_label_id: LabelId::new(1).expect("label id should be non-zero"),
            label_map: HashMap::new(),
            vertex_type_map: HashMap::new(),
            edge_type_map: HashMap::new(),
        }
    }

    #[inline]
    pub fn add_label(&mut self, name: String) -> Option<LabelId> {
        let label_id = self.next_label_id;
        match self.label_map.entry(name) {
            Entry::Occupied(_) => None,
            Entry::Vacant(e) => {
                self.next_label_id = self.next_label_id.checked_add(1)?;
                e.insert(label_id);
                Some(label_id)
            }
        }
    }

    #[inline]
    pub fn remove_label(&mut self, name: &str) -> bool {
        self.label_map.remove(name).is_some()
    }

    #[inline]
    pub fn add_vertex_type(
        &mut self,
        label_set: LabelSet,
        vertex_type: Arc<MemoryVertexTypeCatalog>,
    ) -> bool {
        match self.vertex_type_map.entry(label_set) {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                e.insert(vertex_type);
                true
            }
        }
    }

    #[inline]
    pub fn remove_vertex_type(&mut self, label_set: &LabelSet) -> bool {
        self.vertex_type_map.remove(label_set).is_some()
    }

    #[inline]
    pub fn add_edge_type(
        &mut self,
        label_set: LabelSet,
        edge_type: Arc<MemoryEdgeTypeCatalog>,
    ) -> bool {
        match self.edge_type_map.entry(label_set) {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                e.insert(edge_type);
                true
            }
        }
    }

    #[inline]
    pub fn remove_edge_type(&mut self, label_set: &LabelSet) -> bool {
        self.edge_type_map.remove(label_set).is_some()
    }
}

impl GraphTypeProvider for MemoryGraphTypeCatalog {
    #[inline]
    fn get_label_id(&self, name: &str) -> CatalogResult<Option<LabelId>> {
        Ok(self.label_map.get(name).copied())
    }

    #[inline]
    fn get_vertex_type(&self, key: &LabelSet) -> CatalogResult<Option<VertexTypeRef>> {
        Ok(self.vertex_type_map.get(key).map(|v| v.clone() as _))
    }

    #[inline]
    fn get_edge_type(&self, key: &LabelSet) -> CatalogResult<Option<EdgeTypeRef>> {
        Ok(self.edge_type_map.get(key).map(|e| e.clone() as _))
    }
}

#[derive(Debug)]
pub struct MemoryVertexTypeCatalog {
    label_set: LabelSet,
    property_map: HashMap<String, PropertyRef>,
}

impl MemoryVertexTypeCatalog {
    #[inline]
    pub fn new(label_set: LabelSet) -> Self {
        Self {
            label_set,
            property_map: HashMap::new(),
        }
    }

    #[inline]
    pub fn add_property(&mut self, name: String, property: PropertyRef) -> bool {
        match self.property_map.entry(name) {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                e.insert(property);
                true
            }
        }
    }

    #[inline]
    pub fn remove_property(&mut self, name: &str) -> bool {
        self.property_map.remove(name).is_some()
    }
}

impl PropertySetProvider for MemoryVertexTypeCatalog {
    #[inline]
    fn get_property(&self, name: &str) -> CatalogResult<Option<PropertyRef>> {
        Ok(self.property_map.get(name).cloned())
    }
}

impl VertexTypeProvider for MemoryVertexTypeCatalog {
    #[inline]
    fn label_set(&self) -> &LabelSet {
        &self.label_set
    }
}

#[derive(Debug)]
pub struct MemoryEdgeTypeCatalog {
    label_set: LabelSet,
    src: VertexTypeRef,
    dst: VertexTypeRef,
    property_map: HashMap<String, PropertyRef>,
}

impl MemoryEdgeTypeCatalog {
    #[inline]
    pub fn new(label_set: LabelSet, src: VertexTypeRef, dst: VertexTypeRef) -> Self {
        Self {
            label_set,
            src,
            dst,
            property_map: HashMap::new(),
        }
    }

    #[inline]
    pub fn add_property(&mut self, name: String, property: PropertyRef) -> bool {
        match self.property_map.entry(name) {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                e.insert(property);
                true
            }
        }
    }

    #[inline]
    pub fn remove_property(&mut self, name: &str) -> bool {
        self.property_map.remove(name).is_some()
    }
}

impl PropertySetProvider for MemoryEdgeTypeCatalog {
    #[inline]
    fn get_property(&self, name: &str) -> CatalogResult<Option<PropertyRef>> {
        Ok(self.property_map.get(name).cloned())
    }
}

impl EdgeTypeProvider for MemoryEdgeTypeCatalog {
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
