use std::sync::{Arc, RwLock, Weak};

use arrow::array::BooleanArray;
use crossbeam_skiplist::SkipSet;
use dashmap::DashMap;
use minigu_common::types::{EdgeId, LabelId, PropertyId, VectorIndexKey, VertexId};
use minigu_common::value::ScalarValue;

use super::checkpoint::{CheckpointManager, CheckpointManagerConfig};
use super::transaction::{MemTransaction, MemTxnManager, TransactionHandle, UndoEntry, UndoPtr};
use super::vector_index::filter::create_filter_mask;
use super::vector_index::in_mem_diskann::create_vector_index_config;
use super::vector_index::{InMemDiskANNAdapter, VectorIndex};
use crate::common::model::edge::{Edge, Neighbor};
use crate::common::model::vertex::Vertex;
use crate::common::transaction::{DeltaOp, IsolationLevel, SetPropsOp, Timestamp};
use crate::common::wal::StorageWal;
use crate::common::wal::graph_wal::{Operation, RedoEntry, WalManager, WalManagerConfig};
use crate::error::{
    EdgeNotFoundError, StorageError, StorageResult, TransactionError, VectorIndexError,
    VertexNotFoundError,
};

// Perform the update properties operation
macro_rules! update_properties {
    ($self:expr, $id:expr, $entry:expr, $txn:expr, $indices:expr, $props:expr, $op:ident) => {{
        // Acquire the lock to modify the properties of the vertex/edge
        let mut current = $entry.chain.current.write().unwrap();
        check_write_conflict(current.commit_ts, $txn)?;

        let delta_props = $indices
            .iter()
            .map(|i| current.data.properties.get(*i).unwrap().clone())
            .collect();
        let delta = DeltaOp::$op($id, SetPropsOp {
            indices: $indices,
            props: delta_props,
        });

        let undo_ptr = $entry.chain.undo_ptr.read().unwrap().clone();
        let mut undo_buffer = $txn.undo_buffer.write().unwrap();
        let undo_entry = Arc::new(UndoEntry::new(delta, current.commit_ts, undo_ptr));
        undo_buffer.push(undo_entry.clone());
        *$entry.chain.undo_ptr.write().unwrap() = Arc::downgrade(&undo_entry);

        // Update the commit timestamp to the transaction ID.
        current.commit_ts = $txn.txn_id();

        // Create a new version with updated properties.
        current.data.set_props(&$indices, $props);
    }};
}

// Version metadata (equivalent to version metadata in the referenced paper)
#[derive(Debug)]
/// Stores the current version of an entity, along with transaction metadata.
pub(super) struct CurrentVersion<D> {
    pub(super) data: D,              // The actual data version
    pub(super) commit_ts: Timestamp, // Commit timestamp indicating when it was committed
}

// Version chain structure
#[derive(Debug)]
/// Maintains the version history of an entity, supporting multi-version concurrency control.
pub(super) struct VersionChain<D: Clone> {
    /// The latest version in memory
    pub(super) current: RwLock<CurrentVersion<D>>,
    /// The version history (undo log), points to the first undo entry in the undo buffer
    /// Always records the latest committed version
    pub(super) undo_ptr: RwLock<UndoPtr>,
}

#[derive(Debug)]
/// Represents a versioned vertex in the graph, supporting multi-version concurrency control.
pub(super) struct VersionedVertex {
    pub(super) chain: Arc<VersionChain<Vertex>>,
}

impl VersionedVertex {
    /// Creates a new `VersionedVertex` instance with an initial vertex.
    #[allow(dead_code)]
    pub fn new(initial: Vertex) -> Self {
        Self {
            chain: Arc::new(VersionChain {
                current: RwLock::new(CurrentVersion {
                    data: initial,
                    commit_ts: Timestamp(0), // Initial commit timestamp set to 0
                }),
                undo_ptr: RwLock::new(Weak::new()),
            }),
        }
    }

    pub fn current(&self) -> &RwLock<CurrentVersion<Vertex>> {
        &self.chain.current
    }

    pub fn with_txn_id(initial: Vertex, txn_id: Timestamp) -> Self {
        debug_assert!(txn_id.0 > Timestamp::TXN_ID_START);
        Self {
            chain: Arc::new(VersionChain {
                current: RwLock::new(CurrentVersion {
                    data: initial,
                    commit_ts: txn_id, /* Initial commit timestamp set to txn_id for uncommitted
                                        * changes */
                }),
                undo_ptr: RwLock::new(Weak::new()),
            }),
        }
    }

    /// Returns the visible version of the vertex.
    pub fn get_visible(&self, txn: &MemTransaction) -> StorageResult<Vertex> {
        let current = self.chain.current.read().unwrap();
        let mut visible_vertex = current.data.clone();
        // If the vertex is modified by the same transaction, or the transaction is before the
        // vertex was modified, return the vertex
        let commit_ts = current.commit_ts;
        // If the commit timestamp of current is equal to the transaction id of txn, it means
        // the vertex is modified by the same transaction.
        // If the commit timestamp of current is less than the start timestamp of txn, it means
        // the vertex was modified before the transaction started, and the corresponding transaction
        // has been committed.
        if (commit_ts.is_txn_id() && commit_ts == txn.txn_id())
            || (commit_ts.is_commit_ts() && commit_ts <= txn.start_ts())
        {
            // Check if the current vertex is tombstone
            if visible_vertex.is_tombstone() {
                return Err(StorageError::Transaction(
                    TransactionError::VersionNotVisible(format!(
                        "Vertex is tombstone for {:?}",
                        txn.txn_id()
                    )),
                ));
            }
            Ok(visible_vertex)
        } else {
            // Otherwise, apply the deltas to the vertex
            let undo_ptr = self.chain.undo_ptr.read().unwrap().clone();
            // Closure to apply the deltas to the vertex
            let apply_deltas = |undo_entry: &UndoEntry| match undo_entry.delta() {
                DeltaOp::CreateVertex(original) => visible_vertex = original.clone(),
                DeltaOp::SetVertexProps(_, SetPropsOp { indices, props }) => {
                    visible_vertex.set_props(indices, props.clone());
                }
                DeltaOp::DelVertex(_) => {
                    visible_vertex.is_tombstone = true;
                }
                _ => unreachable!("Unreachable delta op for a vertex"),
            };
            MemTransaction::apply_deltas_for_read(undo_ptr, apply_deltas, txn.start_ts());
            // Check if the vertex is tombstone after applying the deltas
            if visible_vertex.is_tombstone() {
                return Err(StorageError::Transaction(
                    TransactionError::VersionNotVisible(format!(
                        "Vertex is tombstone for {:?}",
                        txn.txn_id()
                    )),
                ));
            }
            Ok(visible_vertex)
        }
    }

    /// Returns whether the vertex is visible.
    pub(super) fn is_visible(&self, txn: &MemTransaction) -> bool {
        // Check if the vertex is visible based on the transaction's start timestamp
        let current = self.chain.current.read().unwrap();
        if (current.commit_ts.is_txn_id() && current.commit_ts == txn.txn_id())
            || (current.commit_ts.is_commit_ts() && current.commit_ts <= txn.start_ts())
        {
            !current.data.is_tombstone()
        } else {
            let undo_ptr = self.chain.undo_ptr.read().unwrap().clone();
            let mut is_visible = !current.data.is_tombstone();
            let apply_deltas = |undo_entry: &UndoEntry| {
                if let DeltaOp::DelVertex(_) = undo_entry.delta() {
                    is_visible = false;
                }
                if let DeltaOp::CreateVertex(_) = undo_entry.delta() {
                    is_visible = true;
                }
            };
            MemTransaction::apply_deltas_for_read(undo_ptr, apply_deltas, txn.start_ts());
            is_visible
        }
    }
}

#[derive(Debug)]
/// Represents a versioned edge in the graph, supporting multi-version concurrency control.
pub(super) struct VersionedEdge {
    pub(super) chain: Arc<VersionChain<Edge>>,
}

impl VersionedEdge {
    /// Creates a new `VersionedEdge` instance with an initial edge.
    #[allow(dead_code)]
    pub fn new(initial: Edge) -> Self {
        Self {
            chain: Arc::new(VersionChain {
                current: RwLock::new(CurrentVersion {
                    data: initial,
                    commit_ts: Timestamp(0), // Initial commit timestamp set to 0
                }),
                undo_ptr: RwLock::new(Weak::new()),
            }),
        }
    }

    pub fn current(&self) -> &RwLock<CurrentVersion<Edge>> {
        &self.chain.current
    }

    pub fn with_modified_ts(initial: Edge, txn_id: Timestamp) -> Self {
        debug_assert!(txn_id.0 > Timestamp::TXN_ID_START);
        Self {
            chain: Arc::new(VersionChain {
                current: RwLock::new(CurrentVersion {
                    data: initial,
                    commit_ts: txn_id,
                }),
                undo_ptr: RwLock::new(Weak::new()),
            }),
        }
    }

    /// Returns the visible version of the edge.
    pub fn get_visible(&self, txn: &MemTransaction) -> StorageResult<Edge> {
        let current = self.chain.current.read().unwrap();
        let mut current_edge = current.data.clone();
        if (current.commit_ts.is_txn_id() && current.commit_ts == txn.txn_id())
            || (current.commit_ts.is_commit_ts() && current.commit_ts <= txn.start_ts())
        {
            // Check if the edge is tombstone
            if current_edge.is_tombstone() {
                return Err(StorageError::Transaction(
                    TransactionError::VersionNotVisible(format!(
                        "Edge is tombstone for {:?}",
                        txn.txn_id()
                    )),
                ));
            }
            Ok(current_edge)
        } else {
            let undo_ptr = self.chain.undo_ptr.read().unwrap().clone();
            let apply_deltas = |undo_entry: &UndoEntry| match undo_entry.delta() {
                DeltaOp::CreateEdge(original) => current_edge = original.clone(),
                DeltaOp::SetEdgeProps(_, SetPropsOp { indices, props }) => {
                    current_edge.set_props(indices, props.clone());
                }
                DeltaOp::DelEdge(_) => {
                    current_edge.is_tombstone = true;
                }
                _ => unreachable!("Unreachable delta op for an edge"),
            };
            MemTransaction::apply_deltas_for_read(undo_ptr, apply_deltas, txn.start_ts());
            // Check if the vertex is tombstone after applying the deltas
            if current_edge.is_tombstone() {
                return Err(StorageError::Transaction(
                    TransactionError::VersionNotVisible(format!(
                        "Edge is tombstone for {:?}",
                        txn.txn_id()
                    )),
                ));
            }
            Ok(current_edge)
        }
    }

