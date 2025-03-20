use std::sync::{Arc, Mutex, RwLock};

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
use crate::transaction::IsolationLevel;

use super::transaction::{CommitTimestamp, MemTransaction, TransactionId, UndoEntry};

// Version metadata (equivalent to version metadata in the referenced paper)
#[derive(Debug)]
/// Stores the current version of an entity, along with transaction metadata.
pub(super) struct CurrentVersion<D> {
    pub(super) data: D,                            // The actual data version
    pub(super) modified_by: Option<TransactionId>, // Transaction ID marking the modification
    pub(super) commit_ts: CommitTimestamp,         // Commit timestamp indicating when it was committed
}

// Version chain structure
#[derive(Debug)]
/// Maintains the version history of an entity, supporting multi-version concurrency control.
pub(super) struct VersionChain<D: Clone> {
    pub(super) current: RwLock<CurrentVersion<D>>, // The latest version in memory
    pub(super) undo_lists: RwLock<Vec<UndoEntry<D>>>, // The version history (undo log)
}

#[derive(Debug)]
/// Represents a versioned vertex in the graph, supporting multi-version concurrency control.
pub(super) struct VersionedVertex {
    pub(super) chain: Arc<VersionChain<Vertex>>,
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
        // Case 2: Return the current committed version if it is valid at `start_ts`.
        if current.modified_by == Some(txn_id) || 
            current.modified_by.is_none() && current.commit_ts <= start_ts
        {
            if current.data.is_tombstone() {
                return None;
            } else {
                return Some(current.data.clone());
            }
        }

