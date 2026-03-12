use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use itertools::Itertools;
use minigu_catalog::provider::{GraphProvider, GraphTypeProvider, SchemaProvider};
use minigu_common::data_type::LogicalType;
use minigu_common::types::{LabelId, VertexId};
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::procedure::Procedure;
use minigu_storage::error::StorageResult;
use minigu_storage::iterators::AdjacencyIteratorTrait;
use minigu_storage::tp::{MemTransaction, MemoryGraph};
use minigu_transaction::{GraphTxnManager, Transaction};
use minigu_transaction::IsolationLevel::Serializable;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use serde::{Deserialize, Serialize};

use super::catalog::AltKey;
use super::{make_alt_key, Statistic};
use crate::procedures::gcard_query::error::GCardResult;

// ----- Schema edge info (string-based, same shape as create_catalog) -----

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchemaEdgeInfo {
    pub edge_name: String,
    pub src_label: String,
    pub dst_label: String,
}

// ----- Path pattern (string-based, canonical form for path set equality) -----

/// Path pattern with node/edge label names. Canonical form: (vs, es) <= (reverse(vs), reverse(es)).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PathPattern {
    pub vs: Vec<String>,
    pub es: Vec<String>,
}

impl fmt::Display for PathPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.vs.is_empty() {
            return Ok(());
        }

        write!(f, "{}", self.vs[0])?;

        for (e, v) in self.es.iter().zip(self.vs.iter().skip(1)) {
            write!(f, " -{}-> {}", e, v)?;
        }

        Ok(())
    }
}

impl PathPattern {
    pub fn new(vs: Vec<String>, es: Vec<String>) -> PathPattern {
        assert_eq!(vs.len(), es.len() + 1);
        let mut rvs = vs.clone();
        rvs.reverse();
        let mut res = es.clone();
        res.reverse();
        if (&vs, &es) <= (&rvs, &res) {
            PathPattern { vs, es }
        } else {
            PathPattern { vs: rvs, es: res }
        }
    }

    pub fn new_without_reverse(vs: Vec<String>, es: Vec<String>) -> PathPattern {
        assert_eq!(vs.len(), es.len() + 1);
        PathPattern { vs, es }
    }

    pub fn to_alt_key(&self) -> AltKey {
        make_alt_key(&self.vs, &self.es)
    }

    /// Canonical key for cache lookup (same pattern reversed is the same).
    pub fn sort(&self) -> Self {
        let mut vs = self.vs.clone();
        vs.reverse();
        let mut es = self.es.clone();
        es.reverse();
        if (&vs, &es) <= (&self.vs, &self.es) {
            PathPattern { vs, es }
        } else {
            PathPattern {
                vs: self.vs.clone(),
                es: self.es.clone(),
            }
        }
    }
}

impl PartialEq for PathPattern {
    fn eq(&self, other: &Self) -> bool {
        self.vs == other.vs && self.es == other.es
    }
}
impl Eq for PathPattern {}

impl Hash for PathPattern {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.vs.hash(state);
        self.es.hash(state);
    }
}

// ----- Cache types (same as create_catalog) -----

type DegreeMap = HashMap<VertexId, u64>;
type PathsByLen = HashMap<usize, HashSet<PathPattern>>;
pub type PatternDegCache = HashMap<PathPattern, HashMap<String, DegreeMap>>;

// ----- Build schema edges from catalog -----

pub fn get_edges_from_catalog(
    catalog: &dyn GraphTypeProvider,
) -> Result<HashMap<String, SchemaEdgeInfo>, anyhow::Error> {
    let mut label_id_to_name: HashMap<LabelId, String> = HashMap::new();
    for name in catalog.label_names() {
        if let Ok(Some(id)) = catalog.get_label_id(&name) {
            label_id_to_name.insert(id, name);
        }
    }
    let mut edges = HashMap::new();
    for edge_type in catalog.edge_type_keys() {
        let edge_type_ref = catalog
            .get_edge_type(&edge_type)?
            .ok_or_else(|| anyhow::anyhow!("edge type not found"))?;
        let src_id = edge_type_ref
            .src()
            .label_set()
            .first()
            .ok_or_else(|| anyhow::anyhow!("empty src label set"))?;
        let dst_id = edge_type_ref
            .dst()
            .label_set()
            .first()
            .ok_or_else(|| anyhow::anyhow!("empty dst label set"))?;
        let edge_label_id = edge_type
            .first()
            .ok_or_else(|| anyhow::anyhow!("empty edge label set"))?;
        let edge_name = label_id_to_name
            .get(&edge_label_id)
            .cloned()
            .unwrap_or_else(|| format!("Unknown_{}", edge_label_id));
        let src_label = label_id_to_name
            .get(&src_id)
            .cloned()
            .unwrap_or_else(|| format!("Unknown_{}", src_id));
        let dst_label = label_id_to_name
            .get(&dst_id)
            .cloned()
            .unwrap_or_else(|| format!("Unknown_{}", dst_id));
        edges.insert(edge_name.clone(), SchemaEdgeInfo {
            edge_name,
            src_label,
            dst_label,
        });
    }
    Ok(edges)
}

