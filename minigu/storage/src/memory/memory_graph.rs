use std::sync::{Arc, RwLock, Weak};

use crossbeam_skiplist::SkipSet;
use dashmap::DashMap;
use minigu_common::datatype::types::{EdgeId, VertexId};
use minigu_common::datatype::value::PropertyValue;

use super::checkpoint::{CheckpointManager, CheckpointManagerConfig};
use super::transaction::{MemTransaction, MemTxnManager};
use crate::error::{
    EdgeNotFoundError, StorageError, StorageResult, TransactionError, VertexNotFoundError,
};
use crate::memory::adjacency_iterator::AdjacencyIterator;
use crate::memory::edge_iterator::EdgeIterator;
use crate::memory::vertex_iterator::VertexIterator;
use crate::model::edge::{Edge, Neighbor};
use crate::model::vertex::Vertex;
use crate::storage::{Graph, MutGraph};
use crate::transaction::{DeltaOp, IsolationLevel, SetPropsOp, Timestamp, UndoEntry, UndoPtr};
use crate::wal::StorageWal;
use crate::wal::graph_wal::{Operation, RedoEntry, WalManager, WalManagerConfig};

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
}

#[allow(dead_code)]
// Basic methods for MemoryGraph
impl MemoryGraph {
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
        for entry in entries {
            self.wal_manager.set_next_lsn(entry.lsn + 1);
            match entry.op {
                Operation::BeginTransaction(start_ts) => {
                    // Create a new transaction
                    let txn = self.begin_transaction_at(
                        Some(entry.txn_id),
                        Some(start_ts),
                        entry.iso_level,
                        true,
                    );
                    assert_eq!(txn.start_ts(), start_ts);
                }
                Operation::CommitTransaction(commit_ts) => {
                    // Commit the transaction
                    let txn = self.txn_manager.active_txns.get(&entry.txn_id).unwrap();
                    txn.value().commit_at(Some(commit_ts), true)?;
                }
                Operation::AbortTransaction => {
                    // Abort the transaction
                    let txn = self.txn_manager.active_txns.get(&entry.txn_id).unwrap();
                    txn.value().abort_at(true)?;
                }
                Operation::Delta(delta) => {
                    // Apply the delta
                    let txn = self.txn_manager.active_txns.get(&entry.txn_id).unwrap();
                    match delta {
                        DeltaOp::CreateVertex(vertex) => {
                            self.create_vertex(txn.value(), vertex)?;
                        }
                        DeltaOp::CreateEdge(edge) => {
                            self.create_edge(txn.value(), edge)?;
                        }
                        DeltaOp::DelVertex(vid) => {
                            self.delete_vertex(txn.value(), vid)?;
                        }
                        DeltaOp::DelEdge(eid) => {
                            self.delete_edge(txn.value(), eid)?;
                        }
                        DeltaOp::SetVertexProps(vid, SetPropsOp { indices, props }) => {
                            self.set_vertex_property(txn.value(), vid, indices, props)?;
                        }
                        DeltaOp::SetEdgeProps(eid, SetPropsOp { indices, props }) => {
                            self.set_edge_property(txn.value(), eid, indices, props)?;
                        }
                        DeltaOp::AddLabel(_) => todo!(),
                        DeltaOp::RemoveLabel(_) => todo!(),
                    }
                }
            }
        }
        Ok(())
    }

    /// Begins a new transaction and returns a `MemTransaction` instance.
    pub fn begin_transaction(
        self: &Arc<Self>,
        isolation_level: IsolationLevel,
    ) -> Arc<MemTransaction> {
        self.begin_transaction_at(None, None, isolation_level, false)
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
}

// Immutable graph methods
impl Graph for MemoryGraph {
    type Adjacency = Neighbor;
    type AdjacencyIter<'a> = AdjacencyIterator<'a>;
    type Edge = Edge;
    type EdgeID = EdgeId;
    type EdgeIter<'a> = EdgeIterator<'a>;
    type Transaction = MemTransaction;
    type Vertex = Vertex;
    type VertexID = VertexId;
    type VertexIter<'a> = VertexIterator<'a>;

