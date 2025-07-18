use std::borrow::Borrow;
use std::hash::{Hash, Hasher};

use minigu_common::data_type::LogicalType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Property {
    name: String,
    logical_type: LogicalType,
    nullable: bool,
}

impl Borrow<str> for Property {
    fn borrow(&self) -> &str {
        &self.name
    }
}

impl Hash for Property {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Property {
    #[inline]
    pub fn new(name: String, logical_type: LogicalType, nullable: bool) -> Self {
        Self {
            name,
            logical_type,
            nullable,
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn logical_type(&self) -> &LogicalType {
        &self.logical_type
    }

    #[inline]
    pub fn nullable(&self) -> bool {
        self.nullable
    }
}
