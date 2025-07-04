use minigu_common::types::{LabelId, VertexId};
use minigu_common::value::ScalarValue;
use serde::{Deserialize, Serialize};

use super::properties::PropertyRecord;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Vertex {
    pub vid: VertexId,
    pub label_id: LabelId,
    pub properties: PropertyRecord,
    // TODO: remove this field, add tombstone flag into the versioned vertex in memory_graph.rs
    pub is_tombstone: bool,
}

impl Vertex {
    /// create a new vertex
    pub fn new(vid: VertexId, label_id: LabelId, properties: PropertyRecord) -> Self {
        Vertex {
            vid,
            label_id,
            properties,
            is_tombstone: false,
        }
    }

    pub fn tombstone(vertex: Vertex) -> Self {
        Vertex {
            vid: vertex.vid,
            label_id: vertex.label_id,
            properties: vertex.properties.clone(),
            is_tombstone: true,
        }
    }

    /// Get the vid
    pub fn vid(&self) -> VertexId {
        self.vid
    }

    pub fn is_tombstone(&self) -> bool {
        self.is_tombstone
    }

    pub fn set_props(&mut self, indices: &[usize], props: Vec<ScalarValue>) {
        for (&index, prop) in indices.iter().zip(props.into_iter()) {
            self.properties.set_prop(index, prop);
        }
    }

    pub fn properties(&self) -> &Vec<ScalarValue> {
        self.properties.props()
    }
}
