//! SQL Logic Test (SLT) adapter for MiniGU graph database
//!
//! This module provides integration between MiniGU and the SQL Logic Test framework(here we use it
//! to test GQL queries), enabling comprehensive testing of graph database functionality through
//! standardized test cases. The adapter maintains session state across multiple GQL statements,
//! allowing for proper transaction testing and complex query sequences.
//!
//! ## Core Components
//!
//! - **ColumnTypeSltWrapper**: Enhanced column type system that extends standard column types with
//!   graph-specific types (Vertex, Edge) for MiniGU's graph database capabilities
//! - **SessionWrapper**: Wrapper around MiniGU's Session that implements the DB trait from
//!   sqllogictest, maintaining persistent session state across test statements
//! - **Type Conversion**: Comprehensive mapping between MiniGU's LogicalType system and
//!   SLT-compatible column types
//!
//! ## Usage
//!
//! The adapter is primarily used by the test framework to execute MiniGU queries
//! and validate results against expected outputs in SLT test files. It enables
//! comprehensive testing of graph database operations including DDL, DML, and DQL
//! statements with proper transaction semantics.

use minigu::common::data_type::LogicalType;
use minigu::error::Error as MiniGuError;
use minigu::session::Session;
use sqllogictest::{ColumnType, DB, DBOutput};

/// Enhanced ColumnType for MiniGU that supports vertex and edge types
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ColumnTypeSltWrapper {
    /// Text, varchar results
    Text,
    /// Integer results
    Integer,
    /// Floating point numbers
    FloatingPoint,
    /// Boolean results
    Boolean,
    /// Vertex type (graph-specific)
    Vertex,
    /// Edge type (graph-specific)
    Edge,
    /// Any other type
    Any,
}

impl ColumnType for ColumnTypeSltWrapper {
    fn from_char(value: char) -> Option<Self> {
        match value {
            'T' => Some(Self::Text),
            'I' => Some(Self::Integer),
            'R' => Some(Self::FloatingPoint),
            'V' => Some(Self::Vertex),
            'E' => Some(Self::Edge),
            'B' => Some(Self::Boolean),
            _ => Some(Self::Any),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Text => 'T',
            Self::Integer => 'I',
            Self::FloatingPoint => 'R',
            Self::Boolean => 'B',
            Self::Vertex => 'V',
            Self::Edge => 'E',
            Self::Any => '?',
        }
    }
}

/// Convert LogicalType to MiniGuColumnType
impl From<&LogicalType> for ColumnTypeSltWrapper {
    fn from(logical_type: &LogicalType) -> Self {
        match logical_type {
            LogicalType::String => Self::Text,
            LogicalType::Int8
            | LogicalType::Int16
            | LogicalType::Int32
            | LogicalType::Int64
            | LogicalType::UInt8
            | LogicalType::UInt16
            | LogicalType::UInt32
            | LogicalType::UInt64 => Self::Integer,
            LogicalType::Float32 | LogicalType::Float64 => Self::FloatingPoint,
            LogicalType::Boolean => Self::Boolean,
            LogicalType::Vertex(_) => Self::Vertex,
            LogicalType::Edge(_) => Self::Edge,
            LogicalType::Record(_) => Self::Any,
            LogicalType::Null => Self::Any,
        }
    }
}

/// Session wrapper for MiniGU that maintains session state across multiple SQL statements
pub struct SessionWrapper(Session);

impl SessionWrapper {
    /// Create a new SessionWrapper from a given Session
    pub fn new(session: Session) -> Self {
        Self(session)
    }
}

/// Implementation of DB trait for SessionWrapper
/// This maintains session state across multiple SQL statements, enabling proper transaction testing
impl DB for SessionWrapper {
    type ColumnType = ColumnTypeSltWrapper;
    type Error = MiniGuError;

    fn run(&mut self, sql: &str) -> Result<DBOutput<Self::ColumnType>, Self::Error> {
        // Execute query using the persistent session
        let result = self.0.query(sql)?;

        // Check if there is a result set
        if let Some(schema) = result.schema() {
            let mut records = Vec::new();
            let mut types = Vec::new();

            // Create column type for each field based on its logical type
            for field in schema.fields() {
                let column_type = ColumnTypeSltWrapper::from(field.ty());
                types.push(column_type);
            }

            // Convert data rows
            for chunk in result.iter() {
                for row in chunk.rows() {
                    let mut row_values = Vec::new();
                    for col_index in 0..schema.fields().len() {
                        let value = row
                            .get(col_index)
                            .unwrap_or(minigu::common::value::ScalarValue::Null);
                        let str_value = convert_scalar_value_to_string(&value);
                        row_values.push(str_value);
                    }
                    records.push(row_values);
                }
            }

            Ok(DBOutput::Rows {
                types,
                rows: records,
            })
        } else {
            // No result set, return statement complete
            Ok(DBOutput::StatementComplete(0))
        }
    }
}

fn opt_to_string<T, F>(opt: &Option<T>, f: F) -> String
where
    F: Fn(&T) -> String,
{
    opt.as_ref().map_or_else(|| "NULL".to_string(), f)
}

