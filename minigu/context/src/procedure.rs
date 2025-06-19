use std::any::Any;
use std::error::Error;
use std::fmt::Debug;

use minigu_catalog::provider::ProcedureProvider;
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::{DataSchemaRef, LogicalType};
use minigu_common::value::ScalarValue;

use crate::session::SessionContext;

pub type ProcedureImpl = Box<
    dyn Fn(
            SessionContext,
            Vec<ScalarValue>,
        ) -> Result<Vec<DataChunk>, Box<dyn Error + Send + Sync + 'static>>
        + Send
        + Sync,
>;

pub struct Procedure {
    parameters: Vec<LogicalType>,
    schema: Option<DataSchemaRef>,
    inner: ProcedureImpl,
}

impl Procedure {
    pub fn new<F>(parameters: Vec<LogicalType>, schema: Option<DataSchemaRef>, inner: F) -> Self
    where
        F: Fn(
                SessionContext,
                Vec<ScalarValue>,
            ) -> Result<Vec<DataChunk>, Box<dyn Error + Send + Sync + 'static>>
            + Send
            + Sync
            + 'static,
    {
        Self {
            parameters,
            schema,
            inner: Box::new(inner),
        }
    }

    pub fn call(
        &self,
        session_context: SessionContext,
        args: Vec<ScalarValue>,
    ) -> Result<Vec<DataChunk>, Box<dyn Error + Send + Sync + 'static>> {
        (self.inner)(session_context, args)
    }
}

impl Debug for Procedure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Procedure")
            .field("parameters", &self.parameters)
            .field("schema", &self.schema)
            .finish()
    }
}

impl ProcedureProvider for Procedure {
    #[inline]
    fn parameters(&self) -> &[LogicalType] {
        &self.parameters
    }

    #[inline]
    fn schema(&self) -> Option<DataSchemaRef> {
        self.schema.clone()
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }
}
