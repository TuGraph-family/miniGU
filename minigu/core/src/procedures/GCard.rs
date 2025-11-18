use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use minigu_catalog::memory::graph_type::MemoryGraphTypeCatalog;
use minigu_catalog::provider::GraphTypeProvider;
use minigu_common::data_type::LogicalType;
use minigu_common::types::{LabelId, VertexId};
use minigu_context::procedure::Procedure;
use minigu_storage::error::StorageResult;
use minigu_storage::iterators::AdjacencyIteratorTrait;
use minigu_storage::tp::{MemTransaction, MemoryGraph};
use serde::{Deserialize, Serialize};
use minigu_storage::tp::iterators::AdjacencyIterator;
use minigu_transaction::GraphTxnManager;
use minigu_transaction::IsolationLevel::Serializable;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
// 这个结构体，描述了一个短路径，边的类型，开始点的类型与结束点的类型
pub struct PathStep {
    pub edge_type: LabelId,
    pub src_type: LabelId,
    pub dst_type: LabelId,
}

// 这个结构体，描述了一个路径，这个路径是有若干个短路径构成的。当然如果steps
// 只有1，那也是一个短路径。
#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct PathPattern {
    pub steps: Vec<PathStep>,
}

impl PathPattern {
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    pub fn first_step(&self) -> &PathStep {
        &self.steps[0]
    }
    
    pub fn src_label_id(&self) -> LabelId {
        self.steps[0].src_type
    }

    pub fn suffix(&self) -> Option<PathPattern> {
        if self.steps.len() <= 1 {
            None
        } else {
            Some(PathPattern {
                steps: self.steps[1..].to_vec(),
            })
        }
    }
}

// 这个结构体，描述了在一个路径下，存在的节点和对应的度的信息， 这个路径可以使短路径，也可以是长路径
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathVertexDegrees {
    pub pattern: PathPattern,
    pub degrees: HashMap<VertexId, u64>,
}

// 这个结构体，实际上会汇总所有路径的Degree sequence
// 信息，并存储为一个HashMap，在后续的计算中，会使用这里的信息完成动态规划的更新
pub type PathDegreesMap = HashMap<PathPattern, PathVertexDegrees>;

// 这个结构体实际上是描述了一个类型ID 通过一个路径，连接到的Degree sequence，这里的 ds
// 是经过排序的。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypePathDegreeSeq {
    pub node_type: LabelId,
    pub path: PathPattern,
    pub degree_seq: Vec<u64>,
}

fn build_schema_graph(
    catalog: &MemoryGraphTypeCatalog,
) -> HashMap<LabelId, Vec<(LabelId, LabelId)>> {
    // 这个方法，从图元数据中构建所有的路径，包括简单路径和复杂路径。
    // 首先会获得所有的边的类型，随后会将所有的边的类型进行汇总，边的信息包括：
    // 开始的点的类型->边的类型->结束点的类型。 这里会将所有的边按照开始点进行汇总，
    // 得到一个HashMap，用来后续进行构造所有可能路径的遍历基础。
    let mut schema_graph: HashMap<LabelId, Vec<(LabelId, LabelId)>> = HashMap::new();
    for edge_type in &catalog.edge_type_keys() {
        let edges = catalog
            .get_edge_type(edge_type)
            .expect("edge type not found")
            .unwrap();
        let src_type = edges.src().label_set().first().unwrap().clone();
        let dst_type = edges.dst().label_set().first().unwrap().clone();
        schema_graph
            .entry(src_type)
            .or_default()
            .push((edge_type.first().unwrap().clone(), dst_type));
    }
    schema_graph
}

fn dfs_paths_schema(
    schema_graph: &HashMap<LabelId, Vec<(LabelId, LabelId)>>,
    current: &mut Vec<PathStep>,
    max_k: usize,
    out: &mut Vec<PathPattern>,
) {
    // 这个函数是一个深度遍历方法，获取一个label之后，会进行深度遍历，遍历的深度为max_K。
    out.push(PathPattern {
        steps: current.clone(),
    });

    if current.len() >= max_k {
        return;
    }

    let last_dst = current.last().unwrap().dst_type.clone();
    if let Some(outs) = schema_graph.get(&last_dst) {
        for (edge_type, dst_type) in outs {
            current.push(PathStep {
                edge_type: edge_type.clone(),
                src_type: last_dst.clone(),
                dst_type: dst_type.clone(),
            });
            dfs_paths_schema(schema_graph, current, max_k, out);
            current.pop();
        }
    }
}

pub fn enumerate_schema_paths(catalog: &MemoryGraphTypeCatalog, max_k: usize) -> Vec<PathPattern> {
    // 这里会从memory graph Type
    // 中获取所有的边，并从每个边的起点开始，遍历得到小于max_K长度的路径，最终得到一个
    // 包含所有路径的Vec，但是这里的路径长度没有排序，所以Vec里面的路径是混在一起的。
    assert!(max_k > 0 && max_k <= 3, "max_k must be 1..=3");
    let schema_graph = build_schema_graph(catalog);
    let mut results = Vec::new();
    for (src_type, outs) in &schema_graph {
        for (edge_type, dst_type) in outs {
            let first = PathStep {
                edge_type: edge_type.clone(),
                src_type: src_type.clone(),
                dst_type: dst_type.clone(),
            };
            let mut current = vec![first];
            dfs_paths_schema(&schema_graph, &mut current, max_k, &mut results);
        }
    }
    results
}

