use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::{Arc, OnceLock};

use minigu_common::types::{EdgeId, LabelId, VertexId};
use minigu_common::{IsolationLevel, Timestamp, global_timestamp_generator};

use crate::ap::olap_graph::{OlapStorage, OlapStorageEdge};
use crate::ap::olap_storage::StorageTransaction;
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
    /// Snapshots for edges soft-deleted in this txn (label_id, dst_id)
    pub deleted_edge_snapshot: parking_lot::RwLock<HashMap<EdgeId, (Option<LabelId>, VertexId)>>,
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
            deleted_edge_snapshot: parking_lot::RwLock::new(HashMap::new()),
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
        let mut edges = self.storage.edges.write().unwrap();
        for (op, _ts) in undo_entries.into_iter() {
            match op {
                DeltaOp::CreateEdge(edge) => {
                    // Use EdgeId mapping to find and update commit_ts
                    if let Some(loc) = self.storage.edge_id_map.get(&edge.eid()) {
                        let (block_idx, offset) = *loc.value();
                        if let Some(block) = edges.get_mut(block_idx)
                            && offset < block.edge_counter
                            && block.edges[offset].eid == edge.eid()
                            && block.edges[offset].commit_ts == self.txn_id
                        {
                            block.edges[offset].commit_ts = commit_ts;
                            block.min_ts = if block.min_ts.is_txn_id() {
                                commit_ts
                            } else {
                                block.min_ts.min(commit_ts)
                            };
                            block.max_ts = if block.max_ts.is_txn_id() {
                                commit_ts
                            } else {
                                block.max_ts.max(commit_ts)
                            };
                            // promote property versions written in this txn to committed
                            let mut prop_cols = self.storage.property_columns.write().unwrap();
                            for column in prop_cols.iter_mut() {
                                if let Some(pb) = column.blocks.get_mut(block_idx)
                                    && offset < pb.values.len()
                                    && let Some(last) = pb.values[offset].last_mut()
                                    && last.ts == self.txn_id
                                {
                                    last.ts = commit_ts;
                                    pb.min_ts = pb.min_ts.min(commit_ts);
                                    pb.max_ts = pb.max_ts.max(commit_ts);
                                }
                            }
                        }
                    }
                }
                DeltaOp::SetEdgeProps(eid, _) => {
                    // Use EdgeId mapping to find and update commit_ts
                    if let Some(loc) = self.storage.edge_id_map.get(&eid) {
                        let (block_idx, offset) = *loc.value();
                        if let Some(block) = edges.get_mut(block_idx)
                            && offset < block.edge_counter
                            && block.edges[offset].eid == eid
                            && block.edges[offset].commit_ts == self.txn_id
                        {
                            block.edges[offset].commit_ts = commit_ts;
                            block.min_ts = if block.min_ts.is_txn_id() {
                                commit_ts
                            } else {
                                block.min_ts.min(commit_ts)
                            };
                            block.max_ts = if block.max_ts.is_txn_id() {
                                commit_ts
                            } else {
                                block.max_ts.max(commit_ts)
                            };
                            // promote property versions written in this txn to committed
                            let mut prop_cols = self.storage.property_columns.write().unwrap();
                            for column in prop_cols.iter_mut() {
                                if let Some(pb) = column.blocks.get_mut(block_idx)
                                    && offset < pb.values.len()
                                    && let Some(last) = pb.values[offset].last_mut()
                                    && last.ts == self.txn_id
                                {
                                    last.ts = commit_ts;
                                    pb.min_ts = pb.min_ts.min(commit_ts);
                                    pb.max_ts = pb.max_ts.max(commit_ts);
                                }
                            }
                        }
                    }
                }
                DeltaOp::DelEdge(eid) => {
                    // Use EdgeId mapping to find and update commit_ts
                    if let Some(loc) = self.storage.edge_id_map.get(&eid) {
                        let (block_idx, offset) = *loc.value();
                        if let Some(block) = edges.get_mut(block_idx)
                            && offset < block.edge_counter
                            && block.edges[offset].eid == eid
                            && block.edges[offset].commit_ts == self.txn_id
                        {
                            block.edges[offset].commit_ts = commit_ts;
                            block.min_ts = if block.min_ts.is_txn_id() {
                                commit_ts
                            } else {
                                block.min_ts.min(commit_ts)
                            };
                            block.max_ts = if block.max_ts.is_txn_id() {
                                commit_ts
                            } else {
                                block.max_ts.max(commit_ts)
                            };
                        }
                    }
                }
                _ => {}
            }
        }

        // Clear deletion snapshots after commit bookkeeping
        self.deleted_edge_snapshot.write().clear();
        self.undo_buffer.write().clear();

        Ok(commit_ts)
    }

    pub fn abort(&self) -> StorageResult<()> {
        // Prevent abort from running after the transaction has been committed
        if let Some(commit_ts) = self.commit_ts.get().copied() {
            return Err(StorageError::Transaction(
                TransactionError::TransactionAlreadyCommitted(format!(
                    "abort called on already committed transaction at {:?}",
                    commit_ts
                )),
            ));
        }
        // Apply undo entries in reverse order
        let mut buffer = self.undo_buffer.write();
        let entries = buffer.clone();
        let mut edges = self.storage.edges.write().unwrap();
        for (op, old_ts) in entries.into_iter().rev() {
            match op {
                DeltaOp::CreateEdge(edge) => {
                    // Undo a creation -> remove the created edge using EdgeId
                    let eid = edge.eid();
                    if let Some(loc) = self.storage.edge_id_map.get(&eid) {
                        let (block_idx, offset) = *loc.value();
                        drop(loc);
                        if let Some(block) = edges.get_mut(block_idx)
                            && offset < block.edge_counter
                            && block.edges[offset].eid == eid
                            && block.edges[offset].commit_ts == self.txn_id
                        {
                            // remove it
                            for j in offset..block.edge_counter - 1 {
                                block.edges[j] = block.edges[j + 1];
                            }
                            block.edge_counter -= 1;
                            for i in offset..block.edge_counter {
                                let moved_eid = block.edges[i].eid;
                                if moved_eid != 0 {
                                    self.storage.edge_id_map.insert(moved_eid, (block_idx, i));
                                }
                            }
                            block.edges[block.edge_counter] = OlapStorageEdge {
                                eid: 0,
                                label_id: NonZeroU32::new(1),
                                dst_id: 1,
                                commit_ts: Timestamp::with_ts(0),
                            };
                            let mut property_cols = self.storage.property_columns.write().unwrap();
                            for property_col in property_cols.iter_mut() {
                                if let Some(property_block) = property_col.blocks.get_mut(block_idx)
                                {
                                    let values = &mut property_block.values;
                                    for i in offset..block.edge_counter {
                                        if i + 1 < values.len() {
                                            values[i] = values[i + 1].clone();
                                        }
                                    }
                                    if block.edge_counter < values.len() {
                                        values[block.edge_counter] = Vec::new();
                                    }
                                }
                            }
                            // Remove from mapping
                            self.storage.edge_id_map.remove(&eid);
                        }
                    }
                }
                DeltaOp::DelEdge(eid) => {
                    // Undo a deletion -> restore the old edge commit_ts
                    // Edge data and properties are still in storage. Restore commit_ts and
                    // label/dst from snapshot (if present). old_commit_ts is from undo entry.
                    if let Some(loc) = self.storage.edge_id_map.get(&eid) {
                        let (block_idx, offset) = *loc.value();
                        if let Some(block) = edges.get_mut(block_idx)
                            && offset < block.edge_counter
                            && block.edges[offset].eid == eid
                        {
                            // Restore edge commit_ts (properties are still in storage)
                            block.edges[offset].commit_ts = old_ts;
                            if let Some((label_id, dst_id)) =
                                self.deleted_edge_snapshot.read().get(&eid).cloned()
                            {
                                block.edges[offset].label_id = label_id;
                                block.edges[offset].dst_id = dst_id;
                            }
                        }
                    }
                }
                DeltaOp::SetEdgeProps(eid, props_op) => {
                    // Restore old property values and commit_ts using EdgeId
                    // old_commit_ts is obtained from undo entry's timestamp
                    if let Some(loc) = self.storage.edge_id_map.get(&eid) {
                        let (block_idx, offset) = *loc.value();
                        if let Some(block) = edges.get_mut(block_idx)
                            && offset < block.edge_counter
                            && block.edges[offset].eid == eid
                        {
                            // Restore props
                            let mut prop_cols = self.storage.property_columns.write().unwrap();
                            for idx in props_op.indices.iter() {
                                if prop_cols.get(*idx).is_none() {
                                    continue;
                                }
                                let column = &mut prop_cols[*idx];
                                if column.blocks.get(block_idx).is_none() {
                                    column.blocks.insert(
                                        block_idx,
                                        crate::ap::olap_graph::PropertyBlock {
                                            values: vec![
                                                Vec::new();
                                                crate::ap::olap_graph::BLOCK_CAPACITY
                                            ],
                                            min_ts: old_ts,
                                            max_ts: old_ts,
                                        },
                                    );
                                }
                                let pb = &mut column.blocks[block_idx];
                                pb.min_ts = pb.min_ts.min(old_ts);
                                pb.max_ts = pb.max_ts.max(old_ts);
                                pb.values[offset].retain(|v| v.ts != self.txn_id);
                            }
                            // Restore commit_ts
                            block.edges[offset].commit_ts = old_ts;
                        }
                    }
                }
                _ => {}
            }
        }

        // clear undo buffer after abort
        buffer.clear();
        self.deleted_edge_snapshot.write().clear();

        Ok(())
    }
}

// Lightweight helpers to record undo entries
impl MemTransaction {
    pub fn push_undo(&self, op: DeltaOp, ts: Timestamp) {
        self.undo_buffer.write().push((op, ts));
    }

    /// Record snapshot of an edge before soft deletion so abort can restore it.
    pub fn record_deleted_edge(&self, eid: EdgeId, label_id: Option<LabelId>, dst_id: VertexId) {
        self.deleted_edge_snapshot
            .write()
            .insert(eid, (label_id, dst_id));
    }
}

impl StorageTransaction for MemTransaction {
    type CommitTimestamp = Timestamp;

    fn txn_id(&self) -> Timestamp {
        self.txn_id
    }

    fn commit(&self) -> StorageResult<Timestamp> {
        self.commit_at(None)
    }

    fn abort(&self) -> StorageResult<()> {
        MemTransaction::abort(self)
    }
}
