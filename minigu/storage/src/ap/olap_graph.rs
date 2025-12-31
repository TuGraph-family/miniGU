use std::hash::{Hash, Hasher};
use std::num::NonZeroU32;
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use bitvec::bitvec;
use bitvec::prelude::Lsb0;
use bitvec::vec::BitVec;
use dashmap::DashMap;
use minigu_common::types::{EdgeId, LabelId, VertexId};
use minigu_common::value::ScalarValue;
use minigu_transaction::Timestamp;
use serde::{Deserialize, Serialize};

use crate::ap::iterators::{
    AdjacencyIterator, AdjacencyIteratorAtTs, EdgeIter, EdgeIterAtTs, VertexIter,
};
use crate::ap::olap_storage::{MutOlapGraph, OlapGraph};
use crate::common::DeltaOp;
use crate::error::EdgeNotFoundError::EdgeNotFound;
use crate::error::VertexNotFoundError::VertexNotFound;
use crate::error::{StorageError, StorageResult};
use crate::model::properties::PropertyRecord;

pub const BLOCK_CAPACITY: usize = 256;
const TOMBSTONE_LABEL_ID: u32 = 1;
const TOMBSTONE_DST_ID: u64 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
struct TxnId(u64);

// TODO：Olap-Vertex (without MVCC)
#[derive(Clone, Debug)]
pub struct OlapVertex {
    // Vertex id (actual id)
    // No need for extra logical id storage for it's used as array index
    pub vid: VertexId,
    // Properties
    pub properties: PropertyRecord,
    // Locate the last block of the vertex
    pub block_offset: usize,
}

impl PartialEq for OlapVertex {
    fn eq(&self, other: &Self) -> bool {
        self.vid == other.vid
    }
}

impl Eq for OlapVertex {}

impl Hash for OlapVertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.vid.hash(state);
    }
}

// Olap-Edge (For Storage)
#[derive(Clone, Debug, Copy)]
pub struct OlapStorageEdge {
    // Edge data
    pub eid: EdgeId, // Global unique edge identifier
    pub label_id: Option<LabelId>,
    pub dst_id: VertexId,
    pub commit_ts: Timestamp,
}
impl OlapStorageEdge {
    // (Temporarily) Stands for null
    fn default() -> OlapStorageEdge {
        OlapStorageEdge {
            eid: 0,
            label_id: NonZeroU32::new(TOMBSTONE_LABEL_ID),
            dst_id: TOMBSTONE_DST_ID,
            commit_ts: Timestamp::with_ts(0),
        }
    }
}

// Olap-Edge (With properties)
#[derive(Clone, Debug)]
pub struct OlapEdge {
    // Edge data
    pub label_id: Option<LabelId>,
    pub src_id: VertexId,
    pub dst_id: VertexId,
    pub properties: OlapPropertyStore,
}

// Olap-Property (Add 'Option' for compaction)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct OlapPropertyStore {
    pub properties: Vec<Option<ScalarValue>>,
}

impl OlapPropertyStore {
    pub fn set_prop(&mut self, index: usize, prop: Option<ScalarValue>) {
        self.properties.insert(index, prop);
    }

    pub fn get(&self, index: usize) -> Option<ScalarValue> {
        self.properties.get(index).cloned().flatten()
    }

    #[allow(dead_code)]
    pub fn new(properties: Vec<Option<ScalarValue>>) -> OlapPropertyStore {
        OlapPropertyStore { properties }
    }
}

// Block of edge array (Header + Actual Storage + MVCC)
#[derive(Clone, Debug)]
pub struct EdgeBlock {
    // Locate the previous block of the same vertex
    pub pre_block_index: Option<usize>,
    #[allow(dead_code)]
    pub cur_block_index: usize,
    pub is_tombstone: bool,
    // Min and max edge id (Eid)
    // For accelerating get_edge
    pub max_label_id: Option<LabelId>,
    pub min_label_id: Option<LabelId>,
    // Min and max to id (However may not be used)
    pub max_dst_id: VertexId,
    pub min_dst_id: VertexId,
    pub min_ts: Timestamp,
    pub max_ts: Timestamp,
    // Edge storage
    pub src_id: VertexId,
    pub edge_counter: usize,
    pub edges: [OlapStorageEdge; BLOCK_CAPACITY],
}

// Edge block after compression
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct CompressedEdgeBlock {
    // Locate the previous block of the same vertex
    pub pre_block_index: Option<usize>,
    pub cur_block_index: usize,
    // Min and max edge id (Eid)
    // For accelerating get_edge
    pub max_label_id: Option<LabelId>,
    pub min_label_id: Option<LabelId>,
    // Min and max to id (Vid)
    pub max_dst_id: VertexId,
    pub min_dst_id: VertexId,
    pub min_ts: Timestamp,
    pub max_ts: Timestamp,
    // Edge storage
    pub src_id: VertexId,
    pub edge_counter: usize,
    pub delta_bit_width: u8,
    pub first_dst_id: VertexId,
    pub compressed_dst_ids: BitVec<u64, Lsb0>,
    pub label_ids: [Option<LabelId>; BLOCK_CAPACITY],
    pub version_ts: Timestamp,
}

