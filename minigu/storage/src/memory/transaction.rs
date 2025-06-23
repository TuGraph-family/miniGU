use std::collections::HashMap;
use std::ops::Deref;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock, RwLock};

use crossbeam_skiplist::SkipMap;
use dashmap::DashSet;
use minigu_common::types::{EdgeId, VertexId};

use super::memory_graph::MemoryGraph;
use crate::error::{
    EdgeNotFoundError, StorageError, StorageResult, TransactionError, VertexNotFoundError,
};
use crate::model::edge::{Edge, Neighbor};
use crate::storage::StorageTransaction;
use crate::transaction::{DeltaOp, IsolationLevel, SetPropsOp, Timestamp, UndoEntry, UndoPtr};
use crate::wal::StorageWal;
use crate::wal::graph_wal::{Operation, RedoEntry};

const PERIODIC_GC_THRESHOLD: u64 = 50;

/// A manager for managing transactions.
pub struct MemTxnManager {
    /// Active transactions' txn.
    pub(super) active_txns: SkipMap<Timestamp, Arc<MemTransaction>>,
    /// All transactions, running or committed.
    pub(super) committed_txns: SkipMap<Timestamp, Arc<MemTransaction>>,
    /// Commit lock to enforce serial commit order
    pub(super) commit_lock: Mutex<()>,
    pub(super) latest_commit_ts: AtomicU64,
    /// The commit timestamp and transaction id
    commit_ts_counter: AtomicU64,
    txn_id_counter: AtomicU64,
    /// The watermark is the minimum start timestamp of the active transactions.
    /// If there is no active transaction, the watermark is the latest commit timestamp.
    pub(super) watermark: AtomicU64,
    last_gc_ts: Mutex<u64>,
}

impl Default for MemTxnManager {
    fn default() -> Self {
        Self {
            active_txns: SkipMap::new(),
            committed_txns: SkipMap::new(),
            commit_lock: Mutex::new(()),
            commit_ts_counter: AtomicU64::new(1),
            txn_id_counter: AtomicU64::new(Timestamp::TXN_ID_START + 1),
            latest_commit_ts: AtomicU64::new(0),
            watermark: AtomicU64::new(0),
            last_gc_ts: Mutex::new(0),
        }
    }
}

impl MemTxnManager {
    /// Create a new MemTxnManager
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new transaction.
    pub fn start_transaction(&self, txn: Arc<MemTransaction>) {
        self.active_txns.insert(txn.txn_id(), txn.clone());
        // Update the watermark
        self.update_watermark();
    }

    /// Unregister a transaction.
    pub fn finish_transaction(&self, txn: &MemTransaction) -> StorageResult<()> {
        let txn_entry = self.active_txns.remove(&txn.txn_id());
        if let Some(txn) = txn_entry {
            let commit_ts = txn.value().commit_ts.get();
            if let Some(commit_ts) = commit_ts {
                self.committed_txns.insert(*commit_ts, txn.value().clone());
            }
            self.update_watermark();
            return Ok(());
        }

        self.periodic_garbage_collect(txn.graph())?;

        Err(StorageError::Transaction(
            TransactionError::TransactionNotFound(format!("{:?}", txn.txn_id())),
        ))
    }

    /// Generate a new commit timestamp.
    pub fn new_commit_ts(&self, ts: Option<Timestamp>) -> Timestamp {
        if let Some(ts) = ts {
            self.commit_ts_counter.store(ts.0 + 1, Ordering::Relaxed);
            ts
        } else {
            Timestamp::with_ts(self.commit_ts_counter.fetch_add(1, Ordering::Relaxed))
        }
    }

    /// Generate a new transaction id.
    pub fn new_txn_id(&self, txn_id: Option<Timestamp>) -> Timestamp {
        if let Some(txn_id) = txn_id {
            self.txn_id_counter.store(txn_id.0 + 1, Ordering::Relaxed);
            txn_id
        } else {
            Timestamp::with_ts(self.txn_id_counter.fetch_add(1, Ordering::Relaxed))
        }
    }

    /// Periodlically garbage collect expired transactions.
    fn periodic_garbage_collect(&self, graph: &MemoryGraph) -> StorageResult<()> {
        // Through acquiring the lock, the garbage collection is single-threaded execution.
        let mut last_gc_ts = self.last_gc_ts.lock().unwrap();
        if self.watermark.load(Ordering::Relaxed) - *last_gc_ts > PERIODIC_GC_THRESHOLD {
            self.garbage_collect(graph)?;
            *last_gc_ts = self.watermark.load(Ordering::Relaxed);
        }

        Ok(())
    }

