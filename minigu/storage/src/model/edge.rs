use serde::{Serialize, Deserialize};
use super::properties::PropertyStore;
use common::datatype::types::{VertexId, LabelId, EdgeId};


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Direction {
    Out,           // Outgoing edge
    In,            // Incoming edge
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Edge {
    pub eid: EdgeId,                // ID of the edge
    pub source_id: VertexId,        // ID of the source vertex
    pub dst_id: VertexId,           // ID of the target vertex
    pub label_id: LabelId,          // Label of the edge
    pub direction: Direction,       // Direction of the edge
    pub properties: PropertyStore,  // Properties of the edge
}

impl Edge {
    pub fn new(eid: EdgeId, source_id: VertexId, dst_id: VertexId, label_id: LabelId, direction: Direction, properties: PropertyStore) -> Self {
        Edge {
            eid,
            source_id,
            dst_id,
            label_id,
            direction,
            properties,
        }
    }
}