// Property block (Column storage)
#[derive(Clone, Debug)]
pub struct PropertyBlock {
    pub min_ts: Timestamp,
    pub max_ts: Timestamp,
    /// Property storage
    pub values: Vec<Option<ScalarValue>>,
}
// Property column storage
#[derive(Debug)]
pub struct PropertyColumn {
    pub blocks: Vec<PropertyBlock>,
}

// Property block after compaction
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct CompressedPropertyBlock {
    pub min_ts: Timestamp,
    pub max_ts: Timestamp,
    pub version_ts: Timestamp,
    pub bitmap: BitVec<u16, Lsb0>,
    // Stands for numbers not null elements in every 16 elements
    pub offsets: [u8; BLOCK_CAPACITY / 16],
    pub values: Vec<ScalarValue>,
}
// Property column after compaction
#[derive(Debug)]
pub struct CompressedPropertyColumn {
    pub blocks: Vec<CompressedPropertyBlock>,
}

// Graph storage for Olap (CSR)
pub struct OlapStorage {
    // For allocating vertex logical id
    pub logic_id_counter: AtomicU64,
    // For allocating edge id
    pub edge_id_counter: AtomicU64,
    // Actual id to logical id mapping
    pub dense_id_map: DashMap<VertexId, VertexId>,
    // EdgeId to (block_idx, offset) mapping for fast lookup
    pub edge_id_map: DashMap<EdgeId, (usize, usize)>,
    // Vertex array (Use lock for without MVCC)
    pub vertices: RwLock<Vec<OlapVertex>>,
    // Edge array
    pub edges: RwLock<Vec<EdgeBlock>>,
    // Property storage
    pub property_columns: RwLock<Vec<PropertyColumn>>,
    // Compaction related
    #[allow(dead_code)]
    pub is_edge_compressed: AtomicBool,
    #[allow(dead_code)]
    pub compressed_edges: RwLock<Vec<CompressedEdgeBlock>>,
    #[allow(dead_code)]
    pub is_property_compressed: AtomicBool,
    #[allow(dead_code)]
    pub compressed_properties: RwLock<Vec<CompressedPropertyColumn>>,
}

#[allow(dead_code)]
impl OlapStorage {
    pub fn compress_edge(&self) {
        if self.is_edge_compressed.load(Ordering::SeqCst) {
            return;
        }
        // 1. Set flag to true
        self.is_edge_compressed.store(true, Ordering::SeqCst);
        let mut edges_borrow = self.edges.write().unwrap();

        // 2. Traverse every block
        for (index, edge_block) in edges_borrow.iter().enumerate() {
            let mut max_delta: u64 = 0;
            // 2.1 Calculate max delta
            for i in 1..edge_block.edges.len() {
                let cur_dst_id = edge_block.edges[i].dst_id;
                let pre_dst_id = edge_block.edges[i - 1].dst_id;
                if cur_dst_id == 1 {
                    break;
                }
                max_delta = max_delta.max(cur_dst_id - pre_dst_id);
            }

            // 2.2 Calculate delta bits width
            let bit_width: u8 = if max_delta == 0 {
                1
            } else {
                (64 - max_delta.leading_zeros()) as u8
            };

            // 3. Start compressing
            // 3.1 Allocate some structs
            let required_bits = bit_width as usize * (edge_block.edge_counter - 1);
            let mut label_ids: [Option<LabelId>; BLOCK_CAPACITY] =
                [NonZeroU32::new(1); BLOCK_CAPACITY];
            let mut compressed_dst_ids: BitVec<u64, Lsb0> = bitvec![u64, Lsb0; 0; required_bits];
            let edges = edge_block.edges;

            // 3.2 Compress edges
            for i in 1..edge_block.edge_counter {
                label_ids[i] = edges[i].label_id;
                let delta = edges[i].dst_id - edges[i - 1].dst_id;
                let start_bit = (i - 1) * bit_width as usize;
                for j in 0..bit_width as usize {
                    let bit_is_set = ((delta >> j) & 1) == 1;
                    compressed_dst_ids.set(start_bit + j, bit_is_set);
                }
            }

            label_ids[0] = edges[0].label_id;
            let version_ts = minigu_transaction::global_timestamp_generator()
                .next()
                .unwrap();
            // 3.3 Build compressed edge block
            self.compressed_edges.write().unwrap().insert(
                index,
                CompressedEdgeBlock {
                    pre_block_index: edge_block.pre_block_index,
                    cur_block_index: index,
                    max_label_id: edge_block.max_label_id,
                    min_label_id: edge_block.min_label_id,
                    max_dst_id: edge_block.max_dst_id,
                    min_dst_id: edge_block.min_dst_id,
                    min_ts: edge_block.min_ts,
                    max_ts: edge_block.max_ts,
                    src_id: edge_block.src_id,
                    edge_counter: edge_block.edge_counter,
                    delta_bit_width: bit_width,
                    first_dst_id: edge_block.edges[0].dst_id,
                    compressed_dst_ids,
                    label_ids,
                    version_ts,
                },
            )
        }
        let _ = std::mem::take(&mut *edges_borrow);
    }

