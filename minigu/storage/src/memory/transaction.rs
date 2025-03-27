use std::sync::{atomic::{AtomicU64, Ordering}, Arc, Mutex, OnceLock, RwLock};

use common::datatype::types::{EdgeId, VertexId};
use crossbeam_epoch::{pin, Atomic, Collector, Owned};
use crossbeam_skiplist::{SkipList, SkipMap, SkipSet};
use dashmap::{DashMap, DashSet};

use crate::{error::{StorageError, StorageResult}, storage::StorageTransaction, transaction::{Timestamp, DeltaOp, IsolationLevel, SetPropsOp, UndoEntry, UndoPtr}};

use super::memory_graph::MemoryGraph;

pub struct MemTxnManager {
    /// Active transactions' start timestamps.
    pub(super) active_txns: SkipMap<Timestamp, Arc<MemTransaction>>, 
    /// All transactions, running or committed.
    pub(super) commited_txns: SkipMap<Timestamp, Arc<MemTransaction>>,
    /// Commit lock to enforce serial commit order
    pub(super) commit_lock: Mutex<()>,
    pub(super) watermark: AtomicU64,
}

impl MemTxnManager {
    /// Create a new MemTxnManager
    pub fn new() -> Self {
        Self {
            active_txns: SkipMap::new(),
            commited_txns: SkipMap::new(),
            commit_lock: Mutex::new(()),
            watermark: AtomicU64::new(Timestamp::TXN_ID_START),
        }
    }

    /// Register a new transaction.
    pub fn start_transaction(&self, txn: Arc<MemTransaction>) {
        self.active_txns.insert(txn.start_ts(), txn.clone());
        // Update the watermark
        self.update_watermark();
    }

    /// Unregister a transaction.
    pub fn finsih_transaction(&self, start_ts: Timestamp) -> StorageResult<()> {
        let txn = self.active_txns.remove(&start_ts);
        if let Some(txn) = txn {
            let commit_ts = txn.value().commit_ts.get().expect("Transaction not committed");
            self.commited_txns.insert(*commit_ts, txn.value().clone());
            self.update_watermark();
        } else {
            return Err(StorageError::TransactionError(format!("Transaction {:?} not found", start_ts)));
        }
        Ok(())
    }

    /// Start a garbage collection of expired transactions.
    pub fn garbage_collect(&self) -> StorageResult<()> {
        // Step1: Obtain the min read timestamp of the active transactions
        let min_read_ts = self.watermark.load(Ordering::Acquire);
        // Step2: Iterate over all transactions, and remove those whose commit timestamp is less than the min read timestamp
        let mut expired_txns = Vec::new();
        for entry in self.commited_txns.iter() {
            // 如何遍历到的事务commit timestamp大于min read ts，说明链表后续的事务都不需要再遍历
            if entry.key().0 > min_read_ts {
                break;
            }

            // Txn has been committed, iterate over its undo buffer
            expired_txns.push(entry.value().clone());
        }
        // Step3: Remove the expired transactions from the commited_txns map
        for txn in expired_txns {
            self.commited_txns.remove(&txn.commit_ts.get().unwrap());
        }

        Ok(())
    }

