use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::vec;

use common::datatype::types::{EdgeId, VertexId};
use common::datatype::value::PropertyValue;
use dashmap::{DashMap, DashSet};

use crate::error::{StorageError, StorageResult};
use crate::memory::adjacency_iterator::AdjacencyIterator;
use crate::memory::edge_iterator::EdgeIterator;
use crate::memory::vertex_iterator::VertexIterator;
use crate::model::edge::{Adjacency, Direction, Edge};
use crate::model::vertex::Vertex;
use crate::storage::{Graph, MutGraph, StorageTransaction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Represents a commit timestamp used for multi-version concurrency control (MVCC).
pub struct CommitTimestamp(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Represents a unique transaction identifier.
pub struct TransactionId(u64);

impl TransactionId {
    const START: u64 = 1 << 63;

    /// Generates a new transaction ID, ensuring atomicity using an atomic counter.
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(TransactionId::START);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

impl CommitTimestamp {
    /// Generates a new commit timestamp using an atomic counter.
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }

    /// Returns the maximum possible commit timestamp.
    pub fn max() -> Self {
        Self(u64::MAX)
    }
}

// Version metadata (equivalent to version metadata in the referenced paper)
#[derive(Debug)]
/// Stores the current version of an entity, along with transaction metadata.
struct CurrentVersion<D> {
    data: D,                            // The actual data version
    modified_by: Option<TransactionId>, // Transaction ID marking the modification
    commit_ts: CommitTimestamp,         // Commit timestamp indicating when it was committed
}

#[derive(Debug, Clone)]
/// Represents an undo log entry for multi-version concurrency control.
struct UndoEntry<D> {
    data: D,                   // Previous version of the data
    begin_ts: CommitTimestamp, // Timestamp when this version became valid
    end_ts: CommitTimestamp,   // Timestamp when this version became invalid
}

// Version chain structure
#[derive(Debug)]
/// Maintains the version history of an entity, supporting multi-version concurrency control.
pub(super) struct VersionChain<D: Clone> {
    current: RwLock<CurrentVersion<D>>, // The latest version in memory
    undo_lists: RwLock<Vec<UndoEntry<D>>>, // The version history (undo log)
}

#[derive(Debug)]
/// Represents a versioned vertex in the graph, supporting multi-version concurrency control.
pub(super) struct VersionedVertex {
    chain: Arc<VersionChain<Vertex>>,
}

#[derive(Debug)]
/// Represents a versioned edge in the graph, supporting multi-version concurrency control.
pub(super) struct VersionedEdge {
    chain: Arc<VersionChain<Edge>>,
}

#[derive(Debug, Clone)]
/// Represents a versioned adjacency entry with timestamps for MVCC.
pub struct VersionedAdjEntry {
    inner: Adjacency,          // The adjacency data
    begin_ts: CommitTimestamp, // Timestamp when this entry became valid
    end_ts: CommitTimestamp,   // Default u64::MAX indicates not deleted
}

pub struct MemGraph {
    // ---- Versioned data storage ----
    vertices: DashMap<VertexId, VersionedVertex>, // Stores versioned vertices
    edges: DashMap<EdgeId, VersionedEdge>,        // Stores versioned edges

    // ---- Adjacency lists (with versioning) ----
    adjacency_out: DashMap<VertexId, Vec<VersionedAdjEntry>>, // Outgoing adjacency list
    adjacency_in: DashMap<VertexId, Vec<VersionedAdjEntry>>,  // Incoming adjacency list

    // ---- Transaction management ----
    active_txns: DashSet<CommitTimestamp>, // Active transactions' start timestamps
    commit_lock: Mutex<()>,                // Commit lock to enforce serial commit order
}

pub struct MemTransaction {
    graph: Arc<MemGraph>, // Reference to the associated in-memory graph

    // ---- Timestamp management ----
    start_ts: CommitTimestamp, // Start timestamp assigned when the transaction begins
    commit_ts: Option<CommitTimestamp>, // Commit timestamp assigned upon committing
    tx_id: TransactionId,      // Unique transaction identifier

    // ---- Read sets ----
    vertex_reads: DashSet<VertexId>, // Set of vertices read by this transaction
    edge_reads: DashSet<EdgeId>,     // Set of edges read by this transaction

    // ---- Write sets ----
    vertex_writes: DashSet<VertexId>, // Set of vertices written by this transaction
    edge_writes: DashSet<EdgeId>,     // Set of edges written by this transaction

    // ---- Undo logs ----
    vertex_undos: DashMap<VertexId, Vec<UndoEntry<Vertex>>>, // Vertex undo logs
    edge_undos: DashMap<EdgeId, Vec<UndoEntry<Edge>>>,       // Edge undo logs
}

impl VersionedVertex {
    /// Creates a new `VersionedVertex` instance with an initial vertex.
    pub fn new(initial: Vertex) -> Self {
        Self {
            chain: Arc::new(VersionChain {
                current: RwLock::new(CurrentVersion {
                    data: initial,
                    modified_by: None,
                    commit_ts: CommitTimestamp(0), // Initial commit timestamp set to 0
                }),
                undo_lists: RwLock::new(Vec::new()),
            }),
        }
    }

    /// Retrieves the latest visible version of the vertex based on MVCC rules.
    pub fn get_visible(&self, start_ts: CommitTimestamp, txn_id: TransactionId) -> Option<Vertex> {
        let current = self.chain.current.read().unwrap();

        // Case 1: Return the uncommitted modification if it belongs to the current transaction.
        if current.modified_by == Some(txn_id) {
            return Some(current.data.clone());
        }

        // Case 2: Return the current committed version if it is valid at `start_ts`.
        if current.modified_by.is_none() && current.commit_ts <= start_ts {
            return Some(current.data.clone());
        }

        // Case 3: Search the version history for the most recent valid version.
        let versions = self.chain.undo_lists.read().unwrap();
        versions
            .iter()
            .rev()
            .find(|entry| entry.begin_ts <= start_ts && start_ts < entry.end_ts)
            .map(|entry| entry.data.clone())
    }
}

impl VersionedEdge {
    /// Creates a new `VersionedEdge` instance with an initial edge.
    pub fn new(initial: Edge) -> Self {
        Self {
            chain: Arc::new(VersionChain {
                current: RwLock::new(CurrentVersion {
                    data: initial,
                    modified_by: None,
                    commit_ts: CommitTimestamp(0), // Initial commit timestamp set to 0
                }),
                undo_lists: RwLock::new(Vec::new()),
            }),
        }
    }

    /// Retrieves the latest visible version of the edge based on MVCC rules.
    pub fn get_visible(&self, start_ts: CommitTimestamp, txn_id: TransactionId) -> Option<Edge> {
        let current = self.chain.current.read().unwrap();

        // Case 1: Return the uncommitted modification if it belongs to the current transaction.
        if current.modified_by == Some(txn_id) {
            return Some(current.data.clone());
        }

        // Case 2: Return the current committed version if it is valid at `start_ts`.
        if current.commit_ts <= start_ts && current.modified_by.is_none() {
            return Some(current.data.clone());
        }

        // Case 3: Search the version history for the most recent valid version.
        let versions = self.chain.undo_lists.read().unwrap();
        versions
            .iter()
            .rev()
            .find(|entry| entry.begin_ts <= start_ts && start_ts < entry.end_ts)
            .map(|entry| entry.data.clone())
    }
}

impl VersionedAdjEntry {
    /// Creates a new adjacency entry with the given edge ID, vertex ID, and begin timestamp.
    fn new(edge_id: EdgeId, vertex_id: VertexId, begin_ts: CommitTimestamp) -> Self {
        Self {
            inner: Adjacency::new(vertex_id, edge_id),
            begin_ts,
            end_ts: CommitTimestamp::max(), // Default end timestamp is set to max (not deleted)
        }
    }

    /// Returns a reference to the underlying adjacency information.
    pub fn inner(&self) -> &Adjacency {
        &self.inner
    }

    /// Returns the begin timestamp of the adjacency entry.
    pub fn begin_ts(&self) -> CommitTimestamp {
        self.begin_ts
    }

    /// Returns the end timestamp of the adjacency entry.
    pub fn end_ts(&self) -> CommitTimestamp {
        self.end_ts
    }

    /// Returns the edge ID of the adjacency entry.
    pub fn edge_id(&self) -> EdgeId {
        self.inner.edge_id()
    }
}

#[allow(dead_code)]
// Basic methods for MemGraph
impl MemGraph {
    /// Creates a new instance of `MemGraph`.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            vertices: DashMap::new(),
            edges: DashMap::new(),
            active_txns: DashSet::new(),
            commit_lock: Mutex::new(()),
            adjacency_out: DashMap::new(),
            adjacency_in: DashMap::new(),
        })
    }

    /// Begins a new transaction and returns a `MemTransaction` instance.
    pub fn begin_transaction(self: &Arc<Self>) -> MemTransaction {
        // Allocate a new transaction ID and read timestamp.
        let tx_id = TransactionId::new();
        let start_ts = CommitTimestamp::new();

        // Register the transaction as active (used for garbage collection and visibility checks).
        self.active_txns.insert(start_ts);

        MemTransaction {
            graph: Arc::clone(self),
            start_ts,
            commit_ts: None, // Assigned upon commit
            tx_id,
            vertex_reads: DashSet::new(),
            edge_reads: DashSet::new(),
            vertex_writes: DashSet::new(),
            edge_writes: DashSet::new(),
            vertex_undos: DashMap::new(),
            edge_undos: DashMap::new(),
        }
    }

    /// Returns a reference to the vertices storage.
    pub fn vertices(&self) -> &DashMap<VertexId, VersionedVertex> {
        &self.vertices
    }

    /// Returns a reference to the edges storage.
    pub fn edges(&self) -> &DashMap<EdgeId, VersionedEdge> {
        &self.edges
    }

    /// Returns a reference to the outgoing adjacency list storage.
    pub fn adjacency_out(&self) -> &DashMap<VertexId, Vec<VersionedAdjEntry>> {
        &self.adjacency_out
    }

    /// Returns a reference to the incoming adjacency list storage.
    pub fn adjacency_in(&self) -> &DashMap<VertexId, Vec<VersionedAdjEntry>> {
        &self.adjacency_in
    }
}