/// Convert ScalarValue to string
fn convert_scalar_value_to_string(value: &minigu::common::value::ScalarValue) -> String {
    use minigu::common::value::ScalarValue;
    match value {
        ScalarValue::Null => "NULL".to_string(),
        ScalarValue::Boolean(opt) => opt_to_string(opt, |v| v.to_string()),
        ScalarValue::Int8(opt) => opt_to_string(opt, |v| v.to_string()),
        ScalarValue::Int16(opt) => opt_to_string(opt, |v| v.to_string()),
        ScalarValue::Int32(opt) => opt_to_string(opt, |v| v.to_string()),
        ScalarValue::Int64(opt) => opt_to_string(opt, |v| v.to_string()),
        ScalarValue::UInt8(opt) => opt_to_string(opt, |v| v.to_string()),
        ScalarValue::UInt16(opt) => opt_to_string(opt, |v| v.to_string()),
        ScalarValue::UInt32(opt) => opt_to_string(opt, |v| v.to_string()),
        ScalarValue::UInt64(opt) => opt_to_string(opt, |v| v.to_string()),
        ScalarValue::Float32(opt) => opt_to_string(opt, |v| v.to_string()),
        ScalarValue::Float64(opt) => opt_to_string(opt, |v| v.to_string()),
        ScalarValue::String(opt) => opt_to_string(opt, |v| v.clone()),
        ScalarValue::Vertex(opt) => opt_to_string(opt, |v| format!("{:?}", v)),
        ScalarValue::Edge(opt) => opt_to_string(opt, |v| format!("{:?}", v)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mini_gu_column_type_basic() {
        // Test basic types
        assert_eq!(
            ColumnTypeSltWrapper::from_char('T'),
            Some(ColumnTypeSltWrapper::Text)
        );
        assert_eq!(
            ColumnTypeSltWrapper::from_char('I'),
            Some(ColumnTypeSltWrapper::Integer)
        );
        assert_eq!(
            ColumnTypeSltWrapper::from_char('R'),
            Some(ColumnTypeSltWrapper::FloatingPoint)
        );
        assert_eq!(
            ColumnTypeSltWrapper::from_char('?'),
            Some(ColumnTypeSltWrapper::Any)
        );

        // Test graph-specific types
        assert_eq!(
            ColumnTypeSltWrapper::from_char('V'),
            Some(ColumnTypeSltWrapper::Vertex)
        );
        assert_eq!(
            ColumnTypeSltWrapper::from_char('E'),
            Some(ColumnTypeSltWrapper::Edge)
        );

        // Test unknown character
        assert_eq!(
            ColumnTypeSltWrapper::from_char('X'),
            Some(ColumnTypeSltWrapper::Any)
        );
    }

    #[test]
    fn test_mini_gu_column_type_to_char() {
        // Test conversion to char
        assert_eq!(ColumnTypeSltWrapper::Text.to_char(), 'T');
        assert_eq!(ColumnTypeSltWrapper::Integer.to_char(), 'I');
        assert_eq!(ColumnTypeSltWrapper::FloatingPoint.to_char(), 'R');
        assert_eq!(ColumnTypeSltWrapper::Vertex.to_char(), 'V');
        assert_eq!(ColumnTypeSltWrapper::Edge.to_char(), 'E');
        assert_eq!(ColumnTypeSltWrapper::Any.to_char(), '?');
    }

    #[test]
    fn test_logical_type_to_column_type() {
        use minigu::common::data_type::{DataField, LogicalType};

        // Test basic types
        assert_eq!(
            ColumnTypeSltWrapper::from(&LogicalType::String),
            ColumnTypeSltWrapper::Text
        );
        assert_eq!(
            ColumnTypeSltWrapper::from(&LogicalType::Int32),
            ColumnTypeSltWrapper::Integer
        );
        assert_eq!(
            ColumnTypeSltWrapper::from(&LogicalType::Float64),
            ColumnTypeSltWrapper::FloatingPoint
        );
        assert_eq!(
            ColumnTypeSltWrapper::from(&LogicalType::Boolean),
            ColumnTypeSltWrapper::Boolean
        );

        // Test graph-specific types
        assert_eq!(
            ColumnTypeSltWrapper::from(&LogicalType::Vertex(vec![])),
            ColumnTypeSltWrapper::Vertex
        );
        assert_eq!(
            ColumnTypeSltWrapper::from(&LogicalType::Edge(vec![])),
            ColumnTypeSltWrapper::Edge
        );

        // Test with fields
        let vertex_with_fields = LogicalType::Vertex(vec![DataField::new(
            "name".to_string(),
            LogicalType::String,
            false,
        )]);
        assert_eq!(
            ColumnTypeSltWrapper::from(&vertex_with_fields),
            ColumnTypeSltWrapper::Vertex
        );
    }

    #[test]
    fn test_session_wrapper_creation() {
        // Test that SessionWrapper can be created successfully
        let config = minigu::database::DatabaseConfig::default();
        let database = minigu::database::Database::open_in_memory(&config).unwrap();
        let session = database.session().unwrap();
        let _wrapper = SessionWrapper::new(session);
    }

    #[test]
    fn test_session_wrapper_basic_query() {
        // Test that SessionWrapper can execute basic queries
        let config = minigu::database::DatabaseConfig::default();
        let database = minigu::database::Database::open_in_memory(&config).unwrap();
        let session = database.session().unwrap();
        let mut wrapper = SessionWrapper::new(session);
        // Use a simple query that should work
        let result = wrapper.run("CALL create_test_graph('test_graph')");
        if let Err(e) = &result {
            println!("Error: {:?}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_session_wrapper_session_persistence() {
        // Test that session state is maintained across multiple queries
        let config = minigu::database::DatabaseConfig::default();
        let database = minigu::database::Database::open_in_memory(&config).unwrap();
        let session = database.session().unwrap();
        let mut wrapper = SessionWrapper::new(session);

        // First query should create session
        let result1 = wrapper.run("CALL create_test_graph('test_graph_1')");
        assert!(result1.is_ok());

        // Second query should use the same session
        let result2 = wrapper.run("CALL create_test_graph('test_graph_2')");
        assert!(result2.is_ok());

        // Both queries should succeed, indicating session persistence
        assert!(result1.is_ok() && result2.is_ok());
    }
}
