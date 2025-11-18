//! `mutate_graph` procedure — apply controlled mutations to an existing in-memory graph.
//!
//! Parameters
//! ----------
//! | # | Name          | Type    | Description |
//! |---|---------------|---------|-------------|
//! | 0 | graph_name    | String  | Graph registered in the current schema |
//! | 1 | operation     | String  | `"add_vertices"` \| `"delete_vertices"` \| `"add_edges"` |
//! | 2 | label_a       | String  | Vertex label (source label for `add_edges`) |
//! | 3 | label_b       | String  | Destination vertex label (for `add_edges`; ignored otherwise) |
//! | 4 | edge_label    | String  | Edge label (for `add_edges`; ignored otherwise) |
//! | 5 | delta_count   | Int64   | Absolute number of elements to add/delete/create (>0 takes priority over percent) |
//! | 6 | delta_percent | Float64 | Percentage of existing label_a vertices to use as count (when delta_count ≤ 0) |
//!
//! Return schema: `(operation: String, label: String, changed: Int64)`
//!
//! GCard update log
//! ----------------
//! If the graph has GCard statistics (i.e. `gcard_update_log` is set on the container),
//! every structural change is lazily recorded so that the statistics stay consistent:
//! * `add_vertices`   — no recording needed (isolated new vertices have no degree)
//! * `delete_vertices`— each deleted vertex's neighbours are recorded before removal
//! * `add_edges`      — each inserted edge is recorded for both endpoints

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use arrow::array::{Int64Array, StringArray};
use minigu_catalog::provider::{GraphTypeProvider, SchemaProvider};
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_common::types::{EdgeId, LabelId, VertexId};
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::procedure::Procedure;
use minigu_storage::common::{Edge, PropertyRecord, Vertex};
use minigu_storage::tp::MemoryGraph;
use minigu_transaction::IsolationLevel::Serializable;
use minigu_transaction::{GraphTxnManager, Transaction};

use crate::procedures::gcard_query::update_log::GCardUpdateLog;

pub fn build_procedure() -> Procedure {
    let parameters = vec![
        LogicalType::String,  // graph_name
        LogicalType::String,  // operation
        LogicalType::String,  // label_a
        LogicalType::String,  // label_b  (add_edges only)
        LogicalType::String,  // edge_label (add_edges only)
        LogicalType::Int64,   // delta_count (>0 = absolute; ≤0 = use percent)
        LogicalType::Float64, // delta_percent (0.0–100.0; used when delta_count ≤ 0)
    ];

    let schema = Arc::new(DataSchema::new(vec![
        DataField::new("operation".into(), LogicalType::String, false),
        DataField::new("label".into(), LogicalType::String, false),
        DataField::new("changed".into(), LogicalType::Int64, false),
    ]));

    Procedure::new(parameters, Some(schema), move |context, args| {
        // ── parse args ───────────────────────────────────────────────────────
        let graph_name = parse_str(&args[0], "graph_name")?;
        let operation  = parse_str(&args[1], "operation")?;
        let label_a    = parse_str(&args[2], "label_a")?;
        let label_b    = parse_str(&args[3], "label_b")?;
        let edge_label = parse_str(&args[4], "edge_label")?;
        let delta_count = args[5]
            .to_i64()
            .map_err(|e| anyhow::anyhow!("delta_count: {:?}", e))?;
        let delta_percent = args[6]
            .to_f64()
            .map_err(|e| anyhow::anyhow!("delta_percent: {:?}", e))?;

        // ── resolve graph ────────────────────────────────────────────────────
        let current_schema = context
            .current_schema
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("no current schema set"))?;

        let provider = current_schema
            .get_graph(&graph_name)
            .map_err(|e| anyhow::anyhow!("{:?}", e))?
            .ok_or_else(|| anyhow::anyhow!("graph `{graph_name}` not found"))?;

        let container = provider
            .downcast_ref::<GraphContainer>()
            .ok_or_else(|| anyhow::anyhow!("graph `{graph_name}` is not a GraphContainer"))?;

        let mem = match container.graph_storage() {
            GraphStorage::Memory(m) => Arc::clone(m),
        };

        let graph_type = container.graph_type();

        // ── label helpers ────────────────────────────────────────────────────
        let label_a_id = resolve_label(&*graph_type, &label_a)?;

        // Build id→name map for GCard logging (needed for neighbour labels).
        let id_to_name: HashMap<LabelId, String> = graph_type
            .label_names()
            .into_iter()
            .filter_map(|name| {
                graph_type.get_label_id(&name).ok()?.map(|id| (id, name))
            })
            .collect();

        // Extract the GCard update log if it exists (None = GCard not built yet).
        let update_log: Option<Arc<Mutex<GCardUpdateLog>>> = container
            .gcard_update_log()
            .and_then(|any| any.downcast::<Mutex<GCardUpdateLog>>().ok());

        // ── dispatch ─────────────────────────────────────────────────────────
        let (changed, disp_label) = match operation.as_str() {
            // Newly added vertices are isolated → no degree change → no GCard log entry.
            "add_vertices" => {
                let n = compute_delta(&mem, label_a_id, delta_count, delta_percent)?;
                let added = add_vertices(&mem, label_a_id, n)?;
                (added as i64, label_a.clone())
            }

            "delete_vertices" => {
                let n = compute_delta(&mem, label_a_id, delta_count, delta_percent)?;
                let deleted = delete_vertices(
                    &mem,
                    label_a_id,
                    &label_a,
                    n,
                    &id_to_name,
                    update_log.as_ref(),
                )?;
                (deleted as i64, label_a.clone())
            }

            "add_edges" => {
                let label_b_id    = resolve_label(&*graph_type, &label_b)?;
                let edge_label_id = resolve_label(&*graph_type, &edge_label)?;
                let n = compute_delta(&mem, label_a_id, delta_count, delta_percent)?;
                let added = add_edges(
                    &mem,
                    label_a_id,
                    label_b_id,
                    edge_label_id,
                    &label_a,
                    &label_b,
                    &edge_label,
                    n,
                    update_log.as_ref(),
                )?;
                (added as i64, format!("{label_a}->{label_b}"))
            }

            other => {
                return Err(anyhow::anyhow!(
                    "unknown operation `{other}`; expected add_vertices | delete_vertices | add_edges"
                )
                .into())
            }
        };

        let chunk = DataChunk::new(vec![
            Arc::new(StringArray::from_iter_values([operation.as_str()])),
            Arc::new(StringArray::from_iter_values([disp_label.as_str()])),
            Arc::new(Int64Array::from_iter_values([changed])),
        ]);
        Ok(vec![chunk])
    })
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn parse_str(
    v: &minigu_common::value::ScalarValue,
    name: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    Ok(v.try_as_string()
        .unwrap_or_else(|| panic!("arg `{name}` must be String"))
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("`{name}` cannot be null"))?
        .clone())
}

