use common::datatype::value::PropertyValue;
use serde::{Serialize, Deserialize};
use super::properties::PropertyStore;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Vertex {
    pub vid: u64,  
    pub label_id: u64,
    pub properties: PropertyStore,  
}

impl Vertex {
    /// create a new vertex
    pub fn new(vid: u64, label_id: u64, properties: PropertyStore) -> Self {
        Vertex {
            vid,
            label_id,
            properties,
        }
    }

    /// Get the vid
    pub fn vid(&self) -> u64 {
        self.vid
    }

    pub fn set_props(&mut self, indices: &Vec<usize>, props: Vec<PropertyValue>) {
        for (&index, prop) in indices.into_iter().zip(props.into_iter()) {
            self.properties.set_prop(index, prop);
        }
    }
}