use minigu_common::datatype::value::PropertyValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct PropertyRecord(Vec<PropertyValue>);

impl PropertyRecord {
    pub fn new(properties: Vec<PropertyValue>) -> Self {
        PropertyRecord(properties)
    }

    pub fn get(&self, index: usize) -> Option<&PropertyValue> {
        self.0.get(index)
    }

    pub fn set_prop(&mut self, index: usize, prop: PropertyValue) {
        self.0[index] = prop;
    }

    pub fn props(&self) -> &Vec<PropertyValue> {
        &self.0
    }
}
