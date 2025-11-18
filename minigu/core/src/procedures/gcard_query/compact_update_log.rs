//! `compact_gcard_log` procedure — flush the pending GCard update log for the current graph.
//!
//! No parameters.  Operates on `context.current_graph`.
//!
//! Returns `(pending_entries: Int64)` — the number of log entries that were compacted.

use std::collections::HashMap;
use std::io;
use std::sync::Arc;

use arrow::array::Int64Array;
use minigu_catalog::provider::GraphTypeProvider;
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_common::types::LabelId;
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::procedure::Procedure;
use minigu_execution::error::ExecutionError;
use minigu_transaction::IsolationLevel::Serializable;
use minigu_transaction::{GraphTxnManager, Transaction};

use crate::procedures::gcard_query::update_log::GCardUpdateLog;
use crate::procedures::gcard_query::Statistic;

// ── GCard compact + apply ─────────────────────────────────────────────────────

/// Expand all pending update-log entries via graph traversal, apply the net degree
/// deltas to the stored [`Statistic`], remove deleted vertices, and clear the log.
///
/// This is the full "compact" described in 更新算法.md:
/// 1. Expand `res_len > 0` entries by walking actual graph neighbours.
/// 2. Accumulate net per-vertex deltas (skipping deleted vertices).
/// 3. Apply deltas to [`Statistic`] and remove deleted vertices.
/// 4. Regenerate [`DegreeSeqGraphCompressed`] from the updated statistics.
/// 5. Clear the log.
///
/// Returns an error if the container has no update log / statistic (run `GCard_build`
/// first) or if a storage operation fails.
pub fn compact_gcard_update_log(container: &GraphContainer) -> anyhow::Result<()> {
    use crate::procedures::gcard_query::update_log::GCardUpdateLog;

    // ── Resolve graph storage ──────────────────────────────────────────────
    let graph = match container.graph_storage() {
        GraphStorage::Memory(g) => Arc::clone(g),
    };

    // ── Build label name ↔ id maps from the graph type catalog ────────────
    let graph_type = container.graph_type();
    let mut label_name_to_id: HashMap<String, LabelId> = HashMap::new();
    for name in graph_type.label_names() {
        if let Ok(Some(id)) = graph_type.get_label_id(&name) {
            label_name_to_id.insert(name, id);
        }
    }
    let label_id_to_name: HashMap<LabelId, String> = label_name_to_id
        .iter()
        .map(|(k, &v)| (v, k.clone()))
        .collect();

    // ── Build edge schema: edge_name → (src_label_id, dst_label_id, edge_label_id) ──
    let edges = crate::procedures::gcard_query::create_catalog::get_edges_from_catalog(
        graph_type.as_ref(),
    )?;
    let edge_schema: HashMap<String, (LabelId, LabelId, LabelId)> = edges
        .iter()
        .filter_map(|(edge_name, info)| {
            let edge_id = *label_name_to_id.get(edge_name)?;
            let src_id = *label_name_to_id.get(&info.src_label)?;
            let dst_id = *label_name_to_id.get(&info.dst_label)?;
            Some((edge_name.clone(), (src_id, dst_id, edge_id)))
        })
        .collect();

    // ── Get update log ────────────────────────────────────────────────────
    let log_arc = container
        .gcard_update_log()
        .ok_or_else(|| anyhow::anyhow!("gcard_update_log not set (run GCard_build first)"))?;
    let log_arc = log_arc
        .downcast::<std::sync::Mutex<GCardUpdateLog>>()
        .map_err(|_| anyhow::anyhow!("gcard_update_log type mismatch"))?;

    // ── Get statistic (clone for mutation) ────────────────────────────────
    let stat_arc = container
        .statistic()
        .ok_or_else(|| anyhow::anyhow!("statistic not set (run GCard_build first)"))?;
    let mut statistic = stat_arc
        .downcast_ref::<Statistic>()
        .ok_or_else(|| anyhow::anyhow!("statistic type mismatch"))?
        .clone();

    // ── Open a read transaction and run compact_and_apply ─────────────────
    let txn = graph
        .txn_manager()
        .begin_transaction(Serializable)
        .map_err(|e| anyhow::anyhow!("begin_transaction: {e}"))?;

    {
        let mut guard = log_arc
            .lock()
            .map_err(|_| anyhow::anyhow!("gcard_update_log mutex poisoned"))?;
        guard.compact_and_apply(
            &graph,
            &txn,
            &edge_schema,
            &label_id_to_name,
            &mut statistic,
        )?;
    }

    txn.commit()
        .map_err(|e| anyhow::anyhow!("commit transaction: {e}"))?;

    // ── Write updated statistic + rebuild DegreeSeqGraphCompressed ────────
    let new_dsgc = statistic
        .to_degree_seq_graph_compressed()
        .map_err(|e| anyhow::anyhow!("to_degree_seq_graph_compressed: {e}"))?;
    container.set_statistic(Arc::new(statistic));
    container.set_degree_seq_graph_compressed(Arc::new(new_dsgc));

    Ok(())
}

pub fn build_procedure() -> Procedure {
    let schema = Arc::new(DataSchema::new(vec![DataField::new(
        "pending_entries".into(),
        LogicalType::Int64,
        false,
    )]));

    Procedure::new(vec![], Some(schema), move |context, _args| {
        let graph_ref = context.current_graph.clone().ok_or_else(|| {
            ExecutionError::Custom(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "no current graph selected",
            )))
        })?;

        let container = graph_ref
            .object()
            .downcast_ref::<GraphContainer>()
            .ok_or_else(|| {
                ExecutionError::Custom(Box::new(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "current graph is not a GraphContainer",
                )))
            })?;

        // Count pending entries before compaction.
        let pending = container
            .gcard_update_log()
            .and_then(|any| any.downcast::<std::sync::Mutex<GCardUpdateLog>>().ok())
            .map(|log| {
                log.lock()
                    .expect("GCardUpdateLog mutex poisoned")
                    .entries
                    .len()
            })
            .unwrap_or(0);

        compact_gcard_update_log(container)?;

        let chunk = DataChunk::new(vec![Arc::new(Int64Array::from_iter_values([
            pending as i64
        ]))]);
        Ok(vec![chunk])
    })
}