// ----- Path enumeration (DFS, same as create_catalog) -----

fn collect_vertex_and_edge_types(
    edges: &HashMap<String, SchemaEdgeInfo>,
) -> (HashSet<String>, HashSet<String>) {
    let mut vertices = HashSet::new();
    let mut edge_type = HashSet::new();
    for e in edges.values() {
        vertices.insert(e.src_label.clone());
        vertices.insert(e.dst_label.clone());
        edge_type.insert(e.edge_name.clone());
    }
    (vertices, edge_type)
}

fn build_undirected_adj(
    edges: &HashMap<String, SchemaEdgeInfo>,
) -> HashMap<String, Vec<(String, String)>> {
    let mut adj: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for e in edges.values() {
        adj.entry(e.src_label.clone())
            .or_default()
            .push((e.edge_name.clone(), e.dst_label.clone()));
        adj.entry(e.dst_label.clone())
            .or_default()
            .push((e.edge_name.clone(), e.src_label.clone()));
    }
    adj
}

fn enumerate_all_paths_walks_in_schema(
    edges: &HashMap<String, SchemaEdgeInfo>,
    max_len: usize,
) -> PathsByLen {
    let (vertex_types, _) = collect_vertex_and_edge_types(edges);
    let adj = build_undirected_adj(edges);
    let mut out: PathsByLen = HashMap::new();

    fn dfs(
        adj: &HashMap<String, Vec<(String, String)>>,
        max_len: usize,
        node_seq: &mut Vec<String>,
        edge_seq: &mut Vec<String>,
        out: &mut PathsByLen,
    ) {
        let cur_len = edge_seq.len();
        if cur_len > 0 {
            out.entry(cur_len)
                .or_default()
                .insert(PathPattern::new(node_seq.clone(), edge_seq.clone()));
        }
        if cur_len == max_len {
            return;
        }
        let cur_node = node_seq.last().unwrap().clone();
        if let Some(nbrs) = adj.get(&cur_node) {
            for (edge_name, next_node) in nbrs.iter() {
                edge_seq.push(edge_name.clone());
                node_seq.push(next_node.clone());
                dfs(adj, max_len, node_seq, edge_seq, out);
                node_seq.pop();
                edge_seq.pop();
            }
        }
    }

    for start in vertex_types.iter() {
        let mut node_seq = vec![start.clone()];
        let mut edge_seq: Vec<String> = Vec::new();
        dfs(&adj, max_len, &mut node_seq, &mut edge_seq, &mut out);
    }
    out
}

// ----- Graph iteration helpers -----

