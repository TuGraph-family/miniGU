use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock, RwLock};

use common::datatype::types::{EdgeId, VertexId};
use crossbeam_skiplist::SkipMap;
use dashmap::DashSet;

use super::memory_graph::MemoryGraph;
use crate::error::{StorageError, StorageResult};
use crate::storage::StorageTransaction;
use crate::transaction::{DeltaOp, IsolationLevel, SetPropsOp, Timestamp, UndoEntry, UndoPtr};

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
    pub(super) watermark: AtomicU64,
    last_gc_ts: Mutex<u64>,
}

impl Default for MemTxnManager {
    fn default() -> Self {
        Self {
            active_txns: SkipMap::new(),
            committed_txns: SkipMap::new(),
            commit_lock: Mutex::new(()),
            latest_commit_ts: AtomicU64::new(Timestamp::new_commit_ts().0),
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
    pub fn finsih_transaction(&self, txn: &MemTransaction) -> StorageResult<()> {
        let txn_entry = self.active_txns.remove(&txn.txn_id());
        if let Some(txn) = txn_entry {
            let commit_ts = txn.value().commit_ts.get();
            if let Some(commit_ts) = commit_ts {
                self.committed_txns.insert(*commit_ts, txn.value().clone());
            }
            self.update_watermark();
            return Ok(());
        }

        self.periodic_garbage_collect()?;

        Err(StorageError::TransactionError(format!(
            "Transaction {:?} not found",
            txn.txn_id(),
        )))
    }

    /// Periodlically garbage collect expired transactions.
    fn periodic_garbage_collect(&self) -> StorageResult<()> {
        // Through acquiring the lock, the garbage collection is single-threaded execution.
        let mut last_gc_ts = self.last_gc_ts.lock().unwrap();
        if self.watermark.load(Ordering::Relaxed) - *last_gc_ts > PERIODIC_GC_THRESHOLD {
            self.garbage_collect()?;
            *last_gc_ts = self.watermark.load(Ordering::Relaxed);
        }

        Ok(())
    }

    /// Start a garbage collection of expired transactions.
    fn garbage_collect(&self) -> StorageResult<()> {
        // Step1: Obtain the min read timestamp of the active transactions
        let min_read_ts = self.watermark.load(Ordering::Acquire);
        // Step2: Iterate over all transactions, and remove those whose commit timestamp is less
        // than the min read timestamp
        let mut expired_txns = Vec::new();
        for entry in self.committed_txns.iter() {
            // 如何遍历到的事务commit timestamp大于min read ts，说明链表后续的事务都不需要再遍历
            if entry.key().0 > min_read_ts {
                break;
            }

            // Txn has been committed, iterate over its undo buffer
            expired_txns.push(entry.value().clone());
        }
        // Step3: Remove the expired transactions from the commited_txns map
        for txn in expired_txns {
            self.committed_txns.remove(txn.commit_ts.get().unwrap());
        }

        Ok(())
    }

    /// Calculate the
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
    pub(super) undo_buffer: RwLock<Vec<UndoEntry>>,
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
            if current.commit_ts != self.txn_id && current.commit_ts > self.start_ts {
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
            if current.commit_ts != self.txn_id && current.commit_ts > self.start_ts {
                return Err(StorageError::ReadConflict(eid.to_string()));
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
        &self,
        undo_ptr: UndoPtr,
        mut callback: T,
        txn_start_ts: Timestamp,
    ) -> StorageResult<()> {
        let mut current = undo_ptr;

        // Get the undo buffer of the transaction that modified the vertex/edge
        while let Some(entry) = self.graph.txn_manager.committed_txns.get(&current.txn_id()) {
            let undo_buffer = entry.value().undo_buffer.read().unwrap();
            let undo_entry = &undo_buffer[current.entry_offset()];

            // Apply the delta to the vertex/edge
            callback(undo_entry);

            // If the timestamp of the entry is less than the txn_start_ts,
            // it means current version is the latest visible version,
            // no need to continue traversing the undo chain
            if undo_entry.timestamp() < txn_start_ts {
                break;
            }
            if let Some(next) = undo_entry.next() {
                current = next;
            } else {
                return Err(StorageError::VersionNotFound(
                    "Can't find a suitable version".to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl StorageTransaction for MemTransaction {
    type CommitTimestamp = Timestamp;

    /// Commits the transaction, applying all changes atomically.
    /// Ensures serializability, updates version chains, and manages adjacency lists.
    fn commit(&self) -> StorageResult<Timestamp> {
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
        let commit_ts = Timestamp::new_commit_ts();
        if let Err(e) = self.commit_ts.set(commit_ts) {
            self.abort()?;
            return Err(StorageError::TransactionError(format!(
                "Transaction {:?} already committed",
                e
            )));
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

        // Step 5: Clean up transaction state and update the `latest_commit_ts`.
        self.graph
            .txn_manager
            .latest_commit_ts
            .store(commit_ts.0, Ordering::SeqCst);
        self.graph.txn_manager.finsih_transaction(self)?;
        Ok(commit_ts)
    }

    /// Aborts the transaction, rolling back all changes.
    fn abort(&self) -> StorageResult<()> {
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
                            current.data.is_tombstone = false;
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
                            current.data.is_tombstone = false;
                            // Update undo pointer to previous version
                            *entry.chain.undo_ptr.write().unwrap() = next;
                        }
                    }
                }
                DeltaOp::AddLabel(_) => todo!(),
                DeltaOp::RemoveLabel(_) => todo!(),
            }
        }

        // Remove transaction from transaction manager
        self.graph.txn_manager.finsih_transaction(self)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;
    use crate::transaction::IsolationLevel;

    #[test]
    #[serial]
    fn test_watermark_tracking() {
        let graph = MemoryGraph::new();
        let txn_start_ts = graph.txn_manager.latest_commit_ts.load(Ordering::Acquire);

        // Start txn0
        let txn0: Arc<MemTransaction> = graph.begin_transaction(IsolationLevel::Serializable);
        assert_eq!(txn0.start_ts().0, txn_start_ts);
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts
        );

        {
            let txn_store_1 = graph.begin_transaction(IsolationLevel::Serializable);
            assert_eq!(txn_store_1.start_ts().0, txn_start_ts);
            let commit_ts = txn_store_1.commit().unwrap();
            assert_eq!(commit_ts.0, txn_start_ts + 1);
        }

        // Watermark should remain unchanged since txn0 is still active
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts
        );

        // Start txn1
        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        assert_eq!(txn1.start_ts().0, txn_start_ts + 1);

        // Watermark should remain unchanged
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts
        );

        // Create and commit txn_store_2
        {
            let txn_store_2 = graph.begin_transaction(IsolationLevel::Serializable);
            assert_eq!(txn_store_2.start_ts().0, txn_start_ts + 1);
            let commit_ts = txn_store_2.commit().unwrap();
            assert_eq!(commit_ts.0, txn_start_ts + 2);
        }

        // Watermark should remain unchanged
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts
        );

        // Start txn2
        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        assert_eq!(txn2.start_ts().0, txn_start_ts + 2);

        // Watermark should remain unchanged
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts
        );

        // Abort txn0
        txn0.abort().unwrap();
        // Watermark should update to txn_start_ts + 1
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts + 1
        );

        // Create and commit txn_store_3
        {
            let txn_store_3 = graph.begin_transaction(IsolationLevel::Serializable);
            assert_eq!(txn_store_3.start_ts().0, txn_start_ts + 2);
            let commit_ts = txn_store_3.commit().unwrap();
            assert_eq!(commit_ts.0, txn_start_ts + 3);
        }

        // Watermark should remain unchanged
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts + 1
        );