    /// GC (Garbage Collection) is triggered after transaction commit.
    /// The following items will be cleaned up:
    ///
    /// 1. Transactions
    ///    - Removes expired transactions (commit_ts < watermark)
    ///
    /// 2. Vertices
    ///    - Removes vertices marked as deleted (tombstone = true)
    ///    - Cleans up old vertex versions (commit_ts < watermark)
    ///
    /// 3. Edges
    ///    - Removes edges marked as deleted (tombstone = true)
    ///    - Cleans up old edge versions (commit_ts < watermark)
    ///
    /// 4. Adjacency Lists
    ///    - Updates adjacency lists for deleted vertices and edges
    pub fn garbage_collect(&self, graph: &MemoryGraph) -> StorageResult<()> {
        // Step1: Obtain the min read timestamp of the active transactions
        let min_read_ts = self.watermark.load(Ordering::Acquire);

        // Clean up expired transactions
        let mut expired_txns = Vec::new();
        // Since both `DeleteVertex` and `DeleteEdge` trigger `DeleteEdge`,
        // we only need to collect and analyze expired edges
        let mut expired_edges: HashMap<EdgeId, Edge> = HashMap::new();
        for entry in self.committed_txns.iter() {
            // If the commit timestamp of the transaction is greater than the min read timestamp,
            // it means the transaction is still active, and the subsequent transactions are also
            // active.
            if entry.key().0 > min_read_ts {
                break;
            }

            for entry in entry.value().undo_buffer.read().unwrap().iter() {
                match entry.delta() {
                    // DeltaOp::CreateEdge means that the edge is deleted in this transaction
                    DeltaOp::CreateEdge(edge) => {
                        expired_edges.insert(edge.eid(), edge.without_properties());
                    }
                    DeltaOp::DelEdge(eid) => {
                        expired_edges.remove(eid);
                    }
                    _ => {}
                }
            }

            // Txn has been committed, iterate over its undo buffer
            expired_txns.push(entry.value().clone());
        }

        // Remove expired transactions containing undo buffer
        for txn in expired_txns {
            self.committed_txns.remove(txn.commit_ts.get().unwrap());
        }

        for (_, edge) in expired_edges {
            // Define a macro to check if an entity is marked as a tombstone
            macro_rules! check_tombstone {
                ($graph:expr, $collection:ident, $id_method:expr) => {
                    $graph
                        .$collection
                        .get($id_method)
                        .map(|v| Some(v.value().chain.current.read().unwrap().data.is_tombstone))
                        .unwrap_or(None)
                };
            }
            let src_tombstone = check_tombstone!(graph, vertices, &edge.src_id());
            let dst_tombstone = check_tombstone!(graph, vertices, &edge.dst_id());
            let edge_tombstone = check_tombstone!(graph, edges, &edge.eid());

            // Remove the vertex and the corresponding adjacencies
            fn remove_vertex_and_adjacencies(graph: &MemoryGraph, vid: VertexId) {
                graph.vertices.remove(&vid);
                let mut incoming_to_remove = Vec::new();
                let mut outgoing_to_remove = Vec::new();

                if let Some(adj_container) = graph.adjacency_list.get(&vid) {
                    for adj in adj_container.incoming().iter() {
                        outgoing_to_remove.push((
                            adj.neighbor_id(),
                            Neighbor::new(adj.label_id(), vid, adj.eid()),
                        ));
                    }
                    for adj in adj_container.outgoing().iter() {
                        incoming_to_remove.push((
                            adj.neighbor_id(),
                            Neighbor::new(adj.label_id(), vid, adj.eid()),
                        ));
                    }
                }

                for (other_vid, euid) in incoming_to_remove {
                    graph.adjacency_list.entry(other_vid).and_modify(|l| {
                        l.incoming().remove(&euid);
                    });
                }

                for (other_vid, euid) in outgoing_to_remove {
                    graph.adjacency_list.entry(other_vid).and_modify(|l| {
                        l.outgoing().remove(&euid);
                    });
                }

                // Remove adjancy list of the vertex
                graph.adjacency_list.remove(&vid);
            }

            if let Some(true) = src_tombstone {
                remove_vertex_and_adjacencies(graph, edge.src_id());
            }

            if let Some(true) = dst_tombstone {
                remove_vertex_and_adjacencies(graph, edge.dst_id());
            }

            if let Some(true) = edge_tombstone {
                graph.edges.remove(&edge.eid());
                graph
                    .adjacency_list
                    .entry(edge.src_id())
                    .and_modify(|adj_container| {
                        adj_container.outgoing().remove(&Neighbor::new(
                            edge.label_id(),
                            edge.dst_id(),
                            edge.eid(),
                        ));
                    });
                graph
                    .adjacency_list
                    .entry(edge.dst_id())
                    .and_modify(|adj_container| {
                        adj_container.incoming().remove(&Neighbor::new(
                            edge.label_id(),
                            edge.src_id(),
                            edge.eid(),
                        ));
                    });
            }
        }

        Ok(())
    }