fn iter_vertices_of_type(
    graph: &MemoryGraph,
    node_type: LabelId,
    txn: &Arc<MemTransaction>,
) -> StorageResult<Vec<VertexId>> {
    graph
        .iter_vertices(txn)?
        .filter_map(|vertex| match vertex {
            Ok(vertex) => {
                if vertex.label_id == node_type {
                    Some(Ok(vertex.vid))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(e)),
        })
        .collect()
}

/// Vertex-centric: degree map for this vertex type over this edge type.
/// edge_id + outgoing only; in your system edge uniquely fixes both endpoints, so no need to
/// resolve or filter by neighbor label.
fn compute_degree_map_for_vertex_type(
    graph: &MemoryGraph,
    txn: &Arc<MemTransaction>,
    vertex_label_id: LabelId,
    edge_label_id: LabelId,
    outgoing: bool,
) -> Result<DegreeMap, anyhow::Error> {
    // (A:label_m)-[m:P]->(C:w) where A.id = xxx;
    let vertices = iter_vertices_of_type(graph, vertex_label_id, txn)?;
    let mut deg: DegreeMap = HashMap::new();
    for u in vertices {
        let mut count = 0u64;
        let mut adj_iter = if outgoing {
            txn.iter_adjacency_outgoing(u)
        } else {
            txn.iter_adjacency_incoming(u)
        };
        adj_iter = AdjacencyIteratorTrait::filter(adj_iter, move |n| n.label_id() == edge_label_id);
        for neighbor_result in adj_iter {
            if neighbor_result.is_ok() {
                count += 1;
            }
        }
        deg.insert(u, count);
    }
    Ok(deg)
}

/// Vertex-centric: extended degree map over this edge (sum suffix_deg over neighbors, on-the-fly).
/// edge_id + outgoing only; edge uniquely fixes both endpoints, so no neighbor label check.
fn dp_extend_for_vertex_type(
    graph: &MemoryGraph,
    txn: &Arc<MemTransaction>,
    vertex_label_id: LabelId,
    edge_label_id: LabelId,
    outgoing: bool,
    suffix_deg: &DegreeMap,
) -> GCardResult<DegreeMap> {
    let vertices = iter_vertices_of_type(graph, vertex_label_id, txn)?;
    let len_of_vertices = vertices.len();
    let mut out = HashMap::new();
    for u in vertices {
        let mut sum = 0u64;
        let mut adj_iter = if outgoing {
            txn.iter_adjacency_outgoing(u)
        } else {
            txn.iter_adjacency_incoming(u)
        };
        adj_iter = AdjacencyIteratorTrait::filter(adj_iter, move |n| n.label_id() == edge_label_id);
        for neighbor_result in adj_iter {
            if let Ok(neighbor) = neighbor_result {
                let vid = neighbor.neighbor_id();
                sum += suffix_deg.get(&vid).copied().unwrap_or(0);
            }
        }
        out.insert(u, sum);
    }
    if out.len() != len_of_vertices {
        println!("len is not same");
    }
    Ok(out)
}

// ----- Degree computation (len1, len>=2) -----
// Outgoing is determined from SchemaEdgeInfo: vertex is edge's src => outgoing true, else false.

fn compute_len1_degrees(
    graph: &MemoryGraph,
    txn: &Arc<MemTransaction>,
    edges: &HashMap<String, SchemaEdgeInfo>,
    label_name_to_id: &HashMap<String, LabelId>,
    patterns: &HashSet<PathPattern>,
    cache: &mut PatternDegCache,
) -> Result<(), anyhow::Error> {
    if patterns.is_empty() {
        return Ok(());
    }
    let mut seen_alt_key: HashSet<AltKey> = HashSet::new();
    let mut pattern_to_handle: Vec<PathPattern> = Vec::new();
    for pattern in patterns {
        if seen_alt_key.insert(pattern.to_alt_key()) {
            pattern_to_handle.push(pattern.clone());
        }
    }

    let computed: Vec<Result<(PathPattern, String, DegreeMap, String, DegreeMap), anyhow::Error>> =
        pattern_to_handle
            .par_iter()
            .map(|pattern| {
                let edge = &pattern.es[0];
                let edge_info = edges
                    .get(edge)
                    .ok_or_else(|| anyhow::anyhow!("edge not found: {}", edge))?;
                let edge_id = label_name_to_id
                    .get(edge)
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("label id not found: {}", edge))?;
                let src_id = label_name_to_id
                    .get(&edge_info.src_label)
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("src label not found"))?;
                let dst_id = label_name_to_id
                    .get(&edge_info.dst_label)
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("dst label not found"))?;

                let left_deg =
                    compute_degree_map_for_vertex_type(graph, txn, src_id, edge_id, true)?;
                let right_deg =
                    compute_degree_map_for_vertex_type(graph, txn, dst_id, edge_id, false)?;

                Ok((
                    pattern.clone(),
                    edge_info.src_label.clone(),
                    left_deg,
                    edge_info.dst_label.clone(),
                    right_deg,
                ))
            })
            .collect();

    for res in computed {
        let (pattern, src_label, left_deg, dst_label, right_deg) = res?;
        let entry = cache.entry(pattern).or_default();
        entry.insert(src_label, left_deg);
        entry.insert(dst_label, right_deg);
    }
    Ok(())
}