// Immutable graph methods
impl Graph for MemGraph {
    type Adjacency = Adjacency;
    type AdjacencyIter<'a> = AdjacencyIterator<'a>;
    type Direction = Direction;
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
        let versioned_vertex = self
            .vertices
            .get(&vid)
            .ok_or(StorageError::VertexNotFound(vid.to_string()))?;

        // Step 2: Perform MVCC visibility check.
        let visible_vertex = versioned_vertex
            .get_visible(txn.start_ts, txn.tx_id)
            .ok_or_else(|| StorageError::VersionNotFound(vid.to_string()))?;

        // Step 3: Check for logical deletion (tombstone).
        if visible_vertex.is_tombstone() {
            return Err(StorageError::VertexNotFound(vid.to_string()));
        }

        // Step 4: Record the vertex read set for conflict detection.
        txn.vertex_reads.insert(vid.clone());

        Ok(visible_vertex)
    }

    /// Retrieves an edge by its ID within the context of a transaction.
    fn get_edge(&self, txn: &MemTransaction, eid: EdgeId) -> StorageResult<Edge> {
        // Step 1: Atomically retrieve the versioned edge (check existence).
        let versioned_edge = self
            .edges
            .get(&eid)
            .ok_or(StorageError::EdgeNotFound(eid.to_string()))?;

        // Step 2: Perform MVCC visibility check.
        let visible_edge = versioned_edge
            .get_visible(txn.start_ts, txn.tx_id)
            .ok_or_else(|| StorageError::VersionNotFound(eid.to_string()))?;

        // Step 3: Check for logical deletion (tombstone).
        if visible_edge.is_tombstone() {
            return Err(StorageError::EdgeNotFound(eid.to_string()));
        }

        // Step 4: Record the edge read set for conflict detection.
        txn.edge_reads.insert(eid.clone());

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
        direction: Direction,
    ) -> StorageResult<Self::AdjacencyIter<'a>> {
        Ok(txn.iter_adjacency(vid, direction))
    }
}

