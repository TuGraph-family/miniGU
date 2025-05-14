use std::collections::HashMap;
use std::fmt;

use enum_as_inner::EnumAsInner;
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
    pub name: String,                   // Property name
    pub data_type: DataType,            // Data type
    pub is_optional: bool,              // Nullable
    pub is_unique: bool,                // Unique constraint
    pub default: Option<PropertyValue>, // Default value
}

impl PropertyMeta {
    pub fn new(
        name: String,
        data_type: DataType,
        is_optional: bool,
        is_unique: bool,
        default: Option<PropertyValue>,
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

/// Property value container
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumAsInner)]
pub enum PropertyValue {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Boolean(bool),
    Map(HashMap<String, PropertyValue>),
    List(Vec<PropertyValue>),
}

impl PropertyValue {
    pub fn data_type(&self) -> DataType {
        match self {
            PropertyValue::Int(_) => DataType::Int,
            PropertyValue::Long(_) => DataType::Long,
            PropertyValue::Float(_) => DataType::Float,
            PropertyValue::Double(_) => DataType::Double,
            PropertyValue::String(_) => DataType::String,
            PropertyValue::Boolean(_) => DataType::Boolean,
            PropertyValue::Map(_) => DataType::Map,
            PropertyValue::List(_) => DataType::List,
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

    // Test DataType enum matching
    #[test]
    fn test_data_type() {
        // Test all types and their corresponding data types
        assert_eq!(PropertyValue::Int(42).data_type(), DataType::Int);
        assert_eq!(PropertyValue::Long(42).data_type(), DataType::Long);
        assert_eq!(PropertyValue::Float(42.0).data_type(), DataType::Float);
        assert_eq!(PropertyValue::Double(42.0).data_type(), DataType::Double);
        assert_eq!(
            PropertyValue::String("hello".into()).data_type(),
            DataType::String
        );
        assert_eq!(PropertyValue::Boolean(true).data_type(), DataType::Boolean);
        let mut map = HashMap::new();
        map.insert("name".to_string(), PropertyValue::String("John".into()));
        map.insert("age".to_string(), PropertyValue::Int(42));
        assert_eq!(PropertyValue::Map(map).data_type(), DataType::Map);
    }

    // Test as_* reference methods
    #[test]
    fn test_as_int() {
        let value = PropertyValue::Int(42);
        assert_eq!(value.as_int(), Some(&42));

        let value = PropertyValue::Long(42);
        assert_eq!(value.as_int(), None);

        let value = PropertyValue::Float(42.0);
        assert_eq!(value.as_int(), None);
    }

    #[test]
    fn test_as_long() {
        let value = PropertyValue::Long(42);
        assert_eq!(value.as_long(), Some(&42));

        let value = PropertyValue::Int(42);
        assert_eq!(value.as_long(), None);
    }

    #[test]
    fn test_as_float() {
        let value = PropertyValue::Float(42.0);
        assert_eq!(value.as_float(), Some(&42.0));

        let value = PropertyValue::Int(42);
        assert_eq!(value.as_float(), None);
    }

    #[test]
    fn test_as_double() {
        let value = PropertyValue::Double(42.0);
        assert_eq!(value.as_double(), Some(&42.0));

        let value = PropertyValue::Float(42.0);
        assert_eq!(value.as_double(), None);
    }

    #[test]
    fn test_as_string() {
        let value = PropertyValue::String("hello".into());
        assert_eq!(value.as_string(), Some(&"hello".to_string()));

        let value = PropertyValue::Int(42);
        assert_eq!(value.as_string(), None);
    }

    #[test]
    fn test_as_boolean() {
        let value = PropertyValue::Boolean(true);
        assert_eq!(value.as_boolean(), Some(&true));

        let value = PropertyValue::String("hello".into());
        assert_eq!(value.as_boolean(), None);
    }

    #[test]
    fn test_as_map() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), PropertyValue::String("John".into()));
        map.insert("age".to_string(), PropertyValue::Int(42));
        let value = PropertyValue::Map(map.clone());
        assert_eq!(value.as_map(), Some(&map));

        let value = PropertyValue::String("hello".into());
        assert_eq!(value.as_map(), None);
    }

    #[test]
    fn test_as_list() {
        let list = vec![PropertyValue::Int(42)];
        let value = PropertyValue::List(list.clone());
        assert_eq!(value.as_list(), Some(&list));

        let value = PropertyValue::String("hello".into());
        assert_eq!(value.as_list(), None);
    }

    // Test into_* conversion methods
    #[test]
    fn test_into_int() {
        let value = PropertyValue::Int(42);
        assert_eq!(value.into_int(), Ok(42));
    }

    #[test]
    fn test_into_long() {
        let value = PropertyValue::Long(42);
        assert_eq!(value.into_long(), Ok(42));
    }

    #[test]
    fn test_into_float() {
        let value = PropertyValue::Float(42.0);
        assert_eq!(value.into_float(), Ok(42.0));
    }

    #[test]
    fn test_into_double() {
        let value = PropertyValue::Double(42.0);
        assert_eq!(value.into_double(), Ok(42.0));
    }

    #[test]
    fn test_into_string() {
        let value = PropertyValue::String("hello".into());
        assert_eq!(value.into_string(), Ok("hello".to_string()));
    }

    #[test]
    fn test_into_boolean() {
        let value = PropertyValue::Boolean(true);
        assert_eq!(value.into_boolean(), Ok(true));
    }

    #[test]
    fn test_into_map() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), PropertyValue::String("John".into()));
        map.insert("age".to_string(), PropertyValue::Int(42));
        let value = PropertyValue::Map(map.clone());
        assert_eq!(value.into_map(), Ok(map));
    }

    #[test]
    fn test_into_list() {
        let list = vec![PropertyValue::Int(42)];
        let value = PropertyValue::List(list.clone());
        assert_eq!(value.into_list(), Ok(list));
    }

    // Test PrimaryKey display
    #[test]
    fn test_primary_key_display() {
        let pk_long = PrimaryKey::Long(42);
        let pk_string = PrimaryKey::String("key123".into());

        assert_eq!(format!("{}", pk_long), "42");
        assert_eq!(format!("{}", pk_string), "key123");
    }
}
