use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;
use minigu::database::{Database, DatabaseConfig};
use minigu::error::Error as MiniGuError;
use sqllogictest::{AsyncDB, DBOutput, DefaultColumnType};

/// MiniGU database adapter for SQLLogicTest
pub struct MiniGuDB {
    database: Arc<Database>,
}

impl MiniGuDB {
    /// Create new MiniGU database adapter
    pub fn new() -> Result<Self, MiniGuError> {
        let config = DatabaseConfig::default();
        let database = Arc::new(Database::open_in_memory(&config)?);
        Ok(Self { database })
    }
}

impl Default for MiniGuDB {
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

#[async_trait]
impl AsyncDB for MiniGuDB {
    type ColumnType = DefaultColumnType;
    type Error = SqlLogicTestError;

    async fn run(&mut self, sql: &str) -> Result<DBOutput<Self::ColumnType>, Self::Error> {
        // Create new session
        let mut session = self.database.session().map_err(SqlLogicTestError::from)?;

        // Execute query
        let result = session.query(sql).map_err(SqlLogicTestError::from)?;

        // Check if there is a result set
        if let Some(schema) = result.schema() {
            let mut records = Vec::new();
            let mut types = Vec::new();

            // Create column type for each field
            for _field in schema.fields() {
                types.push(DefaultColumnType::Any);
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

    async fn shutdown(&mut self) {}
}

/// Convert ScalarValue to string
fn convert_scalar_value_to_string(value: &minigu::common::value::ScalarValue) -> String {
    use minigu::common::value::ScalarValue;

    match value {
        ScalarValue::Null => "NULL".to_string(),
        ScalarValue::Boolean(Some(b)) => b.to_string(),
        ScalarValue::Boolean(None) => "NULL".to_string(),
        ScalarValue::Int8(Some(i)) => i.to_string(),
        ScalarValue::Int8(None) => "NULL".to_string(),
        ScalarValue::Int16(Some(i)) => i.to_string(),
        ScalarValue::Int16(None) => "NULL".to_string(),
        ScalarValue::Int32(Some(i)) => i.to_string(),
        ScalarValue::Int32(None) => "NULL".to_string(),
        ScalarValue::Int64(Some(i)) => i.to_string(),
        ScalarValue::Int64(None) => "NULL".to_string(),
        ScalarValue::UInt8(Some(i)) => i.to_string(),
        ScalarValue::UInt8(None) => "NULL".to_string(),
        ScalarValue::UInt16(Some(i)) => i.to_string(),
        ScalarValue::UInt16(None) => "NULL".to_string(),
        ScalarValue::UInt32(Some(i)) => i.to_string(),
        ScalarValue::UInt32(None) => "NULL".to_string(),
        ScalarValue::UInt64(Some(i)) => i.to_string(),
        ScalarValue::UInt64(None) => "NULL".to_string(),
        ScalarValue::Float32(Some(f)) => f.to_string(),
        ScalarValue::Float32(None) => "NULL".to_string(),
        ScalarValue::Float64(Some(f)) => f.to_string(),
        ScalarValue::Float64(None) => "NULL".to_string(),
        ScalarValue::String(Some(s)) => s.clone(),
        ScalarValue::String(None) => "NULL".to_string(),
        ScalarValue::Vertex(Some(v)) => format!("{:?}", v),
        ScalarValue::Vertex(None) => "NULL".to_string(),
        ScalarValue::Edge(Some(e)) => format!("{:?}", e),
        ScalarValue::Edge(None) => "NULL".to_string(),
    }
}
