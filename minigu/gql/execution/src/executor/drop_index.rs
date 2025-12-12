use std::io;
use std::sync::Arc;

use minigu_catalog::provider::GraphProvider;
use minigu_common::data_chunk::DataChunk;
use minigu_context::error::IndexCatalogError;
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::session::SessionContext;
use minigu_planner::plan::drop_index::DropIndex;
use minigu_storage::tp::MemoryGraph;

use super::{BoxedExecutor, Executor};
use crate::error::{ExecutionError, ExecutionResult};

#[derive(Debug)]
pub struct DropIndexBuilder {
    session_context: SessionContext,
    plan: Arc<DropIndex>,
}

impl DropIndexBuilder {
    pub fn new(session_context: SessionContext, plan: Arc<DropIndex>) -> Self {
        Self {
            session_context,
            plan,
        }
    }

    pub fn into_executor(self) -> BoxedExecutor {
        Box::new(DropIndexExecutor {
            session_context: self.session_context,
            plan: self.plan,
            finished: false,
        })
    }
}

#[derive(Debug)]
pub struct DropIndexExecutor {
    session_context: SessionContext,
    plan: Arc<DropIndex>,
    finished: bool,
}

impl Executor for DropIndexExecutor {
    fn next_chunk(&mut self) -> Option<ExecutionResult<DataChunk>> {
        if self.finished {
            return None;
        }
        self.finished = true;
        if self.plan.no_op {
            return None;
        }
        match self.execute() {
            Ok(()) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

impl DropIndexExecutor {
    fn execute(&self) -> ExecutionResult<()> {
        let graph_ref = self.session_context.current_graph.clone().ok_or_else(|| {
            ExecutionError::Custom(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "current graph is not selected",
            )))
        })?;
        let provider = graph_ref.object().clone();
        let container = GraphProvider::as_any(provider.as_ref())
            .downcast_ref::<GraphContainer>()
            .ok_or_else(|| {
                ExecutionError::Custom(Box::new(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "only in-memory graphs support drop index",
                )))
            })?;
        let graph = match container.graph_storage() {
            GraphStorage::Memory(graph) => Arc::clone(graph),
        };

        self.drop_index(graph.as_ref(), container)
    }

    fn drop_index(&self, graph: &MemoryGraph, container: &GraphContainer) -> ExecutionResult<()> {
        let key = self.plan.index_key.ok_or_else(|| {
            ExecutionError::Custom(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "index key is missing for drop index",
            )))
        })?;
        let metadata = self.plan.metadata.clone();

        let removed = container
            .drop_vector_index(graph, key, metadata.clone())
            .map_err(|err| match err {
                IndexCatalogError::Catalog(e) => ExecutionError::from(e),
                IndexCatalogError::Storage(e) => ExecutionError::from(e),
            })?;
        if !removed && !self.plan.if_exists {
            return Err(ExecutionError::Custom(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                format!("vector index {} not found", self.plan.name),
            ))));
        }

        Ok(())
    }
}