fn iter_vertices_of_type(
    graph: Arc<MemoryGraph>,
    node_type: LabelId,
    txn: &Arc<MemTransaction>,
) -> StorageResult<Vec<VertexId>> {
    // 这个函数会将图中符合labelid的点筛选出来，但是因为现在实现的问题，filter 是在外面做的。
    // 这个后面会不会想办法优化？
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

fn neighbors_matching_step(
    graph: Arc<MemoryGraph>,
    vertex_id: VertexId,
    step: &PathStep,
    txn: &Arc<MemTransaction>,
) -> Vec<VertexId> {
    // 在这个函数中，可以给定一个路径，给定一个起始点，获取他的邻居的id
    if graph.get_vertex(&txn, vertex_id).is_err() {
        return Vec::new();
    }

    let mut adj_iter = txn.iter_adjacency_outgoing(vertex_id);
    adj_iter = AdjacencyIteratorTrait::filter(adj_iter, move |neighbor| {
        neighbor.label_id() == step.edge_type
    });
    let mut matching_neighbors = Vec::new();
    for neighbor_result in adj_iter {
        match neighbor_result {
            Ok(neighbor) => {
                match graph.get_vertex(&txn, neighbor.neighbor_id()) {
                    Ok(neighbor_vertex) => {
                        if neighbor_vertex.label_id == step.dst_type {
                            matching_neighbors.push(neighbor_vertex.vid);
                        }
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
            Err(_) => {
                continue;
            }
        }
    }
    matching_neighbors
}


// PathDegreesMap 存储了这样的信息： 路径信息，以及路径开始的label_id 的节点，以及该节点连接到的路径尾部的数量。
fn agg_degree_sequence(path_degree: &PathDegreesMap) -> Vec<TypePathDegreeSeq> {
    use std::collections::hash_map::Entry;
    // 这里存储的是： (label id && 路径) -> degree  信息。
    let mut agg: HashMap<(LabelId, PathPattern), Vec<u64>> = HashMap::new();
    
    // 这里把 vertex id 的信息去掉了，只保留了频率信息。
    for (pattern, pvd) in path_degree {
        for (vid, d) in &pvd.degrees {
            let key = (pattern.src_label_id(), pattern.clone());
            match agg.entry(key) {
                Entry::Occupied(mut entry) => entry.get_mut().push(*d),
                Entry::Vacant(entry) => {
                    entry.insert(vec![*d]);
                }
            }
        }
    }

    // 这里进行排序，并进一步保存了： label id + path + degree sequence(排序后)
    let mut out = Vec::new();
    for ((label_id, path), mut seq) in agg {
        seq.sort_unstable();
        out.push(TypePathDegreeSeq {
            node_type: label_id,
            path,
            degree_seq: seq,
        })
    }
    out
}

fn save_type_path_degree_seq(
    seqs: &[TypePathDegreeSeq],
    out_dir: &Path,
) -> Result<(), std::io::Error> {
    let summary_dir = out_dir.join("summary");
    fs::create_dir_all(&summary_dir)?;
    let file_path = summary_dir.join("typed_path_degree_sequence.json");
    let f = File::create(file_path)?;
    serde_json::to_writer_pretty(f, &seqs)?;
    Ok(())
}

pub fn build_procedure() -> Procedure {
    // parameters:
    // 1. data folder
    // 2. max length of path.
    // 3. output folder.
    let parameters = vec![LogicalType::String, LogicalType::Int32, LogicalType::String];
    Procedure::new(parameters, None, move |mut context, args| {
        let data_dir = args[0]
            .try_as_string()
            .expect("expecting string value for data folder")
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("expecting string value for data folder"))?
            .to_string();
        let path_len = args[1]
            .try_as_int8()
            .expect("max length of path must be int8")
            .ok_or_else(|| anyhow::anyhow!("expecting int8 for path length"))?;

        let out_put_dir = args[2]
            .try_as_string()
            .expect("expecting string value for output folder")
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("expecting string value for output folder"))?
            .to_string();

        let graph = MemoryGraph::with_config_fresh(Default::default(), Default::default());
        let mut graph_type = MemoryGraphTypeCatalog::new();
        let paths = enumerate_schema_paths(&graph_type, path_len as usize);
        let txn = graph.txn_manager().begin_transaction(Serializable)?;
        let mut results: PathDegreesMap = HashMap::new();
        for current_len in 1..=path_len as usize {
            let path_list: Vec<PathPattern> = paths
                .iter()
                .filter(|p| p.len() == current_len)
                .cloned()
                .collect();
            for path in path_list {
                let first = path.first_step();
                let mut degrees: HashMap<VertexId, u64> = HashMap::new();
                let vertices = iter_vertices_of_type(graph.clone(), first.src_type, &txn)?;
                for v in vertices {
                    let mut cnt: u64 = 0;
                    if current_len == 1 {
                        let neighs = neighbors_matching_step(graph.clone(), v, first, &txn);
                        cnt = neighs.len() as u64;
                    } else {
                        let suffix = path.suffix().expect("suffix exists");
                        let suffix_stats = results
                            .get(&suffix)
                            .unwrap_or_else(|| panic!("suffix error"));
                        let neighs = neighbors_matching_step(graph.clone(), v, first, &txn);
                        for u in neighs {
                            let du = suffix_stats.degrees.get(&u).copied().unwrap_or(0);
                            cnt += du;
                        }
                    }
                    degrees.insert(v, cnt);
                }
                results.insert(path.clone(), PathVertexDegrees {
                    pattern: path.clone(),
                    degrees,
                });
            }
        }
        let to_serialize = agg_degree_sequence(&results);
        save_type_path_degree_seq(&to_serialize, &PathBuf::from(out_put_dir));
        Ok(vec![])
    })
}
