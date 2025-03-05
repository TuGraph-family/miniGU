use common::datatype::types::{LabelId, VertexId};
use common::datatype::value::PropertyValue;
use serde::{Deserialize, Serialize};

use super::properties::PropertyStore;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Vertex {
    pub vid: VertexId,
    pub label_id: LabelId,
    pub properties: PropertyStore,
}

impl Vertex {
    /// create a new vertex
    pub fn new(vid: VertexId, label_id: LabelId, properties: PropertyStore) -> Self {
        Vertex {
            vid,
            label_id,
            properties,
        }
    }

    /// Get the vid
    pub fn vid(&self) -> VertexId {
        self.vid
    }

    pub fn set_props(&mut self, indices: &[usize], props: Vec<PropertyValue>) {
        for (&index, prop) in indices.iter().zip(props.into_iter()) {
            self.properties.set_prop(index, prop);
        }
    }

    pub fn properties(&self) -> &Vec<PropertyValue> {
        self.properties.props()
    }
}
