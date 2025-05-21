use std::fmt::{self, Debug};

use arrow::array::RecordBatch;
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::{DataSchema, DataSchemaRef};

use crate::error::Result;
use crate::metrics::QueryMetrics;

#[derive(Debug)]
pub struct QueryResult {
    schema: Option<DataSchemaRef>,
    metrics: QueryMetrics,
    chunks: Vec<DataChunk>,
}

impl QueryResult {
    pub(crate) fn new(
        schema: Option<DataSchemaRef>,
        metrics: QueryMetrics,
        chunks: Vec<DataChunk>,
    ) -> Self {
        Self {
            schema,
            metrics,
            chunks,
        }
    }

    #[inline]
    pub fn schema(&self) -> Option<&DataSchemaRef> {
        self.schema.as_ref()
    }

    #[inline]
    pub fn metrics(&self) -> &QueryMetrics {
        &self.metrics
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &DataChunk> {
        self.chunks.iter()
    }
}

impl IntoIterator for QueryResult {
    type Item = DataChunk;

    type IntoIter = impl Iterator<Item = DataChunk>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.chunks.into_iter()
    }
}