    pub fn compress_property(&self) {
        if self.is_property_compressed.load(Ordering::SeqCst) {
            return;
        }
        // 1. Set flag to true
        self.is_property_compressed.store(true, Ordering::SeqCst);

        // 2. Initial compressed storage
        let mut property_columns = self.property_columns.write().unwrap();

        let mut compressed_properties = self.compressed_properties.write().unwrap();
        let _column_cnt = property_columns.len();
        let version_ts = minigu_transaction::global_timestamp_generator()
            .next()
            .unwrap();

        // 3. Traverse property columns
        for (column_index, column) in property_columns.iter().enumerate() {
            let mut compressed_blocks = CompressedPropertyColumn { blocks: Vec::new() };

            for (block_index, block) in column.blocks.iter().enumerate() {
                let mut bitmap: BitVec<u16, Lsb0> = bitvec![u16, Lsb0; 0; BLOCK_CAPACITY];
                let mut values: Vec<ScalarValue> = Vec::new();
                let mut offsets: [u8; BLOCK_CAPACITY / 16] = [0u8; BLOCK_CAPACITY / 16];

                for (value_index, value_option) in block.values.iter().enumerate() {
                    if value_option.is_none() {
                        continue;
                    }

                    // Should not panic
                    bitmap.set(value_index, true);
                    values.push(value_option.clone().unwrap());
                }

                for (chunk_index, offset) in
                    offsets.iter_mut().enumerate().take(BLOCK_CAPACITY / 16)
                {
                    let start = chunk_index * 16;
                    let end = start + 16;

                    let ones_count = (start..end).filter(|&i| bitmap[i]).count() as u8;

                    *offset = ones_count;
                }

                compressed_blocks.blocks.insert(
                    block_index,
                    CompressedPropertyBlock {
                        min_ts: block.min_ts,
                        max_ts: block.max_ts,
                        version_ts,
                        bitmap,
                        offsets,
                        values,
                    },
                )
            }
            compressed_properties.insert(column_index, compressed_blocks);
        }

        let _ = std::mem::take(&mut *property_columns);
    }

    /// Transactional variant: write using transaction's txn_id and record undo
    #[allow(dead_code)]
    pub fn create_edge_in_txn(
        &self,
        txn: &crate::ap::transaction::MemTransaction,
        edge: OlapEdge,
    ) -> StorageResult<EdgeId> {
        use crate::common::model::edge::Edge as CommonEdge;

        // 1. Found vertex
        let dense_id = *self.dense_id_map.get(&edge.src_id).ok_or_else(|| {
            StorageError::VertexNotFound(VertexNotFound(format!(
                "Source vertex {} not found",
                edge.src_id
            )))
        })?;
        let mut binding = self.vertices.write().unwrap();
        let vertex = binding.get_mut(dense_id as usize).ok_or_else(|| {
            StorageError::VertexNotFound(VertexNotFound(format!(
                "Source vertex {} not found",
                edge.src_id
            )))
        })?;

        // 2. Initial block (lazy load) if not exists
        if vertex.block_offset == usize::MAX {
            let index = self.edges.read().unwrap().len();
            self.edges.write().unwrap().push(EdgeBlock {
                pre_block_index: None,
                cur_block_index: index,
                is_tombstone: false,
                max_label_id: NonZeroU32::new(1),
                min_label_id: NonZeroU32::new(u32::MAX),
                max_dst_id: 0,
                min_dst_id: u64::MAX,
                min_ts: txn.txn_id,
                max_ts: txn.txn_id,
                edge_counter: 0,
                src_id: edge.src_id,
                edges: [OlapStorageEdge::default(); BLOCK_CAPACITY],
            });
            vertex.block_offset = index;
        } else {
            let edge_count = self
                .edges
                .read()
                .unwrap()
                .get(vertex.block_offset)
                .ok_or_else(|| {
                    StorageError::EdgeNotFound(EdgeNotFound(format!(
                        "Vertex {} not found",
                        vertex.vid
                    )))
                })?
                .edge_counter;
            if edge_count >= BLOCK_CAPACITY {
                let index = self.edges.read().unwrap().len();
                self.edges.write().unwrap().push(EdgeBlock {
                    pre_block_index: Option::from(vertex.block_offset),
                    cur_block_index: index,
                    is_tombstone: false,
                    max_label_id: NonZeroU32::new(1),
                    min_label_id: NonZeroU32::new(u32::MAX),
                    max_dst_id: 0,
                    min_dst_id: u64::MAX,
                    min_ts: txn.txn_id,
                    max_ts: txn.txn_id,
                    src_id: edge.src_id,
                    edge_counter: 0,
                    edges: [OlapStorageEdge::default(); BLOCK_CAPACITY],
                });
                vertex.block_offset = index;
            }
        }

        // 4. Allocate global unique EdgeId
        let eid = self.edge_id_counter.fetch_add(1, Ordering::SeqCst);

        // 5. Insert edge
        let mut binding = self.edges.write().unwrap();
        let block = binding.get_mut(vertex.block_offset).ok_or_else(|| {
            StorageError::EdgeNotFound(EdgeNotFound(format!(
                "Edge block for vertex {} not found",
                vertex.vid
            )))
        })?;

        let insert_pos = block.edges[..block.edge_counter]
            .binary_search_by_key(&(&edge.dst_id, &edge.label_id), |e| {
                (&e.dst_id, &e.label_id)
            })
            .unwrap_or_else(|e| e);

        for i in (insert_pos..block.edge_counter).rev() {
            block.edges[i + 1] = block.edges[i];
        }
        block.edge_counter += 1;

        // set commit_ts to txn id and store eid
        block.edges[insert_pos] = OlapStorageEdge {
            eid,
            label_id: edge.label_id,
            dst_id: edge.dst_id,
            commit_ts: txn.txn_id,
        };

        // Build EdgeId to location mapping
        self.edge_id_map
            .insert(eid, (vertex.block_offset, insert_pos));

        // ensure property columns exist based on edge properties
        let mut property_columns = self.property_columns.write().unwrap();
        while property_columns.len() <= edge.properties.properties.len() {
            property_columns.push(PropertyColumn { blocks: Vec::new() });
        }

        // insert properties
        for (i, column) in property_columns.iter_mut().enumerate() {
            let property_block = if let Some(block) = column.blocks.get_mut(vertex.block_offset) {
                block
            } else {
                column.blocks.insert(
                    vertex.block_offset,
                    PropertyBlock {
                        min_ts: txn.txn_id,
                        max_ts: txn.txn_id,
                        values: vec![None; BLOCK_CAPACITY],
                    },
                );
                column.blocks.get_mut(vertex.block_offset).unwrap()
            };

            property_block.min_ts = property_block.min_ts.min(txn.txn_id);
            property_block.max_ts = property_block.max_ts.max(txn.txn_id);

            for j in (insert_pos..block.edge_counter - 1).rev() {
                property_block.values[j + 1] = property_block.values[j].clone();
            }

            if let Some(property_value) = edge.properties.get(i) {
                property_block.values[insert_pos] = Some(property_value);
            } else {
                property_block.values[insert_pos] = None;
            }
        }
        // update block header using txn id
        block.min_dst_id = edge.dst_id.min(block.min_dst_id);
        block.max_dst_id = edge.dst_id.max(block.max_dst_id);
        block.max_label_id = edge.label_id.max(block.max_label_id);
        block.min_label_id = edge.label_id.min(block.min_label_id);
        block.min_ts = block.min_ts.min(txn.txn_id);
        block.max_ts = block.max_ts.max(txn.txn_id);

        // push undo entry
        let common_edge = CommonEdge::new(
            eid,
            edge.src_id,
            edge.dst_id,
            edge.label_id.unwrap_or_else(|| NonZeroU32::new(1).unwrap()),
            PropertyRecord::default(),
        );
        txn.push_undo(DeltaOp::CreateEdge(common_edge), txn.txn_id);

        Ok(eid)
    }

