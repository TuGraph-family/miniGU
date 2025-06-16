use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported primitive data types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    Int,     // i32
    Long,    // i64
    Float,   // f32
    Double,  // f64
    String,  // String
    Boolean, // bool
    Map,     // reserved for complex data type
    List,    // reserved for complex data type
}

/// Property metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyMeta {
    pub name: String,            // Property name
    pub data_type: DataType,     // Data type
    pub is_optional: bool,       // Nullable
    pub is_unique: bool,         // Unique constraint
    pub default: Option<String>, // Default value as string representation
}

impl PropertyMeta {
    pub fn new(
        name: String,
        data_type: DataType,
        is_optional: bool,
        is_unique: bool,
        default: Option<String>,
    ) -> Self {
        PropertyMeta {
            name,
            data_type,
            is_optional,
            is_unique,
            default,
        }
    }
}

/// Primary key type constraints, supports long and string types currently
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum PrimaryKey {
    Long(i64),
    String(String),
}

impl fmt::Display for PrimaryKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimaryKey::Long(v) => write!(f, "{}", v),
            PrimaryKey::String(v) => write!(f, "{}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_key_display() {
        let pk_long = PrimaryKey::Long(42);
        let pk_string = PrimaryKey::String("key123".into());

        assert_eq!(format!("{}", pk_long), "42");
        assert_eq!(format!("{}", pk_string), "key123");
    }

    #[test]
    fn test_property_meta_creation() {
        let meta = PropertyMeta::new(
            "test_field".to_string(),
            DataType::String,
            true,
            false,
            Some("default_value".to_string()),
        );

        assert_eq!(meta.name, "test_field");
        assert_eq!(meta.data_type, DataType::String);
        assert!(meta.is_optional);
        assert!(!meta.is_unique);
        assert_eq!(meta.default, Some("default_value".to_string()));
    }
}
