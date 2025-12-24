use std::num::NonZeroU32;
use std::sync::{Arc, OnceLock};

use minigu_transaction::{IsolationLevel, Timestamp, Transaction, global_timestamp_generator};

use crate::ap::olap_graph::{BLOCK_CAPACITY, OlapStorage, OlapStorageEdge};
use crate::common::DeltaOp;
use crate::error::{StorageError, StorageResult, TransactionError};

/// Minimal AP transaction that performs in-memory commit/abort
/// Behavior:
/// - Uses a txn id (Timestamp) to mark uncommitted entries in blocks
/// - On commit, allocates a commit_ts and replaces commit_ts fields equal to txn_id with the
///   assigned commit_ts, and updates block `min_ts`/`max_ts` accordingly.
pub struct MemTransaction {
    pub storage: Arc<OlapStorage>,
    pub txn_id: Timestamp,
    pub start_ts: Timestamp,
    pub isolation_level: IsolationLevel,
    pub commit_ts: OnceLock<Timestamp>,
    /// Undo buffer: a sequence of DeltaOp timestamps recorded by the transaction.
    /// For this minimal implementation we store pairs of (DeltaOp, timestamp)
    pub undo_buffer: parking_lot::RwLock<Vec<(DeltaOp, Timestamp)>>,
}

impl MemTransaction {
    pub fn new(
        storage: Arc<OlapStorage>,
        txn_id: Timestamp,
        start_ts: Timestamp,
        isolation_level: IsolationLevel,
    ) -> Self {
        Self {
            storage,
            txn_id,
            start_ts,
            isolation_level,
            commit_ts: OnceLock::new(),
            undo_buffer: parking_lot::RwLock::new(Vec::new()),
        }
    }

    /// Minimal commit: allocate commit_ts and apply in-memory replacements.
    pub fn commit_at(&self, commit_ts_opt: Option<Timestamp>) -> StorageResult<Timestamp> {
        let commit_ts = if let Some(ts) = commit_ts_opt {
            global_timestamp_generator()
                .update_if_greater(ts)
                .map_err(TransactionError::Timestamp)?;
            ts
        } else {
            global_timestamp_generator()
                .next()
                .map_err(TransactionError::Timestamp)?
        };

        if self.commit_ts.set(commit_ts).is_err() {
            return Err(StorageError::Transaction(
                TransactionError::TransactionAlreadyCommitted(format!("{:?}", commit_ts)),
            ));
        }

        // Walk undo buffer and for create/set/del edge ops, replace commit_ts markers
        let undo_entries = self.undo_buffer.read().clone();
        for (op, _ts) in undo_entries.iter() {
            match op {
                DeltaOp::CreateEdge(edge) => {
                    let src = edge.src_id;
                    self.replace_edge_commit_ts_by_label(src, Some(edge.label_id), commit_ts);
                }
                DeltaOp::CreateEdgeAp(edge) => {
                    let src = edge.src_id;
                    self.replace_edge_commit_ts_by_label(src, Some(edge.label_id), commit_ts);
                }
                DeltaOp::SetEdgePropsAp(src, label, _setop, _old_ts) => {
                    self.replace_edge_commit_ts_by_label(*src, *label, commit_ts);
                }
                DeltaOp::DelEdgeAp(src, label, _dst, _props, _old_ts) => {
                    self.replace_edge_commit_ts_by_label(*src, *label, commit_ts);
                }
                _ => {}
            }
        }

        Ok(commit_ts)
    }

    fn replace_edge_commit_ts_by_label(
        &self,
        src_id: u64,
        label: Option<std::num::NonZeroU32>,
        commit_ts: Timestamp,
    ) {
        let mut edges = self.storage.edges.write().unwrap();
        // iterate all blocks for simplicity; optimize later
        for block in edges.iter_mut() {
            if block.is_tombstone {
                continue;
            }
            if block.src_id != src_id {
                continue;
            }
            // update block-level min/max
            for i in 0..block.edge_counter {
                if block.edges[i].label_id == label && block.edges[i].commit_ts == self.txn_id {
                    block.edges[i].commit_ts = commit_ts;
                }
            }
        }
    }

