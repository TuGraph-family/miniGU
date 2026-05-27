use std::sync::Arc;

use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::session::SessionContext;
use minigu_planner::plan::data_modify::Insert;
use minigu_storage::common::{Edge, PropertyRecord, Vertex};
use minigu_storage::tp::MemoryGraph;
use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

use super::{Executor, IntoExecutor};
use crate::error::{ExecutionError, ExecutionResult};

fn execution_error(msg: impl Into<String>) -> ExecutionError {
    ExecutionError::Custom(msg.into().into())
}

pub struct InsertBuilder {
    plan: Insert,
    session: SessionContext,
}

impl InsertBuilder {
    pub fn new(plan: Insert, session: SessionContext) -> Self {
        Self { plan, session }
    }
}

impl IntoExecutor for InsertBuilder {
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let InsertBuilder { plan, session } = self;
            if let Err(e) = insert_impl(&plan, &session) {
                yield Err(e);
            }
        }
        .into_executor()
    }
}

fn insert_impl(plan: &Insert, session: &SessionContext) -> ExecutionResult<()> {
    let graph_ref = session
        .current_graph
        .as_ref()
        .ok_or_else(|| execution_error("No current graph set for INSERT"))?
        .clone();
    let provider = graph_ref.object().clone();
    let container = provider
        .downcast_ref::<GraphContainer>()
        .ok_or_else(|| execution_error("Current graph is not an in-memory container"))?;
    let memory: Arc<MemoryGraph> = match container.graph_storage() {
        GraphStorage::Memory(m) => Arc::clone(m),
    };

    let txn = memory
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .map_err(|e| execution_error(format!("failed to begin transaction: {e}")))?;

    // Create vertices first, recording the assigned vid so edges can reference
    // their endpoints by index.
    let mut vid_map: Vec<u64> = Vec::with_capacity(plan.statement.vertices.len());
    for v in &plan.statement.vertices {
        let vid = memory.alloc_vertex_id();
        let props = PropertyRecord::new(v.properties.iter().map(|p| p.value.clone()).collect());
        memory
            .create_vertex(&txn, Vertex::new(vid, v.label_id, props))
            .map_err(|e| execution_error(format!("create_vertex failed: {e}")))?;
        vid_map.push(vid);
    }

    for e in &plan.statement.edges {
        let eid = memory.alloc_edge_id();
        let props = PropertyRecord::new(e.properties.iter().map(|p| p.value.clone()).collect());
        let src = vid_map[e.src_index];
        let dst = vid_map[e.dst_index];
        memory
            .create_edge(&txn, Edge::new(eid, src, dst, e.label_id, props))
            .map_err(|e| execution_error(format!("create_edge failed: {e}")))?;
    }

    txn.commit()
        .map_err(|e| execution_error(format!("commit failed: {e}")))?;
    Ok(())
}
