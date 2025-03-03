use serde::{Serialize, Deserialize};
use super::properties::PropertyStore;
use common::datatype::types::{VertexId, LabelId};

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
}