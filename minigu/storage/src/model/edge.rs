use common::datatype::types::{EdgeId, LabelId, VertexId};
use common::datatype::value::PropertyValue;
use serde::{Deserialize, Serialize};

use super::properties::PropertyRecord;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
pub struct Neighbor {
    label_id: LabelId,
    neighbor_id: VertexId,
    eid: EdgeId,
}

impl Neighbor {
    pub fn new(label_id: LabelId, neighbor_id: VertexId, eid: EdgeId) -> Self {
        Neighbor {
            label_id,
            neighbor_id,
            eid,
        }
    }

    pub fn label_id(&self) -> LabelId {
        self.label_id
    }

    pub fn eid(&self) -> EdgeId {
        self.eid
    }

    pub fn neighbor_id(&self) -> VertexId {
        self.neighbor_id
    }
}

impl Ord for Neighbor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.label_id
            .cmp(&other.label_id)
            .then_with(|| self.neighbor_id.cmp(&other.neighbor_id))
            .then_with(|| self.eid.cmp(&other.eid))
    }
}

impl PartialOrd for Neighbor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Edge {
    pub label_id: LabelId,
    pub src_id: VertexId,
    pub dst_id: VertexId,
    pub eid: EdgeId,
    pub properties: PropertyRecord,
    pub is_tombstone: bool,
}

impl Edge {
    pub fn new(
        eid: EdgeId,
        src_id: VertexId,
        dst_id: VertexId,
        label_id: LabelId,
        properties: PropertyRecord,
    ) -> Self {
        Edge {
            label_id,
            src_id,
            dst_id,
            eid,
            properties,
            is_tombstone: false,
        }
    }

    pub fn tombstone(edge: Edge) -> Self {
        Edge {
            label_id: edge.label_id,
            src_id: edge.src_id,
            dst_id: edge.dst_id,
            eid: edge.eid,
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

    pub fn label_id(&self) -> LabelId {
        self.label_id
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

    pub fn without_properties(&self) -> Self {
        Edge {
            label_id: self.label_id,
            src_id: self.src_id,
            dst_id: self.dst_id,
            eid: self.eid,
            properties: PropertyRecord::new(vec![]),
            is_tombstone: self.is_tombstone,
        }
    }
}
