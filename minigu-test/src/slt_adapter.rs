use std::sync::{Arc, Mutex};

use minigu::common::data_type::LogicalType;
use minigu::database::{Database, DatabaseConfig};
use minigu::error::Error as MiniGuError;
use minigu::session::Session;
use sqllogictest::{ColumnType, DB, DBOutput};

/// Enhanced ColumnType for MiniGU that supports vertex and edge types
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MiniGuColumnType {
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

impl ColumnType for MiniGuColumnType {
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
impl From<&LogicalType> for MiniGuColumnType {
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
#[derive(Clone)]
pub struct SessionWrapper {
    database: Arc<Database>,
    session: Option<Arc<Mutex<Session>>>,
}

impl SessionWrapper {
    /// Create new SessionWrapper with a fresh database
    pub fn new() -> Result<Self, MiniGuError> {
        let config = DatabaseConfig::default();
        let database = Arc::new(Database::open_in_memory(&config)?);
        Ok(Self {
            database,
            session: None,
        })
    }

    /// Get or create a session for this wrapper
    fn get_or_create_session(&mut self) -> Result<Arc<Mutex<Session>>, MiniGuError> {
        if self.session.is_none() {
            self.session = Some(Arc::new(Mutex::new(self.database.session()?)));
        }
        Ok(self.session.as_ref().unwrap().clone())
    }

    /// Reset the session (useful for testing transaction rollback scenarios)
    pub fn reset_session(&mut self) -> Result<(), MiniGuError> {
        self.session = Some(Arc::new(Mutex::new(self.database.session()?)));
        Ok(())
    }
}

impl Default for SessionWrapper {
    fn default() -> Self {
        Self::new().expect("Failed to create SessionWrapper")
    }
}

/// Implementation of DB trait for SessionWrapper
/// This maintains session state across multiple SQL statements, enabling proper transaction testing
impl DB for SessionWrapper {
    type ColumnType = MiniGuColumnType;
    type Error = MiniGuError;

    fn run(&mut self, sql: &str) -> Result<DBOutput<Self::ColumnType>, Self::Error> {
        // Get or create session (maintains session state across calls)
        let session_arc = self.get_or_create_session()?;
        let mut session = session_arc.lock().map_err(|_| MiniGuError::SessionClosed)?;

        // Execute query using the persistent session
        let result = session.query(sql)?;

        // Check if there is a result set
        if let Some(schema) = result.schema() {
            let mut records = Vec::new();
            let mut types = Vec::new();

            // Create column type for each field based on its logical type
            for field in schema.fields() {
                let column_type = MiniGuColumnType::from(field.ty());
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
            MiniGuColumnType::from_char('T'),
            Some(MiniGuColumnType::Text)
        );
        assert_eq!(
            MiniGuColumnType::from_char('I'),
            Some(MiniGuColumnType::Integer)
        );
        assert_eq!(
            MiniGuColumnType::from_char('R'),
            Some(MiniGuColumnType::FloatingPoint)
        );
        assert_eq!(
            MiniGuColumnType::from_char('?'),
            Some(MiniGuColumnType::Any)
        );

        // Test graph-specific types
        assert_eq!(
            MiniGuColumnType::from_char('V'),
            Some(MiniGuColumnType::Vertex)
        );
        assert_eq!(
            MiniGuColumnType::from_char('E'),
            Some(MiniGuColumnType::Edge)
        );

        // Test unknown character
        assert_eq!(
            MiniGuColumnType::from_char('X'),
            Some(MiniGuColumnType::Any)
        );
    }

    #[test]
    fn test_mini_gu_column_type_to_char() {
        // Test conversion to char
        assert_eq!(MiniGuColumnType::Text.to_char(), 'T');
        assert_eq!(MiniGuColumnType::Integer.to_char(), 'I');
        assert_eq!(MiniGuColumnType::FloatingPoint.to_char(), 'R');
        assert_eq!(MiniGuColumnType::Vertex.to_char(), 'V');
        assert_eq!(MiniGuColumnType::Edge.to_char(), 'E');
        assert_eq!(MiniGuColumnType::Any.to_char(), '?');
    }

    #[test]
    fn test_logical_type_to_column_type() {
        use minigu::common::data_type::{DataField, LogicalType};

        // Test basic types
        assert_eq!(
            MiniGuColumnType::from(&LogicalType::String),
            MiniGuColumnType::Text
        );
        assert_eq!(
            MiniGuColumnType::from(&LogicalType::Int32),
            MiniGuColumnType::Integer
        );
        assert_eq!(
            MiniGuColumnType::from(&LogicalType::Float64),
            MiniGuColumnType::FloatingPoint
        );
        assert_eq!(
            MiniGuColumnType::from(&LogicalType::Boolean),
            MiniGuColumnType::Boolean
        );

        // Test graph-specific types
        assert_eq!(
            MiniGuColumnType::from(&LogicalType::Vertex(vec![])),
            MiniGuColumnType::Vertex
        );
        assert_eq!(
            MiniGuColumnType::from(&LogicalType::Edge(vec![])),
            MiniGuColumnType::Edge
        );

        // Test with fields
        let vertex_with_fields = LogicalType::Vertex(vec![DataField::new(
            "name".to_string(),
            LogicalType::String,
            false,
        )]);
        assert_eq!(
            MiniGuColumnType::from(&vertex_with_fields),
            MiniGuColumnType::Vertex
        );
    }

    #[test]
    fn test_session_wrapper_creation() {
        // Test that SessionWrapper can be created successfully
        let wrapper = SessionWrapper::new();
        assert!(wrapper.is_ok());
    }

    #[test]
    fn test_session_wrapper_default() {
        // Test that SessionWrapper implements Default
        let wrapper = SessionWrapper::default();
        // Database should be created successfully
        assert!(wrapper.database.session().is_ok());
    }

    #[test]
    fn test_session_wrapper_clone() {
        // Test that SessionWrapper can be cloned
        let wrapper = SessionWrapper::new().unwrap();
        let cloned = wrapper.clone();
        // Both should be able to create sessions
        assert!(wrapper.database.session().is_ok());
        assert!(cloned.database.session().is_ok());
    }

    #[test]
    fn test_session_wrapper_reset() {
        // Test that session can be reset
        let mut wrapper = SessionWrapper::new().unwrap();
        let result = wrapper.reset_session();
        assert!(result.is_ok());
    }

    #[test]
    fn test_session_wrapper_basic_query() {
        // Test that SessionWrapper can execute basic queries
        let mut wrapper = SessionWrapper::new().unwrap();
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
        let mut wrapper = SessionWrapper::new().unwrap();

        // First query should create session
        let result1 = wrapper.run("CALL create_test_graph('test_graph_1')");
        assert!(result1.is_ok());

        // Second query should use the same session
        let result2 = wrapper.run("CALL create_test_graph('test_graph_2')");
        assert!(result2.is_ok());

        // Both queries should succeed, indicating session persistence
        // We can't directly compare DBOutput, but we can verify both succeeded
        assert!(result1.is_ok() && result2.is_ok());
    }
}
