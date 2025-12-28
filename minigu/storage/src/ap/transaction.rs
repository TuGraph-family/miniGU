use std::num::NonZeroU32;
use std::sync::{Arc, OnceLock};

use minigu_transaction::{IsolationLevel, Timestamp, Transaction, global_timestamp_generator};

use crate::ap::olap_graph::{OlapStorage, OlapStorageEdge};
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
        for (op, _ts) in undo_entries.into_iter() {
            match op {
                DeltaOp::CreateEdge(edge) => {
                    // Use EdgeId mapping to find and update commit_ts
                    if let Some(loc) = self.storage.edge_id_map.get(&edge.eid()) {
                        let (block_idx, offset) = *loc.value();
                        let mut edges = self.storage.edges.write().unwrap();
                        if let Some(block) = edges.get_mut(block_idx)
                            && offset < block.edge_counter
                            && block.edges[offset].eid == edge.eid()
                            && block.edges[offset].commit_ts == self.txn_id
                        {
                            block.edges[offset].commit_ts = commit_ts;
                        }
                    }
                }
                DeltaOp::SetEdgeProps(eid, _) => {
                    // Use EdgeId mapping to find and update commit_ts
                    if let Some(loc) = self.storage.edge_id_map.get(&eid) {
                        let (block_idx, offset) = *loc.value();
                        let mut edges = self.storage.edges.write().unwrap();
                        if let Some(block) = edges.get_mut(block_idx)
                            && offset < block.edge_counter
                            && block.edges[offset].eid == eid
                            && block.edges[offset].commit_ts == self.txn_id
                        {
                            block.edges[offset].commit_ts = commit_ts;
                        }
                    }
                }
                DeltaOp::DelEdge(eid) => {
                    // Use EdgeId mapping to find and update commit_ts
                    if let Some(loc) = self.storage.edge_id_map.get(&eid) {
                        let (block_idx, offset) = *loc.value();
                        let mut edges = self.storage.edges.write().unwrap();
                        if let Some(block) = edges.get_mut(block_idx)
                            && offset < block.edge_counter
                            && block.edges[offset].eid == eid
                            && block.edges[offset].commit_ts == self.txn_id
                        {
                            block.edges[offset].commit_ts = commit_ts;
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(commit_ts)
    }

    pub fn abort(&self) -> StorageResult<()> {
        // Apply undo entries in reverse order
        let mut buffer = self.undo_buffer.write();
        let entries = buffer.clone();
        for (op, old_ts) in entries.into_iter().rev() {
            match op {
                DeltaOp::CreateEdge(edge) => {
                    // Undo a creation -> remove the created edge using EdgeId
                    let eid = edge.eid();
                    if let Some(loc) = self.storage.edge_id_map.get(&eid) {
                        let (block_idx, offset) = *loc.value();
                        let mut edges = self.storage.edges.write().unwrap();
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
                            block.edges[block.edge_counter] = OlapStorageEdge {
                                eid: 0,
                                label_id: NonZeroU32::new(1),
                                dst_id: 1,
                                commit_ts: Timestamp::with_ts(0),
                            };
                            // Remove from mapping
                            self.storage.edge_id_map.remove(&eid);
                        }
                    }
                }
                DeltaOp::DelEdge(eid) => {
                    // Undo a deletion -> restore the old edge commit_ts
                    // Edge data and properties are still in storage, only commit_ts was marked
                    // old_commit_ts is obtained from undo entry's timestamp
                    if let Some(loc) = self.storage.edge_id_map.get(&eid) {
                        let (block_idx, offset) = *loc.value();
                        let mut edges = self.storage.edges.write().unwrap();
                        if let Some(block) = edges.get_mut(block_idx)
                            && offset < block.edge_counter
                            && block.edges[offset].eid == eid
                        {
                            // Restore edge commit_ts (properties are still in storage)
                            block.edges[offset].commit_ts = old_ts;
                        }
                    }
                }
                DeltaOp::SetEdgeProps(eid, props_op) => {
                    // Restore old property values and commit_ts using EdgeId
                    // old_commit_ts is obtained from undo entry's timestamp
                    if let Some(loc) = self.storage.edge_id_map.get(&eid) {
                        let (block_idx, offset) = *loc.value();
                        let mut edges = self.storage.edges.write().unwrap();
                        if let Some(block) = edges.get_mut(block_idx)
                            && offset < block.edge_counter
                            && block.edges[offset].eid == eid
                        {
                            // Restore props
                            let mut prop_cols = self.storage.property_columns.write().unwrap();
                            for (k, idx) in props_op.indices.iter().enumerate() {
                                if prop_cols.get(*idx).is_none() {
                                    continue;
                                }
                                let column = &mut prop_cols[*idx];
                                if column.blocks.get(block_idx).is_none() {
                                    column.blocks.insert(
                                        block_idx,
                                        crate::ap::olap_graph::PropertyBlock {
                                            values: vec![
                                                None;
                                                crate::ap::olap_graph::BLOCK_CAPACITY
                                            ],
                                            min_ts: old_ts,
                                            max_ts: old_ts,
                                        },
                                    );
                                }
                                let pb = &mut column.blocks[block_idx];
                                let old_val = props_op.props[k].clone();
                                pb.values[offset] = Some(old_val);
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
