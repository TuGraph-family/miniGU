use common::datatype::value::PropertyValue;
use serde::{Serialize, Deserialize};
use super::properties::PropertyStore;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Direction {
    Out,           // Outgoing edge
    In,            // Incoming edge
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Edge {
    pub eid: u64,                   // ID of the edge
    pub source_id: u64,             // ID of the source vertex
    pub dst_id: u64,                // ID of the target vertex
    pub label_id: u64,              // Label of the edge
    pub direction: Direction,       // Direction of the edge
    pub properties: PropertyStore,  // Properties of the edge
}

impl Edge {
    pub fn new(eid: u64, source_id: u64, dst_id: u64, label_id: u64, direction: Direction, properties: PropertyStore) -> Self {
        Edge {
            eid,
            source_id,
            dst_id,
            label_id,
            direction,
            properties,
        }
    }

    pub fn set_props(&mut self, indices: &Vec<usize>, props: Vec<PropertyValue>) {
        for (&index, prop) in indices.into_iter().zip(props.into_iter()) {
            self.properties.set_prop(index, prop);
        }
    }
}