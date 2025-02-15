use common::datatype::value::PropertyValue;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropertyStore {
    properties: Vec<PropertyValue>,
}

impl PropertyStore {
    pub fn new(properties: Vec<PropertyValue>) -> Self {
        PropertyStore {
            properties: properties,
        }
    }

    pub fn get(&self, index: usize) -> Option<&PropertyValue> {
        self.properties.get(index)
    }
}