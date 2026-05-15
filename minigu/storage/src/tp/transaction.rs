use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock, RwLock};

use dashmap::DashSet;
use minigu_common::types::{EdgeId, VertexId};
use minigu_common::value::ScalarValue;
use minigu_transaction::{
    GraphTxnManager, LockStrategy, Transaction, UndoEntry as GenericUndoEntry,
    UndoPtr as GenericUndoPtr, global_timestamp_generator,
};
pub use minigu_transaction::{IsolationLevel, Timestamp};

use super::memory_graph::{AdjacencyContainer, MemoryGraph, VersionedEdge, VersionedVertex};
use crate::common::model::edge::{Edge, Neighbor};
use crate::common::model::vertex::Vertex;
use crate::common::wal::graph_wal::{Operation, RedoEntry};
use crate::common::{DeltaOp, SetPropsOp};
use crate::error::{
    EdgeNotFoundError, StorageError, StorageResult, TransactionError, VertexNotFoundError,
};

/// Type alias for storage-specific undo entry
pub type UndoEntry = GenericUndoEntry<DeltaOp>;

/// Type alias for storage-specific undo pointer
pub type UndoPtr = GenericUndoPtr<DeltaOp>;

#[derive(Clone)]
pub enum WriteKind {
    InsertVertex(Vertex),
    UpdateVertex { before: Vertex, after: Vertex },
    DeleteVertex { before: Vertex },
    InsertEdge(Edge),
    UpdateEdge { before: Edge, after: Edge },
    DeleteEdge { before: Edge },
}

#[derive(Clone)]
pub struct WriteIntent {
    pub guard_ts: Timestamp,
    pub kind: WriteKind,
}

pub struct MemTransaction {
    graph: Arc<MemoryGraph>, // Reference to the associated in-memory graph

    // ---- Transaction Config ----
    isolation_level: IsolationLevel, // Isolation level of the transaction
    lock_strategy: LockStrategy,     // Locking strategy (pessimistic or optimistic)

    // ---- Timestamp management ----
    /// Start timestamp assigned when the transaction begins
    start_ts: Timestamp,
    commit_ts: OnceLock<Timestamp>, // Commit timestamp assigned upon committing
    txn_id: Timestamp,              // Unique transaction identifier

    // ---- Read sets ----
    pub(super) vertex_reads: DashSet<VertexId>, // Set of vertices read by this transaction
    pub(super) edge_reads: DashSet<EdgeId>,     // Set of edges read by this transaction

    // ---- OCC write intents ----
    pub(super) vertex_writes: RwLock<HashMap<VertexId, WriteIntent>>,
    pub(super) edge_writes: RwLock<HashMap<EdgeId, WriteIntent>>,

    // ---- Undo logs ----
    pub(super) undo_buffer: RwLock<Vec<Arc<UndoEntry>>>,

    // ---- Write-ahead-log for crash recovery ----
    pub(super) redo_buffer: RwLock<Vec<RedoEntry>>,

    // ---- Transaction state tracking ----
    /// Flag to track whether the transaction has been explicitly handled (committed or aborted)
    is_handled: Arc<AtomicBool>,
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
        self.commit_at(None, false)
    }

    fn abort(&self) -> Result<(), Self::Error> {
        self.abort_at(false)
    }
}