// Mutable graph methods
impl MutGraph for MemGraph {
    /// Inserts a new vertex into the graph within a transaction.
    fn create_vertex(&self, txn: &MemTransaction, vertex: Vertex) -> StorageResult<VertexId> {
        let vid = vertex.vid().clone();

        // Atomically check if the vertex exists and insert if not.
        let entry = self.vertices.entry(vid.clone()).or_insert_with(|| {
            // Create an uncommitted version of the vertex.
            let version = VersionedVertex::new(vertex);
            // Mark the vertex as created by this transaction.
            version.chain.current.write().unwrap().modified_by = Some(txn.tx_id);
            version
        });

        // Conflict detection: Ensure the vertex was not created by another transaction.
        let current = entry.chain.current.read().unwrap();
        if current.modified_by != None && current.modified_by != Some(txn.tx_id) {
            return Err(StorageError::VertexAlreadyExists(vid.to_string()));
        }

        // Record in the transaction's write set.
        txn.vertex_writes.insert(vid.clone());
        Ok(vid)
    }

    /// Inserts a new edge into the graph within a transaction.
    fn create_edge(&self, txn: &MemTransaction, edge: Edge) -> StorageResult<EdgeId> {
        let eid = edge.eid().clone();

        // Check if source and destination vertices exist.
        if self.get_vertex(txn, edge.src_id()).is_err() {
            return Err(StorageError::VertexNotFound(edge.src_id().to_string()));
        }
        if self.get_vertex(txn, edge.dst_id()).is_err() {
            return Err(StorageError::VertexNotFound(edge.dst_id().to_string()));
        }

        // Atomically check if the edge exists and insert if not.
        let entry = self.edges.entry(eid.clone()).or_insert_with(|| {
            let version = VersionedEdge::new(edge.clone());
            version.chain.current.write().unwrap().modified_by = Some(txn.tx_id);
            version
        });

        // Conflict detection.
        let current = entry.chain.current.read().unwrap();
        if current.modified_by != None && current.modified_by != Some(txn.tx_id) {
            return Err(StorageError::EdgeAlreadyExists(eid.to_string()));
        }

        // Record in the transaction's write set.
        txn.edge_writes.insert(eid.clone());
        Ok(eid)
    }

