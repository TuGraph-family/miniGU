use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

use arrow::array::{ArrayRef, Float32Array, UInt64Array};
use minigu_common::data_chunk::DataChunk;
use minigu_common::value::{ScalarValue, VectorValue};
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::session::SessionContext;
use minigu_planner::plan::vector_index_scan::VectorIndexScan;
use minigu_storage::tp::MemoryGraph;
use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

use super::{BoxedExecutor, Executor};
use crate::error::{ExecutionError, ExecutionResult};

const DEFAULT_L_VALUE: u32 = 100;

/// Builds an executor that performs ANN search directly against the storage vector index.
///
/// TODO(minigu-vector-search): allow injecting additional execution parameters (bitmap filter,
/// search configuration) once planner/binder provide them.
#[derive(Debug)]
pub struct VectorIndexScanBuilder {
    session_context: SessionContext,
    plan: Arc<VectorIndexScan>,
}

impl VectorIndexScanBuilder {
    pub fn new(session_context: SessionContext, plan: Arc<VectorIndexScan>) -> Self {
        Self {
            session_context,
            plan,
        }
    }

    pub fn into_executor(self) -> BoxedExecutor {
        Box::new(VectorIndexScanExecutor {
            session_context: self.session_context,
            plan: self.plan,
            finished: false,
        })
    }
}

#[derive(Debug)]
pub struct VectorIndexScanExecutor {
    session_context: SessionContext,
    plan: Arc<VectorIndexScan>,
    finished: bool,
}

impl Executor for VectorIndexScanExecutor {
    fn next_chunk(&mut self) -> Option<ExecutionResult<DataChunk>> {
        if self.finished {
            return None;
        }
        self.finished = true;
        Some(self.execute_scan())
    }
}

impl VectorIndexScanExecutor {
    /// Executes a single ANN search against the current graph.
    ///
    /// The method is intentionally one-shot; callers should guard with `finished` to avoid
    /// re-running the search.
    fn execute_scan(&self) -> ExecutionResult<DataChunk> {
        let graph = self.resolve_memory_graph()?;
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Snapshot)
            .map_err(ExecutionError::from)?;

        let result = self.scan_with_graph(graph.as_ref());
        match result {
            Ok(chunk) => {
                txn.commit().map_err(ExecutionError::from)?;
                Ok(chunk)
            }
            Err(err) => {
                // Best-effort abort; ignore errors to avoid masking original failure.
                // TODO(minigu-vector-search): attach abort failures as diagnostics for debugging.
                let _ = txn.abort();
                Err(err)
            }
        }
    }

    fn resolve_memory_graph(&self) -> Result<Arc<MemoryGraph>, ExecutionError> {
        let graph_ref = self
            .session_context
            .current_graph
            .clone()
            .ok_or_else(|| exec_error("current graph is not selected"))?;
        let provider = graph_ref.object().clone();
        let container = provider
            .as_any()
            .downcast_ref::<GraphContainer>()
            .ok_or_else(|| exec_error("only in-memory graphs support vector scans"))?;
        match container.graph_storage() {
            GraphStorage::Memory(graph) => Ok(Arc::clone(graph)),
        }
    }

    fn scan_with_graph(&self, graph: &MemoryGraph) -> ExecutionResult<DataChunk> {
        // TODO(minigu-vector-search): support parameter/column vector expressions once binder
        // permits.
        if self.plan.limit == 0 {
            let id_array: ArrayRef =
                Arc::new(UInt64Array::from_iter_values(std::iter::empty::<u64>()));
            let distance_array: ArrayRef =
                Arc::new(Float32Array::from_iter_values(std::iter::empty::<f32>()));
            return Ok(DataChunk::new(vec![id_array, distance_array]));
        }

        let query_scalar = self
            .plan
            .query
            .clone()
            .evaluate_scalar()
            .ok_or_else(|| exec_error("query vector must be a constant expression"))?;
        let vector_value = extract_vector(query_scalar)?;
        if vector_value.dimension() != self.plan.dimension {
            return Err(exec_error(format!(
                "query vector dimension {} does not match bound dimension {}",
                vector_value.dimension(),
                self.plan.dimension
            )));
        }

        let l_value = DEFAULT_L_VALUE.max(self.plan.limit as u32);
        // TODO(minigu-vector-search): thread bitmap filters from MATCH into this call once
        // binder/executor can surface the candidate set.
        let results = graph
            .vector_search(
                self.plan.index_key,
                &vector_value,
                self.plan.limit,
                l_value,
                None,
                false,
            )
            .map_err(ExecutionError::from)?;

        let (vertex_ids, distances): (Vec<u64>, Vec<f32>) = results.into_iter().unzip();
        let id_array: ArrayRef = Arc::new(UInt64Array::from_iter_values(vertex_ids));
        let distance_array: ArrayRef = Arc::new(Float32Array::from_iter_values(distances));
        Ok(DataChunk::new(vec![id_array, distance_array]))
    }
}

#[derive(Debug)]
struct VectorIndexScanExecError(String);

impl Display for VectorIndexScanExecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for VectorIndexScanExecError {}

fn exec_error(message: impl Into<String>) -> ExecutionError {
    ExecutionError::Custom(Box::new(VectorIndexScanExecError(message.into())))
}

fn extract_vector(value: ScalarValue) -> ExecutionResult<VectorValue> {
    value.get_vector().map_err(exec_error)
}