    /// Transactional variant of `set_edge_property`.
    /// Sets properties and marks the edge's `commit_ts` to `txn.txn_id`, and records an undo entry.
    #[allow(dead_code)]
    pub fn set_edge_property_in_txn(
        &self,
        txn: &crate::ap::transaction::MemTransaction,
        eid: <OlapStorage as OlapGraph>::EdgeID,
        indices: Vec<usize>,
        props: Vec<ScalarValue>,
    ) -> StorageResult<()> {
        // Use EdgeId mapping for fast lookup
        let (block_idx, offset) = *self
            .edge_id_map
            .get(&eid)
            .ok_or_else(|| {
                StorageError::EdgeNotFound(EdgeNotFound(format!("Edge {} not found", eid)))
            })?
            .value();

        let mut edges_lock = self.edges.write().unwrap();
        let block = edges_lock.get_mut(block_idx).ok_or_else(|| {
            StorageError::EdgeNotFound(EdgeNotFound(format!("Edge block {} not found", block_idx)))
        })?;

        if block.is_tombstone || offset >= block.edge_counter {
            return Err(StorageError::EdgeNotFound(EdgeNotFound(format!(
                "Edge {} not found",
                eid
            ))));
        }

        let edge = &mut block.edges[offset];
        if edge.eid != eid {
            return Err(StorageError::EdgeNotFound(EdgeNotFound(format!(
                "Edge {} not found",
                eid
            ))));
        }

        // collect old property values for undo
        let mut old_props: Vec<minigu_common::value::ScalarValue> = Vec::new();
        {
            let property_columns = self.property_columns.read().unwrap();
            for &idx in indices.iter() {
                if let Some(col) = property_columns.get(idx)
                    && let Some(pb) = col.blocks.get(block_idx)
                    && let Some(val_opt) = pb.values.get(offset)
                    && let Some(v) = val_opt.clone()
                {
                    old_props.push(v);
                    continue;
                }
                old_props.push(minigu_common::value::ScalarValue::Null);
            }
        }

        // capture old commit_ts
        let old_commit_ts = edge.commit_ts;

        let set_op = crate::common::SetPropsOp {
            indices: indices.clone(),
            props: old_props.clone(),
        };
        // Push SetEdgeProps with the previous edge snapshot and the SetPropsOp
        txn.push_undo(
            crate::common::DeltaOp::SetEdgeProps(eid, set_op),
            old_commit_ts,
        );

        // mark edge as modified by txn
        edge.commit_ts = txn.txn_id;
        // update block-level ts
        block.min_ts = block.min_ts.min(txn.txn_id);
        block.max_ts = block.max_ts.max(txn.txn_id);

        // update properties
        let mut property_columns = self.property_columns.write().unwrap();
        for (i, prop) in indices.into_iter().zip(props.into_iter()) {
            let column = &mut property_columns[i];
            // ensure property block exists for this block index
            if column.blocks.get(block_idx).is_none() {
                column.blocks.insert(
                    block_idx,
                    PropertyBlock {
                        min_ts: txn.txn_id,
                        max_ts: txn.txn_id,
                        values: vec![None; BLOCK_CAPACITY],
                    },
                );
            }
            let property_block = &mut column.blocks[block_idx];
            property_block.values[offset] = Some(prop);
        }
        Ok(())
    }

