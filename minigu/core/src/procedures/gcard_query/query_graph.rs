use std::collections::{BinaryHeap, HashMap, HashSet};
use std::sync::Arc;

use minigu_catalog::provider::{GraphTypeProvider, PropertiesProvider};
use minigu_common::types::{EdgeId, VertexId};
use minigu_common::value::ScalarValue;
use minigu_context::graph::GraphContainer;
use minigu_storage::common::iterators::AdjacencyIteratorTrait;
use minigu_storage::common::model::edge::Edge;
use minigu_storage::common::model::vertex::Vertex;
use minigu_storage::tp::transaction::{IsolationLevel, MemTransaction};
use minigu_storage::tp::MemoryGraph;
use minigu_transaction::Transaction;
use minigu_transaction::manager::GraphTxnManager;
use rayon::prelude::*;

use crate::procedures::gcard_query::abs_graph::AbstractGraph;
use crate::procedures::gcard_query::catalog::{make_alt_key, DegreeSeqGraphCompressed};
use crate::procedures::gcard_query::degreepiecewise::Pcf;
use crate::procedures::gcard_query::error::{GCardError, GCardResult};
use crate::procedures::gcard_query::graph::{Endpoints, GraphSkeleton};
use crate::procedures::gcard_query::types::{
    AbstractEdge, CandidateTree, ComparisonOp, PredicateDef, PredicateId, PredicateLocation,
};
use crate::procedures::gcard_query::union_find::UnionFind;
use crate::procedures::gcard_query::PredicateApplyType;
use crate::procedures::gcard_query::PredicateApplyType::INNER;

#[derive(Debug, Clone)]
pub struct QueryEdge {
    pub id: EdgeId,
    pub label: String,
    pub src_vertex_id: VertexId,
    pub dst_vertex_id: VertexId,
    pub predicates: Vec<PredicateDef>,
}

impl Endpoints for QueryEdge {
    fn src(&self) -> VertexId {
        self.src_vertex_id
    }

    fn dst(&self) -> VertexId {
        self.dst_vertex_id
    }
}

pub struct QueryGraph {
    pub(crate) inner: GraphSkeleton<QueryEdge>,
    pub predicate_index: HashMap<PredicateId, (PredicateLocation, usize)>,
}

struct Path {
    pub start: VertexId,
    pub end: VertexId,
    pub vertices: Vec<VertexId>,
    pub edges: Vec<EdgeId>,
}

