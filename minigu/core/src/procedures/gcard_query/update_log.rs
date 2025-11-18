//! Lazy update log for GCard statistics.
//!
//! When edges/vertices are inserted or deleted, callers append [`UpdateEntry`] records here
//! instead of immediately recomputing multi-hop degree statistics.  [`GCardUpdateLog::compact_and_apply`]
//! later expands all pending entries via graph traversal, applies the net deltas to
//! [`Statistic`], removes any deleted vertices, and clears the log.
//!
//! Stored type-erased inside [`GraphContainer`] (same pattern as `Statistic` /
//! `DegreeSeqGraphCompressed`).  Consumers in `minigu-core` downcast via
//! `Arc<Mutex<GCardUpdateLog>>`.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use minigu_common::types::{LabelId, VertexId};
use minigu_storage::tp::{MemTransaction, MemoryGraph};

use crate::procedures::gcard_query::catalog::AltKey;
use crate::procedures::gcard_query::statistic::Statistic;

// ── Entry ─────────────────────────────────────────────────────────────────────

/// A single pending degree-delta record.
///
/// `template` is an [`AltKey`] whose **leftmost** vertex label equals `label`.
/// Example layouts (interleaved vertex / edge labels, same as `make_alt_key`):
///
/// | path hops | AltKey contents                                      |
/// |-----------|------------------------------------------------------|
/// | 1-hop     | `[tracked_label, edge_label, other_label]`           |
/// | 2-hop     | `[tracked_label, e1, mid_label, e2, other_label]`    |
#[derive(Debug, Clone)]
pub struct UpdateEntry {
    /// Path template; `label` is always the leftmost vertex.
    pub template: AltKey,
    /// Label of the endpoint nodes recorded in `nodes`.
    pub label: String,
    /// Vertex ID → degree delta.  +1 for insertion, -1 for deletion.
    pub nodes: HashMap<VertexId, i64>,
    /// Remaining expansion depth.  0 means this entry is final (no graph traversal needed
    /// during the apply phase).  Created entries start at `max_k - 1`.
    pub res_len: usize,
}

// ── Log ───────────────────────────────────────────────────────────────────────

/// Pending GCard update log stored inside [`GraphContainer`] as
/// `Arc<Mutex<GCardUpdateLog>>` (type-erased).
#[derive(Debug, Default)]
pub struct GCardUpdateLog {
    /// All pending update entries, in arrival order.
    pub entries: Vec<UpdateEntry>,
    /// Max path length K used when building statistics; determines initial `res_len`.
    pub max_k: usize,
    /// Vertices deleted since the last compact.  Their statistics will be completely
    /// removed from [`Statistic`] during [`compact_and_apply`], and they are skipped
    /// during graph traversal.
    pub deleted_vertices: HashSet<VertexId>,
}

