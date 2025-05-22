use std::sync::Arc;
use std::time::Duration;

use arrow::array::create_array;
use gql_parser::parse_gql;
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::{DataField, DataSchema, LogicalType};

use crate::database::DatabaseContext;
use crate::error::Result;
use crate::metrics::QueryMetrics;
use crate::result::QueryResult;

#[derive(Debug)]
pub struct Session {}

impl Session {
    pub fn query(&self, query: &str) -> Result<QueryResult> {
        // TODO: Remove the placeholder code.
        let programs = parse_gql(query)?;
        let col1 = create_array!(Int32, [Some(1), Some(2), None]);
        let col2 = create_array!(Utf8, ["a", "b", "c"]);
        let chunk = DataChunk::new(vec![col1, col2]);
        let schema = DataSchema::new(vec![
            DataField::new("a".to_string(), LogicalType::Int32, true),
            DataField::new("b".to_string(), LogicalType::String, false),
        ]);
        let metrics = QueryMetrics::default();
        Ok(QueryResult::new(Some(Arc::new(schema)), metrics, vec![
            chunk,
        ]))
    }
}

pub struct SessionContext {
    database: Arc<DatabaseContext>,
}