    /// Retrieves a vertex by its ID within the context of a transaction.
    fn get_vertex(&self, txn: &MemTransaction, vid: VertexId) -> StorageResult<Vertex> {
        // Step 1: Atomically retrieve the versioned vertex (check existence).
        let versioned_vertex = self.vertices.get(&vid).ok_or(StorageError::VertexNotFound(
            VertexNotFoundError::VertexNotFound(vid.to_string()),
        ))?;

        // Step 2: Perform MVCC visibility check.
        let current_version = versioned_vertex.chain.current.read().unwrap();
        let commit_ts = current_version.commit_ts;
        match txn.isolation_level() {
            IsolationLevel::Serializable => {
                // Check if the vertex is modified by other transactions
                if commit_ts.is_txn_id() && commit_ts != txn.txn_id() {
                    return Err(StorageError::Transaction(
                        TransactionError::WriteReadConflict(format!(
                            "Vertex is being modified by transaction {:?}",
                            commit_ts
                        )),
                    ));
                }
                txn.vertex_reads.insert(vid);
            }
            IsolationLevel::Snapshot => {
                // Optimistic read allowed, no read set recording
            }
        }
        // The vertex is visible, which means it is either modified by txn or nobody
        let mut visible_vertex = current_version.data.clone();
        // Only when the vertex is modified by nobody and txn started before the vertex was
        // modified, we need to apply the deltas to the vertex
        if commit_ts.is_commit_ts() && commit_ts > txn.start_ts() {
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
    fn get_edge(&self, txn: &MemTransaction, eid: EdgeId) -> StorageResult<Edge> {
        // Step 1: Atomically retrieve the versioned edge (check existence).
        let versioned_edge = self.edges.get(&eid).ok_or(StorageError::EdgeNotFound(
            EdgeNotFoundError::EdgeNotFound(eid.to_string()),
        ))?;

        // Step 2: Perform MVCC visibility check.
        let current_version = versioned_edge.chain.current.read().unwrap();
        let commit_ts = current_version.commit_ts;
        match txn.isolation_level() {
            IsolationLevel::Serializable => {
                // Check if the edge is modified by other transactions
                if commit_ts.is_txn_id() && commit_ts != txn.txn_id() {
                    return Err(StorageError::Transaction(
                        TransactionError::WriteReadConflict(format!(
                            "Edge is being modified by transaction {:?}",
                            commit_ts
                        )),
                    ));
                }
                txn.edge_reads.insert(eid);
            }
            IsolationLevel::Snapshot => {
                // Optimistic read allowed, no read set recording
            }
        }
        // The edge is visible, which means it is either modified by txn or nobody
        let mut visible_edge = current_version.data.clone();
        // Only when the edge is modified by nobody and txn started before the edge was
        // modified, we need to apply the deltas to the edge
        if commit_ts.is_commit_ts() && commit_ts > txn.start_ts() {
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
    fn iter_vertices<'a>(
        &'a self,
        txn: &'a Self::Transaction,
    ) -> StorageResult<Self::VertexIter<'a>> {
        Ok(txn.iter_vertices())
    }

    /// Returns an iterator over all edges within a transaction.
    fn iter_edges<'a>(&'a self, txn: &'a Self::Transaction) -> StorageResult<Self::EdgeIter<'a>> {
        Ok(txn.iter_edges())
    }

    /// Returns an iterator over the adjacency list of a vertex in a given direction.
    fn iter_adjacency<'a>(
        &'a self,
        txn: &'a Self::Transaction,
        vid: Self::VertexID,
    ) -> StorageResult<Self::AdjacencyIter<'a>> {
        Ok(txn.iter_adjacency(vid))
    }
}

