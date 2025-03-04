use common::datatype::value::PropertyValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PropertyStore {
    properties: Vec<PropertyValue>,
}

impl PropertyStore {
    pub fn new(properties: Vec<PropertyValue>) -> Self {
        PropertyStore { properties }
    }

    pub fn get(&self, index: usize) -> Option<&PropertyValue> {
        self.properties.get(index)
    }

    pub fn set_prop(&mut self, index: usize, prop: PropertyValue) {
        self.properties[index] = prop;
    }

    pub fn props(&self) -> &Vec<PropertyValue> {
        &self.properties
    }
}
