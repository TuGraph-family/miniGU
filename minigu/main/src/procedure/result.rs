use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::DataSchemaRef;

#[derive(Debug, Clone)]
pub struct ProcedureResult {
    schema: Option<DataSchemaRef>,
    chunks: Vec<DataChunk>,
}

impl ProcedureResult {
    pub fn new(schema: Option<DataSchemaRef>, chunks: Vec<DataChunk>) -> Self {
        Self { schema, chunks }
    }
}