// Mutable graph methods
impl MutGraph for MemoryGraph {
    /// Inserts a new vertex into the graph within a transaction.
    fn create_vertex(&self, txn: &MemTransaction, vertex: Vertex) -> StorageResult<VertexId> {
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
    fn create_edge(&self, txn: &MemTransaction, edge: Edge) -> StorageResult<EdgeId> {
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
    fn delete_vertex(&self, txn: &MemTransaction, vid: VertexId) -> StorageResult<()> {
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

        // Mark the vertex as deleted
        let tombstone = Vertex::tombstone(current.data.clone());
        current.data = tombstone;

        // Record the vertex deletion in the transaction
        let delta = DeltaOp::CreateVertex(current.data.clone());
        let undo_ptr = entry.chain.undo_ptr.read().unwrap().clone();
        let mut undo_buffer = txn.undo_buffer.write().unwrap();
        let undo_entry = Arc::new(UndoEntry::new(delta, current.commit_ts, undo_ptr));
        undo_buffer.push(undo_entry.clone());
        *entry.chain.undo_ptr.write().unwrap() = Arc::downgrade(&undo_entry);

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
    fn delete_edge(&self, txn: &MemTransaction, eid: EdgeId) -> StorageResult<()> {
        // Atomically retrieve the versioned edge (check existence).
        let entry = self.edges.get(&eid).ok_or(StorageError::EdgeNotFound(
            EdgeNotFoundError::EdgeNotFound(eid.to_string()),
        ))?;

        let mut current = entry.chain.current.write().unwrap();
        check_write_conflict(current.commit_ts, txn)?;

        // Mark the edge as deleted
        let tombstone = Edge::tombstone(current.data.clone());
        current.data = tombstone;

        // Record the edge deletion in the transaction
        let delta = DeltaOp::CreateEdge(current.data.clone());
        let undo_ptr = entry.chain.undo_ptr.read().unwrap().clone();
        let mut undo_buffer = txn.undo_buffer.write().unwrap();
        let undo_entry = Arc::new(UndoEntry::new(delta, current.commit_ts, undo_ptr));
        undo_buffer.push(undo_entry.clone());
        *entry.chain.undo_ptr.write().unwrap() = Arc::downgrade(&undo_entry);

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
    fn set_vertex_property(
        &self,
        txn: &MemTransaction,
        vid: VertexId,
        indices: Vec<usize>,
        props: Vec<PropertyValue>,
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
    fn set_edge_property(
        &self,
        txn: &MemTransaction,
        eid: EdgeId,
        indices: Vec<usize>,
        props: Vec<PropertyValue>,
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
}

/// Checks if the vertex is modified by other transactions or has a greater commit timestamp than
/// the current transaction.
/// Current check applies to both Snapshot Isolation and Serializable isolation levels.
#[inline]
fn check_write_conflict(commit_ts: Timestamp, txn: &MemTransaction) -> StorageResult<()> {
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
    use std::{env, fs};

    use minigu_common::datatype::types::LabelId;
    use minigu_common::datatype::value::PropertyValue;
    use {Edge, Vertex};

    use super::*;
    use crate::model::properties::PropertyRecord;
    use crate::storage::StorageTransaction;

    const PERSON: LabelId = 1;
    const FRIEND: LabelId = 1;
    const FOLLOW: LabelId = 2;

    fn create_vertex(id: VertexId, label_id: LabelId, properties: Vec<PropertyValue>) -> Vertex {
        Vertex::new(id, label_id, PropertyRecord::new(properties))
    }

    fn create_edge(
        id: EdgeId,
        src_id: VertexId,
        dst_id: VertexId,
        label_id: LabelId,
        properties: Vec<PropertyValue>,
    ) -> Edge {
        Edge::new(
            id,
            src_id,
            dst_id,
            label_id,
            PropertyRecord::new(properties),
        )
    }

    pub fn mock_checkpoint_config() -> CheckpointManagerConfig {
        let dir = env::temp_dir().join(format!(
            "test_checkpoint_{}_{}",
            chrono::Utc::now(),
            rand::random::<u32>()
        ));
        fs::create_dir_all(&dir).unwrap();
        CheckpointManagerConfig {
            checkpoint_dir: dir,
            max_checkpoints: 3,
            auto_checkpoint_interval_secs: 0, // Disable auto checkpoints for testing
            checkpoint_prefix: "test_checkpoint".to_string(),
            transaction_timeout_secs: 10,
        }
    }

    pub fn mock_wal_config() -> WalManagerConfig {
        let file_name = format!(
            "test_wal_{}_{}.log",
            chrono::Utc::now(),
            rand::random::<u32>()
        );
        let path = env::temp_dir().join(file_name);
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
            PropertyValue::String("Alice".into()),
            PropertyValue::Int(25),
        ]);

        let bob = create_vertex(2, PERSON, vec![
            PropertyValue::String("Bob".into()),
            PropertyValue::Int(28),
        ]);

        let carol = create_vertex(3, PERSON, vec![
            PropertyValue::String("Carol".into()),
            PropertyValue::Int(24),
        ]);

        let david = create_vertex(4, PERSON, vec![
            PropertyValue::String("David".into()),
            PropertyValue::Int(27),
        ]);

        // Add vertices to the graph
        graph.create_vertex(&txn, alice).unwrap();
        graph.create_vertex(&txn, bob).unwrap();
        graph.create_vertex(&txn, carol).unwrap();
        graph.create_vertex(&txn, david).unwrap();

        // Create friend edges
        let friend1 = create_edge(1, 1, 2, FRIEND, vec![PropertyValue::String(
            "2020-01-01".into(),
        )]);

        let friend2 = create_edge(2, 2, 3, FRIEND, vec![PropertyValue::String(
            "2021-03-15".into(),
        )]);

        // Create follow edges
        let follow1 = create_edge(3, 1, 3, FOLLOW, vec![PropertyValue::String(
            "2022-06-01".into(),
        )]);

        let follow2 = create_edge(4, 4, 1, FOLLOW, vec![PropertyValue::String(
            "2022-07-15".into(),
        )]);

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
            PropertyValue::String("Eve".into()),
            PropertyValue::Int(24),
        ])
    }

    fn create_vertex_frank() -> Vertex {
        create_vertex(6, PERSON, vec![
            PropertyValue::String("Frank".into()),
            PropertyValue::Int(25),
        ])
    }

    fn create_edge_alice_to_eve() -> Edge {
        create_edge(5, 1, 5, FRIEND, vec![PropertyValue::String(
            "2025-03-31".into(),
        )])
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
    fn test_serializable_prevent_dirty_read() {
        let (graph, _cleaner) = mock_graph();

        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let alice_vid = 1;
        graph
            .set_vertex_property(&txn1, alice_vid, vec![1], vec![PropertyValue::Int(26)])
            .unwrap();

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        // Under serializable isolation, dirty read (read vertex/edge which is modified by other
        // transactions) is not allowed
        assert!(graph.get_vertex(&txn2, alice_vid).is_err());

        let _ = txn1.commit().unwrap();
        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        let read_v1 = graph.get_vertex(&txn2, alice_vid).unwrap();
        assert_eq!(read_v1.properties()[1], PropertyValue::Int(26));
        assert!(txn2.commit().is_ok());
    }

    #[test]
    fn test_serializable_prevent_write_write_conflict() {
        let (graph, _cleaner) = mock_graph();

        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let alice_vid = 1;
        graph
            .set_vertex_property(&txn1, alice_vid, vec![1], vec![PropertyValue::Int(26)])
            .unwrap();

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        assert!(
            graph
                .set_vertex_property(&txn2, alice_vid, vec![1], vec![PropertyValue::Int(27)])
                .is_err()
        );
    }

    #[test]
    fn test_serializable_prevent_non_repeatable_read() {
        let (graph, _cleaner) = mock_graph();

        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let alice_vid = 1;
        let read_v1 = graph.get_vertex(&txn1, alice_vid).unwrap();
        assert_eq!(read_v1.properties()[1], PropertyValue::Int(25));

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        graph
            .set_vertex_property(&txn2, alice_vid, vec![1], vec![PropertyValue::Int(26)])
            .unwrap();
        assert!(txn2.commit().is_ok());

        // Txn1 should should not see the update from txn2
        let read_v1 = graph.get_vertex(&txn1, alice_vid).unwrap();
        assert_eq!(read_v1.properties()[1], PropertyValue::Int(25));
    }

    #[test]
    fn test_serializable_prevent_phantom() {
        let (graph, _cleaner) = mock_graph();

        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        // Read verticex with age ranging from 24 to 28
        let iter1 = txn1.iter_vertices().filter_map(|v| v.ok()).filter(|v| {
            v.properties()[1].as_int().unwrap() >= &24 && v.properties()[1].as_int().unwrap() <= &28
        });
        let mut count = 0;
        for _ in iter1 {
            count += 1;
        }
        assert_eq!(count, 4);

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        let eve = create_vertex_eve();
        graph.create_vertex(&txn2, eve).unwrap();
        assert!(txn2.commit().is_ok());

        // Txn1 should not be able to read vertex `eve`,
        // since it is created after txn1 started
        let iter2 = txn1.iter_vertices().filter_map(|v| v.ok()).filter(|v| {
            v.properties()[1].as_int().unwrap() >= &24 && v.properties()[1].as_int().unwrap() <= &28
        });
        let mut count = 0;
        for _ in iter2 {
            count += 1;
        }
        assert_eq!(count, 4);
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
        assert_eq!(old_v1.properties()[1], PropertyValue::Int(24));
        assert!(
            graph
                .set_vertex_property(&txn2, vid1, vec![1], vec![PropertyValue::Int(25)])
                .is_ok()
        );
        assert!(txn2.commit().is_ok());

        let txn3 = graph.begin_transaction(IsolationLevel::Serializable);
        let new_v1: Vertex = graph.get_vertex(&txn3, vid1).unwrap();
        assert_eq!(new_v1.properties()[1], PropertyValue::Int(25));
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
            .set_vertex_property(&txn2, vid1, vec![0], vec![PropertyValue::Int(25)])
            .unwrap();
        assert!(txn2.commit().is_ok());

        let txn3 = graph.begin_transaction(IsolationLevel::Serializable);
        let v = graph.get_vertex(&txn3, vid1).unwrap();
        assert_eq!(v.properties()[0], PropertyValue::Int(25));
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
            let iter1 = txn2
                .iter_vertices()
                .filter_map(|v| v.ok())
                .filter(|v| v.properties()[0].as_string().unwrap() == "Eve");
            let mut count = 0;
            for _ in iter1 {
                count += 1;
            }
            assert_eq!(count, 1);
        }
        {
            let iter2 = txn2.iter_vertices().filter_map(|v| v.ok()).filter(|v| {
                v.properties()[1].as_int().unwrap() >= &20
                    && v.properties()[1].as_int().unwrap() <= &25
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
            PropertyRecord::new(vec![PropertyValue::String("2023-01-01".into())]),
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
            1,
            PropertyRecord::new(vec![PropertyValue::String("Before Checkpoint".into())]),
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
            1,
            PropertyRecord::new(vec![PropertyValue::String("After Checkpoint".into())]),
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
            PropertyValue::String("Before Checkpoint".into())
        );

        assert_eq!(recovered_vertex2.vid(), vertex2.vid());
        assert_eq!(
            recovered_vertex2.properties()[0],
            PropertyValue::String("After Checkpoint".into())
        );
    }
}
