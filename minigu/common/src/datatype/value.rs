use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::error::{ConversionError, DataError};

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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

    /// Returns a reference to the inner i32 value if this is an Int variant, None otherwise
    pub fn as_int(&self) -> Option<&i32> {
        match self {
            PropertyValue::Int(v) => Some(v),
            _ => None,
        }
    }

    /// Returns a reference to the inner i64 value if this is a Long variant, None otherwise
    pub fn as_long(&self) -> Option<&i64> {
        match self {
            PropertyValue::Long(v) => Some(v),
            _ => None,
        }
    }

    /// Returns a reference to the inner f32 value if this is a Float variant, None otherwise
    pub fn as_float(&self) -> Option<&f32> {
        match self {
            PropertyValue::Float(v) => Some(v),
            _ => None,
        }
    }

    /// Returns a reference to the inner f64 value if this is a Double variant, None otherwise
    pub fn as_double(&self) -> Option<&f64> {
        match self {
            PropertyValue::Double(v) => Some(v),
            _ => None,
        }
    }

    /// Returns a reference to the inner String value if this is a String variant, None otherwise
    pub fn as_string(&self) -> Option<&String> {
        match self {
            PropertyValue::String(v) => Some(v),
            _ => None,
        }
    }

    /// Returns a reference to the inner bool value if this is a Boolean variant, None otherwise
    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            PropertyValue::Boolean(v) => Some(v),
            _ => None,
        }
    }

    /// Returns a reference to the inner HashMap if this is a Map variant, None otherwise
    pub fn as_map(&self) -> Option<&HashMap<String, PropertyValue>> {
        match self {
            PropertyValue::Map(v) => Some(v),
            _ => None,
        }
    }

    /// Returns a reference to the inner Vec if this is a List variant, None otherwise
    pub fn as_list(&self) -> Option<&Vec<PropertyValue>> {
        match self {
            PropertyValue::List(v) => Some(v),
            _ => None,
        }
    }

    /// Attempts to convert into an i32, returning an error if this is not an Int variant
    pub fn try_into_int(self) -> Result<i32, DataError> {
        match self {
            PropertyValue::Int(v) => Ok(v),
            _ => Err(DataError::ConversionError(ConversionError::TypeMismatch {
                expected: DataType::Int,
                actual: self.data_type(),
            })),
        }
    }

    /// Attempts to convert into an i64, returning an error if this is not a Long variant
    pub fn try_into_long(self) -> Result<i64, DataError> {
        match self {
            PropertyValue::Long(v) => Ok(v),
            _ => Err(DataError::ConversionError(ConversionError::TypeMismatch {
                expected: DataType::Long,
                actual: self.data_type(),
            })),
        }
    }

    /// Attempts to convert into an f32, returning an error if this is not a Float variant
    pub fn try_into_float(self) -> Result<f32, DataError> {
        match self {
            PropertyValue::Float(v) => Ok(v),
            _ => Err(DataError::ConversionError(ConversionError::TypeMismatch {
                expected: DataType::Float,
                actual: self.data_type(),
            })),
        }
    }

    /// Attempts to convert into an f64, returning an error if this is not a Double variant
    pub fn try_into_double(self) -> Result<f64, DataError> {
        match self {
            PropertyValue::Double(v) => Ok(v),
            _ => Err(DataError::ConversionError(ConversionError::TypeMismatch {
                expected: DataType::Double,
                actual: self.data_type(),
            })),
        }
    }

    /// Attempts to convert into a String, returning an error if this is not a String variant
    pub fn try_into_string(self) -> Result<String, DataError> {
        match self {
            PropertyValue::String(v) => Ok(v),
            _ => Err(DataError::ConversionError(ConversionError::TypeMismatch {
                expected: DataType::String,
                actual: self.data_type(),
            })),
        }
    }

    /// Attempts to convert into a bool, returning an error if this is not a Boolean variant
    pub fn try_into_boolean(self) -> Result<bool, DataError> {
        match self {
            PropertyValue::Boolean(v) => Ok(v),
            _ => Err(DataError::ConversionError(ConversionError::TypeMismatch {
                expected: DataType::Boolean,
                actual: self.data_type(),
            })),
        }
    }

    /// Attempts to convert into a HashMap, returning an error if this is not a Map variant
    pub fn try_into_map(self) -> Result<HashMap<String, PropertyValue>, DataError> {
        match self {
            PropertyValue::Map(v) => Ok(v),
            _ => Err(DataError::ConversionError(ConversionError::TypeMismatch {
                expected: DataType::Map,
                actual: self.data_type(),
            })),
        }
    }

    /// Attempts to convert into a Vec, returning an error if this is not a List variant
    pub fn try_into_list(self) -> Result<Vec<PropertyValue>, DataError> {
        match self {
            PropertyValue::List(v) => Ok(v),
            _ => Err(DataError::ConversionError(ConversionError::TypeMismatch {
                expected: DataType::List,
                actual: self.data_type(),
            })),
        }
    }

    /// Converts into an i32, panicking if this is not an Int variant
    pub fn into_int(self) -> i32 {
        match self {
            PropertyValue::Int(v) => v,
            _ => panic!("called `PropertyValue::into_int()` on a non-int value"),
        }
    }

    /// Converts into an i64, panicking if this is not a Long variant
    pub fn into_long(self) -> i64 {
        match self {
            PropertyValue::Long(v) => v,
            _ => panic!("called `PropertyValue::into_long()` on a non-long value"),
        }
    }

    /// Converts into an f32, panicking if this is not a Float variant
    pub fn into_float(self) -> f32 {
        match self {
            PropertyValue::Float(v) => v,
            _ => panic!("called `PropertyValue::into_float()` on a non-float value"),
        }
    }

    /// Converts into an f64, panicking if this is not a Double variant
    pub fn into_double(self) -> f64 {
        match self {
            PropertyValue::Double(v) => v,
            _ => panic!("called `PropertyValue::into_double()` on a non-double value"),
        }
    }

    /// Converts into a String, panicking if this is not a String variant
    pub fn into_string(self) -> String {
        match self {
            PropertyValue::String(v) => v,
            _ => panic!("called `PropertyValue::into_string()` on a non-string value"),
        }
    }

    /// Converts into a bool, panicking if this is not a Boolean variant
    pub fn into_boolean(self) -> bool {
        match self {
            PropertyValue::Boolean(v) => v,
            _ => panic!("called `PropertyValue::into_boolean()` on a non-boolean value"),
        }
    }

    /// Converts into a HashMap, panicking if this is not a Map variant
    pub fn into_map(self) -> HashMap<String, PropertyValue> {
        match self {
            PropertyValue::Map(v) => v,
            _ => panic!("called `PropertyValue::into_map()` on a non-map value"),
        }
    }

    /// Converts into a Vec, panicking if this is not a List variant
    pub fn into_list(self) -> Vec<PropertyValue> {
        match self {
            PropertyValue::List(v) => v,
            _ => panic!("called `PropertyValue::into_list()` on a non-list value"),
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

    // Test try_into_* conversion methods
    #[test]
    fn test_try_into_int() {
        let value = PropertyValue::Int(42);
        assert_eq!(value.try_into_int(), Ok(42));

        let value = PropertyValue::Long(42);
        assert_eq!(
            value.try_into_int(),
            Err(DataError::ConversionError(ConversionError::TypeMismatch {
                expected: DataType::Int,
                actual: DataType::Long,
            }))
        );
    }

    #[test]
    fn test_try_into_long() {
        let value = PropertyValue::Long(42);
        assert_eq!(value.try_into_long(), Ok(42));

        let value = PropertyValue::Int(42);
        assert_eq!(
            value.try_into_long(),
            Err(DataError::ConversionError(ConversionError::TypeMismatch {
                expected: DataType::Long,
                actual: DataType::Int,
            }))
        );
    }

    #[test]
    fn test_try_into_float() {
        let value = PropertyValue::Float(42.0);
        assert_eq!(value.try_into_float(), Ok(42.0));

        let value = PropertyValue::Int(42);
        assert_eq!(
            value.try_into_float(),
            Err(DataError::ConversionError(ConversionError::TypeMismatch {
                expected: DataType::Float,
                actual: DataType::Int,
            }))
        );
    }

    #[test]
    fn test_try_into_double() {
        let value = PropertyValue::Double(42.0);
        assert_eq!(value.try_into_double(), Ok(42.0));

        let value = PropertyValue::Float(42.0);
        assert_eq!(
            value.try_into_double(),
            Err(DataError::ConversionError(ConversionError::TypeMismatch {
                expected: DataType::Double,
                actual: DataType::Float,
            }))
        );
    }

    #[test]
    fn test_try_into_string() {
        let value = PropertyValue::String("hello".into());
        assert_eq!(value.try_into_string(), Ok("hello".to_string()));

        let value = PropertyValue::Int(42);
        assert_eq!(
            value.try_into_string(),
            Err(DataError::ConversionError(ConversionError::TypeMismatch {
                expected: DataType::String,
                actual: DataType::Int,
            }))
        );
    }

    #[test]
    fn test_try_into_boolean() {
        let value = PropertyValue::Boolean(true);
        assert_eq!(value.try_into_boolean(), Ok(true));

        let value = PropertyValue::String("hello".into());
        assert_eq!(
            value.try_into_boolean(),
            Err(DataError::ConversionError(ConversionError::TypeMismatch {
                expected: DataType::Boolean,
                actual: DataType::String,
            }))
        );
    }

    // Test into_* conversion methods
    #[test]
    fn test_into_int() {
        let value = PropertyValue::Int(42);
        assert_eq!(value.into_int(), 42);
    }

    #[test]
    #[should_panic(expected = "called `PropertyValue::into_int()` on a non-int value")]
    fn test_into_int_panic() {
        let value = PropertyValue::Long(42);
        value.into_int();
    }

    #[test]
    fn test_into_long() {
        let value = PropertyValue::Long(42);
        assert_eq!(value.into_long(), 42);
    }

    #[test]
    #[should_panic(expected = "called `PropertyValue::into_long()` on a non-long value")]
    fn test_into_long_panic() {
        let value = PropertyValue::Int(42);
        value.into_long();
    }

    #[test]
    fn test_into_float() {
        let value = PropertyValue::Float(42.0);
        assert_eq!(value.into_float(), 42.0);
    }

    #[test]
    #[should_panic(expected = "called `PropertyValue::into_float()` on a non-float value")]
    fn test_into_float_panic() {
        let value = PropertyValue::Int(42);
        value.into_float();
    }

    #[test]
    fn test_into_double() {
        let value = PropertyValue::Double(42.0);
        assert_eq!(value.into_double(), 42.0);
    }

    #[test]
    #[should_panic(expected = "called `PropertyValue::into_double()` on a non-double value")]
    fn test_into_double_panic() {
        let value = PropertyValue::Float(42.0);
        value.into_double();
    }

    #[test]
    fn test_into_string() {
        let value = PropertyValue::String("hello".into());
        assert_eq!(value.into_string(), "hello");
    }

    #[test]
    #[should_panic(expected = "called `PropertyValue::into_string()` on a non-string value")]
    fn test_into_string_panic() {
        let value = PropertyValue::Int(42);
        value.into_string();
    }

    #[test]
    fn test_into_boolean() {
        let value = PropertyValue::Boolean(true);
        assert_eq!(value.into_boolean(), true);
    }

    #[test]
    #[should_panic(expected = "called `PropertyValue::into_boolean()` on a non-boolean value")]
    fn test_into_boolean_panic() {
        let value = PropertyValue::String("hello".into());
        value.into_boolean();
    }

    #[test]
    fn test_into_map() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), PropertyValue::String("John".into()));
        map.insert("age".to_string(), PropertyValue::Int(42));
        let value = PropertyValue::Map(map.clone());
        assert_eq!(value.into_map(), map);
    }

    #[test]
    #[should_panic(expected = "called `PropertyValue::into_map()` on a non-map value")]
    fn test_into_map_panic() {
        let value = PropertyValue::String("hello".into());
        value.into_map();
    }

    #[test]
    fn test_into_list() {
        let list = vec![PropertyValue::Int(42)];
        let value = PropertyValue::List(list.clone());
        assert_eq!(value.into_list(), list);
    }

    #[test]
    #[should_panic(expected = "called `PropertyValue::into_list()` on a non-list value")]
    fn test_into_list_panic() {
        let value = PropertyValue::String("hello".into());
        value.into_list();
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