fn extend_end_degree(
    graph: &MemoryGraph,
    txn: &Arc<MemTransaction>,
    edges: &HashMap<String, SchemaEdgeInfo>,
    label_name_to_id: &HashMap<String, LabelId>,
    cache_ro: &PatternDegCache,
    start_vertex_label: &str,
    first_edge_label: &str,
    suffix: &PathPattern,
) -> GCardResult<DegreeMap> {
    let suffix_left_key = suffix.vs.get(0).expect("suffix empty").to_string();

    let suffix_left_deg = cache_ro
        .get(&suffix.sort())
        .expect("suffix left deg missing")
        .get(&suffix_left_key)
        .expect("suffix left deg missing");

    let edge_info = edges.get(first_edge_label).expect("edge");
    let edge_id = label_name_to_id
        .get(first_edge_label)
        .copied()
        .expect("label id");

    let vertex_id = label_name_to_id
        .get(start_vertex_label)
        .copied()
        .expect("vertex label");

    let outgoing = edge_info.src_label == start_vertex_label;
    dp_extend_for_vertex_type(graph, txn, vertex_id, edge_id, outgoing, suffix_left_deg)
}

fn compute_len_ge2_degrees(
    graph: &MemoryGraph,
    txn: &Arc<MemTransaction>,
    edges: &HashMap<String, SchemaEdgeInfo>,
    label_name_to_id: &HashMap<String, LabelId>,
    patterns: &HashSet<PathPattern>,
    cache: &mut PatternDegCache,
) -> Result<(), anyhow::Error> {
    let cache_ro: &PatternDegCache = cache;

    let mut seen_alt_key: HashSet<AltKey> = HashSet::new();
    let mut pattern_to_handle: HashSet<PathPattern> = HashSet::new();
    for pattern in patterns {
        if seen_alt_key.insert(pattern.to_alt_key()) {
            pattern_to_handle.insert(pattern.clone());
        }
    }
    let computed: Vec<Result<(PathPattern, String, DegreeMap, String, DegreeMap), anyhow::Error>> =
        pattern_to_handle
            .par_iter()
            .map(|pattern| {
                let v_seq = &pattern.vs;
                let e_seq = &pattern.es;

                let left_node = v_seq[0].to_string();
                let right_node = v_seq.last().unwrap().to_string();

                let suffix =
                    PathPattern::new_without_reverse(v_seq[1..].to_vec(), e_seq[1..].to_vec());
                let left_deg = extend_end_degree(
                    graph,
                    txn,
                    edges,
                    label_name_to_id,
                    cache_ro,
                    &left_node,
                    &e_seq[0],
                    &suffix,
                )?;

                let rp = PathPattern::new_without_reverse(
                    v_seq.iter().cloned().rev().collect(),
                    e_seq.iter().cloned().rev().collect(),
                );
                let rsuffix =
                    PathPattern::new_without_reverse(rp.vs[1..].to_vec(), rp.es[1..].to_vec());
                let right_deg = extend_end_degree(
                    graph,
                    txn,
                    edges,
                    label_name_to_id,
                    cache_ro,
                    &right_node,
                    &rp.es[0],
                    &rsuffix,
                )?;

                Ok((pattern.clone(), left_node, left_deg, right_node, right_deg))
            })
            .collect();

    for res in computed {
        let (pattern, left_node, left_deg, right_node, right_deg) = res?;
        let entry = cache.entry(pattern).or_default();
        entry.insert(left_node, left_deg);
        entry.insert(right_node, right_deg);
    }
    Ok(())
}

// ----- Neighbor-cached, dependency-driven computation -----

/// Key for a one-hop neighbor cache entry.
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct HopKey {
    vertex_label: String,
    edge_label: String,
    outgoing: bool,
}

/// Cached neighbor lists: for each vertex of `vertex_label`, the list of neighbor IDs
/// reachable via `edge_label` in the given direction.
type NeighborList = HashMap<VertexId, Vec<VertexId>>;

/// Build the neighbor list for one HopKey by scanning the graph once.
fn build_neighbor_list(
    graph: &MemoryGraph,
    txn: &Arc<MemTransaction>,
    vertex_label_id: LabelId,
    edge_label_id: LabelId,
    outgoing: bool,
) -> Result<NeighborList, anyhow::Error> {
    let vertices = iter_vertices_of_type(graph, vertex_label_id, txn)?;
    let mut result: NeighborList = HashMap::with_capacity(vertices.len());
    for u in vertices {
        let mut neighbors = Vec::new();
        let mut adj_iter = if outgoing {
            txn.iter_adjacency_outgoing(u)
        } else {
            txn.iter_adjacency_incoming(u)
        };
        adj_iter = AdjacencyIteratorTrait::filter(adj_iter, move |n| n.label_id() == edge_label_id);
        for neighbor_result in adj_iter {
            if let Ok(neighbor) = neighbor_result {
                neighbors.push(neighbor.neighbor_id());
            }
        }
        result.insert(u, neighbors);
    }
    Ok(result)
}

