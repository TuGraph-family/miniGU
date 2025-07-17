use std::fmt;
use std::sync::Arc;

use minigu::database::{Database, DatabaseConfig};
use minigu::error::Error as MiniGuError;
use sqllogictest::{DB, DBOutput, DefaultColumnType};

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
    type ColumnType = DefaultColumnType;
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