    pub fn abort(&self) -> StorageResult<()> {
        // Apply undo entries in reverse order
        let mut buffer = self.undo_buffer.write();
        let entries = buffer.clone();
        for (op, _ts) in entries.iter().rev() {
            match op {
                DeltaOp::CreateEdge(edge) | DeltaOp::CreateEdgeAp(edge) => {
                    // Undo a creation -> remove the created edge
                    let src = edge.src_id;
                    let label = Some(edge.label_id);
                    let dst = edge.dst_id;
                    let mut edges = self.storage.edges.write().unwrap();
                    for block in edges.iter_mut() {
                        if block.is_tombstone || block.src_id != src {
                            continue;
                        }
                        // find matching entry
                        for i in 0..block.edge_counter {
                            if block.edges[i].label_id == label
                                && block.edges[i].dst_id == dst
                                && block.edges[i].commit_ts == self.txn_id
                            {
                                // remove it
                                for j in i..block.edge_counter - 1 {
                                    block.edges[j] = block.edges[j + 1];
                                }
                                block.edge_counter -= 1;
                                block.edges[block.edge_counter] = OlapStorageEdge {
                                    label_id: NonZeroU32::new(1),
                                    dst_id: 1,
                                    commit_ts: Timestamp::with_ts(0),
                                };
                                break;
                            }
                        }
                    }
                }
                DeltaOp::DelEdgeAp(src, label, dst, props, old_ts) => {
                    // Undo a deletion -> re-insert the old edge and restore properties
                    let mut edges = self.storage.edges.write().unwrap();
                    // find block for src (use first block matching src)
                    let mut block_idx_opt: Option<usize> = None;
                    for (idx, block) in edges.iter().enumerate() {
                        if !block.is_tombstone && block.src_id == *src {
                            block_idx_opt = Some(idx);
                            break;
                        }
                    }
                    // if not found, create new block
                    if block_idx_opt.is_none() {
                        let idx = edges.len();
                        edges.push(crate::ap::olap_graph::EdgeBlock {
                            pre_block_index: None,
                            cur_block_index: idx,
                            is_tombstone: false,
                            max_label_id: NonZeroU32::new(1),
                            min_label_id: NonZeroU32::new(u32::MAX),
                            max_dst_id: 0,
                            min_dst_id: u64::MAX,
                            min_ts: *old_ts,
                            max_ts: *old_ts,
                            src_id: *src,
                            edge_counter: 0,
                            edges: [OlapStorageEdge {
                                label_id: NonZeroU32::new(1),
                                dst_id: 1,
                                commit_ts: Timestamp::with_ts(0),
                            }; BLOCK_CAPACITY],
                        });
                        block_idx_opt = Some(idx);
                    }
                    let block_idx = block_idx_opt.unwrap();
                    let block = &mut edges[block_idx];
                    // insert in order by dst,label
                    let mut insert_pos = block.edge_counter;
                    for i in 0..block.edge_counter {
                        if (block.edges[i].dst_id, block.edges[i].label_id) > (*dst, *label) {
                            insert_pos = i;
                            break;
                        }
                    }
                    for i in (insert_pos..block.edge_counter).rev() {
                        block.edges[i + 1] = block.edges[i];
                    }
                    block.edge_counter += 1;
                    block.edges[insert_pos] = crate::ap::olap_graph::OlapStorageEdge {
                        label_id: *label,
                        dst_id: *dst,
                        commit_ts: *old_ts,
                    };
                    // restore properties
                    let mut prop_cols = self.storage.property_columns.write().unwrap();
                    for (col_idx, col) in prop_cols.iter_mut().enumerate() {
                        if col.blocks.get(block_idx).is_none() {
                            col.blocks.insert(
                                block_idx,
                                crate::ap::olap_graph::PropertyBlock {
                                    values: vec![None; crate::ap::olap_graph::BLOCK_CAPACITY],
                                    min_ts: *old_ts,
                                    max_ts: *old_ts,
                                },
                            );
                        }
                        let pb = &mut col.blocks[block_idx];
                        // copy from props (PropertyRecord)
                        if let Some(val) = props.get(col_idx) {
                            pb.values[insert_pos] = Some(val.clone());
                        } else {
                            pb.values[insert_pos] = None;
                        }
                    }
                }
                DeltaOp::SetEdgePropsAp(src, label, setop, old_ts) => {
                    // restore old property values and commit_ts
                    let mut edges = self.storage.edges.write().unwrap();
                    for block in edges.iter_mut() {
                        if block.is_tombstone || block.src_id != *src {
                            continue;
                        }
                        for i in 0..block.edge_counter {
                            if block.edges[i].label_id == *label {
                                // restore props
                                let mut prop_cols = self.storage.property_columns.write().unwrap();
                                for (k, idx) in setop.indices.iter().enumerate() {
                                    if prop_cols.get(*idx).is_none() {
                                        continue;
                                    }
                                    let column = &mut prop_cols[*idx];
                                    if column.blocks.get(block.cur_block_index).is_none() {
                                        column.blocks.insert(
                                            block.cur_block_index,
                                            crate::ap::olap_graph::PropertyBlock {
                                                values: vec![
                                                    None;
                                                    crate::ap::olap_graph::BLOCK_CAPACITY
                                                ],
                                                min_ts: *old_ts,
                                                max_ts: *old_ts,
                                            },
                                        );
                                    }
                                    let pb = &mut column.blocks[block.cur_block_index];
                                    let old_val = setop.props[k].clone();
                                    pb.values[i] = Some(old_val);
                                }
                                // restore commit_ts
                                block.edges[i].commit_ts = *old_ts;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // clear undo buffer after abort
        buffer.clear();

        Ok(())
    }
}

// Lightweight helpers to record undo entries
impl MemTransaction {
    pub fn push_undo(&self, op: DeltaOp, ts: Timestamp) {
        self.undo_buffer.write().push((op, ts));
    }
}

impl Transaction for MemTransaction {
    type Error = StorageError;

    fn txn_id(&self) -> Timestamp {
        self.txn_id
    }

    fn start_ts(&self) -> Timestamp {
        self.start_ts
    }

    fn commit_ts(&self) -> Option<Timestamp> {
        self.commit_ts.get().copied()
    }

    fn isolation_level(&self) -> &IsolationLevel {
        &self.isolation_level
    }

    fn commit(&self) -> Result<Timestamp, Self::Error> {
        self.commit_at(None)
    }

    fn abort(&self) -> Result<(), Self::Error> {
        MemTransaction::abort(self)
    }
}