/// Compute degree map from a cached neighbor list (just count neighbors).
fn degree_map_from_neighbor_list(neighbors: &NeighborList) -> DegreeMap {
    neighbors
        .iter()
        .map(|(&vid, nbrs)| (vid, nbrs.len() as u64))
        .collect()
}

/// Extend degree map using cached neighbor list + suffix degree map.
/// For each vertex u, sum suffix_deg[neighbor] over all cached neighbors.
fn extend_with_neighbor_cache(
    neighbors: &NeighborList,
    suffix_deg: &DegreeMap,
) -> DegreeMap {
    neighbors
        .iter()
        .map(|(&vid, nbrs)| {
            let sum: u64 = nbrs
                .iter()
                .map(|n| suffix_deg.get(n).copied().unwrap_or(0))
                .sum();
            (vid, sum)
        })
        .collect()
}

/// Lazy, dependency-driven computation with neighbor caching.
///
/// Instead of building ALL neighbor caches upfront, this function builds them
/// one at a time (ordered by shortest-dependent pattern length) and immediately
/// computes any patterns whose dependencies are fully satisfied.  This cascades:
/// computing a len-1 pattern may unlock len-2 patterns whose HopKeys are already
/// cached.  Neighbor caches are evicted as soon as all their consumers are done,
/// keeping peak memory low.
fn compute_all_degrees_cached(
    graph: &MemoryGraph,
    txn: &Arc<MemTransaction>,
    edges: &HashMap<String, SchemaEdgeInfo>,
    label_name_to_id: &HashMap<String, LabelId>,
    schema_path: &PathsByLen,
    max_k: usize,
) -> Result<PatternDegCache, anyhow::Error> {
    // Step 1: Collect all unique patterns (deduplicated by alt_key).
    let mut all_patterns: Vec<PathPattern> = Vec::new();
    let mut seen_alt_keys: HashSet<AltKey> = HashSet::new();
    for len in 1..=max_k {
        if let Some(patterns) = schema_path.get(&len) {
            for p in patterns {
                if seen_alt_keys.insert(p.to_alt_key()) {
                    all_patterns.push(p.clone());
                }
            }
        }
    }

    // Step 2: Precompute dependency info for each pattern.
    //   - left_hop / right_hop: neighbor caches needed
    //   - suffix_keys: canonical suffix patterns that must be computed first (empty for len-1)
    let mut pattern_left_hop: HashMap<PathPattern, HopKey> = HashMap::new();
    let mut pattern_right_hop: HashMap<PathPattern, HopKey> = HashMap::new();
    let mut pattern_suffix_keys: HashMap<PathPattern, HashSet<PathPattern>> = HashMap::new();
    let mut hop_consumer_count: HashMap<HopKey, usize> = HashMap::new();
    let mut hop_min_len: HashMap<HopKey, usize> = HashMap::new();

    for pattern in &all_patterns {
        let left_hop = {
            let edge_info = edges.get(&pattern.es[0]).expect("edge not found");
            HopKey {
                vertex_label: pattern.vs[0].clone(),
                edge_label: pattern.es[0].clone(),
                outgoing: edge_info.src_label == pattern.vs[0],
            }
        };
        let right_hop = {
            let last_edge = pattern.es.last().unwrap();
            let last_vertex = pattern.vs.last().unwrap();
            let edge_info = edges.get(last_edge).expect("edge not found");
            HopKey {
                vertex_label: last_vertex.clone(),
                edge_label: last_edge.clone(),
                outgoing: edge_info.src_label == *last_vertex,
            }
        };

        let pat_len = pattern.es.len();
        *hop_consumer_count.entry(left_hop.clone()).or_insert(0) += 1;
        *hop_consumer_count.entry(right_hop.clone()).or_insert(0) += 1;
        hop_min_len
            .entry(left_hop.clone())
            .and_modify(|m| *m = (*m).min(pat_len))
            .or_insert(pat_len);
        hop_min_len
            .entry(right_hop.clone())
            .and_modify(|m| *m = (*m).min(pat_len))
            .or_insert(pat_len);

        let mut suffix_keys: HashSet<PathPattern> = HashSet::new();
        if pat_len >= 2 {
            let left_suffix = PathPattern::new_without_reverse(
                pattern.vs[1..].to_vec(),
                pattern.es[1..].to_vec(),
            );
            suffix_keys.insert(left_suffix.sort());

            let rp = PathPattern::new_without_reverse(
                pattern.vs.iter().cloned().rev().collect(),
                pattern.es.iter().cloned().rev().collect(),
            );
            let right_suffix = PathPattern::new_without_reverse(
                rp.vs[1..].to_vec(),
                rp.es[1..].to_vec(),
            );
            suffix_keys.insert(right_suffix.sort());
        }

        pattern_left_hop.insert(pattern.clone(), left_hop);
        pattern_right_hop.insert(pattern.clone(), right_hop);
        pattern_suffix_keys.insert(pattern.clone(), suffix_keys);
    }

    // Step 3: Order HopKeys by min pattern length (build short-pattern deps first).
    let mut ordered_hops: Vec<HopKey> = hop_consumer_count.keys().cloned().collect();
    ordered_hops.sort_by_key(|h| hop_min_len.get(h).copied().unwrap_or(usize::MAX));

    // Step 4: Lazy build loop — build one HopKey at a time, then cascade.
    let mut neighbor_caches: HashMap<HopKey, NeighborList> = HashMap::new();
    let mut cache: PatternDegCache = HashMap::new();
    let mut computed_patterns: HashSet<PathPattern> = HashSet::new();
    let mut pending_consumers = hop_consumer_count.clone();

    for hop in &ordered_hops {
        // Build this HopKey's neighbor cache.
        let vertex_label_id = *label_name_to_id
            .get(&hop.vertex_label)
            .ok_or_else(|| anyhow::anyhow!("vertex label not found: {}", hop.vertex_label))?;
        let edge_label_id = *label_name_to_id
            .get(&hop.edge_label)
            .ok_or_else(|| anyhow::anyhow!("edge label not found: {}", hop.edge_label))?;
        let nl = build_neighbor_list(graph, txn, vertex_label_id, edge_label_id, hop.outgoing)?;
        println!("Built neighbor cache {:?} ({} vertices)", hop, nl.len());
        neighbor_caches.insert(hop.clone(), nl);

        // Cascade: compute all patterns whose dependencies are now fully satisfied.
        // This loop handles transitivity: computing len-1 patterns may unlock len-2, etc.
        loop {
            let ready: Vec<PathPattern> = all_patterns
                .iter()
                .filter(|p| {
                    !computed_patterns.contains(*p) && {
                        let lh = &pattern_left_hop[*p];
                        let rh = &pattern_right_hop[*p];
                        let sks = &pattern_suffix_keys[*p];
                        neighbor_caches.contains_key(lh)
                            && neighbor_caches.contains_key(rh)
                            && sks.iter().all(|sk| computed_patterns.contains(sk))
                    }
                })
                .cloned()
                .collect();

            if ready.is_empty() {
                break;
            }

            // Compute ready patterns in parallel.
            let cache_ref: &PatternDegCache = &cache;
            let results: Vec<
                Result<(PathPattern, String, DegreeMap, String, DegreeMap), anyhow::Error>,
            > = ready
                .par_iter()
                .map(|pattern| {
                    let v_seq = &pattern.vs;
                    let e_seq = &pattern.es;
                    let len = e_seq.len();
                    let left_node = v_seq[0].clone();
                    let right_node = v_seq.last().unwrap().clone();

                    // Left degree
                    let left_hop = &pattern_left_hop[pattern];
                    let left_neighbors = neighbor_caches.get(left_hop).unwrap();
                    let left_deg = if len == 1 {
                        degree_map_from_neighbor_list(left_neighbors)
                    } else {
                        let suffix = PathPattern::new_without_reverse(
                            v_seq[1..].to_vec(),
                            e_seq[1..].to_vec(),
                        );
                        let suffix_key = suffix.vs[0].clone();
                        let suffix_deg = cache_ref
                            .get(&suffix.sort())
                            .and_then(|m| m.get(&suffix_key))
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "suffix deg missing for {} in pattern {}",
                                    suffix_key,
                                    suffix
                                )
                            })?;
                        extend_with_neighbor_cache(left_neighbors, suffix_deg)
                    };

                    // Right degree
                    let right_hop = &pattern_right_hop[pattern];
                    let right_neighbors = neighbor_caches.get(right_hop).unwrap();
                    let right_deg = if len == 1 {
                        degree_map_from_neighbor_list(right_neighbors)
                    } else {
                        let rp = PathPattern::new_without_reverse(
                            v_seq.iter().cloned().rev().collect(),
                            e_seq.iter().cloned().rev().collect(),
                        );
                        let rsuffix = PathPattern::new_without_reverse(
                            rp.vs[1..].to_vec(),
                            rp.es[1..].to_vec(),
                        );
                        let rsuffix_key = rsuffix.vs[0].clone();
                        let rsuffix_deg = cache_ref
                            .get(&rsuffix.sort())
                            .and_then(|m| m.get(&rsuffix_key))
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "rsuffix deg missing for {} in pattern {}",
                                    rsuffix_key,
                                    rp
                                )
                            })?;
                        extend_with_neighbor_cache(right_neighbors, rsuffix_deg)
                    };

                    Ok((pattern.clone(), left_node, left_deg, right_node, right_deg))
                })
                .collect();

            // Merge results into cache, mark computed, decrement consumer counts.
            for result in results {
                let (pattern, left_node, left_deg, right_node, right_deg) = result?;

                let lh = &pattern_left_hop[&pattern];
                let rh = &pattern_right_hop[&pattern];
                if let Some(c) = pending_consumers.get_mut(lh) {
                    *c = c.saturating_sub(1);
                }
                if let Some(c) = pending_consumers.get_mut(rh) {
                    *c = c.saturating_sub(1);
                }

                let canonical = pattern.sort();
                let entry = cache.entry(canonical.clone()).or_default();
                entry.insert(left_node, left_deg);
                entry.insert(right_node, right_deg);
                computed_patterns.insert(canonical);
            }

            // Evict neighbor caches whose consumers have all been served.
            let evict_keys: Vec<HopKey> = pending_consumers
                .iter()
                .filter(|(_, count)| **count == 0)
                .map(|(k, _)| k.clone())
                .collect();
            for key in &evict_keys {
                if let Some(nl) = neighbor_caches.remove(key) {
                    let mem_bytes = nl.len() * std::mem::size_of::<VertexId>()
                        + nl.values()
                            .map(|v| v.len() * std::mem::size_of::<VertexId>())
                            .sum::<usize>();
                    println!(
                        "Evicted neighbor cache {:?} ({} vertices, ~{:.2} MB)",
                        key,
                        nl.len(),
                        mem_bytes as f64 / 1024.0 / 1024.0,
                    );
                }
                pending_consumers.remove(key);
            }
        }
    }

    Ok(cache)
}