    /// Transactional variant of `delete_edge`.
    /// Deletes the edge (in-place removal) and records undo entry with the edge id.
    #[allow(dead_code)]
    pub fn delete_edge_in_txn(
        &self,
        txn: &crate::ap::transaction::MemTransaction,
        eid: <OlapStorage as OlapGraph>::EdgeID,
    ) -> StorageResult<()> {
        // Use EdgeId mapping for fast lookup
        let (block_idx, offset) = *self
            .edge_id_map
            .get(&eid)
            .ok_or_else(|| {
                StorageError::EdgeNotFound(EdgeNotFound(format!("Edge {} not found", eid)))
            })?
            .value();

        let edges_lock = self.edges.read().unwrap();
        let block = edges_lock.get(block_idx).ok_or_else(|| {
            StorageError::EdgeNotFound(EdgeNotFound(format!("Edge block {} not found", block_idx)))
        })?;

        if block.is_tombstone || offset >= block.edge_counter {
            return Err(StorageError::EdgeNotFound(EdgeNotFound(format!(
                "Edge {} not found",
                eid
            ))));
        }

        let edge_data = &block.edges[offset];
        if edge_data.eid != eid {
            return Err(StorageError::EdgeNotFound(EdgeNotFound(format!(
                "Edge {} not found",
                eid
            ))));
        }

        // Save old_commit_ts as timestamp in undo entry
        let old_commit_ts = edge_data.commit_ts;
        drop(edges_lock);

        let mut props_vec: Vec<minigu_common::value::ScalarValue> = Vec::new();
        {
            let property_columns = self.property_columns.read().unwrap();
            for col in property_columns.iter() {
                if let Some(pb) = col.blocks.get(block_idx)
                    && let Some(opt) = pb.values.get(offset)
                    && let Some(v) = opt.clone()
                {
                    props_vec.push(v);
                    continue;
                }
                props_vec.push(minigu_common::value::ScalarValue::Null);
            }
        }
        // Push DelEdge with the previous edge id (old_commit_ts passed as timestamp)
        txn.push_undo(crate::common::DeltaOp::DelEdge(eid), old_commit_ts);

        // Mark edge as deleted (set commit_ts to txn_id)
        let mut edge_blocks = self.edges.write().unwrap();
        let edge_block = &mut edge_blocks[block_idx];
        edge_block.edges[offset].commit_ts = txn.txn_id;

        edge_block.min_ts = edge_block.min_ts.min(txn.txn_id);
        edge_block.max_ts = edge_block.max_ts.max(txn.txn_id);

        Ok(())
    }
}

impl OlapGraph for OlapStorage {
    type Adjacency = OlapEdge;
    type AdjacencyIter<'a> = AdjacencyIterator<'a>;
    type AdjacencyIterAtTs<'a> = AdjacencyIteratorAtTs<'a>;
    type Edge = OlapEdge;
    // TODO: type EdgeID = EdgeId;
    type EdgeID = EdgeId;
    type EdgeIter<'a> = EdgeIter<'a>;
    type EdgeIterAtTs<'a> = EdgeIterAtTs<'a>;
    type Transaction = crate::ap::transaction::MemTransaction;
    type Vertex = OlapVertex;
    type VertexID = VertexId;
    type VertexIter<'a> = VertexIter<'a>;

    fn get_vertex(
        &self,
        _txn: &Self::Transaction,
        id: Self::VertexID,
    ) -> StorageResult<Self::Vertex> {
        // 1. Find dense mapping id
        let logical_id = match self.dense_id_map.get(&id) {
            Some(mapping) => *mapping.value() as usize,
            None => {
                return Err(StorageError::from(VertexNotFound(format!(
                    "Vertex {id} not found"
                ))));
            }
        };

        // 2. Directly access vertex data without lock
        let borrow = self.vertices.read().unwrap();
        let vertex = borrow.get(logical_id).ok_or_else(|| {
            StorageError::EdgeNotFound(EdgeNotFound(format!("Edge {id} not found")))
        })?;

        // 3. Clone and return the vertex data
        Ok(vertex.clone())
    }

    fn get_edge(&self, _txn: &Self::Transaction, eid: Self::EdgeID) -> StorageResult<Self::Edge> {
        // Use EdgeId mapping for fast lookup
        let (block_idx, offset) = *self
            .edge_id_map
            .get(&eid)
            .ok_or_else(|| {
                StorageError::EdgeNotFound(EdgeNotFound(format!("Edge {} not found", eid)))
            })?
            .value();

        let edges = self.edges.read().unwrap();
        let block = edges.get(block_idx).ok_or_else(|| {
            StorageError::EdgeNotFound(EdgeNotFound(format!("Edge block {} not found", block_idx)))
        })?;

        if block.is_tombstone || offset >= block.edge_counter {
            return Err(StorageError::EdgeNotFound(EdgeNotFound(format!(
                "Edge {} not found",
                eid
            ))));
        }

        let edge = &block.edges[offset];
        if edge.eid != eid {
            return Err(StorageError::EdgeNotFound(EdgeNotFound(format!(
                "Edge {} not found",
                eid
            ))));
        }

        let edge_with_props = OlapEdge {
            label_id: edge.label_id,
            src_id: block.src_id,
            dst_id: edge.dst_id,
            properties: {
                let mut props = OlapPropertyStore {
                    properties: Vec::new(),
                };
                for (col_idx, column) in self.property_columns.read().unwrap().iter().enumerate() {
                    if let Some(val) = column
                        .blocks
                        .get(block_idx)
                        .and_then(|blk| blk.values.get(offset))
                        .cloned()
                    {
                        props.set_prop(col_idx, val);
                    }
                }
                props
            },
        };
        Ok(edge_with_props)
    }