        // Case 3: Search the version history for the most recent valid version.
        let versions = self.chain.undo_lists.read().unwrap();
        if let Some(entry) = versions
                    .iter()
                    .rev()
                    .find(|entry| entry.begin_ts() <= start_ts && start_ts < entry.end_ts()) {
            if entry.data().is_tombstone() {
                return None;
            } else {
                return Some(entry.data().clone());
            }
        } else {
            None
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
        // Case 2: Return the current committed version if it is valid at `start_ts`.
        if current.modified_by == Some(txn_id) ||
            current.modified_by.is_none() && current.commit_ts <= start_ts {
            if current.data.is_tombstone() {
                return None;
            } else {
                return Some(current.data.clone());
            }
        }

        // Case 3: Search the version history for the most recent valid version.
        let versions = self.chain.undo_lists.read().unwrap();
        if let Some(vertex) = versions
            .iter()
            .rev()
            .find(|entry| entry.begin_ts() <= start_ts && start_ts < entry.end_ts()) {
            if vertex.data().is_tombstone() {
                return None;
            } else {
                return Some(vertex.data().clone());
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
/// Represents a versioned adjacency entry with timestamps for MVCC.
pub(super) struct VersionedAdjEntry {
    pub(super) inner: Adjacency,          // The adjacency data
    pub(super) begin_ts: CommitTimestamp, // Timestamp when this entry became valid
    pub(super) end_ts: CommitTimestamp,   // Default u64::MAX indicates not deleted
}

impl VersionedAdjEntry {
    /// Creates a new adjacency entry with the given edge ID, vertex ID, and begin timestamp.
    pub(super) fn new(edge_id: EdgeId, vertex_id: VertexId, begin_ts: CommitTimestamp) -> Self {
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


pub struct MemGraph {
    // ---- Versioned data storage ----
    pub(super) vertices: DashMap<VertexId, VersionedVertex>, // Stores versioned vertices
    pub(super) edges: DashMap<EdgeId, VersionedEdge>,        // Stores versioned edges

    // ---- Adjacency lists (with versioning) ----
    pub(super) adjacency_out: DashMap<VertexId, Vec<VersionedAdjEntry>>, // Outgoing adjacency list
    pub(super) adjacency_in: DashMap<VertexId, Vec<VersionedAdjEntry>>,  // Incoming adjacency list

    // ---- Transaction management ----
    pub(super) active_txns: DashSet<CommitTimestamp>, // Active transactions' start timestamps
    pub(super) commit_lock: Mutex<()>,                // Commit lock to enforce serial commit order
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
    pub fn begin_transaction(self: &Arc<Self>, isolation_level: IsolationLevel) -> MemTransaction {
        // Allocate a new transaction ID and read timestamp.
        let tx_id = TransactionId::new();
        let start_ts = CommitTimestamp::new();

        // Register the transaction as active (used for garbage collection and visibility checks).
        self.active_txns.insert(start_ts);

        MemTransaction::with_memgraph(self.clone(), tx_id, start_ts, isolation_level) 
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
        let visible_vertex: Vertex = versioned_vertex
            .get_visible(txn.start_ts(), txn.tx_id())
            .ok_or_else(|| StorageError::VersionNotFound(vid.to_string()))?;

        // Step 3: Check for logical deletion (tombstone).
        if visible_vertex.is_tombstone() {
            return Err(StorageError::VertexNotFound(vid.to_string()));
        }

        // Step 4: Record the vertex read set for conflict detection.
        if let IsolationLevel::Serializable  = txn.isolation_level() {
            txn.vertex_reads.insert(visible_vertex.vid());
        }

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
            .get_visible(txn.start_ts(), txn.tx_id())
            .ok_or_else(|| StorageError::VersionNotFound(eid.to_string()))?;

        // Step 3: Check for logical deletion (tombstone).
        if visible_edge.is_tombstone() {
            return Err(StorageError::EdgeNotFound(eid.to_string()));
        }

        // Step 4: Record the edge read set for conflict detection.
        if let IsolationLevel::Serializable = txn.isolation_level()  {
            txn.edge_reads.insert(eid.clone());
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
            version.chain.current.write().unwrap().modified_by = Some(txn.tx_id());
            version
        });

        // Conflict detection: Ensure the vertex was not created by another transaction.
        let current = entry.chain.current.read().unwrap();
        if current.modified_by != None && current.modified_by != Some(txn.tx_id()) {
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
            version.chain.current.write().unwrap().modified_by = Some(txn.tx_id());
            version
        });

        // Conflict detection.
        let current = entry.chain.current.read().unwrap();
        if current.modified_by != None && current.modified_by != Some(txn.tx_id()) {
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
        if current.modified_by != None && current.modified_by != Some(txn.tx_id()) {
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
            .push(UndoEntry::new(
                current.data.clone(),
                current.commit_ts,
                txn.start_ts(),
            )
            );

        // Apply logical deletion (mark as uncommitted).
        current.data = tombstone;
        current.modified_by = Some(txn.tx_id());
        current.commit_ts = txn.start_ts();

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
        if current.modified_by != None && current.modified_by != Some(txn.tx_id()) {
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
            .push(UndoEntry::new(
                current.data.clone(),
                current.commit_ts,
                txn.start_ts(),
            ));

        // Apply logical deletion (mark as uncommitted).
        current.data = tombstone;
        current.modified_by = Some(txn.tx_id());
        current.commit_ts = txn.start_ts();

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
        if current.modified_by != None && current.modified_by != Some(txn.tx_id()) {
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
            .push(UndoEntry::new(
                current.data.clone(),
                current.commit_ts,
                txn.start_ts(),
            ));

        // Apply the updated version (mark as uncommitted).
        current.data = new_data;
        current.modified_by = Some(txn.tx_id());
        current.commit_ts = txn.start_ts();

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
        if current.modified_by != None && current.modified_by != Some(txn.tx_id()) {
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
            .push(UndoEntry::new(
                current.data.clone(),
                current.commit_ts,
                txn.start_ts(),
            ));

        // Apply the updated version (mark as uncommitted).
        current.data = new_data;
        current.modified_by = Some(txn.tx_id());
        current.commit_ts = txn.start_ts();

        // Record in the transaction's write set.
        txn.edge_writes.insert(eid);
        Ok(())
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
        let mut txn1 = graph.begin_transaction(IsolationLevel::Serializable);

        let v1 = create_vertex_alice();
        let vid1 = graph.create_vertex(&txn1, v1.clone()).unwrap();
        let _ = txn1.commit().unwrap();

        let mut txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        let read_v1 = graph.get_vertex(&txn2, vid1).unwrap();
        assert_eq!(read_v1, v1);
        assert!(txn2.commit().is_ok());
    }

    #[test]
    fn test_transaction_isolation() {
        let graph = MemGraph::new();

        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_alice();
        let vid1 = graph.create_vertex(&txn1, v1.clone()).unwrap();

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        assert!(graph.get_vertex(&txn2, vid1).is_err());

        let _ = txn1.abort();
        assert!(graph.get_vertex(&txn2, vid1).is_err());
    }

    #[test]
    fn test_mvcc_version_chain() {
        let graph = MemGraph::new();

        let mut txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_alice();
        let vid1 = graph.create_vertex(&txn1, v1).unwrap();
        assert!(txn1.commit().is_ok());

        let mut txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        let old_v1: Vertex = graph.get_vertex(&txn2, vid1).unwrap();
        assert_eq!(old_v1.properties()[1], PropertyValue::Int(25));
        assert!(
            graph
                .set_vertex_property(&txn2, vid1, vec![1], vec![PropertyValue::Int(26)])
                .is_ok()
        );
        assert!(txn2.commit().is_ok());

        let txn3 = graph.begin_transaction(IsolationLevel::Serializable);
        let new_v1: Vertex = graph.get_vertex(&txn3, vid1).unwrap();
        assert_eq!(new_v1.properties()[1], PropertyValue::Int(26));
    }

    #[test]
    fn test_delete_with_tombstone() {
        let graph = MemGraph::new();

        let mut txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_alice();
        let vid1 = graph.create_vertex(&txn1, v1).unwrap();
        assert!(txn1.commit().is_ok());

        let mut txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        graph.delete_vertex(&txn2, vid1).unwrap();
        assert!(txn2.commit().is_ok());

        let txn3 = graph.begin_transaction(IsolationLevel::Serializable);
        assert!(graph.get_vertex(&txn3, vid1).is_err());
    }

    #[test]
    fn test_conflict_detection() {
        let graph = MemGraph::new();

        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_alice();
        graph.create_vertex(&txn1, v1).unwrap();

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        assert!(graph.create_vertex(&txn2, create_vertex_alice()).is_err());
        assert!(graph.create_vertex(&txn2, create_vertex_bob()).is_ok());
    }

    #[test]
    fn test_adjacency_versioning() {
        let graph = MemGraph::new();

        let mut txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_alice();
        let v2 = create_vertex_bob();

        let vid1 = graph.create_vertex(&txn1, v1).unwrap();
        let vid2 = graph.create_vertex(&txn1, v2).unwrap();
        assert!(txn1.commit().is_ok());

        let mut txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        let e1 = create_edge_alice_to_bob();
        let eid1 = graph.create_edge(&txn2, e1).unwrap();
        assert!(txn2.commit().is_ok());

        {
            let adj = graph.adjacency_out.get(&vid1).unwrap();
            assert!(adj.len() == 1 && adj[0].end_ts == CommitTimestamp::max());
        }

        let txn3 = graph.begin_transaction(IsolationLevel::Serializable);
        let e1 = graph.get_edge(&txn3, eid1).unwrap();
        assert!(e1.src_id() == vid1 && e1.dst_id() == vid2);
        let _ = txn3.abort();

        let mut txn4 = graph.begin_transaction(IsolationLevel::Serializable);
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

        let txn = graph.begin_transaction(IsolationLevel::Serializable);
        let vid1 = graph.create_vertex(&txn, create_vertex_alice()).unwrap();
        let _ = txn.abort();

        let txn_check = graph.begin_transaction(IsolationLevel::Serializable);
        assert!(graph.get_vertex(&txn_check, vid1).is_err());
    }

    #[test]
    fn test_property_update_flow() {
        let graph = MemGraph::new();

        let mut txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_alice();
        let vid1 = graph.create_vertex(&txn1, v1).unwrap();
        assert!(txn1.commit().is_ok());

        let mut txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        graph
            .set_vertex_property(&txn2, vid1, vec![0], vec![PropertyValue::Int(42)])
            .unwrap();
        assert!(txn2.commit().is_ok());

        let txn3 = graph.begin_transaction(IsolationLevel::Serializable);
        let v = graph.get_vertex(&txn3, vid1).unwrap();
        assert_eq!(v.properties()[0], PropertyValue::Int(42));
    }

    #[test]
    fn test_read_after_write_conflict() {
        let graph = MemGraph::new();

        let mut txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let vid1 = graph.create_vertex(&txn1, create_vertex_alice()).unwrap();
        assert!(txn1.commit().is_ok());

        let mut txn2 = graph.begin_transaction(IsolationLevel::Serializable);
        let _ = graph.get_vertex(&txn2, vid1).unwrap();

        let mut txn3 = graph.begin_transaction(IsolationLevel::Serializable);
        graph
            .set_vertex_property(&txn3, vid1, vec![0], vec![PropertyValue::Int(99)])
            .unwrap();
        assert!(txn3.commit().is_ok());

        assert!(txn2.commit().is_err());
    }

    #[test]
    fn test_vertex_iterator() {
        let graph = MemGraph::new();

        let mut txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_alice();
        let v2 = create_vertex_bob();
        let _ = graph.create_vertex(&txn1, v1).unwrap();
        let _ = graph.create_vertex(&txn1, v2).unwrap();
        assert!(txn1.commit().is_ok());

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
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

        let mut txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_alice();
        let v2 = create_vertex_bob();
        let _ = graph.create_vertex(&txn1, v1).unwrap();
        let _ = graph.create_vertex(&txn1, v2).unwrap();
        let e1 = create_edge_alice_to_bob();
        let _ = graph.create_edge(&txn1, e1).unwrap();
        assert!(txn1.commit().is_ok());

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
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

        let mut txn1 = graph.begin_transaction(IsolationLevel::Serializable);
        let v1 = create_vertex_alice();
        let v2 = create_vertex_bob();
        let vid1 = graph.create_vertex(&txn1, v1).unwrap();
        let _ = graph.create_vertex(&txn1, v2).unwrap();
        let e1 = create_edge_alice_to_bob();
        let _ = graph.create_edge(&txn1, e1).unwrap();
        assert!(txn1.commit().is_ok());

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
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