    /// Returns whether the edge is visible.
    pub fn is_visible(&self, txn: &MemTransaction) -> bool {
        // Check if the src and dst vertices of edge are visible
        let (src, dst);
        {
            let current = self.chain.current.read().unwrap();
            src = current.data.dst_id();
            dst = current.data.src_id();
        }
        if txn
            .graph()
            .vertices()
            .get(&src)
            .map(|v| v.is_visible(txn))
            .unwrap_or(false)
            && txn
                .graph()
                .vertices()
                .get(&dst)
                .map(|v| v.is_visible(txn))
                .unwrap_or(false)
        {
            // Check if the vertex is visible based on the transaction's start timestamp
            let current = self.chain.current.read().unwrap();
            if (current.commit_ts.is_txn_id() && current.commit_ts == txn.txn_id())
                || (current.commit_ts.is_commit_ts() && current.commit_ts <= txn.start_ts())
            {
                !current.data.is_tombstone()
            } else {
                let undo_ptr = self.chain.undo_ptr.read().unwrap().clone();
                let mut is_visible = !current.data.is_tombstone();
                let apply_deltas = |undo_entry: &UndoEntry| match undo_entry.delta() {
                    DeltaOp::CreateEdge(_) => {
                        is_visible = true;
                    }
                    DeltaOp::DelEdge(_) => {
                        is_visible = false;
                    }
                    _ => {}
                };
                MemTransaction::apply_deltas_for_read(undo_ptr, apply_deltas, txn.start_ts());
                is_visible
            }
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub(super) struct AdjacencyContainer {
    pub(super) incoming: Arc<SkipSet<Neighbor>>,
    pub(super) outgoing: Arc<SkipSet<Neighbor>>,
}

impl AdjacencyContainer {
    pub fn new() -> Self {
        Self {
            incoming: Arc::new(SkipSet::new()),
            outgoing: Arc::new(SkipSet::new()),
        }
    }

    pub fn incoming(&self) -> &Arc<SkipSet<Neighbor>> {
        &self.incoming
    }

    pub fn outgoing(&self) -> &Arc<SkipSet<Neighbor>> {
        &self.outgoing
    }
}

pub struct MemoryGraph {
    // ---- Versioned data storage ----
    pub(super) vertices: DashMap<VertexId, VersionedVertex>, // Stores versioned vertices
    pub(super) edges: DashMap<EdgeId, VersionedEdge>,        // Stores versioned edges

    // ---- Adjacency list ----
    pub(super) adjacency_list: DashMap<VertexId, AdjacencyContainer>,

    // ---- Transaction management ----
    pub(super) txn_manager: MemTxnManager,

    // ---- Write-ahead-log for crash recovery ----
    pub(super) wal_manager: WalManager,

    // ---- Checkpoint management ----
    pub(super) checkpoint_manager: Option<CheckpointManager>,

    // ---- Vector indices ----
    pub(super) vector_indices: DashMap<VectorIndexKey, Box<dyn VectorIndex>>,
}

#[allow(dead_code)]
impl MemoryGraph {
    // ===== Basic methods =====
    /// Creates a new [`MemoryGraph`] instance using default configurations,
    /// and recovers its state from the latest checkpoint and WAL.
    ///
    /// This is a convenience method equivalent to:
    /// `MemoryGraph::with_config_recovered(Default::default(), Default::default())`
    pub fn new() -> Arc<Self> {
        Self::with_config_recovered(Default::default(), Default::default())
    }

    /// Creates a new [`MemoryGraph`] instance with the provided configuration,
    /// and recovers its state from persisted checkpoint and WAL.
    ///
    /// This function performs a full recovery process:
    /// - If a checkpoint is available, it restores the graph from that checkpoint and applies any
    ///   remaining WAL entries.
    /// - If no checkpoint is found, it reconstructs the graph entirely from WAL.
    ///
    /// # Returns
    ///
    /// A reference-counted [`MemoryGraph`] containing the recovered graph state.
    pub fn with_config_recovered(
        checkpoint_config: CheckpointManagerConfig,
        wal_config: WalManagerConfig,
    ) -> Arc<Self> {
        // Recover from checkpoint and WAL
        Self::recover_from_checkpoint_and_wal(checkpoint_config, wal_config).unwrap()
    }

    /// Creates a new [`MemoryGraph`] instance from scratch without performing recovery.
    ///
    /// This method initializes an empty in-memory graph with configured WAL and
    /// checkpoint managers. It is typically used for testing or creating a clean
    /// graph instance with no prior state.
    ///
    /// # Returns
    ///
    /// A new reference-counted [`MemoryGraph`] with no historical state.
    pub fn with_config_fresh(
        checkpoint_config: CheckpointManagerConfig,
        wal_config: WalManagerConfig,
    ) -> Arc<Self> {
        let graph = Arc::new(Self {
            vertices: DashMap::new(),
            edges: DashMap::new(),
            adjacency_list: DashMap::new(),
            txn_manager: MemTxnManager::new(),
            wal_manager: WalManager::new(wal_config),
            checkpoint_manager: None,
            vector_indices: DashMap::new(),
        });

        // Initialize the checkpoint manager
        let checkpoint_manager = CheckpointManager::new(graph.clone(), checkpoint_config).unwrap();
        unsafe {
            let graph_ptr = Arc::as_ptr(&graph) as *mut MemoryGraph;
            (*graph_ptr).checkpoint_manager = Some(checkpoint_manager);
        }

        graph
    }

    /// Recovers the graph from WAL entries
    pub fn recover_from_wal(self: &Arc<Self>) -> StorageResult<()> {
        let entries = self.wal_manager.wal().read().unwrap().read_all()?;
        self.apply_wal_entries(entries)
    }

    /// Applies a list of WAL entries to the graph
    pub fn apply_wal_entries(self: &Arc<Self>, entries: Vec<RedoEntry>) -> StorageResult<()> {
        let mut txn: Option<TransactionHandle> = None;
        for entry in entries {
            self.wal_manager.set_next_lsn(entry.lsn + 1);
            match entry.op {
                Operation::BeginTransaction(start_ts) => {
                    // Create a new transaction
                    let t = self.begin_transaction_at(
                        Some(entry.txn_id),
                        Some(start_ts),
                        entry.iso_level,
                        true,
                    );
                    txn = Some(TransactionHandle::new(t));
                }
                Operation::CommitTransaction(commit_ts) => {
                    // Commit the transaction
                    if let Some(txn) = txn.as_ref() {
                        txn.commit_at(Some(commit_ts), true)?;
                        txn.mark_handled(); // Avoid dropping the transaction handle
                    }
                    txn = None;
                }
                Operation::AbortTransaction => {
                    // Abort the transaction
                    if let Some(txn) = txn.as_ref() {
                        txn.abort_at(true)?;
                        txn.mark_handled(); // Avoid dropping the transaction handle
                    }
                    txn = None;
                }
                Operation::Delta(delta) => {
                    // Apply the delta
                    if let Some(txn) = txn.as_ref() {
                        match delta {
                            DeltaOp::CreateVertex(vertex) => {
                                self.create_vertex(txn, vertex)?;
                            }
                            DeltaOp::CreateEdge(edge) => {
                                self.create_edge(txn, edge)?;
                            }
                            DeltaOp::DelVertex(vid) => {
                                self.delete_vertex(txn, vid)?;
                            }
                            DeltaOp::DelEdge(eid) => {
                                self.delete_edge(txn, eid)?;
                            }
                            DeltaOp::SetVertexProps(vid, SetPropsOp { indices, props }) => {
                                self.set_vertex_property(txn, vid, indices, props)?;
                            }
                            DeltaOp::SetEdgeProps(eid, SetPropsOp { indices, props }) => {
                                self.set_edge_property(txn, eid, indices, props)?;
                            }
                            DeltaOp::AddLabel(_) => todo!(),
                            DeltaOp::RemoveLabel(_) => todo!(),
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Begins a new transaction and returns a `TransactionHandle` instance.
    pub fn begin_transaction(
        self: &Arc<Self>,
        isolation_level: IsolationLevel,
    ) -> TransactionHandle {
        let txn = self.begin_transaction_at(None, None, isolation_level, false);
        TransactionHandle::new(txn)
    }

    pub fn begin_transaction_at(
        self: &Arc<Self>,
        txn_id: Option<Timestamp>,
        start_ts: Option<Timestamp>,
        isolation_level: IsolationLevel,
        skip_wal: bool,
    ) -> Arc<MemTransaction> {
        // Update the counters
        let txn_id = self.txn_manager.new_txn_id(txn_id);
        let start_ts = self.txn_manager.new_commit_ts(start_ts);

        // Acquire the checkpoint lock to prevent new transactions from being created
        // while we are creating a checkpoint
        let _checkpoint_lock = self
            .checkpoint_manager
            .as_ref()
            .unwrap()
            .checkpoint_lock
            .read()
            .unwrap();

        // Register the transaction as active (used for garbage collection and visibility checks).
        let txn = Arc::new(MemTransaction::with_memgraph(
            self.clone(),
            txn_id,
            start_ts,
            isolation_level,
        ));
        self.txn_manager.start_transaction(txn.clone());

        // Write `Operation::BeginTransaction` to WAL,
        // unless the function is called when recovering from WAL
        if !skip_wal {
            let wal_entry = RedoEntry {
                lsn: self.wal_manager.next_lsn(),
                txn_id: txn.txn_id(),
                iso_level: *txn.isolation_level(),
                op: Operation::BeginTransaction(txn.start_ts()),
            };
            self.wal_manager
                .wal()
                .write()
                .unwrap()
                .append(&wal_entry)
                .unwrap();
        }

        txn
    }

    /// Returns a reference to the vertices storage.
    pub(super) fn vertices(&self) -> &DashMap<VertexId, VersionedVertex> {
        &self.vertices
    }

    /// Returns a reference to the edges storage.
    pub(super) fn edges(&self) -> &DashMap<EdgeId, VersionedEdge> {
        &self.edges
    }

    // ===== Read-only graph methods =====
    /// Retrieves a vertex by its ID within the context of a transaction.
    pub fn get_vertex(&self, txn: &TransactionHandle, vid: VertexId) -> StorageResult<Vertex> {
        // Step 1: Atomically retrieve the versioned vertex (check existence).
        let versioned_vertex = self.vertices.get(&vid).ok_or(StorageError::VertexNotFound(
            VertexNotFoundError::VertexNotFound(vid.to_string()),
        ))?;

        // Step 2: Perform MVCC visibility check.
        let current_version = versioned_vertex.chain.current.read().unwrap();
        let commit_ts = current_version.commit_ts;
        match txn.isolation_level() {
            IsolationLevel::Serializable => {
                // Insert the vertex ID into the read set
                txn.vertex_reads.insert(vid);
            }
            IsolationLevel::Snapshot => {
                // Optimistic read allowed, no read set recording
            }
        }
        let mut visible_vertex = current_version.data.clone();
        // Only when the vertex is modified by other transactions, or txn started before the vertex
        // was modified, we need to apply the deltas to the vertex
        if (commit_ts.is_txn_id() && commit_ts != txn.txn_id())
            || (commit_ts.is_commit_ts() && commit_ts > txn.start_ts())
        {
            let undo_ptr = versioned_vertex.chain.undo_ptr.read().unwrap().clone();
            let apply_deltas = |undo_entry: &UndoEntry| match undo_entry.delta() {
                DeltaOp::CreateVertex(original) => visible_vertex = original.clone(),
                DeltaOp::SetVertexProps(_, SetPropsOp { indices, props }) => {
                    visible_vertex.set_props(indices, props.clone());
                }
                DeltaOp::DelVertex(_) => {
                    visible_vertex.is_tombstone = true;
                }
                _ => unreachable!("Unreachable delta op for a vertex"),
            };
            MemTransaction::apply_deltas_for_read(undo_ptr, apply_deltas, txn.start_ts());
        }

        // Step 3: Check for logical deletion.
        if visible_vertex.is_tombstone() {
            return Err(StorageError::VertexNotFound(
                VertexNotFoundError::VertexTombstone(vid.to_string()),
            ));
        }

        Ok(visible_vertex)
    }

    /// Retrieves an edge by its ID within the context of a transaction.
    pub fn get_edge(&self, txn: &TransactionHandle, eid: EdgeId) -> StorageResult<Edge> {
        // Step 1: Atomically retrieve the versioned edge (check existence).
        let versioned_edge = self.edges.get(&eid).ok_or(StorageError::EdgeNotFound(
            EdgeNotFoundError::EdgeNotFound(eid.to_string()),
        ))?;

        // Step 2: Perform MVCC visibility check.
        let current_version = versioned_edge.chain.current.read().unwrap();
        let commit_ts = current_version.commit_ts;
        match txn.isolation_level() {
            IsolationLevel::Serializable => {
                // Insert the edge ID into the read set
                txn.edge_reads.insert(eid);
            }
            IsolationLevel::Snapshot => {
                // Optimistic read allowed, no read set recording
            }
        }
        let mut visible_edge = current_version.data.clone();
        // Only when the edge is modified by other transactions, or txn started before the edge was
        // modified, we need to apply the deltas to the edge
        if (commit_ts.is_txn_id() && commit_ts != txn.txn_id())
            || (commit_ts.is_commit_ts() && commit_ts > txn.start_ts())
        {
            let undo_ptr = versioned_edge.chain.undo_ptr.read().unwrap().clone();
            let apply_deltas = |undo_entry: &UndoEntry| match undo_entry.delta() {
                DeltaOp::CreateEdge(original) => visible_edge = original.clone(),
                DeltaOp::SetEdgeProps(_, SetPropsOp { indices, props }) => {
                    visible_edge.set_props(indices, props.clone());
                }
                DeltaOp::DelEdge(_) => {
                    visible_edge.is_tombstone = true;
                }
                _ => unreachable!("Unreachable delta op for an edge"),
            };
            MemTransaction::apply_deltas_for_read(undo_ptr, apply_deltas, txn.start_ts());
        }

        // Step 3: Check for logical deletion (tombstone).
        if visible_edge.is_tombstone() {
            return Err(StorageError::EdgeNotFound(
                EdgeNotFoundError::EdgeTombstone(eid.to_string()),
            ));
        }

        Ok(visible_edge)
    }

    /// Returns an iterator over all vertices within a transaction.
    pub fn iter_vertices<'a>(
        &'a self,
        txn: &'a TransactionHandle,
    ) -> StorageResult<Box<dyn Iterator<Item = StorageResult<Vertex>> + 'a>> {
        Ok(Box::new(txn.iter_vertices()))
    }

    /// Returns an iterator over all edges within a transaction.
    pub fn iter_edges<'a>(
        &'a self,
        txn: &'a TransactionHandle,
    ) -> StorageResult<Box<dyn Iterator<Item = StorageResult<Edge>> + 'a>> {
        Ok(Box::new(txn.iter_edges()))
    }

    /// Returns an iterator over the adjacency list of a vertex in a given direction.
    pub fn iter_adjacency<'a>(
        &'a self,
        txn: &'a TransactionHandle,
        vid: VertexId,
    ) -> StorageResult<Box<dyn Iterator<Item = StorageResult<Neighbor>> + 'a>> {
        Ok(Box::new(txn.iter_adjacency(vid)))
    }

    // ===== Mutable graph methods =====
    /// Inserts a new vertex into the graph within a transaction.
    pub fn create_vertex(
        &self,
        txn: &TransactionHandle,
        vertex: Vertex,
    ) -> StorageResult<VertexId> {
        let vid = vertex.vid();
        let entry = self
            .vertices
            .entry(vid)
            .or_insert_with(|| VersionedVertex::with_txn_id(vertex.clone(), txn.txn_id()));

        let current = entry.chain.current.read().unwrap();
        // Conflict detection: ensure the vertex is visible or not modified by other transactions
        check_write_conflict(current.commit_ts, txn)?;

        // Record the vertex creation in the transaction
        let delta = DeltaOp::DelVertex(vid);
        let next_ptr = entry.chain.undo_ptr.read().unwrap().clone();
        let mut undo_buffer = txn.undo_buffer.write().unwrap();
        let undo_entry = if current.commit_ts == txn.txn_id() {
            Arc::new(UndoEntry::new(delta, Timestamp(0), next_ptr))
        } else {
            Arc::new(UndoEntry::new(delta, current.commit_ts, next_ptr))
        };
        undo_buffer.push(undo_entry.clone());
        *entry.chain.undo_ptr.write().unwrap() = Arc::downgrade(&undo_entry);

        // Record redo entry
        let wal_entry = RedoEntry {
            lsn: 0, // Temporary set to 0, will be updated when commit
            txn_id: txn.txn_id(),
            iso_level: *txn.isolation_level(),
            op: Operation::Delta(DeltaOp::CreateVertex(vertex)),
        };
        txn.redo_buffer.write().unwrap().push(wal_entry);

        Ok(vid)
    }

    /// Inserts a new edge into the graph within a transaction.
    pub fn create_edge(&self, txn: &TransactionHandle, edge: Edge) -> StorageResult<EdgeId> {
        let eid = edge.eid();
        let src_id = edge.src_id();
        let dst_id = edge.dst_id();
        let label_id = edge.label_id();

        // Check if source and destination vertices exist.
        self.get_vertex(txn, edge.src_id())?;

        self.get_vertex(txn, edge.dst_id())?;

        let entry = self
            .edges
            .entry(eid)
            .or_insert_with(|| VersionedEdge::with_modified_ts(edge.clone(), txn.txn_id()));

        let current = entry.chain.current.read().unwrap();
        // Conflict detection: ensure the edge is visible or not modified by other transactions
        check_write_conflict(current.commit_ts, txn)?;

        // Record the edge creation in the transaction
        let delta_edge = DeltaOp::DelEdge(eid);
        let undo_ptr = entry.chain.undo_ptr.read().unwrap().clone();
        // Update the undo_entry logical pointer
        let mut undo_buffer = txn.undo_buffer.write().unwrap();
        let undo_entry = Arc::new(UndoEntry::new(delta_edge, current.commit_ts, undo_ptr));
        undo_buffer.push(undo_entry.clone());
        *entry.chain.undo_ptr.write().unwrap() = Arc::downgrade(&undo_entry);

        // Record the adjacency list updates in the transaction
        self.adjacency_list
            .entry(src_id)
            .or_insert_with(AdjacencyContainer::new)
            .outgoing()
            .insert(Neighbor::new(label_id, dst_id, eid));
        self.adjacency_list
            .entry(dst_id)
            .or_insert_with(AdjacencyContainer::new)
            .incoming()
            .insert(Neighbor::new(label_id, src_id, eid));

        // Write to WAL
        let wal_entry = RedoEntry {
            lsn: 0, // Temporary set to 0, will be updated when commit
            txn_id: txn.txn_id(),
            iso_level: *txn.isolation_level(),
            op: Operation::Delta(DeltaOp::CreateEdge(edge)),
        };
        txn.redo_buffer.write().unwrap().push(wal_entry);

        Ok(eid)
    }

    /// Deletes a vertex from the graph within a transaction.
    pub fn delete_vertex(&self, txn: &TransactionHandle, vid: VertexId) -> StorageResult<()> {
        // Atomically retrieve the versioned vertex (check existence).
        let entry = self.vertices.get(&vid).ok_or(StorageError::VertexNotFound(
            VertexNotFoundError::VertexNotFound(vid.to_string()),
        ))?;

        let mut current = entry.chain.current.write().unwrap();
        check_write_conflict(current.commit_ts, txn)?;

        // Delete all edges associated with the vertex
        if let Some(adjacency_container) = self.adjacency_list.get(&vid) {
            for adj in adjacency_container.incoming().iter() {
                if self.edges.get(&adj.value().eid()).is_some() {
                    self.delete_edge(txn, adj.value().eid())?;
                }
            }
            for adj in adjacency_container.outgoing().iter() {
                if self.edges.get(&adj.value().eid()).is_some() {
                    self.delete_edge(txn, adj.value().eid())?;
                }
            }
        }

        // Record the vertex deletion in the transaction
        let delta = DeltaOp::CreateVertex(current.data.clone());
        let undo_ptr = entry.chain.undo_ptr.read().unwrap().clone();
        let mut undo_buffer = txn.undo_buffer.write().unwrap();
        let undo_entry = Arc::new(UndoEntry::new(delta, current.commit_ts, undo_ptr));
        undo_buffer.push(undo_entry.clone());
        *entry.chain.undo_ptr.write().unwrap() = Arc::downgrade(&undo_entry);

        // Mark the vertex as deleted
        let tombstone = Vertex::tombstone(current.data.clone());
        current.data = tombstone;
        current.commit_ts = txn.txn_id();

        // Write to WAL
        let wal_entry = RedoEntry {
            lsn: 0, // Temporary set to 0, will be updated when commit
            txn_id: txn.txn_id(),
            iso_level: *txn.isolation_level(),
            op: Operation::Delta(DeltaOp::DelVertex(vid)),
        };
        txn.redo_buffer.write().unwrap().push(wal_entry);

        Ok(())
    }

    /// Deletes an edge from the graph within a transaction.
    pub fn delete_edge(&self, txn: &TransactionHandle, eid: EdgeId) -> StorageResult<()> {
        // Atomically retrieve the versioned edge (check existence).
        let entry = self.edges.get(&eid).ok_or(StorageError::EdgeNotFound(
            EdgeNotFoundError::EdgeNotFound(eid.to_string()),
        ))?;

        let mut current = entry.chain.current.write().unwrap();
        check_write_conflict(current.commit_ts, txn)?;

        // Record the edge deletion in the transaction
        let delta = DeltaOp::CreateEdge(current.data.clone());
        let undo_ptr = entry.chain.undo_ptr.read().unwrap().clone();
        let mut undo_buffer = txn.undo_buffer.write().unwrap();
        let undo_entry = Arc::new(UndoEntry::new(delta, current.commit_ts, undo_ptr));
        undo_buffer.push(undo_entry.clone());
        *entry.chain.undo_ptr.write().unwrap() = Arc::downgrade(&undo_entry);

        // Mark the edge as deleted
        let tombstone = Edge::tombstone(current.data.clone());
        current.data = tombstone;
        current.commit_ts = txn.txn_id();

        // Write to WAL
        let wal_entry = RedoEntry {
            lsn: 0, // Temporary set to 0, will be updated when commit
            txn_id: txn.txn_id(),
            iso_level: *txn.isolation_level(),
            op: Operation::Delta(DeltaOp::DelEdge(eid)),
        };
        txn.redo_buffer.write().unwrap().push(wal_entry);

        Ok(())
    }

    /// Updates the properties of a vertex within a transaction.
    pub fn set_vertex_property(
        &self,
        txn: &TransactionHandle,
        vid: VertexId,
        indices: Vec<usize>,
        props: Vec<ScalarValue>,
    ) -> StorageResult<()> {
        // Atomically retrieve the versioned vertex (check existence).
        let entry = self.vertices.get(&vid).ok_or(StorageError::VertexNotFound(
            VertexNotFoundError::VertexNotFound(vid.to_string()),
        ))?;

        update_properties!(
            self,
            vid,
            entry,
            txn,
            indices.clone(),
            props.clone(),
            SetVertexProps
        );

        // Write to WAL
        let wal_entry = RedoEntry {
            lsn: 0, // Temporary set to 0, will be updated when commit
            txn_id: txn.txn_id(),
            iso_level: *txn.isolation_level(),
            op: Operation::Delta(DeltaOp::SetVertexProps(vid, SetPropsOp { indices, props })),
        };
        txn.redo_buffer.write().unwrap().push(wal_entry);

        Ok(())
    }

    /// Updates the properties of an edge within a transaction.
    pub fn set_edge_property(
        &self,
        txn: &TransactionHandle,
        eid: EdgeId,
        indices: Vec<usize>,
        props: Vec<ScalarValue>,
    ) -> StorageResult<()> {
        // Atomically retrieve the versioned edge (check existence).
        let entry = self.edges.get(&eid).ok_or(StorageError::EdgeNotFound(
            EdgeNotFoundError::EdgeNotFound(eid.to_string()),
        ))?;

        update_properties!(
            self,
            eid,
            entry,
            txn,
            indices.clone(),
            props.clone(),
            SetEdgeProps
        );

        // Write to WAL
        let wal_entry = RedoEntry {
            lsn: 0, // Temporary set to 0, will be updated when commit
            txn_id: txn.txn_id(),
            iso_level: *txn.isolation_level(),
            op: Operation::Delta(DeltaOp::SetEdgeProps(eid, SetPropsOp { indices, props })),
        };
        txn.redo_buffer.write().unwrap().push(wal_entry);

        Ok(())
    }

    // ===== Vector index methods =====
    /// Collect vectors from graph nodes for the specified label and property
    fn collect_vectors_for_index(
        &self,
        txn: &TransactionHandle,
        label_id: LabelId,
        property_id: PropertyId,
    ) -> StorageResult<Vec<(u64, Vec<f32>)>> {
        let mut vectors = Vec::new();

        // Iterate through all vertices in the graph
        let vertex_iter = self.iter_vertices(txn)?;

        for vertex_result in vertex_iter {
            let vertex = vertex_result?;

            // Skip vertices that don't match the specified label
            if vertex.label_id != label_id {
                continue;
            }

            let node_id = vertex.vid();

            // Check if vertex has the specified property at the given index
            if let Ok(idx) = usize::try_from(property_id) {
                if let Some(property_value) = vertex.properties().get(idx) {
                    // Check if the property is a vector type
                    match property_value {
                        ScalarValue::Vector(Some(vector_data)) => {
                            // Convert F32 wrapper to f32
                            let vector: Vec<f32> = vector_data
                                .iter()
                                .map(|f32_val| f32_val.into_inner())
                                .collect();
                            vectors.push((node_id, vector));
                        }
                        ScalarValue::Vector(None) => {
                            // Skip null vector values
                            continue;
                        }
                        _ => {
                            // Property exists but is not a vector - skip
                            continue;
                        }
                    }
                }
            }
        }

        Ok(vectors)
    }

    /// Build a vector index for the specified property within a specific label
    /// Automatically infers vector dimension from the data and validates consistency
    pub fn build_vector_index(
        &self,
        txn: &TransactionHandle,
        index_key: VectorIndexKey,
    ) -> StorageResult<()> {
        let label_id = index_key.label_id;
        let property_id = index_key.property_id;

        // Collect vectors from graph nodes with the specified label and property
        let vectors = self.collect_vectors_for_index(txn, label_id, property_id)?;
        if vectors.is_empty() {
            return Err(StorageError::VectorIndex(VectorIndexError::EmptyDataset));
        }
        // Infer dimension from the first vector
        let inferred_dimension = vectors[0].1.len();
        // Validate all vectors have consistent dimensions
        for (_node_id, vector) in &vectors {
            if vector.len() != inferred_dimension {
                return Err(StorageError::VectorIndex(
                    VectorIndexError::InvalidDimension {
                        expected: inferred_dimension,
                        actual: vector.len(),
                    },
                ));
            }
        }

        // Validate dimension is supported by DiskANN SIMD implementation
        match inferred_dimension {
            104 | 128 | 256 => {
                // Supported dimensions, continue with index building
            }
            _ => {
                return Err(StorageError::VectorIndex(
                    VectorIndexError::UnsupportedOperation(format!(
                        "Dimension {} not supported. Only dimensions 104, 128, 256 are supported.",
                        inferred_dimension
                    )),
                ));
            }
        }

        // Create index configuration with intelligent capacity based on actual vector count
        let vector_count = vectors.len();
        let index_config = create_vector_index_config(inferred_dimension, vector_count);
        let mut adapter = InMemDiskANNAdapter::new(index_config)?;
        adapter.build(&vectors)?;

        // Store the index in the hash map using the provided composite key
        self.vector_indices
            .insert(index_key, Box::new(adapter) as Box<dyn VectorIndex>);

        Ok(())
    }

    /// Get vector index for the specified label and property
    pub fn get_vector_index(
        &self,
        index_key: VectorIndexKey,
    ) -> Option<dashmap::mapref::one::Ref<VectorIndexKey, Box<dyn VectorIndex>>> {
        self.vector_indices.get(&index_key)
    }

    /// Perform vector similarity search (returns node IDs)
    ///
    /// # Arguments
    /// * `index_key` - The VectorIndexKey identifying the vector index (label + property)
    /// * `query` - Query vector for similarity search
    /// * `k` - Number of nearest neighbors to return
    /// * `l_value` - Search list size parameter
    /// * `filter_bitmap` - Optional boolean array indicating which nodes to consider
    /// * `should_pre` - should pre-filter
    pub fn vector_search(
        &self,
        index_key: VectorIndexKey,
        query: &[f32],
        k: usize,
        l_value: u32,
        filter_bitmap: Option<&BooleanArray>,
        should_pre: bool,
    ) -> StorageResult<Vec<u64>> {
        let index_ref = self.get_vector_index(index_key).ok_or_else(|| {
            StorageError::VectorIndex(VectorIndexError::IndexNotFound(format!(
                "index_key: {:?}",
                index_key
            )))
        })?;

        // Validate query vector dimension matches index dimension
        if query.len() != index_ref.get_dimension() {
            return Err(StorageError::VectorIndex(
                VectorIndexError::InvalidDimension {
                    expected: index_ref.get_dimension(),
                    actual: query.len(),
                },
            ));
        }

        // Convert BooleanArray to optimal FilterMask if provided
        let filter_mask = filter_bitmap.map(|bitmap| {
            let candidate_vector_ids = Self::bitmap_to_vector_ids(bitmap, &**index_ref);
            create_filter_mask(candidate_vector_ids, index_ref.size())
        });
        let results = index_ref.search(query, k, l_value, filter_mask.as_deref(), should_pre)?;

        Ok(results)
    }

    /// Extract node IDs from a boolean bitmap where the value is true
    fn extract_true_node_ids(bitmap: &BooleanArray) -> Vec<u64> {
        bitmap
            .iter()
            .enumerate()
            .filter_map(|(idx, value)| {
                if value.unwrap_or(false) {
                    Some(idx as u64)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Convert a boolean bitmap to a list of vector IDs for filtering
    fn bitmap_to_vector_ids(bitmap: &BooleanArray, index: &dyn VectorIndex) -> Vec<u32> {
        Self::extract_true_node_ids(bitmap)
            .into_iter()
            .filter_map(|node_id| index.node_to_vector_id(node_id))
            .collect()
    }

    /// Get mutable vector index for the specified label and property
    fn get_mutable_vector_index(
        &self,
        index_key: VectorIndexKey,
    ) -> Option<dashmap::mapref::one::RefMut<VectorIndexKey, Box<dyn VectorIndex>>> {
        self.vector_indices.get_mut(&index_key)
    }

    /// Insert vectors into the specified vector index
    pub fn insert_into_vector_index(
        &self,
        index_key: VectorIndexKey,
        vectors: &[(u64, Vec<f32>)],
    ) -> StorageResult<()> {
        if vectors.is_empty() {
            return Ok(());
        }

        let mut index_ref = self.get_mutable_vector_index(index_key).ok_or_else(|| {
            StorageError::VectorIndex(VectorIndexError::IndexNotFound(format!(
                "label_id: {}, property_id: {}",
                index_key.label_id, index_key.property_id
            )))
        })?;

        // Validate dimension consistency for all vectors
        let expected_dim = index_ref.get_dimension();
        for (_node_id, vector) in vectors {
            if vector.len() != expected_dim {
                return Err(StorageError::VectorIndex(
                    VectorIndexError::InvalidDimension {
                        expected: expected_dim,
                        actual: vector.len(),
                    },
                ));
            }
        }

        index_ref.insert(vectors)?;

        Ok(())
    }

    /// Delete vectors from the specified vector index
    pub fn delete_from_vector_index(
        &self,
        index_key: VectorIndexKey,
        node_ids: &[u64],
    ) -> StorageResult<()> {
        if node_ids.is_empty() {
            return Ok(());
        }

        let mut index_ref = self.get_mutable_vector_index(index_key).ok_or_else(|| {
            StorageError::VectorIndex(VectorIndexError::IndexNotFound(format!(
                "label_id: {}, property_id: {}",
                index_key.label_id, index_key.property_id
            )))
        })?;

        index_ref.soft_delete(node_ids)?;

        Ok(())
    }
}

/// Checks if the vertex is modified by other transactions or has a greater commit timestamp than
/// the current transaction.
/// Current check applies to both Snapshot Isolation and Serializable isolation levels.
#[inline]
fn check_write_conflict(commit_ts: Timestamp, txn: &TransactionHandle) -> StorageResult<()> {
    match commit_ts {
        // If the vertex is modified by other transactions, return write-write conflict
        ts if ts.is_txn_id() && ts != txn.txn_id() => Err(StorageError::Transaction(
            TransactionError::WriteWriteConflict(format!(
                "Data is being modified by transaction {:?}",
                ts
            )),
        )),
        // If the vertex is committed by other transactions and its commit timestamp is greater
        // than the start timestamp of the current transaction, return version not visible
        ts if ts.is_commit_ts() && ts > txn.start_ts() => Err(StorageError::Transaction(
            TransactionError::VersionNotVisible(format!(
                "Data version not visible for {:?}",
                txn.txn_id()
            )),
        )),
        _ => Ok(()),
    }
}

#[cfg(test)]
pub mod tests {
    use std::fs;

    use minigu_common::types::LabelId;
    use minigu_common::value::{F32, ScalarValue};
    use {Edge, Vertex};

    use super::*;
    use crate::model::properties::PropertyRecord;

    const PERSON: LabelId = LabelId::new(1).unwrap();
    const FRIEND: LabelId = LabelId::new(2).unwrap();
    const FOLLOW: LabelId = LabelId::new(3).unwrap();

    // Vector index test constants
    const _NAME_PROPERTY_ID: PropertyId = 0;
    const EMBEDDING_PROPERTY_ID: PropertyId = 1;
    const TEST_DIMENSION: usize = 104; // Supported dimensions: 104, 128, 256

    fn create_vertex(id: VertexId, label_id: LabelId, properties: Vec<ScalarValue>) -> Vertex {
        Vertex::new(id, label_id, PropertyRecord::new(properties))
    }

    fn create_edge(
        id: EdgeId,
        src_id: VertexId,
        dst_id: VertexId,
        label_id: LabelId,
        properties: Vec<ScalarValue>,
    ) -> Edge {
        Edge::new(
            id,
            src_id,
            dst_id,
            label_id,
            PropertyRecord::new(properties),
        )
    }

    /// Creates a test vertex with vector embedding
    fn create_vertex_with_vector(id: VertexId, name: &str, embedding: Vec<f32>) -> Vertex {
        let vector_value =
            ScalarValue::Vector(Some(embedding.into_iter().map(F32::from).collect()));

        Vertex::new(
            id,
            PERSON,
            PropertyRecord::new(vec![
                ScalarValue::String(Some(name.to_string())), // Property 0: name
                vector_value,                                // Property 1: embedding
            ]),
        )
    }

    /// Generates 200 small-scale test vectors with big coordinates to ensure DiskANN graph
    /// connectivity
    fn create_small_scale_test_vectors() -> Vec<(VertexId, String, Vec<f32>)> {
        let count = 200;
        let points_per_cluster = 25; // 25 points per cluster, 8 clusters
        // Not all graph nodes have vectors; so vids to vector search are non-contiguous (sparse
        // subset)
        let start_id: VertexId = 5;
        let stride: VertexId = 3;

        (0..count)
            .map(|i| {
                let cluster_id = i / points_per_cluster;
                let point_in_cluster = i % points_per_cluster;

                let mut vector = vec![0.0f32; TEST_DIMENSION];

                // Large coordinate cluster centers (avoid small value precision issues)
                let center_x = (cluster_id as f32) * 20.0 + 30.0; // [30, 50, 70, 90, 110, 130, 150, 170]
                let center_y = (cluster_id as f32) * 15.0 + 25.0; // [25, 40, 55, 70, 85, 100, 115, 130]
                let center_z = (cluster_id as f32) * 12.0 + 20.0; // [20, 32, 44, 56, 68, 80, 92, 104]

                // Intra-cluster distribution (ensure overlapping connectivity)
                let spread = 12.0; // cluster spread range
                let offset_x = ((point_in_cluster as f32) * 2.1).sin() * spread;
                let offset_y = ((point_in_cluster as f32) * 1.8).cos() * spread;
                let offset_z = ((point_in_cluster as f32) * 2.5).sin() * spread;

                vector[0] = center_x + offset_x;
                vector[1] = center_y + offset_y;
                vector[2] = center_z + offset_z;

                // Other dimensions: add unique identifiers
                let start = 3;
                let end = std::cmp::min(10, TEST_DIMENSION);

                for (j, item) in vector.iter_mut().enumerate().skip(start).take(end - start) {
                    *item = (i as f32) * 0.1 + (j as f32) * 0.2 + 5.0;
                }
                let vid: VertexId = start_id + (i as VertexId) * stride;
                (vid, format!("small_scale_{}", i), vector)
            })
            .collect()
    }

    pub fn mock_checkpoint_config() -> CheckpointManagerConfig {
        let temp_dir = temp_dir::TempDir::with_prefix("test_checkpoint_").unwrap();
        let dir = temp_dir.path().to_owned();
        // TODO: Pass the temp dir to the caller so that it can be cleaned up.
        temp_dir.leak();
        CheckpointManagerConfig {
            checkpoint_dir: dir,
            max_checkpoints: 3,
            auto_checkpoint_interval_secs: 0, // Disable auto checkpoints for testing
            checkpoint_prefix: "test_checkpoint".to_string(),
            transaction_timeout_secs: 10,
        }
    }

    pub fn mock_wal_config() -> WalManagerConfig {
        let temp_file = temp_file::TempFileBuilder::new()
            .prefix("test_wal_")
            .suffix(".log")
            .build()
            .unwrap();
        let path = temp_file.path().to_owned();
        // TODO: Pass the temp file to the caller so that it can be cleaned up.
        temp_file.leak();
        WalManagerConfig { wal_path: path }
    }

    pub struct Cleaner {
        wal_path: std::path::PathBuf,
        checkpoint_dir: std::path::PathBuf,
    }

    impl Cleaner {
        pub fn new(
            checkpoint_config: &CheckpointManagerConfig,
            wal_config: &WalManagerConfig,
        ) -> Self {
            Self {
                wal_path: wal_config.wal_path.clone(),
                checkpoint_dir: checkpoint_config.checkpoint_dir.clone(),
            }
        }
    }

    impl Drop for Cleaner {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.wal_path);
            let _ = fs::remove_dir_all(&self.checkpoint_dir);
        }
    }

    pub fn mock_graph() -> (Arc<MemoryGraph>, Cleaner) {
        let checkpoint_config = mock_checkpoint_config();
        let wal_config = mock_wal_config();
        mock_graph_with_config(checkpoint_config, wal_config)
    }

    pub fn mock_empty_graph() -> (Arc<MemoryGraph>, Cleaner) {
        let checkpoint_config = mock_checkpoint_config();
        let wal_config = mock_wal_config();
        let cleaner = Cleaner::new(&checkpoint_config, &wal_config);
        let graph = MemoryGraph::with_config_fresh(checkpoint_config, wal_config);
        (graph, cleaner)
    }

    // Create a graph with 4 vertices and 4 edges
    pub fn mock_graph_with_config(
        checkpoint_config: CheckpointManagerConfig,
        wal_config: WalManagerConfig,
    ) -> (Arc<MemoryGraph>, Cleaner) {
        let cleaner = Cleaner::new(&checkpoint_config, &wal_config);
        let graph = MemoryGraph::with_config_recovered(mock_checkpoint_config(), mock_wal_config());

        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        let alice = create_vertex(1, PERSON, vec![
            ScalarValue::String(Some("Alice".to_string())),
            ScalarValue::Int32(Some(25)),
        ]);

        let bob = create_vertex(2, PERSON, vec![
            ScalarValue::String(Some("Bob".to_string())),
            ScalarValue::Int32(Some(28)),
        ]);

        let carol = create_vertex(3, PERSON, vec![
            ScalarValue::String(Some("Carol".to_string())),
            ScalarValue::Int32(Some(24)),
        ]);

        let david = create_vertex(4, PERSON, vec![
            ScalarValue::String(Some("David".to_string())),
            ScalarValue::Int32(Some(27)),
        ]);

        // Add vertices to the graph
        graph.create_vertex(&txn, alice).unwrap();
        graph.create_vertex(&txn, bob).unwrap();
        graph.create_vertex(&txn, carol).unwrap();
        graph.create_vertex(&txn, david).unwrap();

        // Create friend edges
        let friend1 = create_edge(1, 1, 2, FRIEND, vec![ScalarValue::String(Some(
            "2020-01-01".to_string(),
        ))]);

        let friend2 = create_edge(2, 2, 3, FRIEND, vec![ScalarValue::String(Some(
            "2021-03-15".to_string(),
        ))]);

        // Create follow edges
        let follow1 = create_edge(3, 1, 3, FOLLOW, vec![ScalarValue::String(Some(
            "2022-06-01".to_string(),
        ))]);

        let follow2 = create_edge(4, 4, 1, FOLLOW, vec![ScalarValue::String(Some(
            "2022-07-15".to_string(),
        ))]);

        // Add edges to the graph
        graph.create_edge(&txn, friend1).unwrap();
        graph.create_edge(&txn, friend2).unwrap();
        graph.create_edge(&txn, follow1).unwrap();
        graph.create_edge(&txn, follow2).unwrap();

        txn.commit().unwrap();
        (graph, cleaner)
    }

    fn create_vertex_eve() -> Vertex {
        create_vertex(5, PERSON, vec![
            ScalarValue::String(Some("Eve".to_string())),
            ScalarValue::Int32(Some(24)),
        ])
    }

    fn create_vertex_frank() -> Vertex {
        create_vertex(6, PERSON, vec![
            ScalarValue::String(Some("Frank".to_string())),
            ScalarValue::Int32(Some(25)),
        ])
    }

    fn create_edge_alice_to_eve() -> Edge {
        create_edge(5, 1, 5, FRIEND, vec![ScalarValue::String(Some(
            "2025-03-31".to_string(),
        ))])
    }

    #[test]
    fn test_basic_commit_flow() {
        let (graph, _cleaner) = mock_graph();
        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);

        let v1 = create_vertex_eve();
        let vid1 = graph.create_vertex(&txn1, v1.clone()).unwrap();
        let _ = txn1.commit().unwrap();

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        let read_v1 = graph.get_vertex(&txn2, vid1).unwrap();
        assert_eq!(read_v1, v1);
        assert!(txn2.commit().is_ok());
    }

    #[test]
    fn test_mvcc_version_chain() {
        let (graph, _cleaner) = mock_graph();

        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_eve();
        let vid1 = graph.create_vertex(&txn1, v1).unwrap();
        assert!(txn1.commit().is_ok());

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        let old_v1: Vertex = graph.get_vertex(&txn2, vid1).unwrap();
        assert_eq!(old_v1.properties()[1], ScalarValue::Int32(Some(24)));
        assert!(
            graph
                .set_vertex_property(&txn2, vid1, vec![1], vec![ScalarValue::Int32(Some(25))])
                .is_ok()
        );
        assert!(txn2.commit().is_ok());

        let txn3 = graph.begin_transaction(IsolationLevel::Serializable);
        let new_v1: Vertex = graph.get_vertex(&txn3, vid1).unwrap();
        assert_eq!(new_v1.properties()[1], ScalarValue::Int32(Some(25)));
    }

    #[test]
    fn test_delete_with_tombstone() {
        let (graph, _cleaner) = mock_graph();

        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_eve();
        let vid1 = graph.create_vertex(&txn1, v1).unwrap();
        assert!(txn1.commit().is_ok());

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        graph.delete_vertex(&txn2, vid1).unwrap();
        assert!(txn2.commit().is_ok());

        let txn3 = graph.begin_transaction(IsolationLevel::Serializable);
        assert!(graph.get_vertex(&txn3, vid1).is_err());
    }

    #[test]
    fn test_adjacency_versioning() {
        let (graph, _cleaner) = mock_graph();

        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_eve();

        let vid1 = graph.create_vertex(&txn1, v1).unwrap();
        assert!(txn1.commit().is_ok());

        // Create an edge from alice to eve
        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        let e1 = create_edge_alice_to_eve();
        let eid1 = graph.create_edge(&txn2, e1).unwrap();
        let v_alice = graph.get_vertex(&txn2, 1).unwrap();
        let vid_alice = v_alice.vid();
        assert!(txn2.commit().is_ok());

        // Check the edge from alice to eve
        let txn3 = graph.begin_transaction(IsolationLevel::Serializable);
        let e1 = graph.get_edge(&txn3, eid1).unwrap();
        assert!(e1.src_id() == vid_alice && e1.dst_id() == vid1);

        // Check the adjacency list of alice
        {
            let iter = txn3.iter_adjacency(vid_alice);
            let mut count = 0;
            for _ in iter {
                count += 1;
            }
            assert_eq!(count, 4);
        }

        // Check the outgoing adjacency list of alice
        {
            let iter = txn3.iter_adjacency_outgoing(vid_alice);
            let mut count = 0;
            for _ in iter {
                count += 1;
            }
            assert_eq!(count, 3);
        }

        // Check the incoming adjacency list of eve
        {
            let iter = txn3.iter_adjacency_incoming(vid1);
            let mut count = 0;
            for _ in iter {
                count += 1;
            }
            assert_eq!(count, 1);
        }

        let _ = txn3.abort();

        // Delete the edge from alice to eve
        let txn4 = graph.begin_transaction(IsolationLevel::Serializable);
        graph.delete_edge(&txn4, eid1).unwrap();
        assert!(txn4.commit().is_ok());

        let txn5 = graph.begin_transaction(IsolationLevel::Serializable);
        {
            // Check the adjacency list of alice
            let iter = txn5.iter_adjacency(vid_alice);
            let mut count = 0;
            for _ in iter {
                count += 1;
            }
            assert!(count == 3);
        }
        let _ = txn5.abort();
    }

    #[test]
    fn test_rollback_consistency() {
        let (graph, _cleaner) = mock_graph();

        let txn = graph.begin_transaction(IsolationLevel::Serializable);
        let vid1 = graph.create_vertex(&txn, create_vertex_eve()).unwrap();
        let _ = txn.abort();

        let txn_check = graph.begin_transaction(IsolationLevel::Serializable);
        assert!(graph.get_vertex(&txn_check, vid1).is_err());
    }

    #[test]
    fn test_property_update_flow() {
        let (graph, _cleaner) = mock_graph();

        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_eve();
        let vid1 = graph.create_vertex(&txn1, v1).unwrap();
        assert!(txn1.commit().is_ok());

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        graph
            .set_vertex_property(&txn2, vid1, vec![0], vec![ScalarValue::Int32(Some(25))])
            .unwrap();
        assert!(txn2.commit().is_ok());

        let txn3 = graph.begin_transaction(IsolationLevel::Serializable);
        let v = graph.get_vertex(&txn3, vid1).unwrap();
        assert_eq!(v.properties()[0], ScalarValue::Int32(Some(25)));
    }

    #[test]
    fn test_vertex_iterator() {
        let (graph, _cleaner) = mock_graph();

        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_eve();
        let v2 = create_vertex_frank();
        let _ = graph.create_vertex(&txn1, v1).unwrap();
        let _ = graph.create_vertex(&txn1, v2).unwrap();
        assert!(txn1.commit().is_ok());

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        {
            let iter1 =
                txn2.iter_vertices()
                    .filter_map(|v| v.ok())
                    .filter(|v| match &v.properties()[0] {
                        ScalarValue::String(Some(name)) => name == "Eve",
                        _ => false,
                    });
            let mut count = 0;
            for _ in iter1 {
                count += 1;
            }
            assert_eq!(count, 1);
        }
        {
            let iter2 =
                txn2.iter_vertices()
                    .filter_map(|v| v.ok())
                    .filter(|v| match v.properties()[1] {
                        ScalarValue::Int32(Some(age)) => (20..=25).contains(&age),
                        _ => false,
                    });
            let mut count = 0;
            for _ in iter2 {
                count += 1;
            }
            assert_eq!(count, 4);
        }
        let _ = txn2.abort();
    }

    #[test]
    fn test_edge_iterator() {
        let (graph, _cleaner) = mock_graph();

        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_eve();
        let v2 = create_vertex_frank();
        let _ = graph.create_vertex(&txn1, v1).unwrap();
        let _ = graph.create_vertex(&txn1, v2).unwrap();
        let e1 = create_edge_alice_to_eve();
        let _ = graph.create_edge(&txn1, e1).unwrap();
        assert!(txn1.commit().is_ok());

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        {
            let iter1 = txn2
                .iter_edges()
                .filter_map(|e| e.ok())
                .filter(|e| e.src_id() == 1);
            let mut count = 0;
            for _ in iter1 {
                count += 1;
            }
            assert_eq!(count, 3);
        }
        {
            let iter2 = txn2
                .iter_edges()
                .filter_map(|e| e.ok())
                .filter(|e| e.dst_id() == 5);
            let mut count = 0;
            for _ in iter2 {
                count += 1;
            }
            assert_eq!(count, 1);
        }
        {
            let iter3 = txn2
                .iter_edges()
                .filter_map(|e| e.ok())
                .filter(|e| e.label_id() == FRIEND);
            let mut count = 0;
            for _ in iter3 {
                count += 1;
            }
            assert_eq!(count, 3);
        }
        let _ = txn2.abort();
    }

    #[test]
    fn test_adj_iterator() {
        let (graph, _cleaner) = mock_graph();

        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_eve();
        let v2 = create_vertex_frank();
        let vid1 = graph.create_vertex(&txn1, v1).unwrap();
        let _ = graph.create_vertex(&txn1, v2).unwrap();
        let e1 = create_edge_alice_to_eve();
        let _ = graph.create_edge(&txn1, e1).unwrap();
        assert!(txn1.commit().is_ok());

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        {
            let iter1 = txn2.iter_adjacency(vid1);
            let mut count = 0;
            for _ in iter1 {
                count += 1;
            }
            assert_eq!(count, 1);
        }
        let _ = txn2.abort();
    }

    #[test]
    fn test_garbage_collection_after_delete_edge() {
        let (graph, _cleaner) = mock_graph();

        let vid1: VertexId = 1;
        let vid2: VertexId = 2;
        let eid: EdgeId = 1;

        // Check before GC
        {
            let adj = graph.adjacency_list.get(&vid1).unwrap();
            assert!(adj.outgoing().len() == 2);
            assert!(adj.incoming().len() == 1);
            let edge = graph.edges.get(&eid).unwrap();
            assert!(!edge.value().chain.current.read().unwrap().data.is_tombstone);
            assert!(
                edge.value()
                    .chain
                    .undo_ptr
                    .read()
                    .unwrap()
                    .upgrade()
                    .is_some()
            );
        }

        // Delete the edge
        let txn = graph.begin_transaction(IsolationLevel::Serializable);
        graph.delete_edge(&txn, eid).unwrap();
        assert!(txn.commit().is_ok());

        // Commit an empty transaction to update the watermark
        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        assert!(txn2.commit().is_ok());

        // Check before GC
        {
            let adj = graph.adjacency_list.get(&vid1).unwrap();
            // adjacency_list will not be updated until GC
            assert!(adj.outgoing().len() == 2);
            assert!(adj.incoming().len() == 1);
            // reverse edge
            let adj2 = graph.adjacency_list.get(&vid2).unwrap();
            assert!(adj2.outgoing().len() == 1);
            assert!(adj2.incoming().len() == 1);
            // edge is marked as tombstone
            let edge = graph.edges.get(&eid).unwrap();
            assert!(edge.value().chain.current.read().unwrap().data.is_tombstone);
            assert!(
                edge.value()
                    .chain
                    .undo_ptr
                    .read()
                    .unwrap()
                    .upgrade()
                    .is_some()
            );
            // However, iter will check the visibility of the adjacency
            let iter = txn2.iter_adjacency(vid1);
            let mut count = 0;
            for _ in iter {
                count += 1;
            }
            assert!(count == 2);
        }

        graph.txn_manager.garbage_collect(txn2.graph()).unwrap();
        // Check after GC
        {
            let adj = graph.adjacency_list.get(&vid1).unwrap();
            assert!(adj.outgoing().len() == 1);
            assert!(adj.incoming().len() == 1);
            // reverse edge
            let adj2 = graph.adjacency_list.get(&vid2).unwrap();
            assert!(adj2.outgoing().len() == 1);
            assert!(adj2.incoming().is_empty());
            // GC will remove the edge
            assert!(graph.edges.get(&eid).is_none());
        }
    }

    #[test]
    fn test_garbage_collection_after_delete_vertex() {
        let (graph, _cleaner) = mock_graph();

        let vid1 = 1;
        let euid1 = Neighbor::new(FRIEND, 1, 1);

        // Check before GC
        {
            // assert vertex exists
            assert!(
                !graph
                    .vertices
                    .get(&vid1)
                    .unwrap()
                    .chain
                    .current
                    .read()
                    .unwrap()
                    .data
                    .is_tombstone
            );
            // assert edge exists
            assert!(
                !graph
                    .edges
                    .get(&euid1.eid())
                    .unwrap()
                    .chain
                    .current
                    .read()
                    .unwrap()
                    .data
                    .is_tombstone
            );
            // assert adjacency list
            assert!(graph.adjacency_list.get(&vid1).unwrap().outgoing().len() == 2);
            assert!(graph.adjacency_list.get(&vid1).unwrap().incoming().len() == 1);
        }

        // Delete the vertex
        let txn = graph.begin_transaction(IsolationLevel::Serializable);
        graph.delete_vertex(&txn, vid1).unwrap();
        assert!(txn.commit().is_ok());

        // Start a new transaction to update the watermark
        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        assert!(txn2.commit().is_ok());

        // Check before GC
        {
            // assert vertex is tombstone
            assert!(
                graph
                    .vertices
                    .get(&vid1)
                    .unwrap()
                    .chain
                    .current
                    .read()
                    .unwrap()
                    .data
                    .is_tombstone
            );
            // assert edge is tombstone
            assert!(
                graph
                    .edges
                    .get(&euid1.eid())
                    .unwrap()
                    .chain
                    .current
                    .read()
                    .unwrap()
                    .data
                    .is_tombstone
            );
            // assert adjacency list
            assert!(graph.adjacency_list.get(&vid1).unwrap().outgoing().len() == 2);
            assert!(graph.adjacency_list.get(&vid1).unwrap().incoming().len() == 1);
            let iter = txn2.iter_adjacency(vid1);
            let mut count = 0;
            for _ in iter {
                count += 1;
            }
            assert!(count == 0);
        }

        let txn3 = graph.begin_transaction(IsolationLevel::Serializable);
        graph.txn_manager.garbage_collect(txn3.graph()).unwrap();
        // Check after GC
        {
            assert!(graph.vertices.get(&vid1).is_none());
            assert!(graph.edges.get(&euid1.eid()).is_none());
            assert!(graph.adjacency_list.get(&vid1).is_none());
        }
        let _ = txn3.abort();
    }

    #[test]
    fn test_delete_vertex_with_edges() {
        let (graph, _cleaner) = mock_graph();

        let vid: u64 = 1;

        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        {
            // Check visible and invisible edges
            let adj = graph.adjacency_list.get(&vid).unwrap();
            let mut count = 0;
            for euid in adj.incoming().iter() {
                let edge = graph.edges.get(&euid.value().eid()).unwrap();
                assert!(!edge.value().chain.current.read().unwrap().data.is_tombstone);
                count += 1;
            }
            for euid in adj.outgoing().iter() {
                let edge = graph.edges.get(&euid.value().eid()).unwrap();
                assert!(!edge.value().chain.current.read().unwrap().data.is_tombstone);
                count += 1;
            }
            assert!(count == 3);
            // Check visible edges
            let iter = txn1.iter_adjacency(vid);
            let mut count = 0;
            for _ in iter {
                count += 1;
            }
            assert!(count == 3);
        }
        graph.delete_vertex(&txn1, vid).unwrap();
        assert!(txn1.commit().is_ok());

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        {
            // Check visible and invisible edges
            let adj = graph.adjacency_list.get(&vid).unwrap();
            let mut count = 0;
            for euid in adj.incoming().iter() {
                let edge = graph.edges.get(&euid.value().eid()).unwrap();
                assert!(edge.value().chain.current.read().unwrap().data.is_tombstone);
                count += 1;
            }
            for euid in adj.outgoing().iter() {
                let edge = graph.edges.get(&euid.value().eid()).unwrap();
                assert!(edge.value().chain.current.read().unwrap().data.is_tombstone);
                count += 1;
            }
            assert!(count == 3);
            // Check visible edges
            let iter = txn2.iter_adjacency(vid);
            let mut count = 0;
            for _ in iter {
                count += 1;
            }
            assert!(count == 0);
        }
        let _ = txn2.abort();
    }

    #[test]
    fn test_delete_edge_with_vertex_conflict() {
        let (graph, _cleaner) = mock_graph();

        let vid: VertexId = 1;
        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        let _ = create_vertex_eve();
        let _ = graph.create_vertex(&txn2, create_vertex_eve()).unwrap();
        let _ = graph
            .create_edge(&txn2, create_edge_alice_to_eve())
            .unwrap();
        assert!(txn2.commit().is_ok());

        assert!(graph.delete_vertex(&txn1, vid).is_err());
        let _ = txn1.abort();
    }

    #[test]
    fn test_wal_replay() {
        // Creates a new graph
        let checkpoint_config = mock_checkpoint_config();
        let wal_config = mock_wal_config();
        let _cleaner = Cleaner::new(&checkpoint_config, &wal_config);
        let graph = MemoryGraph::with_config_fresh(checkpoint_config.clone(), wal_config.clone());

        // Create and commit a transaction with a vertex
        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_eve();
        let vid1 = graph.create_vertex(&txn1, v1.clone()).unwrap();
        assert!(txn1.commit().is_ok());

        // Create and commit a transaction with another vertex
        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        let v2 = create_vertex_frank();
        let vid2 = graph.create_vertex(&txn2, v2.clone()).unwrap();

        // Create an edge between the vertices
        let e1 = Edge::new(
            100,    // edge id
            vid1,   // from Eve
            vid2,   // to Frank
            FRIEND, // label
            PropertyRecord::new(vec![ScalarValue::String(Some("2023-01-01".to_string()))]),
        );
        let eid1 = graph.create_edge(&txn2, e1.clone()).unwrap();
        assert!(txn2.commit().is_ok());

        // Verify the graph state before recovery
        let txn_verify = graph.begin_transaction(IsolationLevel::Serializable);
        assert_eq!(graph.get_vertex(&txn_verify, vid1).unwrap(), v1);
        assert_eq!(graph.get_vertex(&txn_verify, vid2).unwrap(), v2);
        assert_eq!(graph.get_edge(&txn_verify, eid1).unwrap().src_id(), vid1);
        assert_eq!(graph.get_edge(&txn_verify, eid1).unwrap().dst_id(), vid2);
        txn_verify.abort().unwrap();

        // Create a new graph instance without recovery
        let new_graph =
            MemoryGraph::with_config_fresh(checkpoint_config.clone(), wal_config.clone());

        // Recover the graph from WAL
        assert!(new_graph.recover_from_wal().is_ok());

        // Verify the graph state after recovery
        let txn_after = new_graph.begin_transaction(IsolationLevel::Serializable);
        assert_eq!(new_graph.get_vertex(&txn_after, vid1).unwrap(), v1);
        assert_eq!(new_graph.get_vertex(&txn_after, vid2).unwrap(), v2);
        assert_eq!(new_graph.get_edge(&txn_after, eid1).unwrap().src_id(), vid1);
        assert_eq!(new_graph.get_edge(&txn_after, eid1).unwrap().dst_id(), vid2);
        txn_after.abort().unwrap();
    }

    #[test]
    fn test_checkpoint_and_wal_recovery() {
        // Creates a new graph
        let checkpoint_config = mock_checkpoint_config();
        let wal_config = mock_wal_config();
        let _cleaner = Cleaner::new(&checkpoint_config, &wal_config);
        let graph = MemoryGraph::with_config_fresh(checkpoint_config.clone(), wal_config.clone());

        // Create initial data (before checkpoint)
        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let vertex1 = Vertex::new(
            1,
            LabelId::new(1).unwrap(),
            PropertyRecord::new(vec![ScalarValue::String(Some(
                "Before Checkpoint".to_string(),
            ))]),
        );

        graph.create_vertex(&txn1, vertex1.clone()).unwrap();
        txn1.commit().unwrap();

        // Check the size of wal entries before checkpoint
        let entries = graph.wal_manager.wal().read().unwrap().read_all().unwrap();
        assert_eq!(entries.len(), 3); // txn1 begin, create vertex, commit

        // Create a checkpoint
        let _checkpoint_id = graph
            .create_managed_checkpoint(Some("Test checkpoint".to_string()))
            .unwrap();

        // Check the size of wal entries after checkpoint
        let entries = graph.wal_manager.wal().read().unwrap().read_all().unwrap();
        assert_eq!(entries.len(), 0); // Should be empty as we truncate the WAL

        // Create more data (after checkpoint)
        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        let vertex2 = Vertex::new(
            2,
            LabelId::new(1).unwrap(),
            PropertyRecord::new(vec![ScalarValue::String(Some(
                "After Checkpoint".to_string(),
            ))]),
        );
        graph.create_vertex(&txn2, vertex2.clone()).unwrap();
        txn2.commit().unwrap();

        // Check the size of wal entries before recovery
        let entries = graph.wal_manager.wal().read().unwrap().read_all().unwrap();
        assert_eq!(entries.len(), 3); // txn2 begin, create vertex, commit

        // Now recover a new graph from checkpoint and WAL
        let recovered_graph = MemoryGraph::with_config_recovered(checkpoint_config, wal_config);

        // Check the size of wal entries after recovery
        let entries = recovered_graph
            .wal_manager
            .wal()
            .read()
            .unwrap()
            .read_all()
            .unwrap();

        assert_eq!(entries.len(), 3); // Should be still 3, since we didn't truncate the WAL

        // Verify the recovered graph has both vertices
        let txn = recovered_graph.begin_transaction(IsolationLevel::Serializable);
        let recovered_vertex1 = recovered_graph.get_vertex(&txn, 1).unwrap();
        let recovered_vertex2 = recovered_graph.get_vertex(&txn, 2).unwrap();

        assert_eq!(recovered_vertex1.vid(), vertex1.vid());
        assert_eq!(
            recovered_vertex1.properties()[0],
            ScalarValue::String(Some("Before Checkpoint".to_string()))
        );

        assert_eq!(recovered_vertex2.vid(), vertex2.vid());
        assert_eq!(
            recovered_vertex2.properties()[0],
            ScalarValue::String(Some("After Checkpoint".to_string()))
        );
    }

    #[test]
    fn test_vector_index_build_and_verify() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Test 1: Build index with unsupported dimension should fail
        let unsupported_vectors = vec![
            // 200 dimensions, unsupported (not 104/128/256)
            (1u64, "test1".to_string(), vec![1.0f32; 200]),
            (2u64, "test2".to_string(), vec![2.0f32; 200]),
            (3u64, "test3".to_string(), vec![3.0f32; 200]),
        ];
        for (id, name, embedding) in &unsupported_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }
        // Try to build index with unsupported dimension - should fail
        let result =
            graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID));
        assert!(result.is_err());

        // Clean up unsupported test data
        for (id, _, _) in &unsupported_vectors {
            graph.delete_vertex(&txn, *id)?;
        }

        // Test 2: Build index with supported dimension should succeed
        // Create 200 test vertices with small-scale vectors
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        // Build vector index with small-scale configuration
        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        // Verify index creation and properties
        let index_key = VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID);
        let index = graph
            .get_vector_index(index_key)
            .expect("Index should exist after build");
        assert_eq!(index.size(), 200);
        assert_eq!(index.get_dimension(), TEST_DIMENSION);

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vector_search_accuracy() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Create small-scale test dataset
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        // Build index with small-scale configuration
        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        // Test 1: Search in cluster 1 area (coordinates around 30-42)
        let mut cluster1_query = vec![0.0f32; TEST_DIMENSION];
        cluster1_query[0] = 35.0f32;
        cluster1_query[1] = 30.0f32;
        cluster1_query[2] = 25.0f32;
        let results = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &cluster1_query,
            10,
            50,
            None,
            false,
        )?;
        assert!(!results.is_empty(), "Should find vectors in cluster 1");
        assert!(results.len() <= 10, "Results should not exceed k");

        // Test 2: Search in cluster 2 area (coordinates around 50-62)
        let mut cluster2_query = vec![0.0f32; TEST_DIMENSION];
        cluster2_query[0] = 55.0f32;
        cluster2_query[1] = 45.0f32;
        cluster2_query[2] = 37.0f32;
        let results = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &cluster2_query,
            5,
            30,
            None,
            false,
        )?;
        assert!(!results.is_empty(), "Should find vectors in cluster 2");
        assert!(results.len() <= 5, "Results should not exceed k");

        // Test 3: Invalid dimension (too small) - should fail
        let invalid_query_small = vec![1.0f32; TEST_DIMENSION - 1];
        let result = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &invalid_query_small,
            1,
            20,
            None,
            false,
        );
        assert!(result.is_err(), "Invalid dimension query should fail");
        match result.unwrap_err() {
            StorageError::VectorIndex(VectorIndexError::InvalidDimension { expected, actual }) => {
                assert_eq!(expected, TEST_DIMENSION);
                assert_eq!(actual, TEST_DIMENSION - 1);
            }
            _ => panic!("Expected InvalidDimension error"),
        }

        // Test 4: Invalid dimension (too large) - should fail
        let invalid_query_large = vec![1.0f32; TEST_DIMENSION + 10];
        let result = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &invalid_query_large,
            1,
            20,
            None,
            false,
        );
        assert!(result.is_err(), "Invalid dimension query should fail");
        match result.unwrap_err() {
            StorageError::VectorIndex(VectorIndexError::InvalidDimension { expected, actual }) => {
                assert_eq!(expected, TEST_DIMENSION);
                assert_eq!(actual, TEST_DIMENSION + 10);
            }
            _ => panic!("Expected InvalidDimension error"),
        }

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vector_error_index_not_found() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Try to search without building index
        let query = vec![1.0f32; TEST_DIMENSION];
        let result = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &query,
            1,
            20,
            None,
            false,
        );

        // Should fail with IndexNotFound error
        assert!(result.is_err());

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vector_error_empty_dataset() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Try to build index on empty dataset
        let result =
            graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID));

        // Should fail with appropriate error
        assert!(result.is_err());

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vector_error_dimension_mismatch() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Create index with valid small-scale vectors
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        // Try to search with wrong dimension query
        let wrong_dim_query = vec![0.0f32; 50]; // Wrong dimension
        let result = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &wrong_dim_query,
            1,
            50,
            None,
            false,
        );

        // Should fail due to dimension mismatch
        assert!(result.is_err());

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vertex_id_mapping_correctness() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Create small-scale vertices with specific IDs to test mapping
        let mut test_vectors = create_small_scale_test_vectors();
        // Replace some IDs with specific values for testing
        test_vectors[0].0 = 10u64;
        test_vectors[1].0 = 42u64;
        test_vectors[2].0 = 100u64;
        test_vectors[3].0 = 999u64;
        test_vectors[4].0 = 50000u64;

        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        // Search should return correct vertex IDs for modified vectors
        for (expected_id, _, embedding) in test_vectors.iter().take(5) {
            let results = graph.vector_search(
                VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
                embedding,
                1,
                50,
                None,
                false,
            )?;
            assert_eq!(results.len(), 1);
            assert_eq!(
                results[0], *expected_id,
                "ID mapping failed for vertex {}",
                expected_id
            );
        }

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vector_small_scale_dataset() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Use the standard small-scale dataset (200 points)
        let test_vectors = create_small_scale_test_vectors();

        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        // Build index with small-scale configuration
        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        // Verify index properties
        let index = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap();
        assert_eq!(index.size(), 200);

        // Test search with various k values
        let mut query = vec![0.0f32; TEST_DIMENSION];
        query[0] = 75.0f32; // Search in middle area
        query[1] = 60.0f32;
        query[2] = 45.0f32;
        let results = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &query,
            15,
            50,
            None,
            false,
        )?;
        assert!(!results.is_empty());
        assert!(results.len() <= 15);

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vector_transaction_isolation() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();

        // Transaction 1: Build index with small-scale data
        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn1, vertex)?;
        }

        graph.build_vector_index(&txn1, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;
        txn1.commit()?;

        // Transaction 2: Use index with different isolation levels
        for &isolation in &[IsolationLevel::Snapshot, IsolationLevel::Serializable] {
            let txn2 = graph.begin_transaction(isolation);
            let mut query = vec![0.0f32; TEST_DIMENSION];
            query[0] = 65.0f32; // Search in cluster area
            query[1] = 55.0f32;
            query[2] = 40.0f32;
            let results = graph.vector_search(
                VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
                &query,
                5,
                30,
                None,
                false,
            )?;
            assert!(!results.is_empty());
            txn2.commit()?;
        }

        Ok(())
    }

    #[test]
    fn test_vector_multiple_indices_per_graph() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Create vertices with vectors on different properties using small-scale data
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            // Create property with different embeddings for property 1 and 2
            let embedding_1 = embedding.clone();
            let mut embedding_2 = embedding.clone();
            embedding_2[0] += 15.0; // Larger variation for large coordinates
            embedding_2[1] += 10.0;

            let vertex = Vertex::new(
                *id,
                PERSON,
                PropertyRecord::new(vec![
                    ScalarValue::String(Some(name.clone())),
                    ScalarValue::Vector(Some(embedding_1.into_iter().map(F32::from).collect())),
                    ScalarValue::Vector(Some(embedding_2.into_iter().map(F32::from).collect())),
                ]),
            );
            graph.create_vertex(&txn, vertex)?;
        }

        // Build indices on different properties
        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, 1))?; // Property 1
        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, 2))?; // Property 2

        // Verify both indices work independently
        let mut query = vec![0.0f32; TEST_DIMENSION];
        query[0] = 80.0f32; // Query in large coordinate space
        query[1] = 70.0f32;
        query[2] = 50.0f32;
        let results_1 =
            graph.vector_search(VectorIndexKey::new(PERSON, 1), &query, 3, 30, None, false)?;
        let results_2 =
            graph.vector_search(VectorIndexKey::new(PERSON, 2), &query, 3, 30, None, false)?;

        assert!(!results_1.is_empty());
        assert!(!results_2.is_empty());

        txn.commit()?;
        Ok(())
    }

    /// Creates additional test vectors for insert operations
    fn create_additional_test_vectors(
        start_id: VertexId,
        count: usize,
    ) -> Vec<(VertexId, String, Vec<f32>)> {
        (0..count)
            .map(|i| {
                let id = start_id + i as u64;
                let name = format!("additional_vertex_{}", id);

                // Create vectors in a new cluster area to avoid conflicts with existing test data
                let mut vector = vec![0.0f32; TEST_DIMENSION];
                vector[0] = 200.0 + (i as f32) * 2.0; // New cluster starting at x=200
                vector[1] = 180.0 + (i as f32) * 1.5; // New cluster starting at y=180
                vector[2] = 160.0 + (i as f32) * 1.8; // New cluster starting at z=160

                // Add some variation to other dimensions
                let start = 3;
                let end = std::cmp::min(10, TEST_DIMENSION);
                for (j, item) in vector.iter_mut().enumerate().skip(start).take(end - start) {
                    *item = (id as f32) * 0.1 + (j as f32) * 0.3 + 10.0;
                }

                (id, name, vector)
            })
            .collect()
    }

    /// Verify that a specific vector can be found in search results
    fn verify_vector_in_search_results(
        graph: &MemoryGraph,
        property_id: PropertyId,
        target_vector: &[f32],
        expected_node_id: VertexId,
    ) -> StorageResult<bool> {
        let results = graph.vector_search(
            VectorIndexKey::new(PERSON, property_id),
            target_vector,
            5,
            50,
            None,
            false,
        )?;
        Ok(results.contains(&expected_node_id))
    }

    /// Verify that a specific vector cannot be found in search results
    fn verify_vector_not_in_search_results(
        graph: &MemoryGraph,
        property_id: PropertyId,
        query_vector: &[f32],
        excluded_node_id: VertexId,
    ) -> StorageResult<bool> {
        let results = graph.vector_search(
            VectorIndexKey::new(PERSON, property_id),
            query_vector,
            20,
            100,
            None,
            false,
        )?; // Use larger k to be thorough
        Ok(!results.contains(&excluded_node_id))
    }

    #[test]
    fn test_vector_insert_basic() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Create initial dataset and build index
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        // Verify initial index size
        let initial_size = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();
        assert_eq!(initial_size, 200);

        // Test 1: Insert a single new vector
        let new_vectors = create_additional_test_vectors(1000, 1);
        let (new_id, new_name, new_embedding) = &new_vectors[0];

        // Create the vertex in the graph first
        let new_vertex = create_vertex_with_vector(*new_id, new_name, new_embedding.clone());
        graph.create_vertex(&txn, new_vertex)?;

        // Insert into vector index
        let insert_data = vec![(*new_id, new_embedding.clone())];
        graph.insert_into_vector_index(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &insert_data,
        )?;

        // Verify index size increased: 200 + 1 = 201
        let new_size = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();
        assert_eq!(new_size, initial_size + 1);

        // Verify the inserted vector can be found
        assert!(verify_vector_in_search_results(
            &graph,
            EMBEDDING_PROPERTY_ID,
            new_embedding,
            *new_id
        )?);

        // Test 2:  dimension mismatch - should fail
        let wrong_dimension_vector = vec![1.0f32; 100]; // 100 dimensions vs expected 104
        let wrong_data = vec![(2000u64, wrong_dimension_vector.clone())];

        // Create vertex in graph first (graph layer doesn't check dimensions)
        let wrong_vertex = create_vertex_with_vector(2000, "wrong_dim", wrong_dimension_vector);
        graph.create_vertex(&txn, wrong_vertex)?;

        // Try to insert wrong dimension vector - should fail at insert_into_vector_index level
        let result = graph.insert_into_vector_index(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &wrong_data,
        );

        assert!(result.is_err());
        match result.unwrap_err() {
            StorageError::VectorIndex(VectorIndexError::InvalidDimension { expected, actual }) => {
                assert_eq!(expected, TEST_DIMENSION);
                assert_eq!(actual, 100);
            }
            _ => panic!("Expected InvalidDimension error"),
        }

        // Verify index size unchanged after failed insertion
        let final_size = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();
        assert_eq!(final_size, new_size); // Should remain same as before failed insertion

        // Test 3: capacity limit - should fail when exceeding max_points
        // Current: 200 original + 1 successful = 201 vectors, capacity: 240, remaining: 39
        // Try to insert 50 vectors: 201 + 50 = 251 > 240 → should fail
        let excess_vectors = create_additional_test_vectors(3000, 50); // Create 50 additional vectors
        let mut excess_insert_data = Vec::new();

        // Create vertices in graph first
        for (id, name, embedding) in &excess_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
            excess_insert_data.push((*id, embedding.clone()));
        }

        // Try to insert 50 vectors when capacity allows only 39 more - should fail
        let capacity_result = graph.insert_into_vector_index(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &excess_insert_data,
        );

        assert!(capacity_result.is_err());
        match capacity_result.unwrap_err() {
            StorageError::VectorIndex(VectorIndexError::CapacityExceeded {
                current,
                max_capacity,
            }) => {
                assert_eq!(current, 251); // 201 existing + 50 attempted
                assert_eq!(max_capacity, 240); // 200 * 1.2
            }
            _ => panic!("Expected CapacityExceeded error"),
        }

        // Verify index size unchanged after failed capacity insertion
        let size_after_capacity_failure = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();
        assert_eq!(size_after_capacity_failure, new_size); // Should remain 201 (200 original + 1 successful insert)

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vector_insert_multiple() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Create initial dataset
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        let initial_size = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();

        // Insert multiple vectors
        let new_vectors = create_additional_test_vectors(2000, 5);
        let mut insert_data = Vec::new();

        for (id, name, embedding) in &new_vectors {
            // Create vertices first
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
            insert_data.push((*id, embedding.clone()));
        }

        // Batch insert
        graph.insert_into_vector_index(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &insert_data,
        )?;

        // Verify index size
        let new_size = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();
        assert_eq!(new_size, initial_size + 5);

        // Verify all inserted vectors can be found
        for (id, _, embedding) in &new_vectors {
            assert!(verify_vector_in_search_results(
                &graph,
                EMBEDDING_PROPERTY_ID,
                embedding,
                *id
            )?);
        }

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vector_insert_empty_list() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Create initial dataset
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        let initial_size = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();

        // Insert empty vector list - should succeed but do nothing
        let empty_vectors: Vec<(u64, Vec<f32>)> = vec![];
        let result = graph.insert_into_vector_index(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &empty_vectors,
        );
        assert!(result.is_ok());

        // Verify size unchanged
        let new_size = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();
        assert_eq!(new_size, initial_size);

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vector_insert_index_not_found() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Don't build any index
        let new_vectors = create_additional_test_vectors(3000, 1);
        let insert_data = vec![(new_vectors[0].0, new_vectors[0].2.clone())];

        // Should fail with index not found error
        let result = graph.insert_into_vector_index(VectorIndexKey::new(PERSON, 999), &insert_data);
        assert!(result.is_err());

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vector_delete_basic() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Create initial dataset
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        let initial_size = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();

        // Select a vector to delete (use first vector from test data)
        let (target_id, _, target_embedding) = &test_vectors[0];

        // Verify vector can be found before deletion
        assert!(verify_vector_in_search_results(
            &graph,
            EMBEDDING_PROPERTY_ID,
            target_embedding,
            *target_id
        )?);

        // Delete the vector
        graph.delete_from_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID), &[
            *target_id,
        ])?;

        // Verify index size decreased (soft delete should reduce active count)
        let new_size = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();
        assert_eq!(new_size, initial_size - 1);

        // Verify deleted vector is not found in search results
        assert!(verify_vector_not_in_search_results(
            &graph,
            EMBEDDING_PROPERTY_ID,
            target_embedding,
            *target_id
        )?);

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vector_delete_multiple() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Create initial dataset
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        let initial_size = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();

        // Select multiple vectors to delete (first 3 vectors)
        let delete_ids: Vec<u64> = test_vectors.iter().take(3).map(|(id, _, _)| *id).collect();
        let delete_embeddings: Vec<&Vec<f32>> =
            test_vectors.iter().take(3).map(|(_, _, emb)| emb).collect();

        // Verify vectors can be found before deletion
        for (i, &id) in delete_ids.iter().enumerate() {
            assert!(verify_vector_in_search_results(
                &graph,
                EMBEDDING_PROPERTY_ID,
                delete_embeddings[i],
                id
            )?);
        }

        // Delete multiple vectors
        graph.delete_from_vector_index(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &delete_ids,
        )?;

        // Verify index size decreased
        let new_size = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();
        assert_eq!(new_size, initial_size - 3);

        // Verify deleted vectors are not found in search results
        for (i, &id) in delete_ids.iter().enumerate() {
            assert!(verify_vector_not_in_search_results(
                &graph,
                EMBEDDING_PROPERTY_ID,
                delete_embeddings[i],
                id
            )?);
        }

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vector_delete_empty_list() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Create initial dataset
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        let initial_size = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();

        // Delete empty list - should succeed but do nothing
        let empty_ids: Vec<u64> = vec![];
        let result = graph.delete_from_vector_index(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &empty_ids,
        );
        assert!(result.is_ok());

        // Verify size unchanged
        let new_size = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();
        assert_eq!(new_size, initial_size);

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vector_delete_index_not_found() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Don't build any index
        let delete_ids = vec![1u64, 2u64];

        // Should fail with index not found error
        let result = graph.delete_from_vector_index(VectorIndexKey::new(PERSON, 999), &delete_ids);
        assert!(result.is_err());

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vector_delete_nonexistent_node() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Create initial dataset
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        // Try to delete non-existent node ID
        let nonexistent_ids = vec![9999u64];
        let result = graph.delete_from_vector_index(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &nonexistent_ids,
        );

        // Should fail with appropriate error
        assert!(result.is_err());

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vector_insert_delete_combined() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Create initial dataset
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        let initial_size = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();

        // Phase 1: Insert new vectors
        let new_vectors = create_additional_test_vectors(4000, 3);
        let mut insert_data = Vec::new();

        for (id, name, embedding) in &new_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
            insert_data.push((*id, embedding.clone()));
        }

        graph.insert_into_vector_index(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &insert_data,
        )?;

        // Verify size after insertion
        let after_insert_size = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();
        assert_eq!(after_insert_size, initial_size + 3);

        // Phase 2: Delete some original vectors
        let delete_ids: Vec<u64> = test_vectors.iter().take(2).map(|(id, _, _)| *id).collect();
        graph.delete_from_vector_index(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &delete_ids,
        )?;

        // Verify final size
        let final_size = graph
            .get_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))
            .unwrap()
            .size();
        assert_eq!(final_size, initial_size + 3 - 2); // +3 inserts, -2 deletes

        // Verify inserted vectors are still findable
        for (id, _, embedding) in &new_vectors {
            assert!(verify_vector_in_search_results(
                &graph,
                EMBEDDING_PROPERTY_ID,
                embedding,
                *id
            )?);
        }

        // Verify deleted vectors are not findable
        for &id in &delete_ids {
            let deleted_embedding = &test_vectors
                .iter()
                .find(|(vid, _, _)| *vid == id)
                .unwrap()
                .2;
            assert!(verify_vector_not_in_search_results(
                &graph,
                EMBEDDING_PROPERTY_ID,
                deleted_embedding,
                id
            )?);
        }

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_vector_operations_mixed() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Create initial dataset
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        // Mixed operations: insert, search, delete, search again

        // 1. Insert new vector
        let new_vectors = create_additional_test_vectors(5000, 1);
        let (new_id, new_name, new_embedding) = &new_vectors[0];
        let vertex = create_vertex_with_vector(*new_id, new_name, new_embedding.clone());
        graph.create_vertex(&txn, vertex)?;
        graph.insert_into_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID), &[(
            *new_id,
            new_embedding.clone(),
        )])?;

        // 2. Search for inserted vector
        let search_results = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            new_embedding,
            5,
            50,
            None,
            false,
        )?;
        assert!(search_results.contains(new_id));

        // 3. Delete the inserted vector
        graph.delete_from_vector_index(VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID), &[
            *new_id,
        ])?;

        // 4. Search again - should not find deleted vector
        assert!(verify_vector_not_in_search_results(
            &graph,
            EMBEDDING_PROPERTY_ID,
            new_embedding,
            *new_id
        )?);

        // 5. Verify original vectors are still accessible
        let original_embedding = &test_vectors[10].2;
        let original_id = test_vectors[10].0;
        assert!(verify_vector_in_search_results(
            &graph,
            EMBEDDING_PROPERTY_ID,
            original_embedding,
            original_id
        )?);

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_adaptive_filter_brute_force_search() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Use existing create_small_scale_test_vectors (200 vectors with non-consecutive IDs)
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        // Create BooleanArray filter with low selectivity (5% = ~10 out of 200) to trigger brute
        // force Need to create bitmap that maps to actual node IDs, not array indices
        let max_node_id = test_vectors.iter().map(|(id, _, _)| *id).max().unwrap_or(0);
        let mut filter_bits = vec![false; (max_node_id + 1) as usize];

        // Select every 20th test vector for filtering
        let selected_test_vectors: Vec<_> = test_vectors.iter().step_by(20).collect();
        for (node_id, _, _) in &selected_test_vectors {
            if (*node_id as usize) < filter_bits.len() {
                filter_bits[*node_id as usize] = true;
            }
        }
        let filter_bitmap = arrow::array::BooleanArray::from(filter_bits);

        // Perform brute force search
        let query = &test_vectors[0].2; // Use first vector as query
        let results = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            query,
            5,
            50,
            Some(&filter_bitmap),
            false,
        )?;

        // Verify results
        assert!(!results.is_empty(), "Should find filtered results");
        assert!(results.len() == 5, "Results should be k");

        // Verify all returned IDs should be from the selected set
        let selected_ids: Vec<u64> = selected_test_vectors.iter().map(|(id, _, _)| *id).collect();

        for result_id in &results {
            assert!(
                selected_ids.contains(result_id),
                "Result ID should be in filtered set"
            );
        }

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_adaptive_filter_post_filter_search() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Use existing test vectors
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        // Create BooleanArray filter with high selectivity (50% = ~100 out of 200) to trigger
        // post-filter
        let max_node_id = test_vectors.iter().map(|(id, _, _)| *id).max().unwrap_or(0);
        let mut filter_bits = vec![false; (max_node_id + 1) as usize];

        // Select most test vectors (exclude every 2rd to get ~50% selectivity)
        let selected_test_vectors: Vec<_> = test_vectors
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 2 != 0) // Exclude every 2nd element
            .map(|(_, vector_data)| vector_data)
            .collect();

        for (node_id, _, _) in &selected_test_vectors {
            if (*node_id as usize) < filter_bits.len() {
                filter_bits[*node_id as usize] = true;
            }
        }
        let filter_bitmap = arrow::array::BooleanArray::from(filter_bits);

        // Perform filtered search
        let query = &test_vectors[49].2; // Use middle vector as query
        let results = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            query,
            10,
            100,
            Some(&filter_bitmap),
            false,
        )?;

        // Verify results
        assert!(!results.is_empty(), "Should find filtered results");
        assert!(results.len() <= 10, "Results should not exceed k");

        // Verify all returned IDs should be from the filtered set
        let selected_ids: Vec<u64> = selected_test_vectors.iter().map(|(id, _, _)| *id).collect();

        for result_id in &results {
            assert!(
                selected_ids.contains(result_id),
                "Result ID should be in filtered set"
            );
        }

        // pre-filter search should return the same result as query when k is one
        let result_k1 = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            query,
            1,
            100,
            Some(&filter_bitmap),
            true,
        )?;
        assert_eq!(
            result_k1[0], test_vectors[49].0,
            "result_k1 vid should be same as query vid"
        );

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_adaptive_filter_pre_filter_search() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Use existing test vectors
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        // Create BooleanArray filter with high selectivity (50% = 100 out of 200) to trigger
        // pre-filter
        let max_node_id = test_vectors.iter().map(|(id, _, _)| *id).max().unwrap_or(0);
        let mut filter_bits = vec![false; (max_node_id + 1) as usize];

        // Select most test vectors (exclude every 2rd to get 50% selectivity)
        let selected_test_vectors: Vec<_> = test_vectors
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 2 != 0) // Exclude every 2nd element
            .map(|(_, vector_data)| vector_data)
            .collect();

        for (node_id, _, _) in &selected_test_vectors {
            if (*node_id as usize) < filter_bits.len() {
                filter_bits[*node_id as usize] = true;
            }
        }
        let filter_bitmap = arrow::array::BooleanArray::from(filter_bits);

        // Perform filtered search
        let query = &test_vectors[49].2; // Use middle vector as query
        let results = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            query,
            5,
            100,
            Some(&filter_bitmap),
            true,
        )?;

        // Verify results
        assert!(!results.is_empty(), "Should find filtered results");
        assert!(results.len() == 5, "Results should be k");

        // Verify all returned IDs should be from the filtered set
        let selected_ids: Vec<u64> = selected_test_vectors.iter().map(|(id, _, _)| *id).collect();

        for result_id in &results {
            assert!(
                selected_ids.contains(result_id),
                "Result ID should be in filtered set"
            );
        }

        // pre-filter search should return the same result as query when k is one
        let result_k1 = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            query,
            1,
            100,
            Some(&filter_bitmap),
            true,
        )?;
        assert_eq!(
            result_k1[0], test_vectors[49].0,
            "result_k1 vid should be same as query vid"
        );

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_filter_search_boundary_cases() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Use existing test vectors
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }

        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;
        let query = &test_vectors[0].2;

        // Test 1: Empty filter (all false)
        let max_node_id = test_vectors.iter().map(|(id, _, _)| *id).max().unwrap_or(0);
        let empty_filter =
            arrow::array::BooleanArray::from(vec![false; (max_node_id + 1) as usize]);
        let results = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            query,
            5,
            50,
            Some(&empty_filter),
            false,
        )?;
        assert!(
            results.is_empty(),
            "Empty filter should return empty results"
        );

        // Test 2: Single element filter
        let mut single_filter_bits = vec![false; (max_node_id + 1) as usize];
        let single_node_id = test_vectors[10].0; // Use actual node ID
        single_filter_bits[single_node_id as usize] = true;
        let single_filter = arrow::array::BooleanArray::from(single_filter_bits);
        let results = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            query,
            5,
            50,
            Some(&single_filter),
            false,
        )?;
        assert!(
            results.len() <= 1,
            "Single element filter should return at most 1 result"
        );

        // Test 3: Full filter (all true) - should work like no filter
        let mut full_filter_bits = vec![false; (max_node_id + 1) as usize];
        for (node_id, _, _) in &test_vectors {
            full_filter_bits[*node_id as usize] = true;
        }
        let full_filter = arrow::array::BooleanArray::from(full_filter_bits);
        let results_filtered = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            query,
            5,
            50,
            Some(&full_filter),
            true,
        )?; // pre-filter
        let results_unfiltered = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            query,
            5,
            50,
            None,
            false,
        )?;
        assert_eq!(
            results_filtered.len(),
            results_unfiltered.len(),
            "Full filter should match unfiltered results"
        );

        txn.commit()?;
        Ok(())
    }

    #[test]
    fn test_pre_filter_search_in_cluster() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Use existing test vectors with known clustering
        let test_vectors = create_small_scale_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }
        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        // Create a filter that selects only the first cluster (first 25 vectors, pre-filter search)
        let max_node_id = test_vectors.iter().map(|(id, _, _)| *id).max().unwrap_or(0);
        let mut cluster_filter_bits = vec![false; (max_node_id + 1) as usize];
        // test_vectors: Vec<(VertexId, String, Vec<f32>)>
        for &(node_id, _, _) in test_vectors.iter().take(25) {
            cluster_filter_bits[node_id as usize] = true;
        }
        let cluster_filter = arrow::array::BooleanArray::from(cluster_filter_bits);

        // Pre-filter Search within first cluster using a query from that cluster
        let cluster_query = &test_vectors[10].2; // 10th vector is in first cluster
        let results = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            cluster_query,
            5,
            50,
            Some(&cluster_filter),
            true,
        )?;

        // Verify results are within the first cluster
        let first_cluster_ids: Vec<u64> =
            test_vectors[0..25].iter().map(|(id, _, _)| *id).collect();
        for result_id in &results {
            assert!(
                first_cluster_ids.contains(result_id),
                "Result should be from first cluster"
            );
        }

        // Results should be sorted by similarity (closest first)
        assert!(!results.is_empty(), "Should find results in cluster");
        assert!(results.len() == 5, "Should be k");

        // pre-filter search should return the same result as query when k is one
        let result_k1 = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            cluster_query,
            1,
            50,
            Some(&cluster_filter),
            true,
        )?;
        assert_eq!(
            result_k1[0], test_vectors[10].0,
            "result_k1 vid should be same as query vid"
        );

        txn.commit()?;
        Ok(())
    }

    /// Create predictable test vectors with known distance relationships for accuracy testing
    fn create_predictable_test_vectors() -> Vec<(VertexId, String, Vec<f32>)> {
        let mut vectors = Vec::new();

        // Query vector will be [1.0, 0.0, 0.0, 0.0, ...] (first dimension = 1.0, rest = 0.0)
        // Create test vectors with predictable L2 squared distances:
        // Vector 0: Exact match - distance² = 0.0
        let mut vec0 = vec![0.0f32; TEST_DIMENSION];
        vec0[0] = 1.0;
        vectors.push((100u64, "exact_match".to_string(), vec0));
        // Vector 1: Very close - distance² = 0.01
        let mut vec1 = vec![0.0f32; TEST_DIMENSION];
        vec1[0] = 0.9; // (1.0 - 0.9)² = 0.01
        vectors.push((101u64, "very_close".to_string(), vec1));
        // Vector 2: Close - distance² = 0.04
        let mut vec2 = vec![0.0f32; TEST_DIMENSION];
        vec2[0] = 0.8; // (1.0 - 0.8)² = 0.04
        vectors.push((102u64, "close".to_string(), vec2));
        // Vector 3: Medium distance - distance² = 1.0
        let vec3 = vec![0.0f32; TEST_DIMENSION];
        // Zero vector: (1.0)² + 0² + ... = 1.0
        vectors.push((103u64, "medium".to_string(), vec3));
        // Vector 4: Far - distance² = 2.0
        let mut vec4 = vec![0.0f32; TEST_DIMENSION];
        vec4[1] = 1.0; // 1² + 1² + 0² + ... = 2.0
        vectors.push((104u64, "far".to_string(), vec4));
        // Vector 5: Very far
        let mut vec5 = vec![0.0f32; TEST_DIMENSION];
        vec5[0] = -1.0;
        vec5[1] = 1.0;
        vec5[2] = 1.0;
        vectors.push((105u64, "very_far".to_string(), vec5));

        vectors
    }

    #[test]
    fn test_brute_force_search_accuracy() -> StorageResult<()> {
        let (graph, _cleaner) = mock_empty_graph();
        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        // Create predictable test vectors with known distance relationships
        let test_vectors = create_predictable_test_vectors();
        for (id, name, embedding) in &test_vectors {
            let vertex = create_vertex_with_vector(*id, name, embedding.clone());
            graph.create_vertex(&txn, vertex)?;
        }
        graph.build_vector_index(&txn, VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID))?;

        // Query vector: [1.0, 0.0, 0.0, ...]
        let mut query = vec![0.0f32; TEST_DIMENSION];
        query[0] = 1.0;
        // Test with filter (only include nodes 102, 103, 104)
        let max_node_id = test_vectors.iter().map(|(id, _, _)| *id).max().unwrap_or(0);
        let mut filter_bits = vec![false; (max_node_id + 1) as usize];
        filter_bits[102] = true; // close
        filter_bits[103] = true; // medium  
        filter_bits[104] = true; // far
        let filter = arrow::array::BooleanArray::from(filter_bits);
        let filtered_results = graph.vector_search(
            VectorIndexKey::new(PERSON, EMBEDDING_PROPERTY_ID),
            &query,
            2,
            50,
            Some(&filter),
            false,
        )?;
        assert_eq!(
            filtered_results.len(),
            2,
            "Should return 2 filtered results"
        );
        assert_eq!(
            filtered_results[0], 102,
            "First filtered result should be close (node_102)"
        );
        assert_eq!(
            filtered_results[1], 103,
            "Second filtered result should be medium (node_103)"
        );

        txn.commit()?;
        Ok(())
    }
}