    /// Calculate the watermark based on the active transactions.
    pub fn update_watermark(&self) {
        let min_ts = self
            .active_txns
            .front()
            .map(|v| v.value().start_ts().0)
            .unwrap_or(self.latest_commit_ts.load(Ordering::Acquire))
            .max(self.watermark.load(Ordering::Acquire));
        self.watermark.store(min_ts, Ordering::SeqCst);
    }
}

pub struct MemTransaction {
    graph: Arc<MemoryGraph>, // Reference to the associated in-memory graph

    // ---- Transaction Config ----
    isolation_level: IsolationLevel, // Isolation level of the transaction

    // ---- Timestamp management ----
    /// Start timestamp assigned when the transaction begins
    start_ts: Timestamp,
    commit_ts: OnceLock<Timestamp>, // Commit timestamp assigned upon committing
    txn_id: Timestamp,              // Unique transaction identifier

    // ---- Read sets ----
    pub(super) vertex_reads: DashSet<VertexId>, // Set of vertices read by this transaction
    pub(super) edge_reads: DashSet<EdgeId>,     // Set of edges read by this transaction

    // ---- Undo logs ----
    pub(super) undo_buffer: RwLock<Vec<Arc<UndoEntry>>>,

    // ---- Write-ahead-log for crash recovery ----
    pub(super) redo_buffer: RwLock<Vec<RedoEntry>>,
}

