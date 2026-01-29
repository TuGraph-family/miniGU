use std::convert::TryFrom;
use std::io;
use std::sync::Arc;

use arrow::array::{Array, ArrayRef, AsArray, BooleanArray, Float32Array, UInt64Array};
use arrow::datatypes::UInt64Type;
use minigu_common::data_chunk::DataChunk;
use minigu_common::value::{ScalarValue, VectorValue};
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::session::SessionContext;
use minigu_planner::plan::vector_index_scan::VectorIndexScan;
use minigu_storage::tp::MemoryGraph;
use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

use super::{BoxedExecutor, Executor};
use crate::error::{ExecutionError, ExecutionResult};

/// Default L parameter for DiskANN search
const DEFAULT_L_VALUE: u32 = 100;

pub struct VectorIndexScanBuilder {
    session_context: SessionContext,
    plan: Arc<VectorIndexScan>,
    child: BoxedExecutor,
    binding_column_index: usize,
}

impl VectorIndexScanBuilder {
    pub fn new(
        session_context: SessionContext,
        plan: Arc<VectorIndexScan>,
        child: BoxedExecutor,
        binding_column_index: usize,
    ) -> Self {
        Self {
            session_context,
            plan,
            child,
            binding_column_index,
        }
    }

    pub fn into_executor(self) -> BoxedExecutor {
        Box::new(VectorIndexScanExecutor {
            session_context: self.session_context,
            plan: self.plan,
            child: Some(self.child),
            binding_column_index: self.binding_column_index,
            finished: false,
        })
    }
}

pub struct VectorIndexScanExecutor {
    session_context: SessionContext,
    plan: Arc<VectorIndexScan>,
    child: Option<BoxedExecutor>,
    binding_column_index: usize,
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
    fn execute_scan(&mut self) -> ExecutionResult<DataChunk> {
        let candidate_bitmap = self.consume_child_bitmap()?;
        if self.plan.limit == 0 {
            return Ok(Self::empty_result_chunk());
        }
        let candidate_bitmap = match candidate_bitmap {
            CandidateBitmap::Empty => return Ok(Self::empty_result_chunk()),
            CandidateBitmap::Filter(bitmap) => Some(bitmap),
        };
        let graph = self.resolve_memory_graph()?;
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Snapshot)
            .map_err(ExecutionError::from)?;

        let result = self.scan_with_graph(graph.as_ref(), candidate_bitmap);
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
        let graph_ref = self.session_context.current_graph.clone().ok_or_else(|| {
            ExecutionError::Custom(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "current graph is not selected",
            )))
        })?;
        let provider = graph_ref.object().clone();
        let container = provider.downcast_ref::<GraphContainer>().ok_or_else(|| {
            ExecutionError::Custom(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "only in-memory graphs support vector scans",
            )))
        })?;
        match container.graph_storage() {
            GraphStorage::Memory(graph) => Ok(Arc::clone(graph)),
        }
    }

    fn scan_with_graph(
        &self,
        graph: &MemoryGraph,
        candidate_bitmap: Option<BooleanArray>,
    ) -> ExecutionResult<DataChunk> {
        // TODO(minigu-vector-search): support parameter/column vector expressions once binder
        // permits.
        let query_scalar = self.plan.query.clone().evaluate_scalar().ok_or_else(|| {
            ExecutionError::Custom(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "query vector must be a constant expression",
            )))
        })?;
        let vector_value = extract_vector(query_scalar)?;
        if vector_value.dimension() != self.plan.dimension {
            return Err(ExecutionError::Custom(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "query vector dimension {} does not match bound dimension {}",
                    vector_value.dimension(),
                    self.plan.dimension
                ),
            ))));
        }

        let l_value = DEFAULT_L_VALUE.max(self.plan.limit as u32);
        let filter_bitmap = candidate_bitmap.as_ref();
        let results = graph
            .vector_search(
                self.plan.index_key,
                &vector_value,
                self.plan.limit,
                l_value,
                filter_bitmap,
                self.plan.approximate,
            )
            .map_err(ExecutionError::from)?;

        // [node_id, distance]
        let (vertex_ids, distances): (Vec<u64>, Vec<f32>) = results.into_iter().unzip();
        let mut columns: Vec<ArrayRef> = Vec::new();
        let id_array: ArrayRef =
            Arc::new(UInt64Array::from_iter_values(vertex_ids.iter().copied()));
        let distance_array: ArrayRef =
            Arc::new(Float32Array::from_iter_values(distances.iter().copied()));
        columns.push(id_array);
        columns.push(distance_array);

        Ok(DataChunk::new(columns))
    }
}

fn extract_vector(value: ScalarValue) -> ExecutionResult<VectorValue> {
    value.get_vector().map_err(|_| {
        ExecutionError::Custom(Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "failed to extract vector from scalar value",
        )))
    })
}

impl VectorIndexScanExecutor {
    fn empty_result_chunk() -> DataChunk {
        let id_array: ArrayRef = Arc::new(UInt64Array::from_iter_values(std::iter::empty::<u64>()));
        let distance_array: ArrayRef =
            Arc::new(Float32Array::from_iter_values(std::iter::empty::<f32>()));
        DataChunk::new(vec![id_array, distance_array])
    }

    fn consume_child_bitmap(&mut self) -> ExecutionResult<CandidateBitmap> {
        let child = match self.child.take() {
            Some(child) => child,
            None => {
                return Err(ExecutionError::Custom(Box::new(io::Error::other(
                    "vector index scan child executor has already been consumed",
                ))));
            }
        };

        let mut candidate_indices: Vec<usize> = Vec::new();
        let mut max_index: Option<usize> = None;

        for chunk in child.into_iter() {
            let chunk = chunk?;
            if chunk.is_empty() {
                continue;
            }
            let column = chunk
                .columns()
                .get(self.binding_column_index)
                .ok_or_else(|| {
                    ExecutionError::Custom(Box::new(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "binding column missing from child output",
                    )))
                })?;
            let column = column.as_primitive::<UInt64Type>();
            for row in 0..column.len() {
                if column.is_null(row) {
                    continue;
                }
                let node_id = column.value(row);
                let idx = usize::try_from(node_id).map_err(|_| {
                    ExecutionError::Custom(Box::new(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("vertex id {node_id} exceeds usize range"),
                    )))
                })?;
                if let Some(current_max) = max_index.as_mut() {
                    if idx > *current_max {
                        *current_max = idx;
                    }
                } else {
                    max_index = Some(idx);
                }
                candidate_indices.push(idx);
            }
        }

        let Some(max_index) = max_index else {
            return Ok(CandidateBitmap::Empty);
        };
        let bitmap_len = max_index.checked_add(1).ok_or_else(|| {
            ExecutionError::Custom(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "vertex id overflow while building candidate bitmap",
            )))
        })?;
        let mut filter_bits = vec![false; bitmap_len];
        for idx in candidate_indices {
            filter_bits[idx] = true;
        }

        Ok(CandidateBitmap::Filter(BooleanArray::from(filter_bits)))
    }
}

enum CandidateBitmap {
    Empty,
    Filter(BooleanArray),
}
