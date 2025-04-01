use common::datatype::types::{EdgeId, LabelId, VertexId};
use common::datatype::value::PropertyValue;
use serde::{Deserialize, Serialize};

use super::properties::PropertyStore;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy, Ord, PartialOrd)]
pub enum Direction {
    Out, // Outgoing edge
    In,  // Incoming edge
}

impl Direction {
    pub fn reverse(&self) -> Self {
        match self {
            Direction::Out => Direction::In,
            Direction::In => Direction::Out,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
pub struct EdgeUid {
    label_id: LabelId,
    src_id: VertexId,
    direction: Direction,
    dst_id: VertexId,
    eid: EdgeId,
}

impl EdgeUid {
    pub fn new(
        label_id: LabelId,
        src_id: VertexId,
        direction: Direction,
        dst_id: VertexId,
        eid: EdgeId,
    ) -> Self {
        EdgeUid {
            label_id,
            src_id,
            direction,
            dst_id,
            eid,
        }
    }

    pub fn label_id(&self) -> LabelId {
        self.label_id
    }

    pub fn src_id(&self) -> VertexId {
        self.src_id
    }

    pub fn dst_id(&self) -> VertexId {
        self.dst_id
    }

    pub fn eid(&self) -> EdgeId {
        self.eid
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }
}

impl Ord for EdgeUid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.label_id
            .cmp(&other.label_id)
            .then_with(|| self.src_id.cmp(&other.src_id))
            .then_with(|| self.direction.cmp(&other.direction))
            .then_with(|| self.dst_id.cmp(&other.dst_id))
            .then_with(|| self.eid.cmp(&other.eid))
    }
}

impl PartialOrd for EdgeUid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Edge {
    pub euid: EdgeUid,
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
            euid: EdgeUid::new(label_id, src_id, direction, dst_id, eid),
            properties,
            is_tombstone: false,
        }
    }

    pub fn tombstone(edge: Edge) -> Self {
        Edge {
            euid: edge.euid,
            properties: edge.properties.clone(),
            is_tombstone: true,
        }
    }

    pub fn eid(&self) -> EdgeId {
        self.euid.eid
    }

    pub fn src_id(&self) -> VertexId {
        self.euid.src_id
    }

    pub fn dst_id(&self) -> VertexId {
        self.euid.dst_id
    }

    pub fn label_id(&self) -> LabelId {
        self.euid.label_id
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

    pub fn euid(&self) -> EdgeUid {
        self.euid
    }
}