    /// Deletes a vertex from the graph within a transaction.
    fn delete_vertex(&self, txn: &MemTransaction, vid: VertexId) -> StorageResult<()> {
        // Atomically retrieve the versioned vertex (check existence).
        let entry = self
            .vertices
            .get(&vid)
            .ok_or(StorageError::VertexNotFound(vid.to_string()))?;

        // Acquire a write lock to modify the vertex.
        let mut current = entry.chain.current.write().unwrap();

        // Conflict detection: Ensure the vertex is modified only by this transaction.
        if current.modified_by != None && current.modified_by != Some(txn.tx_id) {
            return Err(StorageError::TransactionError(
                "Concurrent modification detected".to_string(),
            ));
        }

        // Create a tombstone version to logically delete the vertex.
        let tombstone = Vertex::tombstone(current.data.clone());

        // Store the previous version in the transaction's undo log.
        txn.vertex_undos
            .entry(vid.clone())
            .or_insert_with(Vec::new)
            .push(UndoEntry {
                data: current.data.clone(),
                begin_ts: current.commit_ts,
                end_ts: txn.start_ts,
            });

        // Apply logical deletion (mark as uncommitted).
        current.data = tombstone;
        current.modified_by = Some(txn.tx_id);
        current.commit_ts = txn.start_ts;

        // Record in the transaction's write set.
        txn.vertex_writes.insert(vid.clone());
        Ok(())
    }