impl GCardUpdateLog {
    pub fn new(max_k: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_k,
            deleted_vertices: HashSet::new(),
        }
    }

    // ── Compact + Apply ────────────────────────────────────────────────────

    /// Expand all pending entries to `res_len = 0` via graph traversal, accumulate the net
    /// per-vertex degree deltas, apply them to `statistic`, delete removed vertices, and
    /// clear the log.
    ///
    /// `edge_schema`: edge name → `(src_vertex_label_id, dst_vertex_label_id, edge_label_id)`.
    /// `label_id_to_name`: reverse mapping for building expanded template strings.
    ///
    /// **Expansion direction** (from the current node's perspective):
    /// * `w.dst_label == current_label` → incoming iteration  (X → current)
    /// * `w.src_label == current_label` → outgoing iteration  (current → X)
    ///
    /// Nodes in `deleted_vertices` are skipped during traversal; their statistics are
    /// removed from `statistic` at the end.
    pub fn compact_and_apply(
        &mut self,
        graph: &MemoryGraph,
        txn: &Arc<MemTransaction>,
        edge_schema: &HashMap<String, (LabelId, LabelId, LabelId)>,
        label_id_to_name: &HashMap<LabelId, String>,
        statistic: &mut Statistic,
    ) -> anyhow::Result<()> {
        use minigu_storage::iterators::AdjacencyIteratorTrait;

        // ── Build per-label extension table ───────────────────────────────
        // label → [(edge_name, neighbor_label, edge_label_id, outgoing_from_current)]
        let mut label_to_exts: HashMap<String, Vec<(String, String, LabelId, bool)>> =
            HashMap::new();
        for (edge_name, &(src_id, dst_id, edge_id)) in edge_schema {
            let Some(src_name) = label_id_to_name.get(&src_id) else { continue };
            let Some(dst_name) = label_id_to_name.get(&dst_id) else { continue };
            // From dst's perspective: src is an INCOMING neighbor.
            label_to_exts
                .entry(dst_name.clone())
                .or_default()
                .push((edge_name.clone(), src_name.clone(), edge_id, false));
            // From src's perspective: dst is an OUTGOING neighbor.
            label_to_exts
                .entry(src_name.clone())
                .or_default()
                .push((edge_name.clone(), dst_name.clone(), edge_id, true));
        }

        // ── Phase 1: expand → accumulate ─────────────────────────────────
        // acc: (altkey, label) → vertex_id → net delta
        let mut acc: HashMap<(AltKey, String), HashMap<VertexId, i64>> = HashMap::new();
        let mut to_expand: Vec<UpdateEntry> = self.entries.drain(..).collect();

        while let Some(entry) = to_expand.pop() {
            if entry.res_len == 0 {
                // Terminal: accumulate directly, skipping deleted vertices.
                let slot = acc.entry((entry.template, entry.label)).or_default();
                for (vid, delta) in entry.nodes {
                    if !self.deleted_vertices.contains(&vid) {
                        *slot.entry(vid).or_insert(0) += delta;
                    }
                }
                continue;
            }

            let Some(exts) = label_to_exts.get(&entry.label) else {
                // No schema edges for this label; treat as terminal.
                let slot = acc.entry((entry.template, entry.label)).or_default();
                for (vid, delta) in entry.nodes {
                    if !self.deleted_vertices.contains(&vid) {
                        *slot.entry(vid).or_insert(0) += delta;
                    }
                }
                continue;
            };

            // Expand: find neighbours of each (non-deleted) node.
            let mut new_groups: HashMap<(String, String), HashMap<VertexId, i64>> = HashMap::new();
            for (&vid, &delta) in &entry.nodes {
                if self.deleted_vertices.contains(&vid) {
                    continue;
                }
                for (edge_name, nbr_label, edge_label_id, outgoing) in exts {
                    let eid = *edge_label_id;
                    let adj_iter = if *outgoing {
                        txn.iter_adjacency_outgoing(vid)
                    } else {
                        txn.iter_adjacency_incoming(vid)
                    };
                    let adj_iter =
                        AdjacencyIteratorTrait::filter(adj_iter, move |n| n.label_id() == eid);
                    for nbr_result in adj_iter {
                        let nbr = nbr_result?;
                        let nbr_id = nbr.neighbor_id();
                        if !self.deleted_vertices.contains(&nbr_id) {
                            *new_groups
                                .entry((nbr_label.clone(), edge_name.clone()))
                                .or_default()
                                .entry(nbr_id)
                                .or_insert(0) += delta;
                        }
                    }
                }
            }

            // Emit one new entry per (neighbor_label, edge_name) group.
            for ((nbr_label, edge_name), nodes) in new_groups {
                let nodes: HashMap<VertexId, i64> =
                    nodes.into_iter().filter(|(_, d)| *d != 0).collect();
                if nodes.is_empty() {
                    continue;
                }
                let mut new_parts = vec![nbr_label.clone(), edge_name];
                new_parts.extend_from_slice(&entry.template.0);
                to_expand.push(UpdateEntry {
                    template: AltKey(new_parts),
                    label: nbr_label,
                    nodes,
                    res_len: entry.res_len - 1,
                });
            }
        }

        // ── Phase 2: apply net deltas to statistic ────────────────────────
        for ((altkey, label), nodes) in acc {
            for (vertex_id, delta) in nodes {
                if delta != 0 {
                    statistic.apply_delta(&label, &altkey, vertex_id, delta);
                }
            }
        }

        // ── Phase 3: remove deleted vertices from statistic ───────────────
        for &vid in &self.deleted_vertices {
            statistic.delete_vertex(vid);
        }
        self.deleted_vertices.clear();
        // self.entries already empty (drained above)

        Ok(())
    }

    // ── Public record helpers ──────────────────────────────────────────────

    /// Record an edge insertion `(u:u_label) -[edge_label]-> (v:v_label)`.
    ///
    /// Appends two entries (one per endpoint); no graph traversal is performed.
    pub fn record_insert_edge(
        &mut self,
        u: VertexId,
        u_label: &str,
        v: VertexId,
        v_label: &str,
        edge_label: &str,
    ) {
        self.push_edge_entries(u, u_label, v, v_label, edge_label, 1);
    }

    /// Record an edge deletion `(u:u_label) -[edge_label]-> (v:v_label)`.
    ///
    /// Appends two entries (one per endpoint); no graph traversal is performed.
    pub fn record_delete_edge(
        &mut self,
        u: VertexId,
        u_label: &str,
        v: VertexId,
        v_label: &str,
        edge_label: &str,
    ) {
        self.push_edge_entries(u, u_label, v, v_label, edge_label, -1);
    }

    /// Record a vertex deletion for `(u:u_label)`.
    ///
    /// `u` is added to `deleted_vertices` so it is skipped during compact traversal and
    /// its statistics are removed at the end of [`compact_and_apply`].
    ///
    /// `neighbors` must be collected **before** the vertex is removed from the graph.
    /// Each element is `(neighbor_id, neighbor_label, edge_label)` for every edge
    /// incident to `u`.  One entry per distinct `(neighbor_label, edge_label)` group is
    /// appended to propagate the deletion effect to the neighbours' multi-hop statistics.
    pub fn record_delete_vertex(
        &mut self,
        u: VertexId,
        u_label: &str,
        neighbors: &[(VertexId, String, String)],
    ) {
        self.deleted_vertices.insert(u);

        let res_len = self.max_k.saturating_sub(1);

        let mut groups: HashMap<(String, String), Vec<VertexId>> = HashMap::new();
        for (neighbor_id, neighbor_label, edge_label) in neighbors {
            groups
                .entry((neighbor_label.clone(), edge_label.clone()))
                .or_default()
                .push(*neighbor_id);
        }

        for ((neighbor_label, edge_label), neighbor_ids) in groups {
            let template = AltKey(vec![
                neighbor_label.clone(),
                edge_label,
                u_label.to_string(),
            ]);
            let nodes: HashMap<VertexId, i64> =
                neighbor_ids.into_iter().map(|id| (id, -1)).collect();
            self.entries.push(UpdateEntry {
                template,
                label: neighbor_label,
                nodes,
                res_len,
            });
        }
    }

    // ── Internal helpers ───────────────────────────────────────────────────

    fn push_edge_entries(
        &mut self,
        u: VertexId,
        u_label: &str,
        v: VertexId,
        v_label: &str,
        edge_label: &str,
        delta: i64,
    ) {
        let res_len = self.max_k.saturating_sub(1);

        self.entries.push(UpdateEntry {
            template: AltKey(vec![
                u_label.to_string(),
                edge_label.to_string(),
                v_label.to_string(),
            ]),
            label: u_label.to_string(),
            nodes: HashMap::from([(u, delta)]),
            res_len,
        });

        self.entries.push(UpdateEntry {
            template: AltKey(vec![
                v_label.to_string(),
                edge_label.to_string(),
                u_label.to_string(),
            ]),
            label: v_label.to_string(),
            nodes: HashMap::from([(v, delta)]),
            res_len,
        });
    }
}

// ── Convenience constructor for Arc<Mutex<GCardUpdateLog>> ────────────────────

/// Build the type-erased value that [`GraphContainer`] stores.
pub fn new_log_arc(max_k: usize) -> Arc<Mutex<GCardUpdateLog>> {
    Arc::new(Mutex::new(GCardUpdateLog::new(max_k)))
}
