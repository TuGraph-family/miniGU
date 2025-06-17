use minigu_common::value::ScalarValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct PropertyRecord(Vec<ScalarValue>);

impl PropertyRecord {
    pub fn new(properties: Vec<ScalarValue>) -> Self {
        PropertyRecord(properties)
    }

    pub fn get(&self, index: usize) -> Option<&ScalarValue> {
        self.0.get(index)
    }

    pub fn set_prop(&mut self, index: usize, prop: ScalarValue) {
        self.0[index] = prop;
    }

    pub fn props(&self) -> &Vec<ScalarValue> {
        &self.0
    }
}