    /// Deletes an edge from the graph within a transaction.
    fn delete_edge(&self, txn: &MemTransaction, eid: EdgeId) -> StorageResult<()> {
        // Atomically retrieve the versioned edge (check existence).
        let entry = self
            .edges
            .get(&eid)
            .ok_or(StorageError::EdgeNotFound(eid.to_string()))?;

        // Acquire a write lock to modify the edge.
        let mut current = entry.chain.current.write().unwrap();

        // Conflict detection: Ensure the edge is modified only by this transaction.
        if current.modified_by != None && current.modified_by != Some(txn.tx_id) {
            return Err(StorageError::TransactionError(
                "Concurrent modification detected".to_string(),
            ));
        }

        // Create a tombstone version to logically delete the edge.
        let tombstone = Edge::tombstone(current.data.clone());

        // Store the previous version in the transaction's undo log.
        txn.edge_undos
            .entry(eid.clone())
            .or_insert_with(Vec::new)
            .push(UndoEntry {
                data: current.data.clone(),
                begin_ts: current.commit_ts,
                end_ts: txn.start_ts,
            });

        // Apply logical deletion (mark as uncommitted).
        current.data = tombstone;
        current.modified_by = Some(txn.tx_id);
        current.commit_ts = txn.start_ts;

        // Record in the transaction's write set.
        txn.edge_writes.insert(eid.clone());
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
        let entry = self
            .vertices
            .get(&vid)
            .ok_or(StorageError::VertexNotFound(vid.to_string()))?;

        // Acquire a write lock to modify the vertex.
        let mut current = entry.chain.current.write().unwrap();

        // Conflict detection: Ensure the vertex is modified only by this transaction.
        if current.modified_by != None && current.modified_by != Some(txn.tx_id) {
            return Err(StorageError::TransactionError(
                "Concurrent modification detected".to_string(),
            ));
        }

        // Create a new version with updated properties.
        let mut new_data = current.data.clone();
        new_data.set_props(&indices, props);

        // Store the previous version in the transaction's undo log.
        txn.vertex_undos
            .entry(vid)
            .or_insert_with(Vec::new)
            .push(UndoEntry {
                data: current.data.clone(),
                begin_ts: current.commit_ts,
                end_ts: txn.start_ts,
            });

        // Apply the updated version (mark as uncommitted).
        current.data = new_data;
        current.modified_by = Some(txn.tx_id);
        current.commit_ts = txn.start_ts;

        // Record in the transaction's write set.
        txn.vertex_writes.insert(vid);
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
        let entry = self
            .edges
            .get(&eid)
            .ok_or(StorageError::EdgeNotFound(eid.to_string()))?;

        // Acquire a write lock to modify the edge.
        let mut current = entry.chain.current.write().unwrap();

        // Conflict detection: Ensure the edge is modified only by this transaction.
        if current.modified_by != None && current.modified_by != Some(txn.tx_id) {
            return Err(StorageError::TransactionError(
                "Concurrent modification detected".to_string(),
            ));
        }

        // Create a new version with updated properties.
        let mut new_data = current.data.clone();
        new_data.set_props(&indices, props);

        // Store the previous version in the transaction's undo log.
        txn.edge_undos
            .entry(eid)
            .or_insert_with(Vec::new)
            .push(UndoEntry {
                data: current.data.clone(),
                begin_ts: current.commit_ts,
                end_ts: txn.start_ts,
            });

        // Apply the updated version (mark as uncommitted).
        current.data = new_data;
        current.modified_by = Some(txn.tx_id);
        current.commit_ts = txn.start_ts;

        // Record in the transaction's write set.
        txn.edge_writes.insert(eid);
        Ok(())
    }
}