    fn get_edge_at_ts(
        &self,
        txn: &Self::Transaction,
        eid: Self::EdgeID,
    ) -> StorageResult<Option<Self::Edge>> {
        // Use EdgeId mapping for fast lookup
        let (block_idx, offset) = match self.edge_id_map.get(&eid) {
            Some(loc) => *loc.value(),
            None => return Ok(None),
        };

        let edges = self.edges.read().unwrap();
        let block = match edges.get(block_idx) {
            Some(b) => b,
            None => return Ok(None),
        };

        if block.is_tombstone || offset >= block.edge_counter {
            return Ok(None);
        }

        if txn.txn_id.raw() < block.min_ts.raw() {
            return Ok(None);
        }

        let edge = &block.edges[offset];
        if edge.eid != eid {
            return Ok(None);
        }

        let is_visible = if let Some(commit_ts) = txn.commit_ts.get() {
            if edge.commit_ts.is_txn_id() {
                edge.commit_ts.raw() == txn.txn_id.raw()
            } else {
                edge.commit_ts.raw() <= commit_ts.raw()
            }
        } else {
            edge.commit_ts.is_txn_id() && (edge.commit_ts.raw() == txn.txn_id.raw())
        };

        if !is_visible {
            return Ok(None);
        }

        let edge_with_props = OlapEdge {
            label_id: edge.label_id,
            src_id: block.src_id,
            dst_id: edge.dst_id,
            properties: {
                let mut props = OlapPropertyStore {
                    properties: Vec::new(),
                };
                for (col_idx, column) in self.property_columns.read().unwrap().iter().enumerate() {
                    if let Some(val) = column
                        .blocks
                        .get(block_idx)
                        .and_then(|blk| blk.values.get(offset))
                        .cloned()
                    {
                        props.set_prop(col_idx, val);
                    }
                }
                props
            },
        };
        Ok(Some(edge_with_props))
    }

    fn iter_vertices<'a>(
        &'a self,
        _txn: &'a Self::Transaction,
    ) -> StorageResult<Self::VertexIter<'a>> {
        Ok(VertexIter {
            storage: self,
            idx: 0,
        })
    }

    fn iter_edges<'a>(&'a self, _txn: &'a Self::Transaction) -> StorageResult<Self::EdgeIter<'a>> {
        Ok(EdgeIter {
            storage: self,
            block_idx: 0,
            offset: 0,
        })
    }

    fn iter_edges_at_ts<'a>(
        &'a self,
        txn: &'a Self::Transaction,
    ) -> StorageResult<Self::EdgeIterAtTs<'a>> {
        Ok(EdgeIterAtTs {
            storage: self,
            block_idx: 0,
            offset: 0,
            txn_id: Some(txn.txn_id),
            commit_ts: txn.commit_ts.clone(),
        })
    }

    fn iter_adjacency<'a>(
        &'a self,
        _txn: &'a Self::Transaction,
        vid: Self::VertexID,
    ) -> StorageResult<Self::AdjacencyIter<'a>> {
        let vertex = self.get_vertex(_txn, vid)?;

        Ok(AdjacencyIterator {
            storage: self,
            vertex_id: vid,
            block_idx: vertex.block_offset,
            offset: 0,
        })
    }

    fn iter_adjacency_at_ts<'a>(
        &'a self,
        txn: &'a Self::Transaction,
        vid: Self::VertexID,
    ) -> StorageResult<Self::AdjacencyIterAtTs<'a>> {
        let vertex = self.get_vertex(txn, vid)?;
        Ok(AdjacencyIteratorAtTs {
            storage: self,
            vertex_id: vid,
            block_idx: vertex.block_offset,
            offset: 0,
            txn_id: Some(txn.txn_id),
            commit_ts: txn.commit_ts.clone(),
        })
    }
}

impl MutOlapGraph for OlapStorage {
    fn create_vertex(
        &self,
        _txn: &Self::Transaction,
        vertex: Self::Vertex,
    ) -> StorageResult<Self::VertexID> {
        let mut clone = vertex.clone();
        clone.block_offset = usize::MAX;
        // 1. Check whether vertex has existed
        let is_existed = self.dense_id_map.contains_key(&vertex.vid);

        if is_existed {
            Err(StorageError::VertexNotFound(VertexNotFound(format!(
                "Vertex {} is existed",
                vertex.vid
            ))))
        } else {
            // 2. Allocate logical id
            let index = self
                .logic_id_counter
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            self.dense_id_map.insert(vertex.vid, index);
            // 3. Insert vertex on index position
            let vid = clone.vid;
            self.vertices.write().unwrap().insert(index as usize, clone);
            Ok(vid)
        }
    }

