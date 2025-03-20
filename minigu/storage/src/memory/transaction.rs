use std::sync::{atomic::{AtomicU64, Ordering}, Arc};

use common::datatype::types::{EdgeId, VertexId};
use dashmap::{DashMap, DashSet};

use crate::{error::{StorageError, StorageResult}, model::{edge::Edge, vertex::Vertex}, storage::StorageTransaction, transaction::IsolationLevel};

use super::memory_graph::{MemGraph, VersionedAdjEntry};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Represents a commit timestamp used for multi-version concurrency control (MVCC).
pub struct CommitTimestamp(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Represents a unique transaction identifier.
pub struct TransactionId(u64);

impl TransactionId {
    const START: u64 = 1 << 63;

    /// Generates a new transaction ID, ensuring atomicity using an atomic counter.
    pub fn new() -> Self {
        // Static counter initialized once, persists between calls. 
        static COUNTER: AtomicU64 = AtomicU64::new(TransactionId::START);
        // Transaction ID only needs to be atomically incremented
        // and does not require strict memory order.
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl CommitTimestamp {
    /// Generates a new commit timestamp using an atomic counter.
    pub fn new() -> Self {
        // Static counter initialized once, persists between calls. 
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        // Only one transaction can commit at a time.
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    /// Returns the maximum possible commit timestamp.
    pub fn max() -> Self {
        Self(u64::MAX)
    }
}

#[derive(Debug, Clone)]
/// Represents an undo log entry for multi-version concurrency control.
pub(super) struct UndoEntry<T> {
    data: T,                   // Previous version of the data
    begin_ts: CommitTimestamp, // Timestamp when this version became valid
    end_ts: CommitTimestamp,   // Timestamp when this version became invalid
}

impl<T> UndoEntry<T> {
    /// Create a UndoEntry
    pub(super) fn new(data: T, begin_ts: CommitTimestamp, end_ts: CommitTimestamp) -> Self {
        Self {
            data,
            begin_ts,
            end_ts,
        }
    }

    /// Get the data of the undo entry.
    pub(super) fn data(&self) -> &T {
        &self.data
    }

    /// Get the begin timestamp of the undo entry.
    pub(super) fn begin_ts(&self) -> CommitTimestamp {
        self.begin_ts
    }

    /// Get the end timestamp of the undo entry.
    pub(super) fn end_ts(&self) -> CommitTimestamp {
        self.end_ts
    }
}

pub struct MemTransaction {
    graph: Arc<MemGraph>, // Reference to the associated in-memory graph

    // ---- Transaction Config ----
    isolation_level: IsolationLevel, // Isolation level of the transaction

    // ---- Timestamp management ----
    start_ts: CommitTimestamp, // Start timestamp assigned when the transaction begins
    commit_ts: Option<CommitTimestamp>, // Commit timestamp assigned upon committing
    tx_id: TransactionId,      // Unique transaction identifier

    // ---- Read sets ----
    pub(super) vertex_reads: DashSet<VertexId>, // Set of vertices read by this transaction
    pub(super) edge_reads: DashSet<EdgeId>,     // Set of edges read by this transaction

    // ---- Write sets ----
    pub(super) vertex_writes: DashSet<VertexId>, // Set of vertices written by this transaction
    pub(super) edge_writes: DashSet<EdgeId>,     // Set of edges written by this transaction

    // ---- Undo logs ----
    pub(super) vertex_undos: DashMap<VertexId, Vec<UndoEntry<Vertex>>>, // Vertex undo logs
    pub(super) edge_undos: DashMap<EdgeId, Vec<UndoEntry<Edge>>>,       // Edge undo logs
}

impl MemTransaction {
    pub(super) fn with_memgraph(
        graph: Arc<MemGraph>,
        txn_id: TransactionId,
        start_ts: CommitTimestamp,
        isolation_level: IsolationLevel
    ) -> Self {
        Self {
            graph,
            isolation_level,
            start_ts,
            commit_ts: None,
            tx_id: txn_id,
            vertex_reads: DashSet::new(),
            edge_reads: DashSet::new(),
            vertex_writes: DashSet::new(),
            edge_writes: DashSet::new(),
            vertex_undos: DashMap::new(),
            edge_undos: DashMap::new(),
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
                .ok_or(StorageError::VertexNotFound(vid.to_string()))?;

            let current = entry.chain.current.read().unwrap();
            // Check if the vertex was modified after the transaction started.
            if current.commit_ts > self.start_ts
                || (current.modified_by != Some(self.tx_id) && current.modified_by != None)
            {
                return Err(StorageError::ReadConflict(vid.to_string()));
            }
        }

        // Validate edge read set
        for eid in self.edge_reads.iter() {
            let entry = self
                .graph
                .edges
                .get(&eid)
                .ok_or(StorageError::EdgeNotFound(eid.to_string()))?;

            let current = entry.chain.current.read().unwrap();
            // Check if the edge was modified after the transaction started.
            if current.commit_ts > self.start_ts
                || (current.modified_by != Some(self.tx_id) && current.modified_by != None)
            {
                return Err(StorageError::ReadConflict(eid.to_string()));
            }
        }

        Ok(())
    }

    /// Invalidates adjacency entries when a vertex is deleted.
    /// This marks all outgoing and incoming edges as deleted by updating their end timestamps.
    fn invalidate_adjacency_by_vid(
        &self,
        vid: u64,
        commit_ts: CommitTimestamp,
    ) -> StorageResult<()> {
        let max_commit_ts = CommitTimestamp::max();

        // Invalidate outgoing edges
        if let Some(mut entries) = self.graph.adjacency_out.get_mut(&vid) {
            for entry in entries.iter_mut() {
                if entry.end_ts() == max_commit_ts {
                    entry.end_ts = commit_ts;
                }
            }
        }

        // Invalidate incoming edges
        if let Some(mut entries) = self.graph.adjacency_in.get_mut(&vid) {
            for entry in entries.iter_mut() {
                if entry.end_ts() == max_commit_ts {
                    entry.end_ts = commit_ts;
                }
            }
        }

        Ok(())
    }

    /// Updates adjacency lists when a new edge is added.
    /// Ensures that old adjacency entries are invalidated before inserting new ones.
    fn update_adjacency_by_eid(&self, eid: &u64, commit_ts: CommitTimestamp) -> StorageResult<()> {
        let (src_id, dst_id) = {
            let entry = self
                .graph
                .edges
                .get(eid)
                .ok_or(StorageError::EdgeNotFound(eid.to_string()))?;
            let edge = entry.chain.current.read().unwrap().data.clone();
            (edge.src_id(), edge.dst_id())
        };

        // Update outgoing adjacency list: Invalidate old entry + Insert new entry
        self.graph
            .adjacency_out
            .entry(src_id)
            .and_modify(|entries| {
                if let Some(pos) = entries.iter().rposition(|e| e.edge_id() == *eid) {
                    // Step 1: Invalidate last matching old entry
                    entries[pos].end_ts = commit_ts;
                }
                // Step 2: Insert a new version entry
                entries.push(VersionedAdjEntry::new(*eid, dst_id, commit_ts));
            })
            .or_insert_with(|| vec![VersionedAdjEntry::new(*eid, dst_id, commit_ts)]);

        // Update incoming adjacency list
        self.graph
            .adjacency_in
            .entry(dst_id)
            .and_modify(|entries| {
                if let Some(pos) = entries.iter().rposition(|e| e.edge_id() == *eid) {
                    // Invalidate last matching old entry
                    entries[pos].end_ts = commit_ts;
                }
                // Insert a new version entry
                entries.push(VersionedAdjEntry::new(*eid, src_id, commit_ts));
            })
            .or_insert_with(|| vec![VersionedAdjEntry::new(*eid, src_id, commit_ts)]);

        Ok(())
    }

    /// Invalidates all adjacency list entries for a given edge.
    /// Updates the end timestamp of affected adjacency entries to mark them as deleted.
    fn invalidate_adjacency_by_eid(
        &self,
        eid: &EdgeId,
        commit_ts: CommitTimestamp,
    ) -> StorageResult<()> {
        let (src_id, dst_id) = {
            let entry = self
                .graph
                .edges
                .get(eid)
                .ok_or(StorageError::EdgeNotFound(eid.to_string()))?;
            let edge = entry.chain.current.read().unwrap().data.clone();
            (edge.src_id(), edge.dst_id())
        };

        // Invalidate outgoing adjacency list
        if let Some(mut entries) = self.graph.adjacency_out.get_mut(&src_id) {
            if let Some(pos) = entries.iter().rposition(|e| e.edge_id() == *eid) {
                entries[pos].end_ts = commit_ts;
            }
        }

        // Invalidate incoming adjacency list
        if let Some(mut entries) = self.graph.adjacency_in.get_mut(&dst_id) {
            if let Some(pos) = entries.iter().rposition(|e| e.edge_id() == *eid) {
                entries[pos].end_ts = commit_ts;
            }
        }

        Ok(())
    }

    /// Rolls back modifications to vertices.
    /// Restores the last committed version of each modified vertex.
    fn rollback_vertex_writes(&self) {
        let mut to_remove: Vec<VertexId> = Vec::new();
        for vid in self.vertex_writes.iter() {
            if let Some(entry) = self.graph.vertices.get(&vid) {
                // Restore current vertex from transaction's undo log (using the latest undo entry).
                if let Some(vertex_undo_entries) = self.vertex_undos.get(&vid) {
                    let mut current = entry.chain.current.write().unwrap();
                    let last_undo = vertex_undo_entries[0].clone();
                    current.data = last_undo.data;
                    current.commit_ts = last_undo.begin_ts;
                    current.modified_by = None;
                } else {
                    // If no undo entry exists, the vertex was created by this transaction.
                    to_remove.push(*vid);
                }
            }
        }
        // Remove newly created vertices.
        for vid in to_remove.iter() {
            self.graph.vertices.remove(vid);
        }
    }

    /// Rolls back modifications to edges.
    /// Restores the last committed version of each modified edge.
    fn rollback_edge_writes(&self) {
        let mut to_remove: Vec<EdgeId> = Vec::new();
        for eid in self.edge_writes.iter() {
            if let Some(entry) = self.graph.edges.get(&eid) {
                if let Some(edge_undo_entries) = self.edge_undos.get(&eid) {
                    let mut current = entry.chain.current.write().unwrap();
                    let last_undo = edge_undo_entries[0].clone();
                    current.data = last_undo.data;
                    current.commit_ts = last_undo.begin_ts;
                    current.modified_by = None;
                } else {
                    // If no undo entry exists, the edge was created by this transaction.
                    to_remove.push(*eid);
                }
            }
        }
        // Remove newly created edges.
        for eid in to_remove.iter() {
            self.graph.edges.remove(eid);
        }
    }

    /// Returns the start timestamp of the transaction.
    pub fn start_ts(&self) -> CommitTimestamp {
        self.start_ts
    }

    /// Returns the transaction ID.
    pub fn tx_id(&self) -> TransactionId {
        self.tx_id
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
    pub fn graph(&self) -> &Arc<MemGraph> {
        &self.graph
    }

    /// Returns the isolution level
    pub fn isolation_level(&self) -> &IsolationLevel {
        &self.isolation_level
    }
}

impl StorageTransaction for MemTransaction {
    type CommitTimestamp = CommitTimestamp;

    /// Commits the transaction, applying all changes atomically.
    /// Ensures serializability, updates version chains, and manages adjacency lists.
    fn commit(&mut self) -> StorageResult<CommitTimestamp> {
        // Acquire the global commit lock to enforce serial execution of commits.
        let _guard = self.graph.commit_lock.lock().unwrap();

        // Step 1: Validate serializability if isolution level is Serializable.
        if let IsolationLevel::Serializable = self.isolation_level {
            if let Err(e) = self.validate_read_sets() {
                self.abort()?;
                return Err(e);
            }
        }

        // Step 2: Assign a commit timestamp (atomic operation).
        let commit_ts = CommitTimestamp::new();
        self.commit_ts = Some(commit_ts);

        // Step 3: Process vertex write set.
        {
            for vid in self.vertex_writes.iter() {
                let entry = self
                    .graph
                    .vertices
                    .get(&vid)
                    .ok_or(StorageError::VertexNotFound(vid.to_string()))?;

                // Acquire write lock and update version chain.
                let mut current = entry.chain.current.write().unwrap();
                current.modified_by = None; // Remove modification marker.
                current.commit_ts = commit_ts; // Assign the final commit timestamp.

                // Merge undo logs.
                let mut undo_list = entry.chain.undo_lists.write().unwrap();
                if let Some(mut undo_entries) = self.vertex_undos.get_mut(&vid) {
                    for undo_entry in undo_entries.iter_mut() {
                        undo_entry.end_ts = commit_ts; // Finalize undo log timestamps.
                        undo_list.push(undo_entry.clone());
                    }
                }

                // Invalidate adjacency list entries if the vertex is deleted.
                if current.data.is_tombstone() {
                    self.invalidate_adjacency_by_vid(*vid, commit_ts)?;
                }
            }
        }

        // Step 4: Process edge write set.
        {
            for eid in self.edge_writes.iter() {
                let entry = self
                    .graph
                    .edges
                    .get(&eid)
                    .ok_or(StorageError::EdgeNotFound(eid.to_string()))?;

                // Update the current version.
                {
                    let mut current = entry.chain.current.write().unwrap();
                    current.modified_by = None; // Remove modification marker.
                    current.commit_ts = commit_ts; // Assign the final commit timestamp.
                } // Drop the write lock on current.

                // Merge undo logs.
                {
                    let mut undo_list = entry.chain.undo_lists.write().unwrap();
                    if let Some(mut undo_entries) = self.edge_undos.get_mut(&eid) {
                        for undo_entry in undo_entries.iter_mut() {
                            undo_entry.end_ts = commit_ts; // Finalize undo log timestamps.
                            undo_list.push(undo_entry.clone());
                        }
                    }
                }

                // Update adjacency list entries.
                let is_tombstone = {
                    let current = entry.chain.current.read().unwrap();
                    current.data.is_tombstone()
                };

                if is_tombstone {
                    self.invalidate_adjacency_by_eid(&eid, commit_ts)?;
                } else {
                    self.update_adjacency_by_eid(&eid, commit_ts)?;
                }
            }
        }

        // Step 5: Clean up transaction state.
        self.graph.active_txns.remove(&self.start_ts);
        Ok(commit_ts)
    }

    /// Aborts the transaction, rolling back all changes.
    fn abort(&self) -> StorageResult<()> {
        // Roll back all vertex modifications.
        self.rollback_vertex_writes();
        // Roll back all edge modifications.
        self.rollback_edge_writes();

        // Remove the transaction from the active transaction list.
        self.graph.active_txns.remove(&self.start_ts);
        Ok(())
    }
}