impl StorageTransaction for MemTransaction {
    type CommitTimestamp = CommitTimestamp;

    /// Commits the transaction, applying all changes atomically.
    /// Ensures serializability, updates version chains, and manages adjacency lists.
    fn commit(&mut self) -> StorageResult<CommitTimestamp> {
        // Acquire the global commit lock to enforce serial execution of commits.
        let _guard = self.graph.commit_lock.lock().unwrap();

        // Step 1: Assign a commit timestamp (atomic operation).
        let commit_ts = CommitTimestamp::new();
        self.commit_ts = Some(commit_ts);

        // Step 2: Validate serializability
        self.validate_read_sets()?;

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

impl MemTransaction {
    /// Validates the read set to ensure serializability.
    /// If a vertex or edge has been modified since the transaction started, it returns a read
    /// conflict error.
    fn validate_read_sets(&self) -> StorageResult<()> {
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
                if entry.end_ts == max_commit_ts {
                    entry.end_ts = commit_ts;
                }
            }
        }

        // Invalidate incoming edges
        if let Some(mut entries) = self.graph.adjacency_in.get_mut(&vid) {
            for entry in entries.iter_mut() {
                if entry.end_ts == max_commit_ts {
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
}

#[cfg(test)]
mod tests {
    use common::datatype::value::PropertyValue;
    use {Edge, Vertex};

    use super::*;
    use crate::model::properties::PropertyStore;

    fn create_vertex_alice() -> Vertex {
        let properties = vec![
            PropertyValue::String("Alice".into()),
            PropertyValue::Int(25),
        ];
        Vertex::new(100, 1, PropertyStore::new(properties))
    }

    fn create_vertex_bob() -> Vertex {
        let properties = vec![PropertyValue::String("Bob".into()), PropertyValue::Int(30)];
        Vertex::new(101, 1, PropertyStore::new(properties))
    }

    fn create_edge_alice_to_bob() -> Edge {
        let properties = vec![PropertyValue::String("friend".into())];
        Edge::new(
            200,
            100,
            101,
            2,
            crate::model::edge::Direction::Out,
            PropertyStore::new(properties),
        )
    }

    #[test]
    fn test_basic_commit_flow() {
        let graph = MemGraph::new();
        let mut txn1 = graph.begin_transaction();

        let v1 = create_vertex_alice();
        let vid1 = graph.create_vertex(&txn1, v1.clone()).unwrap();
        let _ = txn1.commit().unwrap();

        let mut txn2 = graph.begin_transaction();
        let read_v1 = graph.get_vertex(&txn2, vid1).unwrap();
        assert_eq!(read_v1, v1);
        assert!(txn2.commit().is_ok());
    }

    #[test]
    fn test_transaction_isolation() {
        let graph = MemGraph::new();

        let txn1 = graph.begin_transaction();
        let v1 = create_vertex_alice();
        let vid1 = graph.create_vertex(&txn1, v1.clone()).unwrap();

        let txn2 = graph.begin_transaction();
        assert!(graph.get_vertex(&txn2, vid1).is_err());

        let _ = txn1.abort();
        assert!(graph.get_vertex(&txn2, vid1).is_err());
    }

    #[test]
    fn test_mvcc_version_chain() {
        let graph = MemGraph::new();

        let mut txn1 = graph.begin_transaction();
        let v1 = create_vertex_alice();
        let vid1 = graph.create_vertex(&txn1, v1).unwrap();
        assert!(txn1.commit().is_ok());

        let mut txn2 = graph.begin_transaction();
        let old_v1: Vertex = graph.get_vertex(&txn2, vid1).unwrap();
        assert_eq!(old_v1.properties()[1], PropertyValue::Int(25));
        assert!(
            graph
                .set_vertex_property(&txn2, vid1, vec![1], vec![PropertyValue::Int(26)])
                .is_ok()
        );
        assert!(txn2.commit().is_ok());

        let txn3 = graph.begin_transaction();
        let new_v1: Vertex = graph.get_vertex(&txn3, vid1).unwrap();
        assert_eq!(new_v1.properties()[1], PropertyValue::Int(26));
    }

    #[test]
    fn test_delete_with_tombstone() {
        let graph = MemGraph::new();

        let mut txn1 = graph.begin_transaction();
        let v1 = create_vertex_alice();
        let vid1 = graph.create_vertex(&txn1, v1).unwrap();
        assert!(txn1.commit().is_ok());

        let mut txn2 = graph.begin_transaction();
        graph.delete_vertex(&txn2, vid1).unwrap();
        assert!(txn2.commit().is_ok());

        let txn3 = graph.begin_transaction();
        assert!(graph.get_vertex(&txn3, vid1).is_err());
    }

    #[test]
    fn test_conflict_detection() {
        let graph = MemGraph::new();

        let txn1 = graph.begin_transaction();
        let v1 = create_vertex_alice();
        graph.create_vertex(&txn1, v1).unwrap();

        let txn2 = graph.begin_transaction();
        assert!(graph.create_vertex(&txn2, create_vertex_alice()).is_err());
        assert!(graph.create_vertex(&txn2, create_vertex_bob()).is_ok());
    }

    #[test]
    fn test_adjacency_versioning() {
        let graph = MemGraph::new();

        let mut txn1 = graph.begin_transaction();
        let v1 = create_vertex_alice();
        let v2 = create_vertex_bob();

        let vid1 = graph.create_vertex(&txn1, v1).unwrap();
        let vid2 = graph.create_vertex(&txn1, v2).unwrap();
        assert!(txn1.commit().is_ok());

        let mut txn2 = graph.begin_transaction();
        let e1 = create_edge_alice_to_bob();
        let eid1 = graph.create_edge(&txn2, e1).unwrap();
        assert!(txn2.commit().is_ok());

        {
            let adj = graph.adjacency_out.get(&vid1).unwrap();
            assert!(adj.len() == 1 && adj[0].end_ts == CommitTimestamp::max());
        }

        let txn3 = graph.begin_transaction();
        let e1 = graph.get_edge(&txn3, eid1).unwrap();
        assert!(e1.src_id() == vid1 && e1.dst_id() == vid2);
        let _ = txn3.abort();

        let mut txn4 = graph.begin_transaction();
        graph.delete_edge(&txn4, eid1).unwrap();
        let ts_del = txn4.commit().unwrap();

        {
            let adj = graph.adjacency_out.get(&vid1).unwrap();
            assert!(adj.len() == 1 && adj[0].end_ts == ts_del);
        }
    }

    #[test]
    fn test_rollback_consistency() {
        let graph = MemGraph::new();

        let txn = graph.begin_transaction();
        let vid1 = graph.create_vertex(&txn, create_vertex_alice()).unwrap();
        let _ = txn.abort();

        let txn_check = graph.begin_transaction();
        assert!(graph.get_vertex(&txn_check, vid1).is_err());
    }

    #[test]
    fn test_property_update_flow() {
        let graph = MemGraph::new();

        let mut txn1 = graph.begin_transaction();
        let v1 = create_vertex_alice();
        let vid1 = graph.create_vertex(&txn1, v1).unwrap();
        assert!(txn1.commit().is_ok());

        let mut txn2 = graph.begin_transaction();
        graph
            .set_vertex_property(&txn2, vid1, vec![0], vec![PropertyValue::Int(42)])
            .unwrap();
        assert!(txn2.commit().is_ok());

        let txn3 = graph.begin_transaction();
        let v = graph.get_vertex(&txn3, vid1).unwrap();
        assert_eq!(v.properties()[0], PropertyValue::Int(42));
    }

    #[test]
    fn test_read_after_write_conflict() {
        let graph = MemGraph::new();

        let mut txn1 = graph.begin_transaction();
        let vid1 = graph.create_vertex(&txn1, create_vertex_alice()).unwrap();
        assert!(txn1.commit().is_ok());

        let mut txn2 = graph.begin_transaction();
        let _ = graph.get_vertex(&txn2, vid1).unwrap();

        let mut txn3 = graph.begin_transaction();
        graph
            .set_vertex_property(&txn3, vid1, vec![0], vec![PropertyValue::Int(99)])
            .unwrap();
        assert!(txn3.commit().is_ok());

        assert!(txn2.commit().is_err());
    }

    #[test]
    fn test_vertex_iterator() {
        let graph = MemGraph::new();

        let mut txn1 = graph.begin_transaction();
        let v1 = create_vertex_alice();
        let v2 = create_vertex_bob();
        let _ = graph.create_vertex(&txn1, v1).unwrap();
        let _ = graph.create_vertex(&txn1, v2).unwrap();
        assert!(txn1.commit().is_ok());

        let txn2 = graph.begin_transaction();
        {
            let iter1 = txn2
                .iter_vertices()
                .filter_map(|v| v.ok())
                .filter(|v| v.properties()[0].as_string().unwrap() == "Alice");
            let mut count = 0;
            for _ in iter1 {
                count += 1;
            }
            assert_eq!(count, 1);
        }
        {
            let iter2 = txn2.iter_vertices().filter_map(|v| v.ok()).filter(|v| {
                v.properties()[1].as_int().unwrap() > 20 && v.properties()[1].as_int().unwrap() < 35
            });
            let mut count = 0;
            for _ in iter2 {
                count += 1;
            }
            assert_eq!(count, 2);
        }
        let _ = txn2.abort();
    }

    #[test]
    fn test_edge_iterator() {
        let graph = MemGraph::new();

        let mut txn1 = graph.begin_transaction();
        let v1 = create_vertex_alice();
        let v2 = create_vertex_bob();
        let _ = graph.create_vertex(&txn1, v1).unwrap();
        let _ = graph.create_vertex(&txn1, v2).unwrap();
        let e1 = create_edge_alice_to_bob();
        let _ = graph.create_edge(&txn1, e1).unwrap();
        assert!(txn1.commit().is_ok());

        let txn2 = graph.begin_transaction();
        {
            let iter1 = txn2
                .iter_edges()
                .filter_map(|e| e.ok())
                .filter(|e| e.src_id() == 100);
            let mut count = 0;
            for _ in iter1 {
                count += 1;
            }
            assert_eq!(count, 1);
        }
        {
            let iter2 = txn2
                .iter_edges()
                .filter_map(|e| e.ok())
                .filter(|e| e.dst_id() == 101);
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
                .filter(|e| e.dst_id() == 200);
            let mut count = 0;
            for _ in iter3 {
                count += 1;
            }
            assert_eq!(count, 0);
        }
        let _ = txn2.abort();
    }

    #[test]
    fn test_adj_interator() {
        let graph = MemGraph::new();

        let mut txn1 = graph.begin_transaction();
        let v1 = create_vertex_alice();
        let v2 = create_vertex_bob();
        let vid1 = graph.create_vertex(&txn1, v1).unwrap();
        let _ = graph.create_vertex(&txn1, v2).unwrap();
        let e1 = create_edge_alice_to_bob();
        let _ = graph.create_edge(&txn1, e1).unwrap();
        assert!(txn1.commit().is_ok());

        let txn2 = graph.begin_transaction();
        {
            let mut iter1 = txn2.iter_adjacency(vid1, crate::model::edge::Direction::Out);
            let mut count = 0;
            while let Some(_) = iter1.next() {
                count += 1;
            }
            assert_eq!(count, 1);
        }
        let _ = txn2.abort();
    }
}
