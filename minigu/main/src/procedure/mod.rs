pub mod registry;

use std::fmt::Debug;
use std::sync::Arc;

use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::{DataSchemaRef, LogicalType};
use minigu_execution::evaluator::datum::DatumRef;

use crate::error::Result;
use crate::session::SessionContext;

pub trait Procedure: Debug + Send + Sync {
    fn description(&self) -> &str;

    fn parameters(&self) -> &[LogicalType];

    fn schema(&self) -> Option<DataSchemaRef>;

    fn execute(&self, context: SessionContext, args: Vec<DatumRef>) -> Result<Vec<DataChunk>>;
}
