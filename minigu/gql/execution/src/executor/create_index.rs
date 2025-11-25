use std::io;
use std::sync::Arc;

use minigu_catalog::provider::{GraphIndexCatalogRef, VectorIndexMetadata};
use minigu_common::data_chunk::DataChunk;
use minigu_context::error::IndexCatalogError;
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::session::SessionContext;
use minigu_planner::plan::create_index::CreateIndex;
use minigu_storage::tp::{MemTransaction, MemoryGraph};
use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

use super::{BoxedExecutor, Executor};
use crate::error::{ExecutionError, ExecutionResult};

#[derive(Debug)]
pub struct CreateIndexBuilder {
    session_context: SessionContext,
    plan: Arc<CreateIndex>,
}

impl CreateIndexBuilder {
    pub fn new(session_context: SessionContext, plan: Arc<CreateIndex>) -> Self {
        Self {
            session_context,
            plan,
        }
    }

    pub fn into_executor(self) -> BoxedExecutor {
        Box::new(CreateIndexExecutor {
            session_context: self.session_context,
            plan: self.plan,
            finished: false,
        })
    }
}

#[derive(Debug)]
pub struct CreateIndexExecutor {
    session_context: SessionContext,
    plan: Arc<CreateIndex>,
    finished: bool,
}

impl Executor for CreateIndexExecutor {
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

impl CreateIndexExecutor {
    fn execute(&self) -> ExecutionResult<()> {
        // IF NOT EXISTS and already present => no-op.
        if self.plan.no_op {
            return Ok(());
        }

        let graph_ref = self.session_context.current_graph.clone().ok_or_else(|| {
            ExecutionError::Custom(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "current graph is not selected",
            )))
        })?;
        let provider = graph_ref.object().clone();
        let container = provider
            .as_any()
            .downcast_ref::<GraphContainer>()
            .ok_or_else(|| {
                ExecutionError::Custom(Box::new(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "only in-memory graphs support create index",
                )))
            })?;
        let graph = match container.graph_storage() {
            GraphStorage::Memory(graph) => Arc::clone(graph),
        };
        let index_catalog = provider.index_catalog().ok_or_else(|| {
            ExecutionError::Custom(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "index catalog is not available for current graph",
            )))
        })?;
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .map_err(ExecutionError::from)?;

        let result = self.build_index(graph.as_ref(), &txn, container, index_catalog);
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
        index_catalog: GraphIndexCatalogRef,
    ) -> ExecutionResult<()> {
        if index_catalog
            .get_vector_index(self.plan.index_key)?
            .is_some()
        {
            if self.plan.if_not_exists {
                return Ok(());
            }
            return Err(ExecutionError::Custom(Box::new(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "vector index on label {} property {} already exists",
                    self.plan.label, self.plan.property
                ),
            ))));
        }

        let meta = VectorIndexMetadata {
            name: self.plan.name.clone(),
            key: self.plan.index_key,
            metric: self.plan.metric,
            dimension: self.plan.dimension,
        };
        let created = container
            .create_vector_index(graph, txn, meta.clone())
            .map_err(|err| match err {
                IndexCatalogError::Catalog(e) => ExecutionError::from(e),
                IndexCatalogError::Storage(e) => ExecutionError::from(e),
            })?;
        if !created && self.plan.if_not_exists {
            return Ok(());
        } else if !created {
            return Err(ExecutionError::Custom(Box::new(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "vector index on label {} property {} already exists",
                    self.plan.label, self.plan.property
                ),
            ))));
        }

        Ok(())
    }
}