impl MemTransaction {
    pub(super) fn with_memgraph(
        graph: Arc<MemoryGraph>,
        txn_id: Timestamp,
        start_ts: Timestamp,
        isolation_level: IsolationLevel,
        lock_strategy: LockStrategy,
    ) -> Self {
        Self {
            graph,
            isolation_level,
            lock_strategy,
            start_ts,
            commit_ts: OnceLock::new(),
            txn_id,
            vertex_reads: DashSet::new(),
            edge_reads: DashSet::new(),
            vertex_writes: RwLock::new(HashMap::new()),
            edge_writes: RwLock::new(HashMap::new()),
            undo_buffer: RwLock::new(Vec::new()),
            redo_buffer: RwLock::new(Vec::new()),
            is_handled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Validates the read set to ensure serializability.
    /// If a vertex or edge has been modified since the transaction started, it returns a read
    /// conflict error.
    pub(super) fn validate_read_sets(&self) -> StorageResult<()> {
        // Validate vertex read set
        for vid in self.vertex_reads.iter() {
            let entry = self
                .graph
                .vertices
                .get(&vid)
                .ok_or(StorageError::VertexNotFound(
                    VertexNotFoundError::VertexNotFound(vid.to_string()),
                ))?;

            let current = entry.chain.current.read().unwrap();
            // Check if the vertex was modified after the transaction started.
            if current.commit_ts != self.txn_id && current.commit_ts > self.start_ts {
                return Err(StorageError::Transaction(
                    TransactionError::ReadWriteConflict(format!(
                        "Vertex is being modified by transaction {:?}",
                        current.commit_ts
                    )),
                ));
            }
        }

        // Validate edge read set
        for eid in self.edge_reads.iter() {
            let entry = self
                .graph
                .edges
                .get(&eid)
                .ok_or(StorageError::EdgeNotFound(EdgeNotFoundError::EdgeNotFound(
                    eid.to_string(),
                )))?;

            let current = entry.chain.current.read().unwrap();
            // Check if the edge was modified after the transaction started.
            if current.commit_ts != self.txn_id && current.commit_ts > self.start_ts {
                return Err(StorageError::Transaction(
                    TransactionError::ReadWriteConflict(format!(
                        "Edge is being modified by transaction {:?}",
                        current.commit_ts
                    )),
                ));
            }
        }

        Ok(())
    }

    /// Returns the set of vertex reads in this transaction.
    pub fn vertex_reads(&self) -> &DashSet<VertexId> {
        &self.vertex_reads
    }

    /// Returns the set of edge reads in this transaction.
    pub fn edge_reads(&self) -> &DashSet<EdgeId> {
        &self.edge_reads
    }

    /// Returns a reference to the associated graph.
    pub fn graph(&self) -> &Arc<MemoryGraph> {
        &self.graph
    }

    /// Returns the configured lock strategy.
    pub fn lock_strategy(&self) -> LockStrategy {
        self.lock_strategy
    }

    /// Looks up a write intent for a specific vertex.
    pub fn lookup_vertex_write(&self, vid: VertexId) -> Option<WriteIntent> {
        self.vertex_writes.read().unwrap().get(&vid).cloned()
    }

    /// Looks up a write intent for a specific edge.
    pub fn lookup_edge_write(&self, eid: EdgeId) -> Option<WriteIntent> {
        self.edge_writes.read().unwrap().get(&eid).cloned()
    }

    pub fn record_vertex_update(
        &self,
        vid: VertexId,
        guard_ts: Timestamp,
        before: Vertex,
        new_after: Vertex,
    ) {
        let mut ws = self.vertex_writes.write().unwrap();
        ws.entry(vid)
            .and_modify(|intent| {
                // Keep the first non-zero guard timestamp as conflict-check baseline.
                if intent.guard_ts.raw() == 0 {
                    intent.guard_ts = guard_ts;
                }
                match intent.kind {
                    // Insert followed by update stays as insert with refreshed image.
                    WriteKind::InsertVertex(ref mut v) => {
                        *v = new_after.clone();
                    }
                    // Collapse repeated updates into the latest "after" image.
                    WriteKind::UpdateVertex { ref mut after, .. } => {
                        *after = new_after.clone();
                    }
                    // Delete followed by update is represented as an update intent.
                    WriteKind::DeleteVertex { .. } => {
                        intent.kind = WriteKind::UpdateVertex {
                            before: before.clone(),
                            after: new_after.clone(),
                        };
                    }
                    _ => {}
                }
            })
            .or_insert(WriteIntent {
                guard_ts,
                kind: WriteKind::UpdateVertex {
                    before,
                    after: new_after,
                },
            });
    }

    pub fn record_vertex_delete(&self, vid: VertexId, guard_ts: Timestamp, before: Vertex) {
        let mut ws = self.vertex_writes.write().unwrap();
        if let Some(intent) = ws.get_mut(&vid) {
            // Preserve earliest meaningful guard timestamp for OCC validation.
            if intent.guard_ts.raw() == 0 {
                intent.guard_ts = guard_ts;
            }
            // Delete dominates previous vertex write intents.
            intent.kind = WriteKind::DeleteVertex { before };
        } else {
            ws.insert(
                vid,
                WriteIntent {
                    guard_ts,
                    kind: WriteKind::DeleteVertex { before },
                },
            );
        }
    }

    pub fn record_edge_update(
        &self,
        eid: EdgeId,
        guard_ts: Timestamp,
        before: Edge,
        new_after: Edge,
    ) {
        let mut ws = self.edge_writes.write().unwrap();
        ws.entry(eid)
            .and_modify(|intent| {
                // Keep the first non-zero guard timestamp as conflict-check baseline.
                if intent.guard_ts.raw() == 0 {
                    intent.guard_ts = guard_ts;
                }
                match intent.kind {
                    // Insert followed by update stays as insert with refreshed image.
                    WriteKind::InsertEdge(ref mut e) => {
                        *e = new_after.clone();
                    }
                    // Collapse repeated updates into the latest "after" image.
                    WriteKind::UpdateEdge { ref mut after, .. } => {
                        *after = new_after.clone();
                    }
                    // Delete followed by update is represented as an update intent.
                    WriteKind::DeleteEdge { .. } => {
                        intent.kind = WriteKind::UpdateEdge {
                            before: before.clone(),
                            after: new_after.clone(),
                        };
                    }
                    _ => {}
                }
            })
            .or_insert(WriteIntent {
                guard_ts,
                kind: WriteKind::UpdateEdge {
                    before,
                    after: new_after,
                },
            });
    }

    pub fn record_edge_delete(&self, eid: EdgeId, guard_ts: Timestamp, before: Edge) {
        let mut ws = self.edge_writes.write().unwrap();
        if let Some(intent) = ws.get_mut(&eid) {
            // Preserve earliest meaningful guard timestamp for OCC validation.
            if intent.guard_ts.raw() == 0 {
                intent.guard_ts = guard_ts;
            }
            // Delete dominates previous edge write intents.
            intent.kind = WriteKind::DeleteEdge { before };
        } else {
            ws.insert(
                eid,
                WriteIntent {
                    guard_ts,
                    kind: WriteKind::DeleteEdge { before },
                },
            );
        }
    }

    /// Returns a reference to the undo buffer for garbage collection.
    pub fn undo_buffer(&self) -> &RwLock<Vec<Arc<UndoEntry>>> {
        &self.undo_buffer
    }

    /// Reconstructs a specific version of a Vertex or Edge
    /// based on the undo chain and a target timestamp
    pub(super) fn apply_deltas_for_read<T: FnMut(&UndoEntry)>(
        undo_ptr: UndoPtr,
        mut callback: T,
        txn_start_ts: Timestamp,
    ) {
        let mut undo_ptr = undo_ptr;

        // Get the undo buffer of the transaction that modified the vertex/edge
        while let Some(undo_entry) = undo_ptr.upgrade() {
            // Apply the delta to the vertex/edge
            callback(&undo_entry);

            // If the timestamp of the entry is less than the txn_start_ts,
            // it means current version is the latest visible version,
            // no need to continue traversing the undo chain
            if undo_entry.timestamp() < txn_start_ts {
                break;
            }
            undo_ptr = undo_entry.next();
        }
    }

    /// Marks the transaction as handled (committed or aborted).
    /// This prevents the automatic rollback in the Drop implementation.
    pub fn mark_handled(&self) {
        self.is_handled.store(true, Ordering::Release);
    }

    /// Commits the transaction at a specific commit timestamp.
    pub fn commit_at(
        &self,
        commit_ts: Option<Timestamp>,
        skip_wal: bool,
    ) -> StorageResult<Timestamp> {
        let commit_ts = if let Some(commit_ts) = commit_ts {
            global_timestamp_generator()
                .update_if_greater(commit_ts)
                .map_err(TransactionError::Timestamp)?;
            commit_ts
        } else {
            global_timestamp_generator()
                .next()
                .map_err(TransactionError::Timestamp)?
        };

        let _guard = self.graph.txn_manager.commit_lock.lock().unwrap();

        // Step 1: Validate serializability if isolution level is Serializable.
        if let IsolationLevel::Serializable = self.isolation_level
            && let Err(e) = self.validate_read_sets()
        {
            self.abort()?;
            return Err(e);
        }

        let result = match self.lock_strategy {
            LockStrategy::Pessimistic => self.commit_pessimistic(commit_ts, skip_wal),
            LockStrategy::Optimistic => self.commit_optimistic(commit_ts, skip_wal),
        };

        if result.is_ok() {
            self.is_handled.store(true, Ordering::Release);
        }

        result
    }

    pub fn abort_at(&self, skip_wal: bool) -> StorageResult<()> {
        // Acquire write lock and drain the undo buffer
        let undo_entries: Vec<_> = self.undo_buffer.write().unwrap().drain(..).collect();

        // Process all undo entries
        for undo_entry in undo_entries.into_iter() {
            let commit_ts = undo_entry.timestamp();
            let next = undo_entry.next();
            match undo_entry.delta() {
                DeltaOp::CreateVertex(vertex) => {
                    // For newly created vertices, remove or mark as deleted
                    let vid = vertex.vid();
                    if let Some(entry) = self.graph.vertices.get(&vid) {
                        let mut current = entry.chain.current.write().unwrap();
                        if current.commit_ts == self.txn_id() {
                            // If created by current transaction, restore original state
                            current.data = vertex.clone();
                            current.data.is_tombstone = false;
                            current.commit_ts = commit_ts;
                            *entry.chain.undo_ptr.write().unwrap() = next;
                        }
                    }
                }
                DeltaOp::CreateEdge(edge) => {
                    // For newly created edges, remove or mark as deleted
                    let eid = edge.eid();
                    if let Some(entry) = self.graph.edges.get(&eid) {
                        let mut current = entry.chain.current.write().unwrap();
                        if current.commit_ts == self.txn_id() {
                            // If created by current transaction, restore original state
                            current.data = edge.clone();
                            current.data.is_tombstone = false;
                            current.commit_ts = commit_ts;
                            *entry.chain.undo_ptr.write().unwrap() = next;
                        }
                    }
                }
                DeltaOp::SetVertexProps(vid, SetPropsOp { indices, props }) => {
                    // For property modifications, determine if it's a vertex or edge based on
                    // entity_id Restore vertex properties
                    if let Some(entry) = self.graph.vertices.get(vid) {
                        let mut current = entry.chain.current.write().unwrap();
                        if current.commit_ts == self.txn_id() {
                            // Restore properties
                            current.data.set_props(indices, props.clone());
                            current.commit_ts = commit_ts;
                            // Update undo pointer to previous version
                            *entry.chain.undo_ptr.write().unwrap() = next;
                        }
                    }
                }
                DeltaOp::SetEdgeProps(eid, SetPropsOp { indices, props }) => {
                    // Restore edge properties
                    if let Some(entry) = self.graph.edges.get(eid) {
                        let mut current = entry.chain.current.write().unwrap();
                        if current.commit_ts == self.txn_id() {
                            // Restore properties
                            current.data.set_props(indices, props.clone());
                            current.commit_ts = commit_ts;
                            // Update undo pointer to previous version
                            *entry.chain.undo_ptr.write().unwrap() = next;
                        }
                    }
                }
                DeltaOp::DelVertex(vid) => {
                    // Restore vertex
                    if let Some(entry) = self.graph.vertices.get(vid) {
                        let mut current = entry.chain.current.write().unwrap();
                        if current.commit_ts == self.txn_id() {
                            // Restore deletion flag
                            current.data.is_tombstone = true;
                            current.commit_ts = commit_ts;
                            // Update undo pointer to previous version
                            *entry.chain.undo_ptr.write().unwrap() = next;
                        }
                    }
                }
                DeltaOp::DelEdge(eid) => {
                    // Restore edge
                    if let Some(entry) = self.graph.edges.get(eid) {
                        let mut current = entry.chain.current.write().unwrap();
                        if current.commit_ts == self.txn_id() {
                            // Restore deletion flag
                            current.data.is_tombstone = true;
                            current.commit_ts = commit_ts;
                            // Update undo pointer to previous version
                            *entry.chain.undo_ptr.write().unwrap() = next;
                        }
                    }
                }
                DeltaOp::AddLabel(_) => todo!(),
                DeltaOp::RemoveLabel(_) => todo!(),
            }
        }

        // Write `Operation::AbortTransaction` to WAL,
        // unless the function is called when recovering from WAL
        if !skip_wal {
            let lsn = self.graph.persistence.next_lsn();
            let wal_entry = RedoEntry {
                lsn,
                txn_id: self.txn_id(),
                iso_level: self.isolation_level,
                op: Operation::AbortTransaction,
            };
            self.graph.persistence.append_wal(&wal_entry)?;
            self.graph.persistence.flush_wal()?;
        }

        self.vertex_writes.write().unwrap().clear();
        self.edge_writes.write().unwrap().clear();

        // Remove transaction from transaction manager
        self.graph.txn_manager.finish_transaction(self)?;

        // Mark the transaction as handled
        self.is_handled.store(true, Ordering::Release);

        Ok(())
    }

    fn commit_pessimistic(&self, commit_ts: Timestamp, skip_wal: bool) -> StorageResult<Timestamp> {
        // Mark transaction commit timestamp first; fail fast on duplicate commit.
        if let Err(e) = self.commit_ts.set(commit_ts) {
            self.abort()?;
            return Err(StorageError::Transaction(
                TransactionError::TransactionAlreadyCommitted(format!("{:?}", e)),
            ));
        }

        {
            // Promote all records touched under txn_id lock to final commit_ts.
            macro_rules! update_commit_ts {
                ($self:expr, $entity_type:ident, $id:expr) => {
                    $self
                        .graph()
                        .$entity_type()
                        .get($id)
                        .unwrap()
                        .current()
                        .write()
                        .unwrap()
                        .commit_ts = commit_ts
                };
            }

            let undo_entries = self.undo_buffer.read().unwrap().clone();
            for undo_entry in undo_entries.iter() {
                match undo_entry.delta() {
                    DeltaOp::DelVertex(vid) => update_commit_ts!(self, vertices, vid),
                    DeltaOp::DelEdge(eid) => update_commit_ts!(self, edges, eid),
                    DeltaOp::CreateVertex(vertex) => {
                        update_commit_ts!(self, vertices, &vertex.vid())
                    }
                    DeltaOp::CreateEdge(edge) => update_commit_ts!(self, edges, &edge.eid()),
                    DeltaOp::SetVertexProps(vid, _) => update_commit_ts!(self, vertices, vid),
                    DeltaOp::SetEdgeProps(eid, _) => update_commit_ts!(self, edges, eid),
                    DeltaOp::AddLabel(_) => todo!(),
                    DeltaOp::RemoveLabel(_) => todo!(),
                }
            }
        }

        self.finalize_commit_common(commit_ts, skip_wal)?;

        Ok(commit_ts)
    }

    fn commit_optimistic(&self, commit_ts: Timestamp, skip_wal: bool) -> StorageResult<Timestamp> {
        let graph = self.graph();

        let build_conflict = |msg: String| -> StorageError {
            StorageError::Transaction(TransactionError::WriteWriteConflict(msg))
        };

        // Phase 1: Validation
        let conflict_err = {
            let v_ws = self.vertex_writes.read().unwrap();
            let mut conflict: Option<StorageError> = None;

            for (vid, intent) in v_ws.iter() {
                match intent.kind {
                    WriteKind::InsertVertex(_) => {
                        // OCC insert requires "not exists and not locked by others".
                        if let Some(entry) = graph.vertices.get(vid) {
                            let cur = entry.chain.current.read().unwrap();
                            let cur_ts = cur.commit_ts;
                            if (cur_ts.is_txn_id() && cur_ts != self.txn_id())
                                || cur_ts.is_commit_ts()
                            {
                                conflict = Some(build_conflict(format!(
                                    "Vertex {} already exists or locked by {:?}",
                                    vid, cur_ts
                                )));
                                break;
                            }
                        }
                    }
                    WriteKind::UpdateVertex { .. } | WriteKind::DeleteVertex { .. } => {
                        // Update/Delete require stable base version matching intent.guard_ts.
                        let entry = if let Some(entry) = graph.vertices.get(vid) {
                            entry
                        } else {
                            // guard_ts==0 means "insert then delete in same txn"; missing row is
                            // valid.
                            if matches!(intent.kind, WriteKind::DeleteVertex { .. })
                                && intent.guard_ts.raw() == 0
                            {
                                continue;
                            }
                            conflict = Some(build_conflict(format!("Vertex {} missing", vid)));
                            break;
                        };
                        let cur = entry.chain.current.read().unwrap();
                        let cur_ts = cur.commit_ts;
                        if cur_ts.is_txn_id() && cur_ts != self.txn_id() {
                            conflict = Some(build_conflict(format!(
                                "Vertex {} locked by {:?}",
                                vid, cur_ts
                            )));
                            break;
                        }
                        if cur_ts.is_commit_ts() && cur_ts != intent.guard_ts {
                            conflict = Some(build_conflict(format!(
                                "Vertex {} changed since guard {:?} -> {:?}",
                                vid, intent.guard_ts, cur_ts
                            )));
                            break;
                        }
                    }
                    _ => {}
                }
            }

            drop(v_ws);

            if conflict.is_none() {
                let e_ws = self.edge_writes.read().unwrap();
                for (eid, intent) in e_ws.iter() {
                    match intent.kind {
                        WriteKind::InsertEdge(_) => {
                            // OCC insert requires "not exists and not locked by others".
                            if let Some(entry) = graph.edges.get(eid) {
                                let cur = entry.chain.current.read().unwrap();
                                let cur_ts = cur.commit_ts;
                                if (cur_ts.is_txn_id() && cur_ts != self.txn_id())
                                    || cur_ts.is_commit_ts()
                                {
                                    conflict = Some(build_conflict(format!(
                                        "Edge {} already exists or locked by {:?}",
                                        eid, cur_ts
                                    )));
                                    break;
                                }
                            }
                        }
                        WriteKind::UpdateEdge { .. } | WriteKind::DeleteEdge { .. } => {
                            // Update/Delete require stable base version matching intent.guard_ts.
                            let entry = if let Some(entry) = graph.edges.get(eid) {
                                entry
                            } else {
                                // guard_ts==0 means "insert then delete in same txn"; missing row
                                // is valid.
                                if matches!(intent.kind, WriteKind::DeleteEdge { .. })
                                    && intent.guard_ts.raw() == 0
                                {
                                    continue;
                                }
                                conflict = Some(build_conflict(format!("Edge {} missing", eid)));
                                break;
                            };
                            let cur = entry.chain.current.read().unwrap();
                            let cur_ts = cur.commit_ts;
                            if cur_ts.is_txn_id() && cur_ts != self.txn_id() {
                                conflict = Some(build_conflict(format!(
                                    "Edge {} locked by {:?}",
                                    eid, cur_ts
                                )));
                                break;
                            }
                            if cur_ts.is_commit_ts() && cur_ts != intent.guard_ts {
                                conflict = Some(build_conflict(format!(
                                    "Edge {} changed since guard {:?} -> {:?}",
                                    eid, intent.guard_ts, cur_ts
                                )));
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }

            conflict
        };

        if let Some(err) = conflict_err {
            self.abort()?;
            return Err(err);
        }

        // Helper to compute property delta for undo.
        let compute_prop_delta =
            |before: &[ScalarValue], after: &[ScalarValue]| -> (Vec<usize>, Vec<ScalarValue>) {
                let mut indices = Vec::new();
                let mut props = Vec::new();
                for (idx, (b, a)) in before.iter().zip(after.iter()).enumerate() {
                    if b != a {
                        indices.push(idx);
                        props.push(b.clone());
                    }
                }
                (indices, props)
            };

        // Phase 2: Apply write intents with short-term lock commit_ts = txn_id.
        {
            let mut undo_buf = self.undo_buffer.write().unwrap();
            let mut append_undo = |delta: DeltaOp,
                                   prev_ts: Timestamp,
                                   undo_ptr: UndoPtr,
                                   chain_undo: &RwLock<UndoPtr>| {
                let undo_entry = Arc::new(UndoEntry::new(delta, prev_ts, undo_ptr));
                undo_buf.push(undo_entry.clone());
                *chain_undo.write().unwrap() = Arc::downgrade(&undo_entry);
            };

            let v_ws = self.vertex_writes.read().unwrap();
            for (vid, intent) in v_ws.iter() {
                match &intent.kind {
                    WriteKind::InsertVertex(new_v) => {
                        // Materialize insert and add rollback delta (DelVertex).
                        let entry = graph
                            .vertices
                            .entry(*vid)
                            .or_insert_with(|| VersionedVertex::new(new_v.clone()));
                        let mut current = entry.chain.current.write().unwrap();

                        let prev_ts = current.commit_ts;
                        let undo_ptr = entry.chain.undo_ptr.read().unwrap().clone();

                        let delta = DeltaOp::DelVertex(*vid);
                        append_undo(delta, prev_ts, undo_ptr, &entry.chain.undo_ptr);

                        current.data = new_v.clone();
                        // Use txn_id as short-term write lock during apply phase.
                        current.commit_ts = self.txn_id();
                    }
                    WriteKind::UpdateVertex { before, after } => {
                        // Apply update and store property-level inverse delta for undo.
                        let entry = graph
                            .vertices
                            .get(vid)
                            .ok_or_else(|| build_conflict(format!("Vertex {} missing", vid)))?;
                        let mut current = entry.chain.current.write().unwrap();
                        let prev_ts = current.commit_ts;
                        let undo_ptr = entry.chain.undo_ptr.read().unwrap().clone();

                        let (indices, props) =
                            compute_prop_delta(before.properties(), after.properties());
                        let delta = DeltaOp::SetVertexProps(*vid, SetPropsOp { indices, props });
                        append_undo(delta, prev_ts, undo_ptr, &entry.chain.undo_ptr);

                        current.data = after.clone();
                        current.commit_ts = self.txn_id();
                    }
                    WriteKind::DeleteVertex { before } => {
                        // If missing and guard_ts==0, this is insert-then-delete in same txn.
                        // Recreate transient entry so undo/redo pipeline stays uniform.
                        let entry = if let Some(entry) = graph.vertices.get(vid) {
                            entry
                        } else if intent.guard_ts.raw() == 0 {
                            graph
                                .vertices
                                .insert(*vid, VersionedVertex::new(before.clone()));
                            graph
                                .vertices
                                .get(vid)
                                .ok_or_else(|| build_conflict(format!("Vertex {} missing", vid)))?
                        } else {
                            return Err(build_conflict(format!("Vertex {} missing", vid)));
                        };
                        let mut current = entry.chain.current.write().unwrap();
                        let prev_ts = current.commit_ts;
                        let undo_ptr = entry.chain.undo_ptr.read().unwrap().clone();

                        let delta = DeltaOp::CreateVertex(before.clone());
                        append_undo(delta, prev_ts, undo_ptr, &entry.chain.undo_ptr);

                        current.data = Vertex::tombstone(current.data.clone());
                        current.commit_ts = self.txn_id();
                    }
                    _ => {}
                }
            }

            let e_ws = self.edge_writes.read().unwrap();
            for (eid, intent) in e_ws.iter() {
                match &intent.kind {
                    WriteKind::InsertEdge(new_e) => {
                        // Materialize insert and add rollback delta (DelEdge).
                        let entry = graph
                            .edges
                            .entry(*eid)
                            .or_insert_with(|| VersionedEdge::new(new_e.clone()));
                        let mut current = entry.chain.current.write().unwrap();

                        let prev_ts = current.commit_ts;
                        let undo_ptr = entry.chain.undo_ptr.read().unwrap().clone();

                        let delta = DeltaOp::DelEdge(*eid);
                        append_undo(delta, prev_ts, undo_ptr, &entry.chain.undo_ptr);

                        current.data = new_e.clone();
                        current.commit_ts = self.txn_id();

                        // Maintain adjacency indexes alongside edge materialization.
                        graph
                            .adjacency_list
                            .entry(new_e.src_id())
                            .or_insert_with(AdjacencyContainer::new)
                            .outgoing()
                            .insert(Neighbor::new(new_e.label_id(), new_e.dst_id(), *eid));
                        graph
                            .adjacency_list
                            .entry(new_e.dst_id())
                            .or_insert_with(AdjacencyContainer::new)
                            .incoming()
                            .insert(Neighbor::new(new_e.label_id(), new_e.src_id(), *eid));
                    }
                    WriteKind::UpdateEdge { before, after } => {
                        // Apply update and store property-level inverse delta for undo.
                        let entry = graph
                            .edges
                            .get(eid)
                            .ok_or_else(|| build_conflict(format!("Edge {} missing", eid)))?;
                        let mut current = entry.chain.current.write().unwrap();
                        let prev_ts = current.commit_ts;
                        let undo_ptr = entry.chain.undo_ptr.read().unwrap().clone();

                        let (indices, props) =
                            compute_prop_delta(before.properties(), after.properties());
                        let delta = DeltaOp::SetEdgeProps(*eid, SetPropsOp { indices, props });
                        append_undo(delta, prev_ts, undo_ptr, &entry.chain.undo_ptr);

                        current.data = after.clone();
                        current.commit_ts = self.txn_id();
                    }
                    WriteKind::DeleteEdge { before } => {
                        // If missing and guard_ts==0, this is insert-then-delete in same txn.
                        // Recreate transient entry so undo/redo pipeline stays uniform.
                        let entry = if let Some(entry) = graph.edges.get(eid) {
                            entry
                        } else if intent.guard_ts.raw() == 0 {
                            graph.edges.insert(*eid, VersionedEdge::new(before.clone()));
                            graph
                                .edges
                                .get(eid)
                                .ok_or_else(|| build_conflict(format!("Edge {} missing", eid)))?
                        } else {
                            return Err(build_conflict(format!("Edge {} missing", eid)));
                        };
                        let mut current = entry.chain.current.write().unwrap();
                        let prev_ts = current.commit_ts;
                        let undo_ptr = entry.chain.undo_ptr.read().unwrap().clone();

                        let delta = DeltaOp::CreateEdge(before.clone());
                        append_undo(delta, prev_ts, undo_ptr, &entry.chain.undo_ptr);

                        current.data = Edge::tombstone(current.data.clone());
                        current.commit_ts = self.txn_id();

                        // Keep adjacency indexes consistent with edge tombstone.
                        let src_neighbor =
                            Neighbor::new(current.data.label_id(), current.data.dst_id(), *eid);
                        let dst_neighbor =
                            Neighbor::new(current.data.label_id(), current.data.src_id(), *eid);

                        graph
                            .adjacency_list
                            .entry(current.data.src_id())
                            .and_modify(|adj| {
                                adj.outgoing().remove(&src_neighbor);
                            });
                        graph
                            .adjacency_list
                            .entry(current.data.dst_id())
                            .and_modify(|adj| {
                                adj.incoming().remove(&dst_neighbor);
                            });
                    }
                    _ => {}
                }
            }
        }

        // Phase 3: finalize commit_ts on touched records.
        {
            let v_ws = self.vertex_writes.read().unwrap();
            for (vid, _) in v_ws.iter() {
                if let Some(entry) = graph.vertices.get(vid) {
                    let mut current = entry.chain.current.write().unwrap();
                    if current.commit_ts == self.txn_id() {
                        current.commit_ts = commit_ts;
                    }
                }
            }

            let e_ws = self.edge_writes.read().unwrap();
            for (eid, _) in e_ws.iter() {
                if let Some(entry) = graph.edges.get(eid) {
                    let mut current = entry.chain.current.write().unwrap();
                    if current.commit_ts == self.txn_id() {
                        current.commit_ts = commit_ts;
                    }
                }
            }
        }

        // Mark transaction committed after successful apply/finalize.
        if let Err(e) = self.commit_ts.set(commit_ts) {
            self.abort()?;
            return Err(StorageError::Transaction(
                TransactionError::TransactionAlreadyCommitted(format!("{:?}", e)),
            ));
        }

        self.finalize_commit_common(commit_ts, skip_wal)?;

        Ok(commit_ts)
    }

    /// Drains the transaction redo buffer, appends all delta WAL records plus the commit marker,
    /// flushes WAL to persistence, and updates the checkpoint trigger counter.
    fn flush_redo_and_commit_wal(&self, commit_ts: Timestamp) -> StorageResult<()> {
        let redo_entries = self
            .redo_buffer
            .write()
            .unwrap()
            .drain(..)
            .map(|mut entry| {
                entry.lsn = self.graph.persistence.next_lsn();
                entry
            })
            .collect::<Vec<_>>();
        let wal_count = redo_entries.len();
        for entry in redo_entries {
            self.graph.persistence.append_wal(&entry)?;
        }

        let wal_entry = RedoEntry {
            lsn: self.graph.persistence.next_lsn(),
            txn_id: self.txn_id(),
            iso_level: self.isolation_level,
            op: Operation::CommitTransaction(commit_ts),
        };
        self.graph.persistence.append_wal(&wal_entry)?;
        self.graph.persistence.flush_wal()?;

        for _ in 0..(wal_count + 2) {
            self.graph.increment_wal_counter();
        }

        Ok(())
    }

    /// Performs commit-finalization steps shared by pessimistic and optimistic paths:
    /// optional WAL flush, global commit timestamp publish, write-set cleanup, transaction finish,
    /// and optional auto-checkpoint check.
    fn finalize_commit_common(&self, commit_ts: Timestamp, skip_wal: bool) -> StorageResult<()> {
        if !skip_wal {
            self.flush_redo_and_commit_wal(commit_ts)?;
        }

        self.graph
            .txn_manager
            .latest_commit_ts
            .store(commit_ts.raw(), Ordering::SeqCst);

        self.vertex_writes.write().unwrap().clear();
        self.edge_writes.write().unwrap().clear();

        self.graph.txn_manager.finish_transaction(self)?;
        if !skip_wal {
            self.graph.check_auto_checkpoint()?;
        }

        Ok(())
    }
}

impl Drop for MemTransaction {
    fn drop(&mut self) {
        // Only perform automatic rollback if:
        // 1. The transaction hasn't been explicitly handled (committed or aborted)
        // 2. This is the last reference to the transaction
        // Note: We can't check Arc::strong_count here because we have a &mut self,
        // but the Drop will only be called when Arc count reaches 0
        if !self.is_handled.load(Ordering::Acquire) {
            // Attempt to abort the transaction
            // We ignore errors here since we're in a Drop implementation
            let _ = self.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use minigu_transaction::{GraphTxnManager, IsolationLevel};

    use super::*;
    use crate::tp::memory_graph;

    #[test]
    fn test_watermark_tracking() {
        let graph = memory_graph::tests::mock_empty_graph();
        let _base_commit_ts = graph.txn_manager.latest_commit_ts.load(Ordering::Acquire);

        // Start txn0
        let txn0 = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        let txn0_start_ts = txn0.start_ts().raw();

        // The watermark should be set to the start timestamp of the first active transaction
        assert_eq!(graph.txn_manager.low_watermark().raw(), txn0_start_ts);

        {
            let txn_store_1 = graph
                .txn_manager()
                .begin_transaction(IsolationLevel::Serializable)
                .unwrap();
            let txn1_start_ts = txn_store_1.start_ts().raw();
            // Ensure txn1 started after txn0
            assert!(txn1_start_ts > txn0_start_ts);
            let commit_ts = txn_store_1.commit().unwrap();
            // Ensure commit timestamp is greater than start timestamp
            assert!(commit_ts.raw() > txn1_start_ts);
        }

        // Watermark should remain unchanged since txn0 is still active
        assert_eq!(graph.txn_manager.low_watermark().raw(), txn0_start_ts);

        // Start txn1
        let txn1 = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        let txn1_start_ts = txn1.start_ts().raw();
        // Ensure txn1 starts after txn0
        assert!(txn1_start_ts > txn0_start_ts);

        // Watermark should remain unchanged (still pointing to txn0)
        assert_eq!(graph.txn_manager.low_watermark().raw(), txn0_start_ts);

        // Create and commit txn_store_2
        {
            let txn_store_2 = graph
                .txn_manager()
                .begin_transaction(IsolationLevel::Serializable)
                .unwrap();
            let txn2_start_ts = txn_store_2.start_ts().raw();
            // Ensure txn2 starts after txn1
            assert!(txn2_start_ts > txn1_start_ts);
            let commit_ts = txn_store_2.commit().unwrap();
            // Ensure commit timestamp is greater than start timestamp
            assert!(commit_ts.raw() > txn2_start_ts);
        }

        // Watermark should remain unchanged
        assert_eq!(graph.txn_manager.low_watermark().raw(), txn0_start_ts);

        // Start txn2
        let txn2 = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        let txn2_start_ts = txn2.start_ts().raw();
        // Ensure txn2 starts after txn1
        assert!(txn2_start_ts > txn1_start_ts);

        // Watermark should remain unchanged (still pointing to txn0)
        assert_eq!(graph.txn_manager.low_watermark().raw(), txn0_start_ts);

        // Abort txn0
        txn0.abort().unwrap();
        // Watermark should update to start_ts of txn1 (now the oldest active transaction)
        assert_eq!(graph.txn_manager.low_watermark().raw(), txn1_start_ts);

        // Create and commit txn_store_3
        {
            let txn_store_3 = graph
                .txn_manager()
                .begin_transaction(IsolationLevel::Serializable)
                .unwrap();
            let txn3_start_ts = txn_store_3.start_ts().raw();
            // Ensure txn3 starts after txn2
            assert!(txn3_start_ts > txn2_start_ts);
            let commit_ts = txn_store_3.commit().unwrap();
            // Ensure commit timestamp is greater than start timestamp
            assert!(commit_ts.raw() > txn3_start_ts);
        }

        // Watermark should remain unchanged (still pointing to txn1)
        assert_eq!(graph.txn_manager.low_watermark().raw(), txn1_start_ts);

        // Start txn3
        let txn3 = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        let txn3_start_ts = txn3.start_ts().raw();
        // Ensure txn3 starts after txn2
        assert!(txn3_start_ts > txn2_start_ts);

        // Watermark should remain unchanged (still pointing to txn1)
        assert_eq!(graph.txn_manager.low_watermark().raw(), txn1_start_ts);

        // Abort txn1
        txn1.abort().unwrap();
        // Watermark should be updated to txn2's start timestamp (now the oldest active)
        assert_eq!(graph.txn_manager.low_watermark().raw(), txn2_start_ts);

        // Abort txn2
        txn2.abort().unwrap();
        // Watermark should be updated to txn3's start timestamp (now the only active)
        assert_eq!(graph.txn_manager.low_watermark().raw(), txn3_start_ts);

        // Create and commit txn_store_4
        {
            let txn_store_4 = graph
                .txn_manager()
                .begin_transaction(IsolationLevel::Serializable)
                .unwrap();
            let txn4_start_ts = txn_store_4.start_ts().raw();
            // Ensure txn4 starts after txn3
            assert!(txn4_start_ts > txn3_start_ts);
            let commit_ts = txn_store_4.commit().unwrap();
            // Ensure commit timestamp is greater than start timestamp
            assert!(commit_ts.raw() > txn4_start_ts);
        }

        // Watermark should remain unchanged (still pointing to txn3)
        assert_eq!(graph.txn_manager.low_watermark().raw(), txn3_start_ts);

        // Start txn4
        let txn4 = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        let txn4_start_ts = txn4.start_ts().raw();
        // Ensure txn4 starts after txn3
        assert!(txn4_start_ts > txn3_start_ts);

        // Watermark should remain unchanged (still pointing to txn3)
        assert_eq!(graph.txn_manager.low_watermark().raw(), txn3_start_ts);

        // Abort txn3
        txn3.abort().unwrap();
        // Watermark should be updated to txn4's start timestamp (now the only active)
        assert_eq!(graph.txn_manager.low_watermark().raw(), txn4_start_ts);

        // Abort txn4
        txn4.abort().unwrap();
        // After all transactions are aborted, watermark should be updated
        let current_watermark = graph.txn_manager.low_watermark().raw();
        let latest_commit_ts = graph.txn_manager.latest_commit_ts.load(Ordering::Acquire);
        // Watermark should be at least the latest commit timestamp
        assert!(current_watermark >= latest_commit_ts);

        // Create and commit txn_store_5
        let last_commit_ts = {
            let txn_store_5 = graph
                .txn_manager()
                .begin_transaction(IsolationLevel::Serializable)
                .unwrap();
            let txn5_start_ts = txn_store_5.start_ts().raw();
            // Ensure txn5 starts after previous transactions
            assert!(txn5_start_ts > current_watermark);
            let commit_ts = txn_store_5.commit().unwrap();
            // Ensure commit timestamp is greater than start timestamp
            assert!(commit_ts.raw() > txn5_start_ts);
            commit_ts.raw()
        };

        // The watermark should be updated because there are no active transactions
        let final_watermark = graph.txn_manager.low_watermark().raw();
        // Watermark should be at least the latest commit timestamp
        assert!(final_watermark >= last_commit_ts);

        // Start txn5
        let txn5 = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        let txn5_start_ts = txn5.start_ts().raw();
        // Ensure txn5 starts after the last commit
        assert!(txn5_start_ts > last_commit_ts);

        // Watermark should now be set to txn5's start timestamp
        assert_eq!(graph.txn_manager.low_watermark().raw(), txn5_start_ts);

        // Abort txn5
        txn5.abort().unwrap();
        // After all transactions are aborted, watermark should be updated
        let final_watermark_after_all_aborted = graph.txn_manager.low_watermark().raw();
        // Watermark should be at least the latest commit timestamp
        assert!(final_watermark_after_all_aborted >= last_commit_ts);
    }
}