impl MemTransaction {
    pub(super) fn with_memgraph(
        graph: Arc<MemoryGraph>,
        txn_id: Timestamp,
        start_ts: Timestamp,
        isolation_level: IsolationLevel,
    ) -> Self {
        Self {
            graph,
            isolation_level,
            start_ts,
            commit_ts: OnceLock::new(),
            txn_id,
            vertex_reads: DashSet::new(),
            edge_reads: DashSet::new(),
            undo_buffer: RwLock::new(Vec::new()),
            redo_buffer: RwLock::new(Vec::new()),
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

    /// Returns the start timestamp of the transaction.
    pub fn start_ts(&self) -> Timestamp {
        self.start_ts
    }

    /// Returns the transaction ID.
    pub fn txn_id(&self) -> Timestamp {
        self.txn_id
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

    /// Returns the isolution level
    pub fn isolation_level(&self) -> &IsolationLevel {
        &self.isolation_level
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
}

impl StorageTransaction for MemTransaction {
    type CommitTimestamp = Timestamp;

    /// Commits the transaction, applying all changes atomically.
    /// Ensures serializability, updates version chains, and manages adjacency lists.
    fn commit(&self) -> StorageResult<Timestamp> {
        self.commit_at(None, false)
    }

    /// Aborts the transaction, rolling back all changes.
    fn abort(&self) -> StorageResult<()> {
        self.abort_at(false)
    }
}

impl MemTransaction {
    /// Commits the transaction at a specific commit timestamp.
    pub fn commit_at(
        &self,
        commit_ts: Option<Timestamp>,
        skip_wal: bool,
    ) -> StorageResult<Timestamp> {
        let commit_ts = self.graph.txn_manager.new_commit_ts(commit_ts);

        // Acquire the global commit lock to enforce serial execution of commits.
        let _guard = self.graph.txn_manager.commit_lock.lock().unwrap();

        // Step 1: Validate serializability if isolution level is Serializable.
        if let IsolationLevel::Serializable = self.isolation_level {
            if let Err(e) = self.validate_read_sets() {
                self.abort()?;
                return Err(e);
            }
        }

        // Step 2: Assign a commit timestamp (atomic operation).
        if let Err(e) = self.commit_ts.set(commit_ts) {
            self.abort()?;
            return Err(StorageError::Transaction(
                TransactionError::TransactionAlreadyCommitted(format!("{:?}", e)),
            ));
        }

        // Step 3: Process write in undo buffer.
        {
            // Define a macro to simplify the update of the commit timestamp.
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

        // Step 4: Write redo entry and commit to WAL,
        // unless the function is called when recovering from WAL
        if !skip_wal {
            let redo_entries = self
                .redo_buffer
                .write()
                .unwrap()
                .drain(..)
                .map(|mut entry| {
                    // Update LSN
                    entry.lsn = self.graph.wal_manager.next_lsn();
                    entry
                })
                .collect::<Vec<_>>();
            for entry in redo_entries {
                self.graph
                    .wal_manager
                    .wal()
                    .write()
                    .unwrap()
                    .append(&entry)?;
            }

            // Write `Operation::CommitTransaction` to WAL
            let wal_entry = RedoEntry {
                lsn: self.graph.wal_manager.next_lsn(),
                txn_id: self.txn_id(),
                iso_level: self.isolation_level,
                op: Operation::CommitTransaction(commit_ts),
            };
            self.graph
                .wal_manager
                .wal()
                .write()
                .unwrap()
                .append(&wal_entry)?;
            self.graph.wal_manager.wal().write().unwrap().flush()?;
        }

        // Step 5: Clean up transaction state and update the `latest_commit_ts`.
        self.graph
            .txn_manager
            .latest_commit_ts
            .store(commit_ts.0, Ordering::SeqCst);
        self.graph.txn_manager.finish_transaction(self)?;

        // Step 6: Check if an auto checkpoint should be created
        self.graph.check_auto_checkpoint()?;
        Ok(commit_ts)
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
            let lsn = self.graph.wal_manager.next_lsn();
            let wal_entry = RedoEntry {
                lsn,
                txn_id: self.txn_id(),
                iso_level: self.isolation_level,
                op: Operation::AbortTransaction,
            };
            self.graph
                .wal_manager
                .wal()
                .write()
                .unwrap()
                .append(&wal_entry)?;
            self.graph.wal_manager.wal().write().unwrap().flush()?;
        }

        // Remove transaction from transaction manager
        self.graph.txn_manager.finish_transaction(self)?;
        Ok(())
    }
}

/// A smart pointer wrapper around `Arc<MemTransaction>` that provides automatic rollback
/// functionality when the transaction goes out of scope without being explicitly committed or
/// aborted.
///
/// This wrapper implements the RAII (Resource Acquisition Is Initialization) pattern to ensure
/// that transactions are properly cleaned up in case of panics or when they go out of scope.
pub struct TransactionHandle {
    inner: Arc<MemTransaction>,
    /// Flag to track whether the transaction has been explicitly handled (committed or aborted)
    is_handled: std::cell::Cell<bool>,
}

impl TransactionHandle {
    /// Creates a new `TransactionHandle` wrapper around an `Arc<MemTransaction>`.
    pub fn new(txn: Arc<MemTransaction>) -> Self {
        Self {
            inner: txn,
            is_handled: std::cell::Cell::new(false),
        }
    }

    /// Marks the transaction as handled (committed or aborted).
    /// This prevents the automatic rollback in the Drop implementation.
    pub fn mark_handled(&self) {
        self.is_handled.set(true);
    }

    /// Returns a reference to the inner `Arc<MemTransaction>`.
    pub fn inner(&self) -> &Arc<MemTransaction> {
        &self.inner
    }
}

impl Deref for TransactionHandle {
    type Target = MemTransaction;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Drop for TransactionHandle {
    fn drop(&mut self) {
        // Only perform automatic rollback if:
        // 1. The transaction hasn't been explicitly handled (committed or aborted)
        // 2. This is the last reference to the transaction (strong_count == 1)
        if !self.is_handled.get() {
            // Attempt to abort the transaction
            // We ignore errors here since we're in a Drop implementation
            let _ = self.inner.abort();
            println!("abort transaction {:?}", self.inner.txn_id());
        }
    }
}

impl StorageTransaction for TransactionHandle {
    type CommitTimestamp = Timestamp;

    fn commit(&self) -> StorageResult<Self::CommitTimestamp> {
        let result = self.inner.commit();
        if result.is_ok() {
            self.mark_handled();
        }
        result
    }

    fn abort(&self) -> StorageResult<()> {
        let result = self.inner.abort();
        if result.is_ok() {
            self.mark_handled();
        }
        result
    }
}

impl Clone for TransactionHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            is_handled: std::cell::Cell::new(self.is_handled.get()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::memory_graph;
    use crate::transaction::IsolationLevel;

    #[test]
    fn test_watermark_tracking() {
        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();
        let base_commit_ts = graph.txn_manager.latest_commit_ts.load(Ordering::Acquire);

        // Start txn0
        let txn0 = graph.begin_transaction(IsolationLevel::Serializable);
        assert_eq!(txn0.start_ts().0, base_commit_ts + 1);
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            base_commit_ts + 1
        );

        {
            let txn_store_1 = graph.begin_transaction(IsolationLevel::Serializable);
            assert_eq!(txn_store_1.start_ts().0, base_commit_ts + 2);
            let commit_ts = txn_store_1.commit().unwrap();
            assert_eq!(commit_ts.0, base_commit_ts + 3);
        }

        // Watermark should remain unchanged since txn0 is still active
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn0.start_ts.0
        );

        // Start txn1
        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        assert_eq!(txn1.start_ts().0, base_commit_ts + 4);

        // Watermark should remain unchanged
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn0.start_ts.0
        );

        // Create and commit txn_store_2
        {
            let txn_store_2 = graph.begin_transaction(IsolationLevel::Serializable);
            assert_eq!(txn_store_2.start_ts().0, base_commit_ts + 5);
            let commit_ts = txn_store_2.commit().unwrap();
            assert_eq!(commit_ts.0, base_commit_ts + 6);
        }

        // Watermark should remain unchanged
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn0.start_ts.0
        );

        // Start txn2
        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        assert_eq!(txn2.start_ts().0, base_commit_ts + 7);

        // Watermark should remain unchanged
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn0.start_ts.0
        );

        // Abort txn0
        txn0.abort().unwrap();
        // Watermark should update to start_ts of txn1
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn1.start_ts().0
        );

