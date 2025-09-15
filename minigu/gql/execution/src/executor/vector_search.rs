use std::sync::Arc;

use arrow::array::{FixedSizeListArray, Float32Array};
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::DataSchema;
use minigu_common::types::{VectorIndexKey, VectorMetric};
use minigu_context::session::SessionContext;
use minigu_storage::tp::MemoryGraph;

use super::utils::gen_try;
use super::{Executor, IntoExecutor};
use crate::error::ExecutionError;
use crate::evaluator::BoxedEvaluator;

pub struct VectorSearchBuilder {
    session: SessionContext,
    index_key: VectorIndexKey,
    query_evaluator: BoxedEvaluator,
    k: usize,
    metric: VectorMetric,
    schema: Arc<DataSchema>,
}

impl VectorSearchBuilder {
    pub fn new(
        session: SessionContext,
        index_key: VectorIndexKey,
        query_evaluator: BoxedEvaluator,
        k: usize,
        metric: VectorMetric,
        schema: Arc<DataSchema>,
    ) -> Self {
        Self {
            session,
            index_key,
            query_evaluator,
            k,
            metric,
            schema,
        }
    }
}

impl IntoExecutor for VectorSearchBuilder {
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let VectorSearchBuilder {
                session,
                index_key,
                query_evaluator,
                k,
                metric,
                schema,
            } = self;

            let graph_ref = session
                .current_graph
                .as_ref()
                .or(session.home_graph.as_ref())
                .ok_or_else(|| {
                    ExecutionError::Custom("No current or home graph set".to_string().into())
                });
            let graph_ref = gen_try!(graph_ref);
            let memory_graph = graph_ref
                .as_any()
                .downcast_ref::<MemoryGraph>()
                .ok_or_else(|| {
                    ExecutionError::Custom("Graph is not a MemoryGraph".to_string().into())
                });
            let memory_graph = gen_try!(memory_graph);

            // Evaluate query vector expression using proper evaluator
            // This supports both constant vectors and parameterized/complex queries
            let empty_chunk = DataChunk::new(vec![]); // Empty context for parameter queries
            let query_datum = gen_try!(query_evaluator.evaluate(&empty_chunk));

            let query_vector = if query_datum.is_scalar() {
                // Extract vector from scalar result
                let array = query_datum.as_array();
                if let Some(list_array) = array.as_any().downcast_ref::<FixedSizeListArray>() {
                    let values = list_array.values();
                    if let Some(float_array) = values.as_any().downcast_ref::<Float32Array>() {
                        float_array.values().to_vec()
                    } else {
                        yield Err(ExecutionError::Custom(
                            "Expected Float32Array in vector, got different type".into(),
                        ));
                        return;
                    }
                } else {
                    yield Err(ExecutionError::Custom(
                        "Expected FixedSizeListArray for vector, got different type".into(),
                    ));
                    return;
                }
            } else {
                yield Err(ExecutionError::Custom(
                    "Query vector must be a scalar value".into(),
                ));
                return;
            };

            // return vid. TODO: return (vid, distance)
            let search_results =
                memory_graph.vector_search(index_key, &query_vector, k, 100, None, false); // TODO: Metric validation, the default is L2 now
            let vertex_ids = gen_try!(search_results.map_err(|e| ExecutionError::Custom(
                format!("Vector search failed: {}", e).into()
            )));
            if !vertex_ids.is_empty() {
                // For now, return vertex IDs only
                let columns = vec![Arc::new(arrow::array::UInt64Array::from(vertex_ids)) as _];
                let chunk = DataChunk::new(columns);
                yield Ok(chunk);
            }
        }
        .into_executor()
    }
}