impl std::ops::Deref for QueryGraph {
    type Target = GraphSkeleton<QueryEdge>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl QueryGraph {
    pub fn new() -> Self {
        Self {
            inner: GraphSkeleton {
                vertices: HashMap::new(),
                edges: HashMap::new(),
                outgoing_edges: HashMap::new(),
                incoming_edges: HashMap::new(),
            },
            predicate_index: HashMap::new(),
        }
    }

    pub fn get_predicate_location(&self, predicate_id: PredicateId) -> Option<PredicateLocation> {
        self.predicate_index
            .get(&predicate_id)
            .map(|(location, _)| *location)
    }

    pub fn get_predicate(&self, predicate_id: PredicateId) -> Option<&PredicateDef> {
        self.predicate_index
            .get(&predicate_id)
            .and_then(|(location, idx)| match location {
                PredicateLocation::Vertex(vertex_id) => {
                    self.inner.vertices.get(vertex_id)?.predicates.get(*idx)
                }
                PredicateLocation::Edge(edge_id) => {
                    self.inner.edges.get(edge_id)?.predicates.get(*idx)
                }
            })
    }

    pub fn get_predicates(&self, location: PredicateLocation) -> Vec<&PredicateDef> {
        match location {
            PredicateLocation::Vertex(vertex_id) => self
                .inner
                .vertices
                .get(&vertex_id)
                .map(|v| v.predicates.iter().collect())
                .unwrap_or_default(),
            PredicateLocation::Edge(edge_id) => self
                .inner
                .edges
                .get(&edge_id)
                .map(|e| e.predicates.iter().collect())
                .unwrap_or_default(),
        }
    }

    pub fn score_single_edge(
        &self,
        edge: &QueryEdge,
        predicate_vertices: &HashSet<VertexId>,
        k_path: &HashSet<EdgeId>,
    ) -> u32 {
        if !edge.predicates.is_empty() {
            return 5;
        }
        let src_has_predicate = predicate_vertices.contains(&edge.src_vertex_id);
        let dst_has_predicate = predicate_vertices.contains(&edge.dst_vertex_id);
        if src_has_predicate && dst_has_predicate {
            return 4;
        }
        if k_path.contains(&edge.id) {
            return 3;
        }
        if k_path.contains(&edge.id) {
            return 2;
        }
        1
    }

    pub fn score_edges(&self, k: usize) -> HashMap<EdgeId, u32> {
        let predicate_vertices: HashSet<VertexId> = self
            .inner
            .vertices
            .iter()
            .filter(|(_, v)| !v.predicates.is_empty())
            .map(|(id, _)| *id)
            .collect();
        let k_path_edges = self.find_k_path_edges_between_predicates(&predicate_vertices, k);
        let mut scores = HashMap::new();
        for edge in self.inner.edges.values() {
            let score = self.score_single_edge(edge, &predicate_vertices, &k_path_edges);
            scores.insert(edge.id, score);
        }
        scores
    }

    pub fn find_k_path_edges_between_predicates(
        &self,
        predicate_vertices: &HashSet<VertexId>,
        k: usize,
    ) -> HashSet<EdgeId> {
        let mut k_path_edges = HashSet::new();
        if k == 0 {
            return k_path_edges;
        }

        let predicate_vec: Vec<VertexId> = predicate_vertices.iter().copied().collect();
        for i in 0..predicate_vec.len() {
            for j in (i + 1)..predicate_vec.len() {
                let start = predicate_vec[i];
                let end = predicate_vec[j];
                let paths = self.find_paths_with_length(start, end, k);
                for path in paths {
                    k_path_edges.extend(path);
                }
            }
        }
        k_path_edges
    }

    pub fn find_paths_with_length(
        &self,
        start: VertexId,
        end: VertexId,
        k: usize,
    ) -> Vec<HashSet<EdgeId>> {
        if k == 0 {
            if start == end {
                return vec![HashSet::new()];
            }
            return vec![];
        }
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((start, HashSet::new(), HashSet::from([start])));
        let mut result = Vec::new();
        while let Some((current, path_edges, visited)) = queue.pop_front() {
            if path_edges.len() == k {
                if current == end {
                    result.push(path_edges);
                }
                continue;
            }
            let neighbors = self.get_neighbors(current);
            for neighbor in neighbors {
                if visited.contains(&neighbor) {
                    continue;
                }
                let outgoing = self.get_outgoing_edges(current);
                let incoming = self.get_incoming_edges(current);
                let edge_opt = outgoing
                    .iter()
                    .find(|e| e.dst_vertex_id == neighbor)
                    .or_else(|| incoming.iter().find(|e| e.src_vertex_id == neighbor));
                if let Some(edge) = edge_opt {
                    let mut new_path_edges = path_edges.clone();
                    new_path_edges.insert(edge.id);
                    let mut new_visited = visited.clone();
                    new_visited.insert(neighbor);
                    queue.push_back((neighbor, new_path_edges, new_visited));
                }
            }
        }
        result
    }

    pub fn build_best_spanning_tree(&self, scores: &HashMap<EdgeId, u32>) -> Option<CandidateTree> {
        let mut edges_with_scores: Vec<_> = self
            .inner
            .edges
            .values()
            .map(|edge| {
                let score = scores.get(&edge.id).copied().unwrap_or(0);
                (edge.clone(), score)
            })
            .collect();
        edges_with_scores.sort_by(|a, b| b.1.cmp(&a.1));

        let mut selected_edges = HashSet::new();
        let mut uf = UnionFind::new();
        let mut total_score = 0;

        for (edge, score) in edges_with_scores {
            uf.make_set(edge.src_vertex_id);
            uf.make_set(edge.dst_vertex_id);

            if uf.union(edge.src_vertex_id, edge.dst_vertex_id) {
                selected_edges.insert(edge.id);
                total_score += score;
            }
        }

        if selected_edges.is_empty() {
            None
        } else {
            Some(CandidateTree {
                edge_ids: selected_edges,
                total_score,
            })
        }
    }

    fn build_subgraph_from_edges(&self, selected_edge_ids: &HashSet<EdgeId>) -> QueryGraph {
        let mut subgraph_vertices = HashMap::new();
        let mut subgraph_edges = HashMap::new();
        let mut subgraph_outgoing = HashMap::new();
        let mut subgraph_incoming = HashMap::new();

        for &edge_id in selected_edge_ids {
            if let Some(edge) = self.inner.edges.get(&edge_id) {
                subgraph_edges.insert(edge_id, edge.clone());

                if let Some(vertex) = self.inner.vertices.get(&edge.src_vertex_id) {
                    subgraph_vertices
                        .entry(edge.src_vertex_id)
                        .or_insert_with(|| vertex.clone());
                }

                if let Some(vertex) = self.inner.vertices.get(&edge.dst_vertex_id) {
                    subgraph_vertices
                        .entry(edge.dst_vertex_id)
                        .or_insert_with(|| vertex.clone());
                }

                subgraph_outgoing
                    .entry(edge.src_vertex_id)
                    .or_insert_with(Vec::new)
                    .push(edge_id);

                subgraph_incoming
                    .entry(edge.dst_vertex_id)
                    .or_insert_with(Vec::new)
                    .push(edge_id);
            }
        }

        QueryGraph {
            inner: GraphSkeleton {
                vertices: subgraph_vertices,
                edges: subgraph_edges,
                outgoing_edges: subgraph_outgoing,
                incoming_edges: subgraph_incoming,
            },
            predicate_index: self.predicate_index.clone(),
        }
    }

    /// 返回 (QueryGraph, total_score) 的列表，按分数从高到低。
    pub fn build_k_best_trees(
        &self,
        scores: &HashMap<EdgeId, u32>,
        k: usize,
    ) -> Vec<(QueryGraph, u32)> {
        if k == 0 {
            return Vec::new();
        }

        let first_tree = self.build_best_spanning_tree(scores);
        if first_tree.is_none() {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut seen_trees: Vec<HashSet<EdgeId>> = Vec::new();
        let mut candidates = BinaryHeap::new();

        let first_tree = first_tree.unwrap();
        let first_edge_set: HashSet<EdgeId> = first_tree.edge_ids.clone();

        seen_trees.push(first_edge_set.clone());
        candidates.push(first_tree);

        while result.len() < k && !candidates.is_empty() {
            let current = candidates.pop().unwrap();
            let current_edge_set = current.edge_ids.clone();

            let already_added = result.iter().any(|(g, _): &(QueryGraph, u32)| {
                let g_edges: HashSet<EdgeId> = g.inner.edges.keys().copied().collect();
                g_edges == current_edge_set
            });

            if already_added {
                continue;
            }

            result.push((
                self.build_subgraph_from_edges(&current_edge_set),
                current.total_score,
            ));
            if result.len() >= k {
                break;
            }

            let all_edge_ids: HashSet<EdgeId> = self.inner.edges.keys().copied().collect();
            let mut non_tree_edges: Vec<(EdgeId, u32)> = all_edge_ids
                .difference(&current.edge_ids)
                .map(|&eid| {
                    let score = scores.get(&eid).copied().unwrap_or(0);
                    (eid, score)
                })
                .collect();

            non_tree_edges.sort_by(|a, b| b.1.cmp(&a.1));

            for (new_edge_id, _new_edge_score) in non_tree_edges {
                if let Some(new_edge) = self.inner.edges.get(&new_edge_id) {
                    let src = new_edge.src_vertex_id;
                    let dst = new_edge.dst_vertex_id;

                    if let Some(path_edges) =
                        self.find_path_edges_in_tree(&current.edge_ids, src, dst)
                    {
                        let mut path_edges_with_scores: Vec<(EdgeId, u32)> = path_edges
                            .iter()
                            .map(|&eid| {
                                let score = scores.get(&eid).copied().unwrap_or(0);
                                (eid, score)
                            })
                            .collect();

                        path_edges_with_scores.sort_by(|a, b| a.1.cmp(&b.1));

                        for (edge_to_remove, _old_score) in path_edges_with_scores {
                            let mut new_edge_set = current.edge_ids.clone();
                            new_edge_set.remove(&edge_to_remove);
                            new_edge_set.insert(new_edge_id);

                            if seen_trees.iter().any(|s| s == &new_edge_set) {
                                continue;
                            }

                            if self.is_tree(&new_edge_set) {
                                let total_score: u32 = new_edge_set
                                    .iter()
                                    .map(|&eid| scores.get(&eid).copied().unwrap_or(0))
                                    .sum();

                                let candidate =
                                    crate::procedures::gcard_query::types::CandidateTree {
                                        edge_ids: new_edge_set.clone(),
                                        total_score,
                                    };

                                seen_trees.push(new_edge_set.clone());
                                candidates.push(candidate);
                            }
                        }
                    } else {
                        let mut new_edge_set = current.edge_ids.clone();
                        new_edge_set.insert(new_edge_id);

                        if seen_trees.contains(&new_edge_set) {
                            continue;
                        }

                        if self.is_tree(&new_edge_set) {
                            let total_score: u32 = new_edge_set
                                .iter()
                                .map(|&eid| scores.get(&eid).copied().unwrap_or(0))
                                .sum();

                            let candidate = crate::procedures::gcard_query::types::CandidateTree {
                                edge_ids: new_edge_set.clone(),
                                total_score,
                            };

                            seen_trees.push(new_edge_set.clone());
                            candidates.push(candidate);
                        }
                    }
                }
            }
        }

        result
    }

    pub fn build_abstract_graph(
        &self,
        k: usize,
        tree_num: usize,
        degree_seq_graph: &DegreeSeqGraphCompressed,
        graph_container: Option<&GraphContainer>,
        sample_size: usize,
        predicate_apply_type: &PredicateApplyType,
    ) -> GCardResult<Vec<(AbstractGraph, u32)>> {
        if !self.has_cycle() {
            let abstract_graph = self.build_abstract_graph_from_query_graph(
                self,
                k,
                degree_seq_graph,
                graph_container,
                sample_size,
                predicate_apply_type,
            )?;
            return Ok(vec![(abstract_graph, 0)]);
        }

        let scores = self.score_edges(k);

        let num_trees = tree_num.max(1);
        let trees_with_scores = self.build_k_best_trees(&scores, num_trees);

        let results: Vec<GCardResult<(AbstractGraph, u32)>> = trees_with_scores
            .par_iter()
            .map(|(tree, tree_score)| {
                self.build_abstract_graph_from_query_graph(
                    tree,
                    k,
                    degree_seq_graph,
                    graph_container,
                    sample_size,
                    predicate_apply_type,
                )
                .map(|ag| (ag, *tree_score))
            })
            .collect();

        let mut abstract_graphs = Vec::new();
        for r in results {
            abstract_graphs.push(r?);
        }
        Ok(abstract_graphs)
    }

    fn has_cycle(&self) -> bool {
        let mut uf = UnionFind::new();

        for edge in self.inner.edges.values() {
            uf.make_set(edge.src_vertex_id);
            uf.make_set(edge.dst_vertex_id);
            if !uf.union(edge.src_vertex_id, edge.dst_vertex_id) {
                return true;
            }
        }

        false
    }

    fn build_abstract_graph_from_query_graph(
        &self,
        query_graph: &QueryGraph,
        k: usize,
        degree_seq_graph: &DegreeSeqGraphCompressed,
        graph_container: Option<&GraphContainer>,
        sample_size: usize,
        predicate_apply_type: &PredicateApplyType,
    ) -> GCardResult<AbstractGraph> {
        let pivot_nodes = query_graph.find_pivot_nodes();

        let paths = query_graph.find_paths_from_pivots(&pivot_nodes);

        let mut abstract_graph = AbstractGraph::new();

        let mut next_edge_id: EdgeId = 1;
        for path in paths {
            let abstract_edges = query_graph.build_abstract_edges_for_path(&path, k)?;
            for mut abstract_edge in abstract_edges {
                self.fill_pcf_for_abstract_edge(
                    &mut abstract_edge,
                    degree_seq_graph,
                    graph_container,
                    sample_size,
                    predicate_apply_type,
                )?;

                let src_vertex = self.get_vertex(abstract_edge.src).unwrap();
                let dst_vertex = self.get_vertex(abstract_edge.dst).unwrap();
                if abstract_graph.get_vertex(abstract_edge.src).is_none() {
                    abstract_graph.add_vertex(src_vertex.clone());
                }
                if abstract_graph.get_vertex(abstract_edge.dst).is_none() {
                    abstract_graph.add_vertex(dst_vertex.clone());
                }
                abstract_graph.add_edge(next_edge_id, abstract_edge);
                next_edge_id += 1;
            }
        }

        Ok(abstract_graph)
    }

    fn fill_pcf_for_abstract_edge(
        &self,
        abstract_edge: &mut AbstractEdge,
        degree_seq_graph: &DegreeSeqGraphCompressed,
        graph_container: Option<&GraphContainer>,
        sample_size: usize,
        predicate_apply_type: &PredicateApplyType,
    ) -> GCardResult<()> {
        let mut node_labels = Vec::new();
        for &vertex_id in &abstract_edge.path_vertices {
            if let Some(vertex) = self.inner.vertices.get(&vertex_id) {
                node_labels.push(vertex.label.clone());
            } else {
                return Err(GCardError::VertexNotFound(format!(
                    "Vertex {} not found",
                    vertex_id
                )));
            }
        }

        let mut edge_labels = Vec::new();
        for &edge_id in &abstract_edge.original_edge_ids {
            if let Some(edge) = self.inner.edges.get(&edge_id) {
                edge_labels.push(edge.label.clone());
            } else {
                return Err(GCardError::EdgeNotFound(format!(
                    "Edge {} not found",
                    edge_id
                )));
            }
        }

        let alt_key = make_alt_key(&node_labels, &edge_labels);

        let src_label = self
            .inner
            .vertices
            .get(&abstract_edge.src)
            .ok_or_else(|| {
                GCardError::VertexNotFound(format!("Source vertex {} not found", abstract_edge.src))
            })?
            .label
            .clone();

        let dst_label = self
            .inner
            .vertices
            .get(&abstract_edge.dst)
            .ok_or_else(|| {
                GCardError::VertexNotFound(format!(
                    "Destination vertex {} not found",
                    abstract_edge.dst
                ))
            })?
            .label
            .clone();

        let mut src_pcf_func = degree_seq_graph.get_piece_func_by_path(&alt_key, &src_label);
        let mut dst_pcf_func = degree_seq_graph.get_piece_func_by_path(&alt_key, &dst_label);
        let mut output = String::new();
        for i in 0..node_labels.len() {
            output.push_str(&format!("{} -> ", node_labels[i]));
            if i < edge_labels.len() {
                output.push_str(&format!("{} ->", edge_labels[i]));
            }
        }
        abstract_edge.path_str = output;
        if let Some(graph) = graph_container {
            let selectivity = if !abstract_edge.predicates.is_empty()
                && (matches!(predicate_apply_type, PredicateApplyType::INNER)
                    || matches!(predicate_apply_type, PredicateApplyType::OUTER))
            {
                self.compute_selectivity_with_predicates(graph, abstract_edge, sample_size)?
            } else {
                1.0f64
            };

            abstract_edge.selectivity = selectivity;

            match predicate_apply_type {
                INNER => {
                    src_pcf_func = src_pcf_func.truncate_by_ratio(selectivity);
                    dst_pcf_func = dst_pcf_func.truncate_by_ratio(selectivity);
                }
                _ => {}
            }
        }

        abstract_edge.src_pcf = Arc::new(src_pcf_func);
        abstract_edge.dst_pcf = Arc::new(dst_pcf_func);

        Ok(())
    }

    fn compute_selectivity_with_predicates(
        &self,
        graph_container: &GraphContainer,
        abstract_edge: &AbstractEdge,
        sample_size: usize,
    ) -> GCardResult<f64> {
        let path_query = self.build_path_query(abstract_edge)?;

        let src_label = self
            .inner
            .vertices
            .get(&abstract_edge.src)
            .ok_or_else(|| {
                GCardError::VertexNotFound(format!("Source vertex {} not found", abstract_edge.src))
            })?
            .label
            .clone();

        let graph_type = graph_container.graph_type();
        let src_label_id = GraphTypeProvider::get_label_id(graph_type.as_ref(), &src_label)
            .map_err(|e| {
                GCardError::InvalidData(format!(
                    "Failed to get label_id for {}: {:?}",
                    src_label, e
                ))
            })?
            .ok_or_else(|| {
                GCardError::InvalidData(format!("Label {} not found in graph type", src_label))
            })?;

        let mem = match graph_container.graph_storage() {
            minigu_context::graph::GraphStorage::Memory(m) => Arc::clone(m),
        };
        let txn =
            GraphTxnManager::begin_transaction(mem.txn_manager(), IsolationLevel::Serializable)
                .map_err(|e| {
                    GCardError::InvalidState(format!("Failed to begin transaction: {:?}", e))
                })?;

        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let mut sampled_starts = Vec::new();
        let mut count = 0;

        {
            let it = txn.iter_vertices().filter(|v| {
                v.as_ref()
                    .map(|v| v.label_id == src_label_id)
                    .unwrap_or(false)
            });
            for vertex_result in it {
                let vertex = vertex_result.map_err(|e| {
                    GCardError::InvalidData(format!("Failed to get vertex: {:?}", e))
                })?;

                count += 1;
                if sampled_starts.len() < sample_size {
                    sampled_starts.push(vertex.vid());
                } else {
                    let j = rng.gen_range(0..count);
                    if j < sample_size {
                        sampled_starts[j] = vertex.vid();
                    }
                }
            }
        }

        if sampled_starts.is_empty() {
            return Ok(0.0);
        }

        let struct_count_min: usize = 300;
        let struct_count_max: usize = 4_000;

        let rel_eps: f64 = 0.10;
        let z_95: f64 = 1.96;
        let eps0: f64 = 1e-12;

        let mut struct_success_sample_count: usize = 0;
        let mut sum_struct_weight: f64 = 0.0;
        let mut sum_pred_weight: f64 = 0.0;

        let mut sum_struct_weight_seq: f64 = 0.0;
        let mut sum_pred_weight_seq: f64 = 0.0;
        let mut sum_cross_weight: f64 = 0.0;

        for start_vid in sampled_starts {
            let (struct_weight, pred_weight) =
                self.execute_path_query(&mem, &txn, &path_query, start_vid, graph_type.as_ref())?;

            if struct_weight > 0.0 {
                struct_success_sample_count += 1;
                sum_struct_weight += struct_weight;
                sum_pred_weight += pred_weight;
                sum_struct_weight_seq += struct_weight * struct_weight;
                sum_pred_weight_seq += pred_weight * pred_weight;
                sum_cross_weight += struct_weight * pred_weight;

                if struct_success_sample_count >= struct_count_max {
                    break;
                }

                if struct_success_sample_count < struct_count_min {
                    continue;
                }
                if struct_success_sample_count % 100 == 0 {
                    let k = struct_success_sample_count as f64;
                    let mean_struct_weight: f64 = sum_struct_weight / k;
                    let mean_pred_weight: f64 = sum_pred_weight / k;
                    let selectivity_estimate: f64 = mean_pred_weight / mean_struct_weight;
                    let denom: f64 = k - 1.0;
                    if denom <= 0.0 {
                        continue;
                    }
                    let var_pred_weight: f64 =
                        (sum_pred_weight_seq - k * mean_pred_weight * mean_pred_weight) / denom;
                    let var_struct_weight: f64 = (sum_struct_weight_seq
                        - k * mean_struct_weight * mean_struct_weight)
                        / denom;

                    let cov_pred_struct: f64 =
                        (sum_cross_weight - k * mean_pred_weight * mean_struct_weight) / denom;

                    let mu_x: f64 = mean_pred_weight;
                    let mu_y: f64 = mean_struct_weight;
                    let se_squared: f64 = (var_pred_weight / (mu_y * mu_y)
                        + (mu_x * mu_x) * var_struct_weight / (mu_y.powi(4))
                        - 2.0 * mu_x * cov_pred_struct / (mu_y.powi(3)))
                        / k;
                    if !se_squared.is_finite() || se_squared < 0.0 {
                        continue;
                    }
                    let standard_error: f64 = se_squared.sqrt();
                    let ci_half_width: f64 = z_95 * standard_error;
                    let relative_half_width: f64 =
                        ci_half_width / selectivity_estimate.abs().max(eps0);
                    if relative_half_width <= rel_eps {
                        break;
                    }
                }
            }
        }

        txn.commit().map_err(|e| {
            GCardError::InvalidState(format!("Failed to commit transaction: {:?}", e))
        })?;

        let selectivity = if sum_struct_weight > 0.0 {
            sum_pred_weight as f64 / sum_struct_weight as f64
        } else {
            0.0
        };

        Ok(selectivity)
    }

    fn build_path_query(&self, abstract_edge: &AbstractEdge) -> GCardResult<PathQuery> {
        let mut path_elements = Vec::new();
        let mut vertex_predicates: HashMap<usize, Vec<PredicateDef>> = HashMap::new();
        let mut edge_predicates: HashMap<usize, Vec<PredicateDef>> = HashMap::new();

        for (idx, &vertex_id) in abstract_edge.path_vertices.iter().enumerate() {
            let vertex = self.inner.vertices.get(&vertex_id).ok_or_else(|| {
                GCardError::VertexNotFound(format!("Vertex {} not found", vertex_id))
            })?;
            path_elements.push(PathElement::Vertex {
                label: vertex.label.clone(),
                position: idx * 2,
            });
            let mut v_preds = Vec::new();
            for pred in &abstract_edge.predicates {
                if pred.target == "vertex" && pred.id == vertex_id as u32 {
                    v_preds.push(pred.clone());
                }
            }
            if !v_preds.is_empty() {
                vertex_predicates.insert(idx * 2, v_preds);
            }

            if idx < abstract_edge.original_edge_ids.len() {
                let edge_id = abstract_edge.original_edge_ids[idx];
                let edge = self.inner.edges.get(&edge_id).ok_or_else(|| {
                    GCardError::EdgeNotFound(format!("Edge {} not found", edge_id))
                })?;
                let edge_parts: Vec<&str> = edge.label.split('_').collect();
                let direction = if !edge_parts.is_empty() {
                    let src_label = edge_parts[0];
                    let dst_label = edge_parts.last().copied().unwrap_or("");
                    if src_label == vertex.label {
                        EdgeDirection::Outgoing
                    } else if dst_label == vertex.label {
                        EdgeDirection::Incoming
                    } else {
                        EdgeDirection::Outgoing
                    }
                } else {
                    EdgeDirection::Outgoing
                };

                path_elements.push(PathElement::Edge {
                    label: edge.label.clone(),
                    position: idx * 2 + 1,
                    direction,
                });

                let mut e_preds = Vec::new();
                for pred in &abstract_edge.predicates {
                    if pred.target == "edge" && pred.id == edge_id as u32 {
                        e_preds.push(pred.clone());
                    }
                }
                if !e_preds.is_empty() {
                    edge_predicates.insert(idx * 2 + 1, e_preds);
                }
            }
        }

        Ok(PathQuery {
            path_elements,
            vertex_predicates,
            edge_predicates,
        })
    }

    /// 全采样：迭代维护两个集合，无递归
    /// - Set 1 (struct_matched): 结构性匹配的节点
    /// - Set 2 (pred_matched): 结构性匹配且 predicate 通过的节点
    /// 最终结果 = |Set 2| / |Set 1|
    fn execute_path_query(
        &self,
        mem: &Arc<MemoryGraph>,
        txn: &Arc<MemTransaction>,
        path_query: &PathQuery,
        start_vertex: VertexId,
        graph_type: &dyn GraphTypeProvider,
    ) -> GCardResult<(f64, f64)> {
        let path_elements = &path_query.path_elements;
        if path_elements.is_empty() {
            return Ok((1.0, 1.0));
        }

        let mut struct_matched = HashSet::from([start_vertex]);
        let mut pred_matched = HashSet::from([start_vertex]);

        for path_index in 0..path_elements.len() {
            let element = &path_elements[path_index];

            match element {
                PathElement::Vertex { label, position } => {
                    // 匹配点：检查 Set 2 中符合 vertex predicate 的，不满足的舍弃
                    let label_id = graph_type
                        .get_label_id(label)
                        .map_err(|e| {
                            GCardError::InvalidData(format!(
                                "Failed to get label_id for {}: {:?}",
                                label, e
                            ))
                        })?
                        .ok_or_else(|| {
                            GCardError::InvalidData(format!("Label {} not found", label))
                        })?;

                    pred_matched = pred_matched
                        .into_iter()
                        .filter(|&vid| {
                            let vertex = match mem.get_vertex(txn, vid) {
                                Ok(v) => v,
                                Err(_) => return false,
                            };
                            if vertex.label_id != label_id {
                                return false;
                            }
                            if let Some(predicates) = path_query.vertex_predicates.get(position) {
                                match self
                                    .evaluate_predicates_for_vertex(&vertex, predicates, graph_type)
                                {
                                    Ok(ok) => ok,
                                    Err(_) => false,
                                }
                            } else {
                                true
                            }
                        })
                        .collect();
                }
                PathElement::Edge {
                    label,
                    position,
                    direction,
                } => {
                    let edge_label_id = graph_type
                        .get_label_id(label)
                        .map_err(|e| {
                            GCardError::InvalidData(format!(
                                "Failed to get label_id for {}: {:?}",
                                label, e
                            ))
                        })?
                        .ok_or_else(|| {
                            GCardError::InvalidData(format!("Label {} not found", label))
                        })?;

                    let mut new_struct_matched = HashSet::new();
                    let mut new_pred_matched = HashSet::new();

                    for &from_vid in &struct_matched {
                        let mut adj_iter = match direction {
                            EdgeDirection::Outgoing => txn.iter_adjacency_outgoing(from_vid),
                            EdgeDirection::Incoming => txn.iter_adjacency_incoming(from_vid),
                        };
                        adj_iter = AdjacencyIteratorTrait::filter(adj_iter, move |neighbor| {
                            neighbor.label_id() == edge_label_id
                        });

                        for neighbor_result in adj_iter {
                            let neighbor = match neighbor_result {
                                Ok(n) => n,
                                Err(_) => continue,
                            };
                            let to_vid = neighbor.neighbor_id();
                            new_struct_matched.insert(to_vid);

                            if pred_matched.contains(&from_vid) {
                                let edge_predicate_satisfied = if let Some(predicates) =
                                    path_query.edge_predicates.get(position)
                                {
                                    match mem.get_edge(txn, neighbor.eid()) {
                                        Ok(edge) => self
                                            .evaluate_predicates_for_edge(
                                                &edge, predicates, graph_type,
                                            )
                                            .unwrap_or(false),
                                        Err(_) => false,
                                    }
                                } else {
                                    true
                                };
                                if edge_predicate_satisfied {
                                    new_pred_matched.insert(to_vid);
                                }
                            }
                        }
                    }

                    struct_matched = new_struct_matched;
                    pred_matched = new_pred_matched;

                    if struct_matched.is_empty() {
                        return Ok((0.0, 0.0));
                    }
                }
            }
        }

        let struct_count = struct_matched.len() as f64;
        let pred_count = pred_matched.len() as f64;
        Ok((struct_count, pred_count))
    }

    fn evaluate_predicates_for_vertex(
        &self,
        vertex: &Vertex,
        predicates: &[PredicateDef],
        graph_type: &dyn GraphTypeProvider,
    ) -> GCardResult<bool> {
        let label_set = minigu_catalog::label_set::LabelSet::from_iter(vec![vertex.label_id]);
        let vertex_type = graph_type
            .get_vertex_type(&label_set)
            .map_err(|e| GCardError::InvalidData(format!("Failed to get vertex type: {:?}", e)))?
            .ok_or_else(|| {
                GCardError::InvalidData(format!(
                    "Vertex type not found for label_id {}",
                    vertex.label_id
                ))
            })?;

        for predicate in predicates {
            let (prop_id, _) = vertex_type
                .get_property(&predicate.property)
                .map_err(|e| {
                    GCardError::InvalidData(format!(
                        "Failed to get property {}: {:?}",
                        predicate.property, e
                    ))
                })?
                .ok_or_else(|| {
                    GCardError::InvalidData(format!("Property {} not found", predicate.property))
                })?;

            let prop_value = vertex.properties().get(prop_id as usize).ok_or_else(|| {
                GCardError::InvalidData(format!("Property index {} out of range", prop_id))
            })?;

            if !self.compare_values(prop_value, &predicate.op, &predicate.value)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn evaluate_predicates_for_edge(
        &self,
        edge: &Edge,
        predicates: &[PredicateDef],
        graph_type: &dyn GraphTypeProvider,
    ) -> GCardResult<bool> {
        let label_set = minigu_catalog::label_set::LabelSet::from_iter(vec![edge.label_id]);
        let edge_type = graph_type
            .get_edge_type(&label_set)
            .map_err(|e| GCardError::InvalidData(format!("Failed to get edge type: {:?}", e)))?
            .ok_or_else(|| {
                GCardError::InvalidData(format!(
                    "Edge type not found for label_id {}",
                    edge.label_id
                ))
            })?;

        for predicate in predicates {
            let (prop_id, _) = edge_type
                .get_property(&predicate.property)
                .map_err(|e| {
                    GCardError::InvalidData(format!(
                        "Failed to get property {}: {:?}",
                        predicate.property, e
                    ))
                })?
                .ok_or_else(|| {
                    GCardError::InvalidData(format!("Property {} not found", predicate.property))
                })?;

            let prop_value = edge.properties.get(prop_id as usize).ok_or_else(|| {
                GCardError::InvalidData(format!("Property index {} out of range", prop_id))
            })?;

            if !self.compare_values(prop_value, &predicate.op, &predicate.value)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn compare_values(
        &self,
        value: &ScalarValue,
        op: &ComparisonOp,
        expected: &ScalarValue,
    ) -> GCardResult<bool> {
        use ComparisonOp::*;

        match op {
            Eq => Ok(value == expected),
            Ne => Ok(value != expected),
            Gt => self.compare_ordered(value, expected, |a, b| {
                self.partial_cmp_scalar(a, b)
                    .map(|ord| ord == std::cmp::Ordering::Greater)
            }),
            Ge => self.compare_ordered(value, expected, |a, b| {
                self.partial_cmp_scalar(a, b).map(|ord| {
                    ord == std::cmp::Ordering::Greater || ord == std::cmp::Ordering::Equal
                })
            }),
            Lt => self.compare_ordered(value, expected, |a, b| {
                self.partial_cmp_scalar(a, b)
                    .map(|ord| ord == std::cmp::Ordering::Less)
            }),
            Le => self.compare_ordered(value, expected, |a, b| {
                self.partial_cmp_scalar(a, b)
                    .map(|ord| ord == std::cmp::Ordering::Less || ord == std::cmp::Ordering::Equal)
            }),
        }
    }

    fn compare_ordered<F>(
        &self,
        value: &ScalarValue,
        expected: &ScalarValue,
        cmp: F,
    ) -> GCardResult<bool>
    where
        F: FnOnce(&ScalarValue, &ScalarValue) -> Option<bool>,
    {
        cmp(value, expected).ok_or_else(|| {
            GCardError::InvalidData(format!(
                "Cannot compare values: {:?} and {:?}",
                value, expected
            ))
        })
    }

    fn partial_cmp_scalar(&self, a: &ScalarValue, b: &ScalarValue) -> Option<std::cmp::Ordering> {
        use ScalarValue::*;

        match (a, b) {
            (Int8(Some(a_val)), Int8(Some(b_val))) => Some(a_val.cmp(b_val)),
            (Int16(Some(a_val)), Int16(Some(b_val))) => Some(a_val.cmp(b_val)),
            (Int32(Some(a_val)), Int32(Some(b_val))) => Some(a_val.cmp(b_val)),
            (Int64(Some(a_val)), Int64(Some(b_val))) => Some(a_val.cmp(b_val)),
            (UInt8(Some(a_val)), UInt8(Some(b_val))) => Some(a_val.cmp(b_val)),
            (UInt16(Some(a_val)), UInt16(Some(b_val))) => Some(a_val.cmp(b_val)),
            (UInt32(Some(a_val)), UInt32(Some(b_val))) => Some(a_val.cmp(b_val)),
            (UInt64(Some(a_val)), UInt64(Some(b_val))) => Some(a_val.cmp(b_val)),
            (Float32(Some(a_val)), Float32(Some(b_val))) => Some(a_val.cmp(b_val)),
            (Float64(Some(a_val)), Float64(Some(b_val))) => Some(a_val.cmp(b_val)),
            (String(Some(a_val)), String(Some(b_val))) => Some(a_val.cmp(b_val)),
            (Boolean(Some(a_val)), Boolean(Some(b_val))) => Some(a_val.cmp(b_val)),
            // 跨类型比较：尝试转换为f64
            _ => {
                let a_f64 = self.to_f64_opt(a);
                let b_f64 = self.to_f64_opt(b);
                match (a_f64, b_f64) {
                    (Some(a_f), Some(b_f)) => {
                        use ordered_float::OrderedFloat;
                        Some(OrderedFloat(a_f).cmp(&OrderedFloat(b_f)))
                    }
                    _ => None,
                }
            }
        }
    }

    fn to_f64_opt(&self, value: &ScalarValue) -> Option<f64> {
        use ScalarValue::*;
        match value {
            Int8(Some(v)) => Some(*v as f64),
            Int16(Some(v)) => Some(*v as f64),
            Int32(Some(v)) => Some(*v as f64),
            Int64(Some(v)) => Some(*v as f64),
            UInt8(Some(v)) => Some(*v as f64),
            UInt16(Some(v)) => Some(*v as f64),
            UInt32(Some(v)) => Some(*v as f64),
            UInt64(Some(v)) => Some(*v as f64),
            Float32(Some(v)) => Some(v.into_inner() as f64),
            Float64(Some(v)) => Some(v.into_inner()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct PathQuery {
    path_elements: Vec<PathElement>,
    vertex_predicates: HashMap<usize, Vec<PredicateDef>>,
    edge_predicates: HashMap<usize, Vec<PredicateDef>>,
}

#[derive(Debug, Clone)]
enum EdgeDirection {
    Outgoing,
    Incoming,
}

#[derive(Debug, Clone)]
enum PathElement {
    Vertex {
        label: String,
        position: usize,
    },
    Edge {
        label: String,
        position: usize,
        direction: EdgeDirection,
    },
}

impl QueryGraph {
    fn find_paths_from_pivots(&self, pivot_nodes: &HashSet<VertexId>) -> Vec<Path> {
        if pivot_nodes.is_empty() {
            return self.build_path_from_entire_graph();
        }

        let mut paths = Vec::new();
        let mut visited_edges = HashSet::new();

        for &pivot in pivot_nodes {
            let neighbors = self.get_neighbors(pivot);

            for neighbor in neighbors {
                let outgoing = self.get_outgoing_edges(pivot);
                let incoming = self.get_incoming_edges(pivot);
                let edge_opt = outgoing
                    .iter()
                    .find(|e| e.dst_vertex_id == neighbor)
                    .or_else(|| incoming.iter().find(|e| e.src_vertex_id == neighbor));

                if let Some(edge) = edge_opt {
                    let edge_key = if pivot < neighbor {
                        (pivot, neighbor, edge.id)
                    } else {
                        (neighbor, pivot, edge.id)
                    };

                    if visited_edges.contains(&edge_key) {
                        continue;
                    }
                    visited_edges.insert(edge_key);

                    if let Some(path) = self.traverse_path(pivot, neighbor, edge.id, pivot_nodes) {
                        paths.push(path);
                    }
                }
            }
        }

        paths
    }

    fn build_path_from_entire_graph(&self) -> Vec<Path> {
        if self.inner.vertices.is_empty() {
            return Vec::new();
        }

        let start_vertex = self
            .inner
            .vertices
            .keys()
            .filter(|&&vid| self.get_degree(vid) == 1)
            .min()
            .copied()
            .or_else(|| self.inner.vertices.keys().min().copied())
            .expect("Graph should have at least one vertex");

        let mut vertices = vec![start_vertex];
        let mut edges = Vec::new();
        let mut current_vertex = start_vertex;

        loop {
            let neighbors = self.get_neighbors(current_vertex);
            let mut next_vertex = None;
            let mut next_edge_id = None;
            for neighbor in neighbors {
                if vertices.contains(&neighbor) {
                    continue;
                }

                let outgoing = self.get_outgoing_edges(current_vertex);
                let incoming = self.get_incoming_edges(current_vertex);
                let edge_opt = outgoing
                    .iter()
                    .find(|e| e.dst_vertex_id == neighbor)
                    .or_else(|| incoming.iter().find(|e| e.src_vertex_id == neighbor));

                if let Some(edge) = edge_opt {
                    next_vertex = Some(neighbor);
                    next_edge_id = Some(edge.id);
                    break;
                }
            }

            if let (Some(next_v), Some(next_e)) = (next_vertex, next_edge_id) {
                vertices.push(next_v);
                edges.push(next_e);
                current_vertex = next_v;
            } else {
                break;
            }
        }
        vec![Path {
            start: vertices[0],
            end: vertices[vertices.len() - 1],
            vertices,
            edges,
        }]
    }

    fn traverse_path(
        &self,
        start_pivot: VertexId,
        current: VertexId,
        first_edge_id: EdgeId,
        pivot_nodes: &HashSet<VertexId>,
    ) -> Option<Path> {
        let mut vertices = vec![start_pivot, current];
        let mut edges = vec![first_edge_id];
        let mut visited_vertices = HashSet::from([start_pivot, current]);
        let mut current_vertex = current;

        loop {
            if pivot_nodes.contains(&current_vertex) {
                return Some(Path {
                    start: start_pivot,
                    end: current_vertex,
                    vertices,
                    edges,
                });
            }

            if self.get_degree(current_vertex) == 1 {
                return Some(Path {
                    start: start_pivot,
                    end: current_vertex,
                    vertices,
                    edges,
                });
            }

            let neighbors = self.get_neighbors(current_vertex);
            let mut next_vertex = None;
            let mut next_edge_id = None;

            for neighbor in neighbors {
                if visited_vertices.contains(&neighbor) {
                    continue;
                }

                let outgoing = self.get_outgoing_edges(current_vertex);
                let incoming = self.get_incoming_edges(current_vertex);
                let edge_opt = outgoing
                    .iter()
                    .find(|e| e.dst_vertex_id == neighbor)
                    .or_else(|| incoming.iter().find(|e| e.src_vertex_id == neighbor));

                if let Some(edge) = edge_opt {
                    next_vertex = Some(neighbor);
                    next_edge_id = Some(edge.id);
                    break;
                }
            }

            if let (Some(next_v), Some(next_e)) = (next_vertex, next_edge_id) {
                vertices.push(next_v);
                edges.push(next_e);
                visited_vertices.insert(next_v);
                current_vertex = next_v;
            } else {
                return Some(Path {
                    start: start_pivot,
                    end: current_vertex,
                    vertices,
                    edges,
                });
            }
        }
    }

    fn build_abstract_edges_for_path(
        &self,
        path: &Path,
        k: usize,
    ) -> GCardResult<Vec<AbstractEdge>> {
        let l = path.edges.len();
        if l == 0 {
            return Ok(Vec::new());
        }

        let num_abstract_edges = (l as f64 / k as f64).ceil() as usize;

        if l % k == 0 {
            return self.build_abstract_edges_even(path, k);
        }

        self.build_abstract_edges_optimal(path, k, num_abstract_edges)
    }

    fn build_abstract_edges_even(&self, path: &Path, k: usize) -> GCardResult<Vec<AbstractEdge>> {
        let mut abstract_edges = Vec::new();
        let mut edge_idx = 0;

        while edge_idx < path.edges.len() {
            let end_idx = (edge_idx + k).min(path.edges.len());
            let abstract_edge_edges = &path.edges[edge_idx..end_idx];

            let mut predicates = Vec::new();
            for &edge_id in abstract_edge_edges {
                if let Some(edge) = self.inner.edges.get(&edge_id) {
                    predicates.extend(edge.predicates.clone());
                }
            }

            let src_vertex_idx = edge_idx;
            let dst_vertex_idx = if end_idx < path.vertices.len() {
                end_idx
            } else {
                path.vertices.len() - 1
            };

            let src = path.vertices[src_vertex_idx];
            let dst = path.vertices[dst_vertex_idx];

            let path_vertices = path.vertices[src_vertex_idx..=dst_vertex_idx].to_vec();
            for &vertex_id in &path_vertices {
                if let Some(vertex) = self.inner.vertices.get(&vertex_id) {
                    predicates.extend(vertex.predicates.clone());
                }
            }

            let src_pcf = Arc::new(Pcf::empty());
            let dst_pcf = Arc::new(Pcf::empty());

            abstract_edges.push(AbstractEdge {
                src,
                dst,
                src_pcf,
                dst_pcf,
                predicates,
                original_edge_ids: abstract_edge_edges.to_vec(),
                path_vertices,
                selectivity: 1.0,
                path_str: String::new(),
            });

            edge_idx += k;
        }

        Ok(abstract_edges)
    }

    fn build_abstract_edges_optimal(
        &self,
        path: &Path,
        k: usize,
        num_abstract_edges: usize,
    ) -> GCardResult<Vec<AbstractEdge>> {
        let l = path.edges.len();
        let short_size = l.saturating_sub((num_abstract_edges - 1) * k);

        let mut best_solution: Option<Vec<usize>> = None;
        let mut min_predicate_count = usize::MAX;

        for short_pos in 0..num_abstract_edges {
            let mut solution = vec![k; num_abstract_edges];
            solution[short_pos] = short_size;
            let short_start = short_pos * k;
            let short_end = short_start + short_size;
            let short_predicate_count =
                self.count_predicates_in_range(path, short_start, short_end);

            if short_predicate_count < min_predicate_count {
                min_predicate_count = short_predicate_count;
                best_solution = Some(solution);
            }
        }

        let solution = best_solution.unwrap_or_else(|| {
            let mut sol = vec![k; num_abstract_edges - 1];
            sol.push(l - (num_abstract_edges - 1) * k);
            sol
        });

        let mut abstract_edges = Vec::new();
        let mut edge_idx = 0;

        for &edge_count in &solution {
            let end_idx = (edge_idx + edge_count).min(path.edges.len());
            let abstract_edge_edges = &path.edges[edge_idx..end_idx];

            let mut predicates = Vec::new();
            for &edge_id in abstract_edge_edges {
                if let Some(edge) = self.inner.edges.get(&edge_id) {
                    predicates.extend(edge.predicates.clone());
                }
            }

            let src_vertex_idx = edge_idx;
            let dst_vertex_idx = if end_idx < path.vertices.len() {
                end_idx
            } else {
                path.vertices.len() - 1
            };

            let src = path.vertices[src_vertex_idx];
            let dst = path.vertices[dst_vertex_idx];

            let path_vertices = path.vertices[src_vertex_idx..=dst_vertex_idx].to_vec();
            for &vertex_id in &path_vertices {
                if let Some(vertex) = self.inner.vertices.get(&vertex_id) {
                    predicates.extend(vertex.predicates.clone());
                }
            }

            let src_pcf = Arc::new(Pcf::empty());
            let dst_pcf = Arc::new(Pcf::empty());

            abstract_edges.push(AbstractEdge {
                src,
                dst,
                src_pcf,
                dst_pcf,
                predicates,
                original_edge_ids: abstract_edge_edges.to_vec(),
                path_vertices,
                selectivity: 1.0,
                path_str: String::new(),
            });

            edge_idx += edge_count;
        }

        Ok(abstract_edges)
    }

    fn find_path_edges_in_tree(
        &self,
        tree_edges: &HashSet<EdgeId>,
        src: VertexId,
        dst: VertexId,
    ) -> Option<HashSet<EdgeId>> {
        if src == dst {
            return Some(HashSet::new());
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((src, HashSet::new()));
        let mut visited = HashSet::new();
        visited.insert(src);

        while let Some((current, path)) = queue.pop_front() {
            if current == dst {
                return Some(path);
            }

            for edge in self.get_outgoing_edges(current) {
                if tree_edges.contains(&edge.id) && !visited.contains(&edge.dst()) {
                    let mut new_path = path.clone();
                    new_path.insert(edge.id);
                    visited.insert(edge.dst());
                    queue.push_back((edge.dst(), new_path));
                }
            }

            for edge in self.get_incoming_edges(current) {
                if tree_edges.contains(&edge.id) && !visited.contains(&edge.src()) {
                    let mut new_path = path.clone();
                    new_path.insert(edge.id);
                    visited.insert(edge.src());
                    queue.push_back((edge.src(), new_path));
                }
            }
        }

        None
    }

    fn is_tree(&self, edge_set: &HashSet<EdgeId>) -> bool {
        if edge_set.is_empty() {
            return false;
        }

        let mut uf = UnionFind::new();
        let mut vertex_set = HashSet::new();

        for &edge_id in edge_set {
            if let Some(edge) = self.inner.edges.get(&edge_id) {
                uf.make_set(edge.src_vertex_id);
                uf.make_set(edge.dst_vertex_id);
                vertex_set.insert(edge.src_vertex_id);
                vertex_set.insert(edge.dst_vertex_id);

                if !uf.union(edge.src_vertex_id, edge.dst_vertex_id) {
                    return false;
                }
            }
        }

        edge_set.len() == vertex_set.len() - 1
    }

    fn count_predicates_in_range(
        &self,
        path: &Path,
        start_edge_idx: usize,
        end_edge_idx: usize,
    ) -> usize {
        let mut count = 0;

        for &edge_id in &path.edges[start_edge_idx..end_edge_idx] {
            if let Some(edge) = self.edges.get(&edge_id) {
                count += edge.predicates.len();
            }
        }

        let start_vertex_idx = start_edge_idx;
        let end_vertex_idx = if end_edge_idx < path.vertices.len() {
            end_edge_idx
        } else {
            path.vertices.len() - 1
        };

        for &vertex_id in &path.vertices[start_vertex_idx..=end_vertex_idx] {
            if let Some(vertex) = self.inner.vertices.get(&vertex_id) {
                count += vertex.predicates.len();
            }
        }

        count
    }
}
