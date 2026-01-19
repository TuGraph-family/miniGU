use std::io;
use std::sync::Arc;

use minigu_catalog::provider::{GraphProvider, VectorIndexCatalogEntry};
use minigu_common::data_chunk::DataChunk;
use minigu_context::error::{Error, IndexCatalogError};
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::session::SessionContext;
use minigu_planner::plan::create_vector_index::CreateVectorIndex;
use minigu_storage::tp::{MemTransaction, MemoryGraph};
use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

use super::{BoxedExecutor, Executor};
use crate::error::{ExecutionError, ExecutionResult};

#[derive(Debug)]
pub struct CreateVectorIndexBuilder {
    session_context: SessionContext,
    plan: Arc<CreateVectorIndex>,
}

impl CreateVectorIndexBuilder {
    pub fn new(session_context: SessionContext, plan: Arc<CreateVectorIndex>) -> Self {
        Self {
            session_context,
            plan,
        }
    }

    pub fn into_executor(self) -> BoxedExecutor {
        Box::new(CreateVectorIndexExecutor {
            session_context: self.session_context,
            plan: self.plan,
            finished: false,
        })
    }
}

#[derive(Debug)]
pub struct CreateVectorIndexExecutor {
    session_context: SessionContext,
    plan: Arc<CreateVectorIndex>,
    finished: bool,
}

impl Executor for CreateVectorIndexExecutor {
    fn next_chunk(&mut self) -> Option<ExecutionResult<DataChunk>> {
        if self.finished {
            return None;
        }
        self.finished = true;
        match self.execute() {
            Ok(()) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

impl CreateVectorIndexExecutor {
    fn execute(&self) -> ExecutionResult<()> {
        // IF NOT EXISTS and already present => no-op.
        if self.plan.no_op {
            return Ok(());
        }

        let graph_ref = self
            .session_context
            .current_graph
            .clone()
            .ok_or(Error::CurrentGraphNotSet)?;
        let provider = graph_ref.object().clone();
        let container = GraphProvider::as_any(provider.as_ref())
            .downcast_ref::<GraphContainer>()
            .ok_or_else(|| {
                ExecutionError::Custom(Box::new(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "only in-memory graphs support create vector index",
                )))
            })?;
        let graph = match container.graph_storage() {
            GraphStorage::Memory(graph) => Arc::clone(graph),
        };
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .map_err(ExecutionError::from)?;

        let result = self.build_index(graph.as_ref(), &txn, container);
        match result {
            Ok(()) => {
                txn.commit().map_err(ExecutionError::from)?;
                Ok(())
            }
            Err(err) => {
                let _ = txn.abort();
                Err(err)
            }
        }
    }

    fn build_index(
        &self,
        graph: &MemoryGraph,
        txn: &Arc<MemTransaction>,
        container: &GraphContainer,
    ) -> ExecutionResult<()> {
        let meta = VectorIndexCatalogEntry {
            name: self.plan.name.clone(),
            key: self.plan.index_key,
            metric: self.plan.metric,
            dimension: self.plan.dimension,
        };
        let created = container
            .create_vector_index(graph, txn, meta.clone())
            .map_err(ExecutionError::from)?;
        if !created && self.plan.if_not_exists {
            // Another session may have created the same index between binding and execution.
            return Ok(());
        } else if !created {
            return Err(IndexCatalogError::VectorIndexAlreadyExists(
                self.plan.label.to_string(),
                self.plan.property.to_string(),
            )
            .into());
        }

        Ok(())
    }
}
