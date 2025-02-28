use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Supported primitive data types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    Int,     // i32
    Long,    // i64
    Float,   // f32
    Double,  // f64
    String,  // String
    Boolean, // bool
    Object,  // reserved for complex data type
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyValue {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Boolean(bool),
    Object(HashMap<String, PropertyValue>),
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
            PropertyValue::Object(_) => DataType::Object,
        }
    }

    pub fn as_int(&self) -> Result<i32, ConversionError> {
        match self {
            PropertyValue::Int(v) => Ok(*v),
            _ => Err(ConversionError::TypeMismatch {
                expected: DataType::Int,
                actual: self.data_type(),
            }),
        }
    }

    pub fn as_long(&self) -> Result<i64, ConversionError> {
        match self {
            PropertyValue::Long(v) => Ok(*v),
            _ => Err(ConversionError::TypeMismatch {
                expected: DataType::Long,
                actual: self.data_type(),
            }),
        }
    }

    pub fn as_float(&self) -> Result<f32, ConversionError> {
        match self {
            PropertyValue::Float(v) => Ok(*v),
            _ => Err(ConversionError::TypeMismatch {
                expected: DataType::Float,
                actual: self.data_type(),
            }),
        }
    }

    pub fn as_double(&self) -> Result<f64, ConversionError> {
        match self {
            PropertyValue::Double(v) => Ok(*v),
            _ => Err(ConversionError::TypeMismatch {
                expected: DataType::Double,
                actual: self.data_type(),
            }),
        }
    }

    pub fn as_string(&self) -> Result<String, ConversionError> {
        match self {
            PropertyValue::String(v) => Ok(v.clone()),
            _ => Err(ConversionError::TypeMismatch {
                expected: DataType::String,
                actual: self.data_type(),
            }),
        }
    }

    pub fn as_boolean(&self) -> Result<bool, ConversionError> {
        match self {
            PropertyValue::Boolean(v) => Ok(*v),
            _ => Err(ConversionError::TypeMismatch {
                expected: DataType::Boolean,
                actual: self.data_type(),
            }),
        }
    }

    pub fn as_object(&self) -> Result<&HashMap<String, PropertyValue>, ConversionError> {
        match self {
            PropertyValue::Object(v) => Ok(v),
            _ => Err(ConversionError::TypeMismatch {
                expected: DataType::Object,
                actual: self.data_type(),
            }),
        }
    }
}

/// Type conversion errors
#[derive(Debug, Error, PartialEq)]
pub enum ConversionError {
    #[error("Type mismatch: expected {expected:?}, got {actual:?}")]
    TypeMismatch {
        expected: DataType,
        actual: DataType,
    },
    #[error("Null value not allowed")]
    NullValue,
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
        assert_eq!(PropertyValue::Object(map).data_type(), DataType::Object);
    }

    // Test conversion to specific types (as_* methods)
    #[test]
    fn test_as_int() {
        let value = PropertyValue::Int(42);
        assert_eq!(value.as_int(), Ok(42));

        let value = PropertyValue::Long(42);
        assert_eq!(
            value.as_int(),
            Err(ConversionError::TypeMismatch {
                expected: DataType::Int,
                actual: DataType::Long,
            })
        );

        let value = PropertyValue::Float(42.0);
        assert_eq!(
            value.as_int(),
            Err(ConversionError::TypeMismatch {
                expected: DataType::Int,
                actual: DataType::Float,
            })
        );
    }

    #[test]
    fn test_as_long() {
        let value = PropertyValue::Long(42);
        assert_eq!(value.as_long(), Ok(42));

        let value = PropertyValue::Int(42);
        assert_eq!(
            value.as_long(),
            Err(ConversionError::TypeMismatch {
                expected: DataType::Long,
                actual: DataType::Int,
            })
        );
    }

    #[test]
    fn test_as_float() {
        let value = PropertyValue::Float(42.0);
        assert_eq!(value.as_float(), Ok(42.0));

        let value = PropertyValue::Int(42);
        assert_eq!(
            value.as_float(),
            Err(ConversionError::TypeMismatch {
                expected: DataType::Float,
                actual: DataType::Int,
            })
        );
    }

    #[test]
    fn test_as_double() {
        let value = PropertyValue::Double(42.0);
        assert_eq!(value.as_double(), Ok(42.0));

        let value = PropertyValue::Float(42.0);
        assert_eq!(
            value.as_double(),
            Err(ConversionError::TypeMismatch {
                expected: DataType::Double,
                actual: DataType::Float,
            })
        );
    }

    #[test]
    fn test_as_string() {
        let value = PropertyValue::String("hello".into());
        assert_eq!(value.as_string(), Ok("hello".into()));

        let value = PropertyValue::Int(42);
        assert_eq!(
            value.as_string(),
            Err(ConversionError::TypeMismatch {
                expected: DataType::String,
                actual: DataType::Int,
            })
        );
    }

    #[test]
    fn test_as_boolean() {
        let value = PropertyValue::Boolean(true);
        assert_eq!(value.as_boolean(), Ok(true));

        let value = PropertyValue::String("hello".into());
        assert_eq!(
            value.as_boolean(),
            Err(ConversionError::TypeMismatch {
                expected: DataType::Boolean,
                actual: DataType::String,
            })
        );
    }

    #[test]
    fn test_as_object() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), PropertyValue::String("John".into()));
        map.insert("age".to_string(), PropertyValue::Int(42));
        let value = PropertyValue::Object(map.clone());
        assert_eq!(value.as_object(), Ok(&map));

        let value = PropertyValue::String("hello".into());
        assert_eq!(
            value.as_object(),
            Err(ConversionError::TypeMismatch {
                expected: DataType::Object,
                actual: DataType::String,
            })
        );
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
