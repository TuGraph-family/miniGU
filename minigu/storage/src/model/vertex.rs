use serde::{Serialize, Deserialize};
use super::properties::PropertyStore;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
}