// ----- Build Statistic from PatternDegCache -----

fn build_statistic_from_pattern_cache(cache: &PatternDegCache) -> Result<Statistic, anyhow::Error> {
    let mut statistic = Statistic::default();
    for (path_pattern, endpoint_degrees) in cache {
        let alt_key = path_pattern.to_alt_key();
        for (node_name, degree_map) in endpoint_degrees {
            let vertex_ids: Vec<VertexId> = degree_map.keys().copied().collect();
            let frequencies: Vec<u64> = vertex_ids.iter().map(|v| degree_map[v]).collect();
            statistic
                .insert_or_update(node_name, &vertex_ids, alt_key.clone(), &frequencies)
                .map_err(|e| anyhow::anyhow!("Statistic::insert_or_update: {}", e))?;
        }
    }
    Ok(statistic)
}

pub fn build_procedure() -> Procedure {
    // parameters:
    // 1. graph name
    // 2. max length of path.
    // 3. (optional) mode: 0 = layer-by-layer (default), 1 = neighbor-cached dependency-driven
    // 4. (optional) num_threads: number of rayon threads (default = available parallelism)
    let parameters = vec![
        LogicalType::String,
        LogicalType::Int8,
        LogicalType::UInt8,
        LogicalType::UInt8,
    ];
    Procedure::new(parameters, None, move |mut context, args| {
        let graph_name = args[0]
            .try_as_string()
            .expect("expecting string value for graph_name")
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("expecting string value for graph name"))?
            .to_string();

        let default_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(8);
        let num_threads = args
            .get(3)
            .and_then(|a| a.to_u8().ok())
            .map(|n| if n == 0 { default_threads } else { n as usize })
            .unwrap_or(default_threads);

        let _pool = ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create thread pool: {}", e))?;
        println!(
            "Using {} rayon threads (available parallelism: {})",
            num_threads,
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1)
        );

        let schema = context
            .current_schema
            .ok_or_else(|| anyhow::anyhow!("current schema not set"))?;
        let graph_container = schema
            .get_graph(&graph_name)?
            .ok_or_else(|| anyhow::anyhow!("graph named '{}' not found", graph_name))?;

        let graph_type_ref = graph_container.graph_type();
        let graph_container = graph_container
            .downcast_ref::<GraphContainer>()
            .ok_or_else(|| anyhow::anyhow!("graph '{}' container type mismatch", graph_name))?;

        let graph = match graph_container.graph_storage() {
            GraphStorage::Memory(g) => Arc::clone(g),
            _ => {
                return Err(anyhow::anyhow!("graph '{}' is not a memory graph", graph_name).into());
            }
        };

        let path_len = args[1]
            .try_as_int8()
            .expect("max length of path must be int8")
            .ok_or_else(|| anyhow::anyhow!("expecting int8 for path length"))?;
        let max_k = path_len as usize;

        // Clear any stale GCard data before rebuilding from scratch.
        // Old log entries reference the previous statistic schema and must be discarded.
        graph_container.clear_gcard_data();

        let edges = get_edges_from_catalog(graph_type_ref.as_ref())?;
        let schema_path = enumerate_all_paths_walks_in_schema(&edges, max_k);

        // println!("\n=== Schema Path Patterns ===");
        // for (len, patterns) in &schema_path {
        //     println!("\n长度: {}, 数量: {}", len, patterns.len());
        //     for pattern in patterns {
        //         let mut parts = Vec::new();
        //         for i in 0..pattern.es.len() {
        //             parts.push(pattern.vs[i].clone());
        //             parts.push(pattern.es[i].clone());
        //         }
        //         if let Some(last) = pattern.vs.last() {
        //             parts.push(last.clone());
        //         }
        //         println!("  {}", parts.join("--"));
        //     }
        // }

        let mode = args
            .get(2)
            .and_then(|a| a.to_u8().ok())
            .unwrap_or(0);

        let mut label_name_to_id: HashMap<String, LabelId> = HashMap::new();
        for name in graph_type_ref.label_names() {
            if let Ok(Some(id)) = graph_type_ref.get_label_id(&name) {
                label_name_to_id.insert(name.clone(), id);
                println!("{}-{}", name, id);
            }
        }

        let txn = graph
            .txn_manager()
            .begin_transaction(Serializable)
            .map_err(|e| anyhow::anyhow!("begin_transaction: {}", e))?;

        let cache: PatternDegCache = if mode == 1 {
            println!("Using neighbor-cached dependency-driven mode");
            compute_all_degrees_cached(
                graph.as_ref(),
                &txn,
                &edges,
                &label_name_to_id,
                &schema_path,
                max_k,
            )?
        } else {
            println!("Using layer-by-layer mode");
            let mut cache: PatternDegCache = HashMap::new();
            for len in 1..=max_k {
                let patterns = schema_path.get(&len).cloned().unwrap_or_default();
                if patterns.is_empty() {
                    continue;
                }
                if len == 1 {
                    compute_len1_degrees(
                        graph.as_ref(),
                        &txn,
                        &edges,
                        &label_name_to_id,
                        &patterns,
                        &mut cache,
                    )?;
                } else {
                    compute_len_ge2_degrees(
                        graph.as_ref(),
                        &txn,
                        &edges,
                        &label_name_to_id,
                        &patterns,
                        &mut cache,
                    )?;
                }
            }
            cache
        };

        txn.commit()
            .map_err(|e| anyhow::anyhow!("commit transaction: {}", e))?;

        let statistic = build_statistic_from_pattern_cache(&cache)?;
        let size = statistic.serialized_size();
        println!(
            "Statistic serialized size: {} bytes - {:.2} MB",
            size,
            size as f64 / 1024.0 / 1024.0
        );

        let degree_seq_graph_compressed = statistic
            .to_degree_seq_graph_compressed()
            .map_err(|e| anyhow::anyhow!("to_degree_seq_graph_compressed: {}", e))?;
        graph_container.set_degree_seq_graph_compressed(Arc::new(degree_seq_graph_compressed));
        graph_container.set_statistic(Arc::new(statistic));

        // Initialize (or reset) the lazy update log for the newly built statistics.
        let update_log = crate::procedures::gcard_query::update_log::new_log_arc(max_k);
        graph_container.set_gcard_update_log(update_log);

        Ok(vec![])
    })
}