    fn create_edge(
        &self,
        _txn: &Self::Transaction,
        edge: Self::Edge,
    ) -> StorageResult<Self::EdgeID> {
        // 1. Found vertex
        let dense_id = *self.dense_id_map.get(&edge.src_id).ok_or_else(|| {
            StorageError::VertexNotFound(VertexNotFound(format!(
                "Source vertex {} not found",
                edge.src_id
            )))
        })?;
        let mut binding = self.vertices.write().unwrap();
        let vertex = binding.get_mut(dense_id as usize).ok_or_else(|| {
            StorageError::VertexNotFound(VertexNotFound(format!(
                "Source vertex {} not found",
                edge.src_id
            )))
        })?;

        // 2. Initial block (lazy load) if not exists
        if vertex.block_offset == usize::MAX {
            // Ignore currency problems temporarily
            let index = self.edges.read().unwrap().len();
            self.edges.write().unwrap().push(EdgeBlock {
                pre_block_index: None,
                cur_block_index: index,
                is_tombstone: false,
                max_label_id: NonZeroU32::new(1),
                min_label_id: NonZeroU32::new(u32::MAX),
                max_dst_id: 0,
                min_dst_id: u64::MAX,
                min_ts: Timestamp::max_commit_ts(),
                max_ts: Timestamp::with_ts(0),
                edge_counter: 0,
                src_id: edge.src_id,
                edges: [OlapStorageEdge::default(); BLOCK_CAPACITY],
            });
            vertex.block_offset = index;
        } else {
            // 3. Allocate new block if is full
            let edge_count = self
                .edges
                .read()
                .unwrap()
                .get(vertex.block_offset)
                .ok_or_else(|| {
                    StorageError::EdgeNotFound(EdgeNotFound(format!(
                        "Vertex {} not found",
                        vertex.vid
                    )))
                })?
                .edge_counter;
            if edge_count >= BLOCK_CAPACITY {
                let index = self.edges.read().unwrap().len();
                self.edges.write().unwrap().push(EdgeBlock {
                    pre_block_index: Option::from(vertex.block_offset),
                    cur_block_index: index,
                    is_tombstone: false,
                    max_label_id: NonZeroU32::new(1),
                    min_label_id: NonZeroU32::new(u32::MAX),
                    max_dst_id: 0,
                    min_dst_id: u64::MAX,
                    min_ts: Timestamp::max_commit_ts(),
                    max_ts: Timestamp::with_ts(0),
                    src_id: edge.src_id,
                    edge_counter: 0,
                    edges: [OlapStorageEdge::default(); BLOCK_CAPACITY],
                });
                vertex.block_offset = index;
            }
        }

        // 4. Allocate global unique EdgeId
        let eid = self.edge_id_counter.fetch_add(1, Ordering::SeqCst);

        // 5. Insert edge
        // 5.1 Calculate position
        let mut binding = self.edges.write().unwrap();
        let block = binding.get_mut(vertex.block_offset).ok_or_else(|| {
            StorageError::EdgeNotFound(EdgeNotFound(format!(
                "Edge block for vertex {} not found",
                vertex.vid
            )))
        })?;
        let insert_pos = block.edges[..block.edge_counter]
            .binary_search_by_key(&(&edge.dst_id, &edge.label_id), |e| {
                (&e.dst_id, &e.label_id)
            })
            .unwrap_or_else(|e| e);

        // 5.2 Move elements
        for i in (insert_pos..block.edge_counter).rev() {
            block.edges[i + 1] = block.edges[i];
        }
        block.edge_counter += 1;

        // 5.3 Actual insert with eid
        block.edges[insert_pos] = OlapStorageEdge {
            eid,
            label_id: edge.label_id,
            dst_id: edge.dst_id,
            commit_ts: Timestamp::with_ts(0),
        };

        // Build EdgeId to location mapping
        self.edge_id_map
            .insert(eid, (vertex.block_offset, insert_pos));

        // 5. Insert properties
        for (i, column) in self
            .property_columns
            .write()
            .unwrap()
            .iter_mut()
            .enumerate()
        {
            // 5.1 Get property block or allocate one
            let property_block = if let Some(block) = column.blocks.get_mut(vertex.block_offset) {
                block
            } else {
                column.blocks.insert(
                    vertex.block_offset,
                    PropertyBlock {
                        values: vec![None; BLOCK_CAPACITY],
                        min_ts: Timestamp::with_ts(0),
                        max_ts: Timestamp::max_commit_ts(),
                    },
                );
                column.blocks.get_mut(vertex.block_offset).unwrap()
            };

            // 5.2 Move property elements
            for j in (insert_pos..block.edge_counter - 1).rev() {
                property_block.values[j + 1] = property_block.values[j].clone();
            }

            // 5.3 Insert property
            if let Some(property_value) = edge.properties.get(i) {
                property_block.values[insert_pos] = Some(property_value);
            } else {
                property_block.values[insert_pos] = None;
            }
        }

        // 6.Update block header
        block.min_dst_id = edge.dst_id.min(block.min_dst_id);
        block.max_dst_id = edge.dst_id.max(block.max_dst_id);
        block.max_label_id = edge.label_id.max(block.max_label_id);
        block.min_label_id = edge.label_id.min(block.min_label_id);

        Ok(eid)
    }

