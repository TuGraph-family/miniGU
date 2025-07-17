use std::fmt;
use std::sync::Arc;

use minigu::common::data_type::LogicalType;
use minigu::database::{Database, DatabaseConfig};
use minigu::error::Error as MiniGuError;
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
            _ => Some(Self::Any),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Text => 'T',
            Self::Integer => 'I',
            Self::FloatingPoint => 'R',
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
            LogicalType::Vertex(_) => Self::Vertex,
            LogicalType::Edge(_) => Self::Edge,
            LogicalType::Boolean => Self::Any,
            LogicalType::Record(_) => Self::Any,
            LogicalType::Null => Self::Any,
        }
    }
}

/// MiniGU database adapter for SQLLogicTest
#[derive(Clone)]
pub struct MiniGuDb {
    database: Arc<Database>,
}

impl MiniGuDb {
    /// Create new MiniGU database adapter
    pub fn new() -> Result<Self, MiniGuError> {
        let config = DatabaseConfig::default();
        let database = Arc::new(Database::open_in_memory(&config)?);
        Ok(Self { database })
    }
}

impl Default for MiniGuDb {
    fn default() -> Self {
        Self::new().expect("Failed to create MiniGU database")
    }
}

/// Error wrapper, for compatibility with SQLLogicTest error handling
#[derive(Debug)]
pub struct SqlLogicTestError(MiniGuError);

impl fmt::Display for SqlLogicTestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MiniGU Error: {}", self.0)
    }
}

impl std::error::Error for SqlLogicTestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl From<MiniGuError> for SqlLogicTestError {
    fn from(error: MiniGuError) -> Self {
        SqlLogicTestError(error)
    }
}

impl DB for MiniGuDb {
    type ColumnType = MiniGuColumnType;
    type Error = SqlLogicTestError;

    fn run(&mut self, sql: &str) -> Result<DBOutput<Self::ColumnType>, Self::Error> {
        // Create new session
        let mut session = self.database.session().map_err(SqlLogicTestError::from)?;

        // Execute query
        let result = session.query(sql).map_err(SqlLogicTestError::from)?;

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
            MiniGuColumnType::Any
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
}