fn resolve_label(
    graph_type: &dyn GraphTypeProvider,
    name: &str,
) -> Result<LabelId, Box<dyn std::error::Error + Send + Sync>> {
    graph_type
        .get_label_id(name)
        .map_err(|e| anyhow::anyhow!("{:?}", e))?
        .ok_or_else(|| anyhow::anyhow!("label `{name}` not found in graph schema").into())
}

/// Resolve the absolute count to mutate:
/// - if `delta_count > 0`, use it directly
/// - otherwise compute `ceil(count_of_label_a × delta_percent / 100)`
fn compute_delta(
    mem: &Arc<MemoryGraph>,
    label_id: LabelId,
    delta_count: i64,
    delta_percent: f64,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    if delta_count > 0 {
        return Ok(delta_count as usize);
    }
    if delta_percent <= 0.0 {
        return Err(anyhow::anyhow!(
            "delta_count ≤ 0 and delta_percent ≤ 0; provide a positive count or a positive percent"
        )
        .into());
    }
    let txn = mem.txn_manager().begin_transaction(Serializable)?;
    let total = mem
        .iter_vertices(&txn)?
        .filter_map(|v| v.ok())
        .filter(|v| v.label_id == label_id)
        .count();
    txn.commit()?;
    let n = ((total as f64 * delta_percent / 100.0).ceil() as usize).max(1);
    Ok(n)
}

/// Add `count` new isolated vertices with `label_id`.
/// New vertex IDs start from `global_max_vid + 1`.
/// GCard note: no log entry needed — isolated vertices carry no degree information.
fn add_vertices(
    mem: &Arc<MemoryGraph>,
    label_id: LabelId,
    count: usize,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    let txn = mem.txn_manager().begin_transaction(Serializable)?;

    let max_vid: VertexId = mem
        .iter_vertices(&txn)?
        .filter_map(|v| v.ok())
        .map(|v| v.vid())
        .max()
        .unwrap_or(0);

    for i in 0..count {
        let vid: VertexId = max_vid + 1 + i as u64;
        mem.create_vertex(&txn, Vertex::new(vid, label_id, PropertyRecord::new(vec![])))?;
    }

    txn.commit()?;
    Ok(count)
}

/// Delete up to `count` vertices of `label_id` (first-encountered order).
///
/// Before each deletion the vertex's incident neighbours are collected and written to the
/// GCard update log (if present) so that multi-hop statistics remain consistent.
///
/// Returns the actual number deleted (may be less if fewer matching vertices exist).
fn delete_vertices(
    mem: &Arc<MemoryGraph>,
    label_id: LabelId,
    label_name: &str,
    count: usize,
    id_to_name: &HashMap<LabelId, String>,
    update_log: Option<&Arc<Mutex<GCardUpdateLog>>>,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    let txn = mem.txn_manager().begin_transaction(Serializable)?;

    let targets: Vec<VertexId> = mem
        .iter_vertices(&txn)?
        .filter_map(|v| v.ok())
        .filter(|v| v.label_id == label_id)
        .map(|v| v.vid())
        .take(count)
        .collect();

    let actual = targets.len();

    for vid in targets {
        // Collect all incident neighbours BEFORE deleting the vertex, as required by the log API.
        if let Some(log) = update_log {
            let neighbors = collect_neighbors(mem, &txn, vid, id_to_name);
            log.lock()
                .expect("GCardUpdateLog mutex poisoned")
                .record_delete_vertex(vid, label_name, &neighbors);
        }
        mem.delete_vertex(&txn, vid)?;
    }

    txn.commit()?;
    Ok(actual)
}

