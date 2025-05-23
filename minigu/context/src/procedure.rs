use std::fmt::Debug;

use minigu_common::data_type::{DataSchemaRef, LogicalType};
// pub trait Procedure: Debug + Send + Sync {
//     fn description(&self) -> &str;

//     fn parameters(&self) -> &[LogicalType];

//     fn schema(&self) -> Option<DataSchemaRef>;

//     fn execute(&self, context: SessionContext, args: Vec<DatumRef>) -> Result<Vec<DataChunk>>;
// }

// pub trait 