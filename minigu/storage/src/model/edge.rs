use common::datatype::types::{EdgeId, LabelId, VertexId};
use common::datatype::value::PropertyValue;
use serde::{Deserialize, Serialize};

use super::properties::PropertyStore;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Direction {
    Out, // Outgoing edge
    In,  // Incoming edge
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Edge {
    pub eid: EdgeId,               // ID of the edge
    pub src_id: VertexId,          // ID of the source vertex
    pub dst_id: VertexId,          // ID of the target vertex
    pub label_id: LabelId,         // Label of the edge
    pub direction: Direction,      // Direction of the edge
    pub properties: PropertyStore, // Properties of the edge
    pub is_tombstone: bool,
}

impl Edge {
    pub fn new(
        eid: EdgeId,
        src_id: VertexId,
        dst_id: VertexId,
        label_id: LabelId,
        direction: Direction,
        properties: PropertyStore,
    ) -> Self {
        Edge {
            eid,
            src_id,
            dst_id,
            label_id,
            direction,
            properties,
            is_tombstone: false,
        }
    }

    pub fn tombstone(edge: Edge) -> Self {
        Edge {
            eid: edge.eid,
            src_id: edge.src_id,
            dst_id: edge.dst_id,
            label_id: edge.label_id,
            direction: edge.direction.clone(),
            properties: edge.properties.clone(),
            is_tombstone: true,
        }
    }

    pub fn eid(&self) -> EdgeId {
        self.eid
    }

    pub fn src_id(&self) -> VertexId {
        self.src_id
    }

    pub fn dst_id(&self) -> VertexId {
        self.dst_id
    }

    pub fn is_tombstone(&self) -> bool {
        self.is_tombstone
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Adjacency {
    vertex_id: VertexId,
    edge_id: EdgeId,
}

impl Adjacency {
    pub fn new(vertex_id: VertexId, edge_id: EdgeId) -> Self {
        Adjacency { vertex_id, edge_id }
    }

    pub fn vertex_id(&self) -> VertexId {
        self.vertex_id
    }

    pub fn edge_id(&self) -> EdgeId {
        self.edge_id
    }
}

impl PartialOrd for Adjacency {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// 实现 Ord
impl Ord for Adjacency {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.vertex_id
            .cmp(&other.vertex_id)
            .then(self.edge_id.cmp(&other.edge_id))
    }
}