/// Create `count` edges between `src_label_id` (label_a) and `dst_label_id` (label_b)
/// vertices using round-robin pairing.
/// New edge IDs start from `global_max_eid + 1`.
///
/// Each new edge is recorded in the GCard update log (if present).
#[allow(clippy::too_many_arguments)]
fn add_edges(
    mem: &Arc<MemoryGraph>,
    src_label_id: LabelId,
    dst_label_id: LabelId,
    edge_label_id: LabelId,
    src_label_name: &str,
    dst_label_name: &str,
    edge_label_name: &str,
    count: usize,
    update_log: Option<&Arc<Mutex<GCardUpdateLog>>>,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    let txn = mem.txn_manager().begin_transaction(Serializable)?;

    let src_vids: Vec<VertexId> = mem
        .iter_vertices(&txn)?
        .filter_map(|v| v.ok())
        .filter(|v| v.label_id == src_label_id)
        .map(|v| v.vid())
        .collect();

    let dst_vids: Vec<VertexId> = mem
        .iter_vertices(&txn)?
        .filter_map(|v| v.ok())
        .filter(|v| v.label_id == dst_label_id)
        .map(|v| v.vid())
        .collect();

    if src_vids.is_empty() {
        return Err(anyhow::anyhow!("no vertices found for src label `{src_label_name}`").into());
    }
    if dst_vids.is_empty() {
        return Err(anyhow::anyhow!("no vertices found for dst label `{dst_label_name}`").into());
    }

    let max_eid: EdgeId = mem
        .iter_edges(&txn)?
        .filter_map(|e| e.ok())
        .map(|e| e.eid())
        .max()
        .unwrap_or(0);

    for i in 0..count {
        let src = src_vids[i % src_vids.len()];
        let dst = dst_vids[i % dst_vids.len()];
        let eid: EdgeId = max_eid + 1 + i as u64;
        mem.create_edge(
            &txn,
            Edge::new(eid, src, dst, edge_label_id, PropertyRecord::new(vec![])),
        )?;
        if let Some(log) = update_log {
            log.lock()
                .expect("GCardUpdateLog mutex poisoned")
                .record_insert_edge(src, src_label_name, dst, dst_label_name, edge_label_name);
        }
    }

    txn.commit()?;
    Ok(count)
}

// ── neighbour collection for delete_vertices ──────────────────────────────────

/// Collect all vertices incident to `vid` (both outgoing and incoming edges) as
/// `(neighbor_id, neighbor_label_name, edge_label_name)` tuples.
///
/// Unknown label IDs (absent from `id_to_name`) are silently skipped — this can happen
/// if the schema map is built before all labels are registered.
fn collect_neighbors(
    mem: &Arc<MemoryGraph>,
    txn: &Arc<minigu_storage::tp::transaction::MemTransaction>,
    vid: VertexId,
    id_to_name: &HashMap<LabelId, String>,
) -> Vec<(VertexId, String, String)> {
    let mut neighbors = Vec::new();

    // Outgoing: vid → nbr
    for result in txn.iter_adjacency_outgoing(vid) {
        if let Ok(nbr) = result {
            push_neighbor(mem, txn, nbr.neighbor_id(), nbr.label_id(), id_to_name, &mut neighbors);
        }
    }

    // Incoming: nbr → vid
    for result in txn.iter_adjacency_incoming(vid) {
        if let Ok(nbr) = result {
            push_neighbor(mem, txn, nbr.neighbor_id(), nbr.label_id(), id_to_name, &mut neighbors);
        }
    }

    neighbors
}

fn push_neighbor(
    mem: &Arc<MemoryGraph>,
    txn: &Arc<minigu_storage::tp::transaction::MemTransaction>,
    nbr_vid: VertexId,
    edge_label_id: LabelId,
    id_to_name: &HashMap<LabelId, String>,
    out: &mut Vec<(VertexId, String, String)>,
) {
    let Some(edge_label_name) = id_to_name.get(&edge_label_id) else { return };
    let Ok(nbr_vertex) = mem.get_vertex(txn, nbr_vid) else { return };
    let Some(nbr_label_name) = id_to_name.get(&nbr_vertex.label_id) else { return };
    out.push((nbr_vid, nbr_label_name.clone(), edge_label_name.clone()));
}
