use std::error::Error;

use minigu_catalog::provider::ProcedureProvider;
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::{DataSchemaRef, LogicalType};
use minigu_common::value::ScalarValue;

use crate::session::SessionContext;

pub type ProcedureImpl = fn(
    &SessionContext,
    Vec<ScalarValue>,
) -> Result<Vec<DataChunk>, Box<dyn Error + Send + Sync + 'static>>;

#[derive(Debug)]
pub struct ProcedureContainer {
    parameters: Vec<LogicalType>,
    schema: Option<DataSchemaRef>,
    procedure: ProcedureImpl,
}

impl ProcedureContainer {
    pub fn new(
        parameters: Vec<LogicalType>,
        schema: Option<DataSchemaRef>,
        procedure: ProcedureImpl,
    ) -> Self {
        Self {
            parameters,
            schema,
            procedure,
        }
    }

    pub fn procedure(&self) -> ProcedureImpl {
        self.procedure
    }
}

impl ProcedureProvider for ProcedureContainer {
    fn parameters(&self) -> &[LogicalType] {
        &self.parameters
    }

    fn schema(&self) -> Option<DataSchemaRef> {
        self.schema.clone()
    }
}
