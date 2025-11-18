use std::collections::{HashMap, HashSet, VecDeque};

use minigu_common::types::{EdgeId, VertexId};

use crate::procedures::gcard_query::degreepiecewise::{alpha, alpha_refs, beta_right, Pcf};
use crate::procedures::gcard_query::error::{GCardError, GCardResult};
use crate::procedures::gcard_query::graph::{Endpoints, GraphSkeleton};
use crate::procedures::gcard_query::types::{AbstractEdge, QueryVertex};

impl Endpoints for AbstractEdge {
    fn src(&self) -> VertexId {
        self.src
    }

    fn dst(&self) -> VertexId {
        self.dst
    }
}

pub type AbstractGraph = GraphSkeleton<AbstractEdge>;

impl GraphSkeleton<AbstractEdge> {
    pub fn new() -> Self {
        Self {
            vertices: std::collections::HashMap::new(),
            edges: std::collections::HashMap::new(),
            outgoing_edges: std::collections::HashMap::new(),
            incoming_edges: std::collections::HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: QueryVertex) {
        self.vertices.insert(vertex.id, vertex);
    }

    pub fn add_edge(&mut self, edge_id: EdgeId, edge: AbstractEdge) {
        let (src, dst) = (edge.src, edge.dst);
        self.edges.insert(edge_id, edge);
        self.outgoing_edges.entry(src).or_default().push(edge_id);
        self.incoming_edges.entry(dst).or_default().push(edge_id);
    }

    pub fn remove_edge(&mut self, edge_id: EdgeId) -> Option<AbstractEdge> {
        let edge = self.edges.remove(&edge_id)?;
        let (src, dst) = (edge.src, edge.dst);
        if let Some(v) = self.outgoing_edges.get_mut(&src) {
            v.retain(|&id| id != edge_id);
        }
        if let Some(v) = self.incoming_edges.get_mut(&dst) {
            v.retain(|&id| id != edge_id);
        }
        Some(edge)
    }

    pub fn remove_vertex(&mut self, vertex_id: VertexId) -> Option<QueryVertex> {
        self.vertices.remove(&vertex_id)
    }

    pub fn get_topological_generations(&self, root: VertexId) -> HashMap<VertexId, usize> {
        let mut generations = HashMap::new();
        let mut queue = VecDeque::from([(root, 0usize)]);
        let mut seen = HashSet::from([root]);

        while let Some((v, g)) = queue.pop_front() {
            generations.insert(v, g);
            for neighbor in self.get_neighbors(v) {
                if seen.insert(neighbor) {
                    queue.push_back((neighbor, g + 1));
                }
            }
        }
        generations
    }

    pub fn pick_root(&self) -> Option<VertexId> {
        let vertices: Vec<VertexId> = self.vertices.keys().copied().collect();
        if vertices.is_empty() {
            return None;
        }

        let mut best_root = vertices[0];
        let mut min_max_gen = usize::MAX;

        for &candidate in &vertices {
            let generations = self.get_topological_generations(candidate);
            let max_gen = generations.values().copied().max().unwrap_or(0);
            if max_gen < min_max_gen {
                min_max_gen = max_gen;
                best_root = candidate;
            }
        }
        Some(best_root)
    }

    fn get_parent_edge(
        &self,
        v: VertexId,
        root: VertexId,
        generations: &HashMap<VertexId, usize>,
    ) -> Option<(VertexId, EdgeId)> {
        let cur_gen = *generations.get(&v)?;
        if cur_gen == 0 {
            return None;
        }
        let parent_gen = cur_gen - 1;
        for edge_id in self.get_neighbor_edges(v) {
            let edge = self.edges.get(&edge_id)?;
            let neighbor = if edge.src == v { edge.dst } else { edge.src };
            if generations.get(&neighbor) == Some(&parent_gen) {
                return Some((neighbor, edge_id));
            }
        }
        None
    }

    fn get_children_edges(
        &self,
        v: VertexId,
        generations: &HashMap<VertexId, usize>,
    ) -> Vec<(VertexId, EdgeId)> {
        let cur_gen = match generations.get(&v) {
            Some(&g) => g,
            None => return vec![],
        };
        let child_gen = cur_gen + 1;
        let mut children = Vec::new();
        for edge_id in self.get_neighbor_edges(v) {
            if let Some(edge) = self.edges.get(&edge_id) {
                let neighbor = if edge.src == v { edge.dst } else { edge.src };
                if generations.get(&neighbor) == Some(&child_gen) {
                    children.push((neighbor, edge_id));
                }
            }
        }
        children
    }

    fn get_edge_pcf_at_vertex(&self, edge_id: EdgeId, vertex_id: VertexId) -> Pcf {
        if let Some(edge) = self.edges.get(&edge_id) {
            if edge.src == vertex_id {
                edge.src_pcf.as_ref().clone()
            } else {
                edge.dst_pcf.as_ref().clone()
            }
        } else {
            Pcf::empty()
        }
    }

    pub fn get_es(&mut self) -> GCardResult<f64> {
        if self.vertices.len() <= 1 {
            return Err(GCardError::InvalidState(
                "AbstractGraph must have at least 2 vertices".to_string(),
            ));
        }

        let root = self
            .pick_root()
            .ok_or_else(|| GCardError::InvalidState("Empty graph".to_string()))?;

        let generations = self.get_topological_generations(root);
        let max_gen = generations.values().copied().max().unwrap_or(0);

        let mut child_vertex_pcf: HashMap<VertexId, Pcf> = HashMap::new();

        for cur_gen in (0..=max_gen).rev() {
            let cur_vertices: Vec<VertexId> = generations
                .iter()
                .filter_map(|(v, g)| (*g == cur_gen).then_some(*v))
                .collect();

            for v in cur_vertices {
                let parent_opt = self.get_parent_edge(v, root, &generations);
                let children = self.get_children_edges(v, &generations);

                let parent_to_vertex_pcf = if let Some((parent, parent_edge_id)) = parent_opt {
                    self.get_edge_pcf_at_vertex(parent_edge_id, parent)
                } else {
                    Pcf::empty()
                };

                let vertex_to_parent_pcf = if let Some((_parent, parent_edge_id)) = parent_opt {
                    self.get_edge_pcf_at_vertex(parent_edge_id, v)
                } else {
                    Pcf::empty()
                };

                let  result = if !children.is_empty() {
                    let child_pcfs: Vec<Pcf> = children
                        .iter()
                        .filter_map(|(child_id, _)| child_vertex_pcf.get(child_id).cloned())
                        .collect();
                    let multiplied_child_pcf = if child_pcfs.is_empty() {
                        Pcf::empty()
                    } else {
                        let refs: Vec<&Pcf> = child_pcfs.iter().collect();
                        alpha_refs(&refs)
                    };
                    let projected = if cur_gen > 0 {
                        beta_right(
                            &multiplied_child_pcf,
                            &vertex_to_parent_pcf,
                            &parent_to_vertex_pcf,
                        )
                    } else {
                        multiplied_child_pcf
                    };
                    if cur_gen > 0 {
                        let pcf_refs: Vec<&Pcf> = vec![&projected, &parent_to_vertex_pcf];
                        alpha_refs(&pcf_refs)
                    } else {
                        projected
                    }
                } else {
                    parent_to_vertex_pcf
                };

                if cur_gen > 0 {
                    child_vertex_pcf.insert(v, result);
                } else {
                    return Ok(result.get_num_rows());
                }
            }
        }

        Err(GCardError::InvalidState(
            "Reduction did not reach root".to_string(),
        ))
    }

    pub fn get_selectivity(&self) -> f64 {
        let mut selectivity = 1.0;
        for edge in self.edges.values() {
            selectivity *= edge.selectivity;
        }
        selectivity
    }

    // pub fn get_es_(&mut self) -> GCardResult<f64> {
    //     if self.vertices.len() == 3 && self.edges.len() == 2 {
    //         let degree2_nodes: Vec<VertexId> = self
    //             .vertices
    //             .keys()
    //             .filter(|&vid| self.get_degree(*vid) == 2)
    //             .copied()
    //             .collect();
    //         assert_eq!(degree2_nodes.len(), 1);
    //         return self.get_es_from_flat_single_degree2_path(degree2_nodes[0]);
    //     }
    //     self.reduce_degree2_nodes();
    //     let pcf = self.reduce_pivot_nodes()?;
    //     Ok(pcf.get_num_rows())
    // }
    //
    // fn reduce_degree2_nodes(&mut self) -> GCardResult<()> {
    //     loop {
    //         let degree2_nodes: Vec<VertexId> = self
    //             .vertices
    //             .keys()
    //             .filter(|&vid| self.get_degree(*vid) == 2)
    //             .copied()
    //             .collect();
    //
    //         if degree2_nodes.is_empty() {
    //             break;
    //         }
    //
    //         for node_id in degree2_nodes {
    //             if self.get_degree(node_id) != 2 {
    //                 continue;
    //             }
    //
    //             self.reduce_single_degree2_node(node_id)?;
    //         }
    //     }
    //
    //     Ok(())
    // }
    //
    // fn reduce_single_degree2_node(&mut self, node_id: VertexId) -> GCardResult<()> {
    //     let mut edge_ids = self.get_neighbor_edges(node_id);
    //
    //     if edge_ids.len() != 2 {
    //         return Ok(());
    //     }
    //
    //     let e1_id = edge_ids[0];
    //     let e2_id = edge_ids[1];
    //
    //     let e1 = self
    //         .edges
    //         .get(&e1_id)
    //         .ok_or_else(|| GCardError::EdgeNotFound(e1_id.to_string()))?
    //         .clone();
    //     let e2 = self
    //         .edges
    //         .get(&e2_id)
    //         .ok_or_else(|| GCardError::EdgeNotFound(e2_id.to_string()))?
    //         .clone();
    //     println!(
    //         "merge \n {}\n src:{} \n dst: {} \n {} \n src:{}\n dst:{} \n",
    //         e1.path_str, e1.src_pcf, e1.dst_pcf, e2.path_str, e2.src_pcf, e2.dst_pcf
    //     );
    //
    //     let (new_src, new_dst, rx, ry, sy, sz) = if e1.dst == node_id && e2.src == node_id {
    //         (
    //             e1.src,
    //             e2.dst,
    //             e1.dst_pcf.clone(),
    //             e1.src_pcf.clone(),
    //             e2.dst_pcf.clone(),
    //             e2.src_pcf.clone(),
    //         )
    //     } else if e1.src == node_id && e2.dst == node_id {
    //         (
    //             e2.src,
    //             e1.dst,
    //             e2.dst_pcf.clone(),
    //             e2.src_pcf.clone(),
    //             e1.dst_pcf.clone(),
    //             e1.src_pcf.clone(),
    //         )
    //     } else if e1.dst == node_id && e2.dst == node_id {
    //         (
    //             e1.src,
    //             e2.src,
    //             e1.dst_pcf.clone(),
    //             e1.src_pcf.clone(),
    //             e2.dst_pcf.clone(),
    //             e2.src_pcf.clone(),
    //         )
    //     } else {
    //         (
    //             e1.dst,
    //             e2.dst,
    //             e1.src_pcf.clone(),
    //             e1.dst_pcf.clone(),
    //             e2.src_pcf.clone(),
    //             e2.dst_pcf.clone(),
    //         )
    //     };
    //
    //     let (new_src_pcf, new_dst_pcf) = beta(&rx, &ry, &sy, &sz);
    //     let mut new_predicates = Vec::new();
    //     let mut new_original_edge_ids = Vec::new();
    //     let mut new_path_vertices = Vec::new();
    //
    //     self.remove_edge(e1_id);
    //     self.remove_edge(e2_id);
    //
    //     let new_edge = AbstractEdge {
    //         src: new_src,
    //         dst: new_dst,
    //         src_pcf: Arc::new(new_src_pcf),
    //         dst_pcf: Arc::new(new_dst_pcf),
    //         predicates: new_predicates,
    //         original_edge_ids: new_original_edge_ids,
    //         path_vertices: new_path_vertices,
    //         selectivity: 1.0,
    //         path_str: e1.path_str + &e2.path_str,
    //     };
    //     println!(
    //         "new edge: \n {}\n src:{} \n dst:{}\n",
    //         new_edge.path_str, new_edge.src_pcf, new_edge.dst_pcf
    //     );
    //
    //     let new_edge_id = self
    //         .edges
    //         .keys()
    //         .max()
    //         .copied()
    //         .unwrap_or(0)
    //         .saturating_add(1);
    //
    //     self.add_edge(new_edge_id, new_edge);
    //
    //     self.remove_vertex(node_id);
    //
    //     Ok(())
    // }
    //
    // fn get_es_from_flat_single_degree2_path(&mut self, node_id: VertexId) -> GCardResult<f64> {
    //     let mut edge_ids = self.get_neighbor_edges(node_id);
    //     if edge_ids.len() != 2 {
    //         return Ok(0.0);
    //     }
    //
    //     let e1_id = edge_ids[0];
    //     let e2_id = edge_ids[1];
    //
    //     let e1 = self
    //         .edges
    //         .get(&e1_id)
    //         .ok_or_else(|| GCardError::EdgeNotFound(e1_id.to_string()))?
    //         .clone();
    //     let e2 = self
    //         .edges
    //         .get(&e2_id)
    //         .ok_or_else(|| GCardError::EdgeNotFound(e2_id.to_string()))?
    //         .clone();
    //     if e1.selectivity == 0.0 || e2.selectivity == 0.0 {
    //         return Ok(0.0);
    //     }
    //     let left_pcf = if e1.dst == node_id {
    //         e1.dst_pcf.as_ref().clone()
    //     } else {
    //         e1.src_pcf.as_ref().clone()
    //     };
    //
    //     let right_pcf = if e2.dst == node_id {
    //         e2.dst_pcf.as_ref().clone()
    //     } else {
    //         e2.src_pcf.as_ref().clone()
    //     };
    //     println!("use single flat degree2 path sum");
    //     println!("input path1: {}", e1.path_str);
    //     println!("input path2: {}", e2.path_str);
    //     println!("left pcf: {}", left_pcf);
    //     println!("right pcf: {}", right_pcf);
    //     let pcf = alpha_refs(vec![&left_pcf, &right_pcf].as_ref());
    //     println!("output pcf: {}", pcf);
    //     Ok(pcf.get_num_rows())
    // }
    //
    // fn reduce_pivot_nodes(&mut self) -> GCardResult<Pcf> {
    //     loop {
    //         let pivot_nodes = self.find_pivot_nodes();
    //
    //         match pivot_nodes.len() {
    //             0 => {
    //
    //                 return if let Some((_, edge)) = self.edges.iter().next() {
    //                     println!("get pcf result from single edge, pcf is:{}", edge.src_pcf);
    //                     Ok(edge.src_pcf.as_ref().clone())
    //                 } else {
    //                     Err(GCardError::InvalidState(
    //                         "State error handling path".to_string(),
    //                     ))
    //                 };
    //             }
    //             1 => {
    //                 let pivot_id = pivot_nodes.iter().next().unwrap();
    //                 let leaf_nodes = self.find_leaf_neighbors(*pivot_id);
    //                 let pcf = self.merge_leaf_node_to_pivot(*pivot_id, &leaf_nodes)?;
    //                 return Ok(pcf);
    //             }
    //             _ => {
    //                 let mut pivot_with_counts: Vec<(VertexId, usize)> = pivot_nodes
    //                     .iter()
    //                     .map(|&pivot_id| (pivot_id, self.count_pivot_neighbors(pivot_id)))
    //                     .collect();
    //
    //                 pivot_with_counts.sort_by_key(|(_, count)| *count);
    //
    //                 let (pivot_id, _cnt) = pivot_with_counts[0];
    //                 debug_assert!(
    //                     _cnt <= 1,
    //                     "pivot {} has more than one pivot neighbor, cnt={}",
    //                     pivot_id,
    //                     _cnt
    //                 );
    //
    //                 self.reduce_single_pivot_node(pivot_id)?;
    //             }
    //         }
    //     }
    // }
    //
    // fn count_pivot_neighbors(&self, pivot_id: VertexId) -> usize {
    //     let mut neighbors = HashSet::new();
    //
    //     if let Some(outgoing_ids) = self.outgoing_edges.get(&pivot_id) {
    //         for &edge_id in outgoing_ids {
    //             if let Some(edge) = self.edges.get(&edge_id) {
    //                 neighbors.insert(edge.dst);
    //             }
    //         }
    //     }
    //
    //     if let Some(incoming_ids) = self.incoming_edges.get(&pivot_id) {
    //         for &edge_id in incoming_ids {
    //             if let Some(edge) = self.edges.get(&edge_id) {
    //                 neighbors.insert(edge.src);
    //             }
    //         }
    //     }
    //
    //     neighbors
    //         .iter()
    //         .filter(|&neighbor_id| self.get_degree(*neighbor_id) >= 3)
    //         .count()
    // }
    //
    // fn reduce_single_pivot_node(&mut self, pivot_id: VertexId) -> GCardResult<()> {
    //     let leaf_nodes = self.find_leaf_neighbors(pivot_id);
    //
    //     if !leaf_nodes.is_empty() {
    //         let pivot_to_leaf_pcf = self.merge_leaf_node_to_pivot(pivot_id, &leaf_nodes)?;
    //
    //         for (leaf_id, edge_id) in &leaf_nodes {
    //             self.remove_edge(*edge_id);
    //             self.remove_vertex(*leaf_id);
    //         }
    //
    //         let pivot_neighbor = self.get_pivot_neighbor(pivot_id);
    //
    //         if let Some((vertex, edge)) = pivot_neighbor {
    //             if let Some(pivot_edge) = self.edges.get(&edge) {
    //                 let (pivot_to_pivot_neighbor_pcf, pivot_neighbor_to_pivot_pcf) =
    //                     if pivot_edge.src == pivot_id {
    //                         (pivot_edge.src_pcf.clone(), pivot_edge.dst_pcf.clone())
    //                     } else {
    //                         (pivot_edge.dst_pcf.clone(), pivot_edge.src_pcf.clone())
    //                     };
    //
    //                 let new_pcf = beta_right(
    //                     &pivot_to_leaf_pcf,
    //                     &pivot_to_pivot_neighbor_pcf,
    //                     &pivot_neighbor_to_pivot_pcf,
    //                 );
    //                 println!("reduce pivot node, new pcf is {}", new_pcf);
    //
    //                 let new_edge = AbstractEdge {
    //                     src: pivot_id,
    //                     dst: vertex,
    //                     src_pcf: Arc::new(Pcf::empty()),
    //                     dst_pcf: Arc::new(new_pcf.clone()),
    //                     predicates: Vec::new(),
    //                     original_edge_ids: Vec::new(),
    //                     path_vertices: Vec::new(),
    //                     selectivity: 1.0,
    //                     path_str: String::new(),
    //                 };
    //                 self.remove_edge(edge);
    //                 let new_edge_id = edge;
    //                 self.add_edge(new_edge_id, new_edge);
    //             }
    //         }
    //     }
    //     Ok(())
    // }
    //
    // fn find_leaf_neighbors(&self, pivot_id: VertexId) -> Vec<(VertexId, EdgeId)> {
    //     let mut leaf_nodes = Vec::new();
    //
    //     if let Some(outgoing_ids) = self.outgoing_edges.get(&pivot_id) {
    //         for &edge_id in outgoing_ids {
    //             if let Some(edge) = self.edges.get(&edge_id) {
    //                 if self.get_degree(edge.dst) == 1 {
    //                     leaf_nodes.push((edge.dst, edge_id));
    //                 }
    //             }
    //         }
    //     }
    //
    //     if let Some(incoming_ids) = self.incoming_edges.get(&pivot_id) {
    //         for &edge_id in incoming_ids {
    //             if let Some(edge) = self.edges.get(&edge_id) {
    //                 if self.get_degree(edge.src) == 1 {
    //                     leaf_nodes.push((edge.src, edge_id));
    //                 }
    //             }
    //         }
    //     }
    //
    //     leaf_nodes
    // }
    //
    // fn get_pivot_neighbor(&self, pivot_id: VertexId) -> Option<(VertexId, EdgeId)> {
    //     if let Some(outgoing_ids) = self.outgoing_edges.get(&pivot_id) {
    //         for &edge_id in outgoing_ids {
    //             if let Some(edge) = self.edges.get(&edge_id) {
    //                 if self.get_degree(edge.dst) >= 3 {}
    //             }
    //         }
    //     }
    //
    //     if let Some(incoming_ids) = self.incoming_edges.get(&pivot_id) {
    //         for &edge_id in incoming_ids {
    //             if let Some(edge) = self.edges.get(&edge_id) {
    //                 if self.get_degree(edge.src) >= 3 {
    //                     return Some((edge.dst, edge_id));
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }
    //
    // fn merge_leaf_node_to_pivot(
    //     &mut self,
    //     pivot_id: VertexId,
    //     leaf_nodes: &[(VertexId, EdgeId)],
    // ) -> GCardResult<Pcf> {
    //     println!("reduce povit node");
    //     let mut pivot_to_leaf_pcfs = Vec::new();
    //
    //     for (leaf_id, edge_id) in leaf_nodes {
    //         println!("find leaf {}", self.vertices.get(leaf_id).unwrap().label);
    //         println!("find edge to leaf{}", self.edges.get(edge_id).unwrap().path_str);
    //         if let Some(edge) = self.edges.get(edge_id) {
    //             let pivot_to_leaf_pcf = if edge.dst == pivot_id {
    //                 edge.dst_pcf.as_ref().clone()
    //             } else {
    //                 edge.src_pcf.as_ref().clone()
    //             };
    //             println!("put pcf {} to pivot set", pivot_to_leaf_pcf);
    //             pivot_to_leaf_pcfs.push(pivot_to_leaf_pcf);
    //         }
    //     }
    //
    //     let pcf_refs: Vec<&Pcf> = pivot_to_leaf_pcfs.iter().collect();
    //     let merged_pcf = alpha_refs(&pcf_refs);
    //     println!("get merged pcf {}", merged_pcf);
    //     Ok(merged_pcf)
    // }
    //
    // pub fn get_selectivity(&self) -> f64 {
    //     let mut selectivity = 1.0;
    //     for edge in self.edges.values() {
    //         selectivity *= edge.selectivity;
    //     }
    //     selectivity
    // }
}
