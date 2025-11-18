//! Definitions for Statistic.
//!
//! Statistic is a three-level map: LabelId -> LabelStatistic; each LabelStatistic 含 vertex_ids 与 path_statistic (AltKey -> BlockStatistic).
//! vertex_ids 的长度与所有 path_statistic 里 BlockStatistic 的 bucket_ids 总长一致。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use minigu_common::types::{LabelId, VertexId};

use crate::procedures::gcard_query::block_statistic::BlockStatistic;
use crate::procedures::gcard_query::catalog::{AltKey, CompressedDegreeSeq, DegreeSeqGraphCompressed};
use crate::procedures::gcard_query::error::{GCardError, GCardResult};

type VertexVec = Vec<VertexId>;

const LEN_U64: usize = 8;
const LEN_LABEL: usize = 4; // LabelId as u32

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct LabelStatistic {
    /// 该 label 对应的节点 id；长度与所有 path_statistic 里 block.bucket_ids 总长一致。
    vertex_ids: VertexVec,
    path_statistic: HashMap<AltKey, BlockStatistic>,
}

/// AltKey 序列化占用：8 (段数) + 每段 (8 + utf8 字节数)
fn alt_key_serialized_size(alt_key: &AltKey) -> usize {
    let mut n = LEN_U64;
    for s in &alt_key.0 {
        n += LEN_U64 + s.as_bytes().len();
    }
    n
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Statistic {
    pub label_path_statistic: HashMap<String, LabelStatistic>,
}

impl Statistic {
    /// 构造/更新：`vertex_ids` 与 `frequencies` 一一对应，先按 vertex_ids 递增排序后再用 frequencies 构造 BlockStatistic。
    /// 若该 label 已有 vertex_ids，只校验长度与最小/最大值一致，不覆盖。
    pub fn insert_or_update(
        &mut self,
        label: &str,
        vertex_ids: &[VertexId],
        alt_key: AltKey,
        frequencies: &[u64],
    ) -> GCardResult<()> {
        if vertex_ids.len() != frequencies.len() {
            return Err(GCardError::InvalidData(
                "vertex_ids and frequencies length mismatch".into(),
            ));
        }
        let mut pairs: Vec<(VertexId, u64)> = vertex_ids
            .iter()
            .zip(frequencies.iter())
            .map(|(&v, &f)| (v, f))
            .collect();
        pairs.sort_by_key(|p| p.0);
        let (sorted_vertex_ids, sorted_frequencies): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();

        let block = BlockStatistic::from_u64_sequence(&sorted_frequencies)?;
        let entry = self.label_path_statistic.entry(label.to_string()).or_default();

        if !entry.vertex_ids.is_empty() {
            if entry.vertex_ids.len() != sorted_vertex_ids.len() {
                return Err(GCardError::InvalidData(
                    "vertex_ids length differs from existing".into(),
                ));
            }
            let (ex_min, ex_max) = (
                *entry.vertex_ids.iter().min().unwrap(),
                *entry.vertex_ids.iter().max().unwrap(),
            );
            let (new_min, new_max) = (
                *sorted_vertex_ids.first().unwrap_or(&0),
                *sorted_vertex_ids.last().unwrap_or(&0),
            );
            if ex_min != new_min || ex_max != new_max {
                return Err(GCardError::InvalidData(
                    "vertex_ids min/max differs from existing".into(),
                ));
            }
        } else {
            entry.vertex_ids = sorted_vertex_ids;
        }

        entry.path_statistic.insert(alt_key, block);
        Ok(())
    }

    /// 返回整个 Statistic 中"res_vec 已触达桶上限"的 entry 比例（按 entry 数量加权）。
    ///
    /// 值接近 1.0 表示大量统计上界已经最松，查询精度下降，建议重新执行 `GCard_build`。
    pub fn upper_limit_ratio(&self) -> f64 {
        let mut total = 0usize;
        let mut at_limit_weighted = 0.0f64;
        for ls in self.label_path_statistic.values() {
            for bs in ls.path_statistic.values() {
                let n = bs.bucket_ids.len();
                if n > 0 {
                    at_limit_weighted += bs.upper_limit_ratio() * n as f64;
                    total += n;
                }
            }
        }
        if total == 0 { 0.0 } else { at_limit_weighted / total as f64 }
    }

    /// 返回指定路径 (alt_key) 对应的 upper_limit_ratio。
    /// 只统计匹配该路径的 BlockStatistic。如果没有匹配项返回 None。
    pub fn upper_limit_ratio_for_path(&self, alt_key: &AltKey) -> Option<f64> {
        let mut total = 0usize;
        let mut at_limit_weighted = 0.0f64;
        for ls in self.label_path_statistic.values() {
            if let Some(bs) = ls.path_statistic.get(alt_key) {
                let n = bs.bucket_ids.len();
                if n > 0 {
                    at_limit_weighted += bs.upper_limit_ratio() * n as f64;
                    total += n;
                }
            }
        }
        if total == 0 { None } else { Some(at_limit_weighted / total as f64) }
    }

    /// 获取指定 label_id 与 alt_key 对应的 bucket_ids；不存在则返回 None。
    pub fn get_bucket_ids(&self, label: &str, alt_key: &AltKey) -> Option<&[u8]> {
        self.label_path_statistic
            .get(label)
            .and_then(|ls| ls.path_statistic.get(alt_key))
            .map(|b| b.bucket_ids())
    }

    /// 根据给定的 vertex_id (u64) 查某 label、某 alt_key 下的 (bucket_id, prefix)。
    /// 在 label 的 vertex_ids 中找到该 vertex_id 的秩，返回该秩处的 bucket_id、prefix；找不到返回 None。
    pub fn get_vertex_bucket_prefix(
        &self,
        label: &str,
        alt_key: &AltKey,
        vertex_id: VertexId,
    ) -> Option<(u8, u8)> {
        let ls = self.label_path_statistic.get(label)?;
        let block = ls.path_statistic.get(alt_key)?;
        let i = ls.vertex_ids.binary_search(&vertex_id).ok()?;
        let bucket_id = *block.bucket_ids.get(i)?;
        let prefix = *block.prefix.get(i)?;
        Some((bucket_id, prefix))
    }

    /// 根据 label_id -> node_name 映射，将各 label 下的 path (alt_key -> block) 转成 DegreeSeqGraphCompressed。
    pub fn to_degree_seq_graph_compressed(
        &self,
    ) -> GCardResult<DegreeSeqGraphCompressed> {
        let mut edge_set_to_endpoints: HashMap<AltKey, HashMap<String, CompressedDegreeSeq>> = HashMap::new();
        for (node_name, ls) in &self.label_path_statistic {
            for (alt_key, block) in &ls.path_statistic {
                if let Some(seq) = block.get_compressed_degree_seq()? {
                    edge_set_to_endpoints
                        .entry(alt_key.clone())
                        .or_default()
                        .insert(node_name.clone(), seq);
                }
            }
        }
        Ok(DegreeSeqGraphCompressed {
            edge_set_to_endpoints,
        })
    }

    /// Apply `delta` to the stored frequency of `vertex_id` for (`label`, `altkey`).
    ///
    /// Looks up `vertex_id` by binary search in the sorted `vertex_ids` list, recovers the
    /// current upper-bound frequency from the corresponding `BlockStatistic`, applies `delta`
    /// (clamped to 0), and writes the result back.  Silently no-ops if the vertex or path
    /// is not found (e.g. the vertex was not yet in the statistics when compact is called).
    pub fn apply_delta(&mut self, label: &str, altkey: &AltKey, vertex_id: VertexId, delta: i64) {
        let Some(ls) = self.label_path_statistic.get_mut(label) else { return };
        let Ok(rank) = ls.vertex_ids.binary_search(&vertex_id) else { return };
        let Some(bs) = ls.path_statistic.get_mut(altkey) else { return };
        let current = bs.recover_upper_bound_at_rank(rank).unwrap_or(0);
        let new_val = (current as i64 + delta).max(0) as u64;
        bs.update_at_rank(rank, new_val);
    }

    /// Remove `vertex_id` from every label's `vertex_ids` list and the corresponding rank
    /// from every `BlockStatistic`.
    ///
    /// No-op for labels that do not contain `vertex_id`.
    pub fn delete_vertex(&mut self, vertex_id: VertexId) {
        for ls in self.label_path_statistic.values_mut() {
            let Ok(rank) = ls.vertex_ids.binary_search(&vertex_id) else { continue };
            ls.vertex_ids.remove(rank);
            for bs in ls.path_statistic.values_mut() {
                bs.remove_at_rank(rank);
            }
        }
    }

    /// 序列化布局（用于计算大小）：先写 label 数量(8)，再对每个 label 依次写：
    ///   label_id(4)、vertex_ids 长度(8)、vertex_ids(每项 8 字节)、path 数量(8)、
    ///   再对每个 (alt_key, block)：alt_key 序列化、entry_count(8)、block.serialize()。
    /// 返回按此布局序列化后的总字节数。
    pub fn serialized_size(&self) -> usize {
        let mut total = LEN_U64; // label count
        for (_label_id, ls) in &self.label_path_statistic {
            total += LEN_LABEL; // label_id
            total += LEN_U64 + ls.vertex_ids.len() * LEN_U64; // vertex_ids len + data
            total += LEN_U64; // path_statistic count
            for (alt_key, block) in &ls.path_statistic {
                total += alt_key_serialized_size(alt_key);
                total += block.serialize().len();
            }
        }
        total
    }
}