        // Start txn3
        let txn3 = graph.begin_transaction(IsolationLevel::Serializable);
        assert_eq!(txn3.start_ts().0, txn_start_ts + 3);

        // Watermark should remain unchanged
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts + 1
        );

        // Abort txn1
        txn1.abort().unwrap();
        // Watermark should be updated to txn2's start timestamp
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts + 2
        );

        // Abort txn2
        txn2.abort().unwrap();
        // Watermark should be updated to txn3's start timestamp
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts + 3
        );

        // Create and commit txn_store_4
        {
            let txn_store_4 = graph.begin_transaction(IsolationLevel::Serializable);
            assert_eq!(txn_store_4.start_ts().0, txn_start_ts + 3);
            let commit_ts = txn_store_4.commit().unwrap();
            assert_eq!(commit_ts.0, txn_start_ts + 4);
        }

        // Watermark should remain unchanged
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts + 3
        );

        // Start txn4
        let txn4 = graph.begin_transaction(IsolationLevel::Serializable);
        assert_eq!(txn4.start_ts().0, txn_start_ts + 4);

        // Watermark should remain unchanged
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts + 3
        );

        // Abort txn3
        txn3.abort().unwrap();
        // Watermark should be updated to txn4's start timestamp
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts + 4
        );

        // Abort txn4
        txn4.abort().unwrap();
        // Watermark should remain unchanged
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts + 4
        );

        // Create and commit txn_store_5
        {
            let txn_store_5 = graph.begin_transaction(IsolationLevel::Serializable);
            assert_eq!(txn_store_5.start_ts().0, txn_start_ts + 4);
            let commit_ts = txn_store_5.commit().unwrap();
            assert_eq!(commit_ts.0, txn_start_ts + 5);
        }

        // The watermark should be updated because there are no active transactions
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts + 5
        );

        // Start txn5
        let txn5 = graph.begin_transaction(IsolationLevel::Serializable);
        assert_eq!(txn5.start_ts().0, txn_start_ts + 5);

        // Watermark should remain unchanged
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts + 5
        );

        // Abort txn5
        txn5.abort().unwrap();
        // Watermark should remain unchanged since there are no active transactions
        assert_eq!(
            graph.txn_manager.watermark.load(Ordering::Acquire),
            txn_start_ts + 5
        );
    }
}