        // Create and commit txn_store_3
        {
            let txn_store_3 = graph.begin_transaction(IsolationLevel::Serializable);
            assert_eq!(txn_store_3.start_ts().0, base_commit_ts + 8);
            let commit_ts = txn_store_3.commit().unwrap();
            assert_eq!(commit_ts.0, base_commit_ts + 9);
        }

        // Watermark should remain unchanged
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn1.start_ts().0
        );

        // Start txn3
        let txn3 = graph.begin_transaction(IsolationLevel::Serializable);
        assert_eq!(txn3.start_ts().0, base_commit_ts + 10);

        // Watermark should remain unchanged
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn1.start_ts().0
        );

        // Abort txn1
        txn1.abort().unwrap();
        // Watermark should be updated to txn2's start timestamp
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn2.start_ts().0
        );

        // Abort txn2
        txn2.abort().unwrap();
        // Watermark should be updated to txn3's start timestamp
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn3.start_ts().0
        );

        // Create and commit txn_store_4
        {
            let txn_store_4 = graph.begin_transaction(IsolationLevel::Serializable);
            assert_eq!(txn_store_4.start_ts().0, base_commit_ts + 11);
            let commit_ts = txn_store_4.commit().unwrap();
            assert_eq!(commit_ts.0, base_commit_ts + 12);
        }

        // Watermark should remain unchanged
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn3.start_ts().0
        );

        // Start txn4
        let txn4 = graph.begin_transaction(IsolationLevel::Serializable);
        assert_eq!(txn4.start_ts().0, base_commit_ts + 13);

        // Watermark should remain unchanged
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn3.start_ts().0
        );

        // Abort txn3
        txn3.abort().unwrap();
        // Watermark should be updated to txn4's start timestamp
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn4.start_ts().0
        );

        // Abort txn4
        txn4.abort().unwrap();
        // Watermark should remain unchanged, since there are no active transactions
        // and the latest commit timestamp is still txn4's commit timestamp
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn4.start_ts().0
        );

        // Create and commit txn_store_5
        {
            let txn_store_5 = graph.begin_transaction(IsolationLevel::Serializable);
            assert_eq!(txn_store_5.start_ts().0, base_commit_ts + 14);
            let commit_ts = txn_store_5.commit().unwrap();
            assert_eq!(commit_ts.0, base_commit_ts + 15);
        }

        // The watermark should be updated because there are no active transactions
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            base_commit_ts + 15 // latest commit timestamp
        );

        // Start txn5
        let txn5 = graph.begin_transaction(IsolationLevel::Serializable);
        assert_eq!(txn5.start_ts().0, base_commit_ts + 16);

        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn5.start_ts().0
        );

        // Abort txn5
        txn5.abort().unwrap();
        // Watermark should remain unchanged since there are no active transactions
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn5.start_ts().0
        );
    }
}