    fn delete_vertex(&self, _txn: &Self::Transaction, vid: Self::VertexID) -> StorageResult<()> {
        let mut vertex_iter = self.iter_vertices(_txn)?;
        let mut is_found: bool = false;
        for vertex in vertex_iter.by_ref() {
            if vertex?.vid == vid {
                is_found = true;
                break;
            }
        }

        if !is_found {
            return Err(StorageError::VertexNotFound(VertexNotFound(format!(
                "Vertex {vid} not found"
            ))));
        }

        let index = vertex_iter.idx - 1usize;

        let vertex = self.vertices.read().unwrap().get(index).cloned().unwrap();
        self.vertices.write().unwrap().remove(index);

        let mut current_block_index = Some(vertex.block_offset);
        let mut edge_blocks = self.edges.write().unwrap();
        while let Some(block_index) = current_block_index {
            // Set tombstone
            let edge_block = &mut edge_blocks[block_index];
            edge_block.is_tombstone = true;
            current_block_index = edge_block.pre_block_index;
        }

        Ok(())
    }

    fn delete_edge(&self, _txn: &Self::Transaction, eid: Self::EdgeID) -> StorageResult<()> {
        // Use EdgeId mapping for fast lookup
        let (block_idx, offset) = *self
            .edge_id_map
            .get(&eid)
            .ok_or_else(|| {
                StorageError::EdgeNotFound(EdgeNotFound(format!("Edge {} not found", eid)))
            })?
            .value();

        // Remove edge
        let mut edge_blocks = self.edges.write().unwrap();
        let edge_block = &mut edge_blocks[block_idx];

        if offset >= edge_block.edge_counter || edge_block.edges[offset].eid != eid {
            return Err(StorageError::EdgeNotFound(EdgeNotFound(format!(
                "Edge {} not found",
                eid
            ))));
        }

        let edges = &mut edge_block.edges;

        edge_block.edge_counter -= 1;

        if edge_block.edge_counter == 0 {
            edge_block.is_tombstone = true;
            self.edge_id_map.remove(&eid);
            return Ok(());
        }

        for i in offset..edge_block.edge_counter {
            // Update mapping for moved edges
            if edges[i + 1].eid != 0 {
                self.edge_id_map.insert(edges[i + 1].eid, (block_idx, i));
            }
            edges[i] = edges[i + 1];
        }

        edges[edge_block.edge_counter] = OlapStorageEdge {
            eid: 0,
            label_id: NonZeroU32::new(1),
            dst_id: 1,
            commit_ts: Timestamp::with_ts(0),
        };

        // Remove from mapping
        self.edge_id_map.remove(&eid);

        // Remove property
        let mut property_cols = self.property_columns.write().unwrap();
        for property_col in property_cols.iter_mut() {
            if let Some(property_block) = property_col.blocks.get_mut(block_idx) {
                let values = &mut property_block.values;
                for i in offset..edge_block.edge_counter {
                    if i + 1 < values.len() {
                        values[i] = values[i + 1].clone();
                    }
                }
                if edge_block.edge_counter < values.len() {
                    values[edge_block.edge_counter] = None;
                }
            }
        }

        Ok(())
    }

    fn set_vertex_property(
        &self,
        _txn: &Self::Transaction,
        vid: Self::VertexID,
        indices: Vec<usize>,
        props: Vec<ScalarValue>,
    ) -> StorageResult<()> {
        let logical_id = self.dense_id_map.get(&vid);
        if logical_id.is_none() {
            return Err(StorageError::VertexNotFound(VertexNotFound(format!(
                "Vertex ID {vid} not found"
            ))));
        }
        let logical_id = *logical_id.unwrap();

        let mut vertices = self.vertices.write().unwrap();
        let vertex = &mut vertices[logical_id as usize];

        for (index, prop) in indices.into_iter().zip(props.into_iter()) {
            vertex.properties.set_prop(index, prop);
        }

        Ok(())
    }

    fn set_edge_property(
        &self,
        _txn: &Self::Transaction,
        eid: Self::EdgeID,
        indices: Vec<usize>,
        props: Vec<ScalarValue>,
    ) -> StorageResult<()> {
        // Use EdgeId mapping for fast lookup
        let (block_idx, offset) = *self
            .edge_id_map
            .get(&eid)
            .ok_or_else(|| {
                StorageError::EdgeNotFound(EdgeNotFound(format!("Edge {} not found", eid)))
            })?
            .value();

        let edges = self.edges.read().unwrap();
        let block = edges.get(block_idx).ok_or_else(|| {
            StorageError::EdgeNotFound(EdgeNotFound(format!("Edge block {} not found", block_idx)))
        })?;

        if block.is_tombstone || offset >= block.edge_counter {
            return Err(StorageError::EdgeNotFound(EdgeNotFound(format!(
                "Edge {} not found",
                eid
            ))));
        }

        if block.edges[offset].eid != eid {
            return Err(StorageError::EdgeNotFound(EdgeNotFound(format!(
                "Edge {} not found",
                eid
            ))));
        }
        drop(edges);

        // Update properties
        let mut property_column = self.property_columns.write().unwrap();
        for (index, prop) in indices.into_iter().zip(props.into_iter()) {
            if let Some(column) = property_column.get_mut(index)
                && let Some(block) = column.blocks.get_mut(block_idx)
                && offset < block.values.len()
            {
                block.values[offset] = Some(prop);
            }
        }
        Ok(())
    }
}