    /// Calculate the 
    pub fn update_watermark(&self) {
        let min_ts = self
            .active_txns
            .front()
            .map(|v| v.key().clone())
            .unwrap_or(Timestamp::TXN_START_TS);
        self.watermark.store(min_ts.0, Ordering::SeqCst);
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
    txn_id: Timestamp,      // Unique transaction identifier

    // ---- Read sets ----
    pub(super) vertex_reads: DashSet<VertexId>, // Set of vertices read by this transaction
    pub(super) edge_reads: DashSet<EdgeId>,     // Set of edges read by this transaction

    // ---- Undo logs ----
    // pub(super) vertex_undos: DashMap<VertexId, Vec<UndoEntry<Vertex>>>, // Vertex undo logs
    // pub(super) edge_undos: DashMap<EdgeId, Vec<UndoEntry<Edge>>>,       // Edge undo logs
    pub(super) undo_buffer: RwLock<Vec<UndoEntry>>,
}

impl MemTransaction {
    pub(super) fn with_memgraph(
        graph: Arc<MemoryGraph>,
        txn_id: Timestamp,
        start_ts: Timestamp,
        isolation_level: IsolationLevel
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
            if current.commit_ts!= self.txn_id && current.commit_ts > self.start_ts {
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

    /// Reconstructs a specific version of a Vertex based on the undo chain and a target timestamp
    pub(super) fn apply_deltas_for_read<T: FnMut(&UndoEntry)>(&self, undo_ptr: UndoPtr, mut callback: T, target_ts: Timestamp) -> StorageResult<()> {
        let mut current = undo_ptr;
        
        while let Some(entry) = self.graph.txn_manager.commited_txns.get(&current.txn_id()) {
            let undo_buffer = entry.value().undo_buffer.read().unwrap();
            let undo_entry = &undo_buffer[current.entry_offset()];

            callback(undo_entry);
            
            // Continue traversing the undo chain
            if undo_entry.timestamp() < target_ts {
                // If the timestamp in the undo chain is less than the target timestamp,
                // this version's data has been overwritten, no need to continue traversing
                break;
            }
            if let Some(next) = undo_entry.next() {
                current = next;
            } else {
                return Err(StorageError::VersionNotFound("Can't find a suitable version".to_string()))
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
            return Err(StorageError::TransactionError(format!("Transaction {:?} already committed", e)));
        }

        // Step 3: Process write in undo buffer.
        {
            // 定义宏来简化更新提交时间戳的操作
            macro_rules! update_commit_ts {
                ($self:expr, $entity_type:ident, $id:expr) => {
                    $self.graph().$entity_type().get($id).unwrap().current().write().unwrap().commit_ts = commit_ts
                };
            }

            let undo_entries = self.undo_buffer.read().unwrap().clone();
            for undo_entry in undo_entries.iter() {
                match undo_entry.delta() {
                    DeltaOp::DelVertex(vid) => update_commit_ts!(self, vertices, vid),
                    DeltaOp::DelEdge(eid) => update_commit_ts!(self, edges, eid),
                    DeltaOp::CreateVertex(vertex) => update_commit_ts!(self, vertices, &vertex.vid()),
                    DeltaOp::CreateEdge(edge) => update_commit_ts!(self, edges, &edge.eid()),
                    DeltaOp::SetVertexProps(vid, _) => update_commit_ts!(self, vertices, vid),
                    DeltaOp::SetEdgeProps(eid, _) => update_commit_ts!(self, edges, eid),
                    DeltaOp::AddLabel(_) => todo!(),
                    DeltaOp::RemoveLabel(_) => todo!(),
                }
            }
        }

        // Step 5: Clean up transaction state.
        self.graph.txn_manager.finsih_transaction(self.txn_id())?;
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
                },
                DeltaOp::SetVertexProps(vid, SetPropsOp { indices, props }) => {
                    // For property modifications, determine if it's a vertex or edge based on entity_id
                    // Restore vertex properties
                    if let Some(entry) = self.graph.vertices.get(&vid) {
                        let mut current = entry.chain.current.write().unwrap();
                        if current.commit_ts == self.txn_id() {
                            // Restore properties
                            current.data.set_props(indices, props.clone());
                            // Update undo pointer to previous version
                            *entry.chain.undo_ptr.write().unwrap() = next;
                        }
                    }
                },
                DeltaOp::SetEdgeProps(eid, SetPropsOp { indices, props }) => {
                    // Restore edge properties
                    if let Some(entry) = self.graph.edges.get(&eid) {
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
                    if let Some(entry) = self.graph.vertices.get(&vid) {
                        let mut current = entry.chain.current.write().unwrap();
                        if current.commit_ts == self.txn_id() {
                            // Restore deletion flag
                            current.data.is_tombstone = false;
                            // Update undo pointer to previous version
                            *entry.chain.undo_ptr.write().unwrap() = next;
                        }
                    }
                },
                DeltaOp::DelEdge(eid) => {
                    // Restore edge
                    if let Some(entry) = self.graph.edges.get(&eid) {
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
        self.graph.txn_manager.finsih_transaction(self.txn_id())?;
        Ok(())
    }
}