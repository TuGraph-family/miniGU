use std::sync::{Arc, RwLock};

use common::datatype::types::{EdgeId, VertexId};
use common::datatype::value::PropertyValue;
use dashmap::DashMap;

use crate::error::{StorageError, StorageResult};
use crate::memory::adjacency_iterator::AdjacencyIterator;
use crate::memory::edge_iterator::EdgeIterator;
use crate::memory::vertex_iterator::VertexIterator;
use crate::model::edge::{Adjacency, Direction, Edge};
use crate::model::vertex::Vertex;
use crate::storage::{Graph, MutGraph};
use crate::transaction::{CommitTimestamp, DeltaOp, IsolationLevel, SetPropsOp, UndoEntry, UndoPtr};

use super::transaction::{MemTransaction, MemTxnManager};

// Perform the update properties operation
macro_rules! update_properties {
    ($self:expr, $id:expr, $entry:expr, $txn:expr, $indices:expr, $props:expr, $op: ident) => {{
        // Acquire the lock to modify the properties of the vertex/edge
        let mut current = $entry.chain.current.write().unwrap();

        // Conflict detection: Ensure the vertex is modified only by this transaction
        if current.commit_ts != $txn.txn_id() && current.commit_ts > $txn.start_ts() {
            return Err(StorageError::TransactionError(
                "Concurrent modification detected".to_string(),
            ));
        }

        // Create a new version with updated properties.
        current.data.set_props(&$indices, $props);

        let delta_props = $indices.iter()
            .map(|i| current.data.properties.get(*i).unwrap().clone())
            .collect();
        let delta = DeltaOp::$op($id, SetPropsOp { indices: $indices, props: delta_props });

        let undo_ptr = $entry.chain.undo_ptr.read().unwrap();
        let mut undo_buffer = $txn.undo_buffer.write().unwrap();
        undo_buffer.push(UndoEntry::new(delta, current.commit_ts, undo_ptr.clone()));
        *$entry.chain.undo_ptr.write().unwrap() = Some(UndoPtr::new($txn.txn_id(), undo_buffer.len() - 1));
    }};
}

// Perform the delete vertex/edge operation
macro_rules! delete_entity {
    ($self:expr, $entry:expr, $txn:expr, $entity_type:ident, $delta_variant:ident, $error_type:ident) => {{
        // Acquire the lock to modify the vertex/edge
        let mut current = $entry.chain.current.write().unwrap();

        // Conflict detection: Ensure the vertex/edge is modified only by this transaction
        if current.commit_ts != $txn.txn_id() && current.commit_ts > $txn.start_ts() {
            return Err(StorageError::TransactionError(
                "Concurrent modification detected".to_string(),
            ));
        }

        // Mark the vertex/edge as deleted
        let tombstone = $entity_type::tombstone(current.data.clone());
        current.data = tombstone;

        // Record the vertex/edge deletion in the transaction
        let delta = DeltaOp::$delta_variant(current.data.clone());
        let undo_ptr = $entry.chain.undo_ptr.read().unwrap();
        let mut undo_buffer = $txn.undo_buffer.write().unwrap();
        undo_buffer.push(UndoEntry::new(delta, current.commit_ts, undo_ptr.clone()));
        *$entry.chain.undo_ptr.write().unwrap() = Some(UndoPtr::new($txn.txn_id(), undo_buffer.len() - 1));
    }};
}

// Version metadata (equivalent to version metadata in the referenced paper)
#[derive(Debug)]
/// Stores the current version of an entity, along with transaction metadata.
pub(super) struct CurrentVersion<D> {
    pub(super) data: D,                            // The actual data version
    // pub(super) modified_by: Option<CommitTimestamp>, // Transaction ID marking the modification
    pub(super) commit_ts: CommitTimestamp,         // Commit timestamp indicating when it was committed
}

// Version chain structure
#[derive(Debug)]
/// Maintains the version history of an entity, supporting multi-version concurrency control.
pub(super) struct VersionChain<D: Clone> {
    pub(super) current: RwLock<CurrentVersion<D>>, // The latest version in memory
    pub(super) undo_ptr: RwLock<Option<UndoPtr>>, // The version history (undo log)
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
                    commit_ts: CommitTimestamp(0), // Initial commit timestamp set to 0
                }),
                undo_ptr: RwLock::new(None),
            }),
        }
    }

    pub fn current(&self) -> &RwLock<CurrentVersion<Vertex>> {
        &self.chain.current
    }

    pub fn with_modified_ts(initial: Vertex, txn_id: CommitTimestamp) -> Self {
        debug_assert!(txn_id.0 > CommitTimestamp::TXN_ID_START);
        Self {
            chain: Arc::new(VersionChain {
                current: RwLock::new(CurrentVersion {
                    data: initial,
                    commit_ts: txn_id, // Initial commit timestamp set to 0
                }),
                undo_ptr: RwLock::new(None),
            }),
        }
    }

    pub fn get_visible(&self, txn: &MemTransaction) -> StorageResult<Vertex> {
        let current = self.chain.current.read().unwrap();
        let mut current_vertex = current.data.clone();
        if current.commit_ts == txn.txn_id() || current.commit_ts > txn.start_ts() {
            Ok(current_vertex)
        } else {
            let undo_ptr = self.chain.undo_ptr.read().unwrap();
            if let Some(undo_ptr) = undo_ptr.as_ref() {
                txn.apply_deltas_for_vertex(*undo_ptr, &mut current_vertex, txn.start_ts())?;
            }else {
                return Err(StorageError::VersionNotFound(format!("{:?}", current.commit_ts)));
            }
            Ok(current_vertex)
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
                    commit_ts: CommitTimestamp(0), // Initial commit timestamp set to 0
                }),
                undo_ptr: RwLock::new(None),
            }),
        }
    }

    pub fn current(&self) -> &RwLock<CurrentVersion<Edge>> {
        &self.chain.current
    }

    pub fn with_modified_ts(initial: Edge, txn_id: CommitTimestamp) -> Self {
        debug_assert!(txn_id.0 > CommitTimestamp::TXN_ID_START);
        Self {
            chain: Arc::new(VersionChain {
                current: RwLock::new(CurrentVersion {
                    data: initial,
                    commit_ts: txn_id, 
                }),
                undo_ptr: RwLock::new(None),
            })
        }
    }

    pub fn get_visible(&self, txn: &MemTransaction) -> StorageResult<Edge> {
        let current = self.chain.current.read().unwrap();
        let mut current_edge = current.data.clone();
        if current.commit_ts == txn.txn_id() || current.commit_ts > txn.start_ts() {
            Ok(current_edge)
        } else {
            let undo_ptr = self.chain.undo_ptr.read().unwrap();
            if let Some(undo_ptr) = undo_ptr.as_ref() {
                txn.apply_deltas_for_edge(*undo_ptr, &mut current_edge, txn.start_ts())?;
            }else {
                return Err(StorageError::VersionNotFound(format!("{:?}", current.commit_ts)));
            }
            Ok(current_edge)
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
            end_ts: CommitTimestamp::max_commit_ts(), // Default end timestamp is set to max (not deleted)
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
    pub(super) txn_manager: MemTxnManager,
}

#[allow(dead_code)]
// Basic methods for MemGraph
impl MemGraph {
    /// Creates a new instance of `MemGraph`.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            vertices: DashMap::new(),
            edges: DashMap::new(),
            adjacency_out: DashMap::new(),
            adjacency_in: DashMap::new(),
            txn_manager: MemTxnManager::new(),
        })
    }

    /// Begins a new transaction and returns a `MemTransaction` instance.
    pub fn begin_transaction(self: &Arc<Self>, isolation_level: IsolationLevel) -> Arc<MemTransaction> {
        // Allocate a new transaction ID and read timestamp.
        let txn_id = CommitTimestamp::new_txn_id();
        let start_ts = CommitTimestamp::new_commit_ts();

        // Register the transaction as active (used for garbage collection and visibility checks).
        let txn = Arc::new(
            MemTransaction::with_memgraph(self.clone(), txn_id, start_ts, isolation_level) 
        );
        self.txn_manager.register(txn.clone());
        txn
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
        let current_version = versioned_vertex.chain.current.read().unwrap();
        let commit_ts = current_version.commit_ts;
        let mut visible_vertex = current_version.data.clone();
        if let Some(undo_ptr) = *versioned_vertex.chain.undo_ptr.read().unwrap() {
            txn.apply_deltas_for_vertex(undo_ptr, &mut visible_vertex, txn.start_ts())?;
        } else {
            if commit_ts > txn.start_ts() {
                return Err(StorageError::VersionNotFound(
                    format!("Can't find a suitable version for this vertex with version {:?}", commit_ts),
                ));
            }
        }

        // Step 3: Record the vertex read set for conflict detection.
        if let IsolationLevel::Serializable  = txn.isolation_level() {
            txn.vertex_reads.insert(visible_vertex.vid());
        }

        // Step 4: Check for logical deletion.
        if visible_vertex.is_tombstone() {
            return Err(StorageError::VertexNotFound(vid.to_string()));
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
        let current_version = versioned_edge.chain.current.read().unwrap();
        let commit_ts = current_version.commit_ts;
        let mut visible_edge = current_version.data.clone();
        if let Some(undo_ptr) = *versioned_edge.chain.undo_ptr.read().unwrap() {
            txn.apply_deltas_for_edge(undo_ptr, &mut visible_edge, txn.start_ts())?;
        } else {
            if commit_ts > txn.start_ts() {
                return Err(StorageError::VersionNotFound(
                    format!("Can't find a suitable version for this edge with version {:?}", commit_ts),
                ));
            }
        }

        // Step 3: Record the edge read set for conflict detection.
        if let IsolationLevel::Serializable = txn.isolation_level() {
            txn.edge_reads.insert(eid);
        }

        // Step 4: Check for logical deletion (tombstone).
        if visible_edge.is_tombstone() {
            return Err(StorageError::EdgeNotFound(eid.to_string()));
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
        let entry = self.vertices.entry(vid.clone()).or_insert_with(|| 
            VersionedVertex::with_modified_ts(vertex, txn.txn_id())
        );

        // Conflict detection: Ensure the vertex does not exist
        let current = entry.chain.current.read().unwrap();
        if current.commit_ts != txn.txn_id() && current.commit_ts > txn.start_ts() {
            return Err(StorageError::VertexAlreadyExists(vid.to_string()));
        }

        // Record the vertex creation in the transaction
        let delta = DeltaOp::DelVertex(vid);
        let delta_ts = entry.chain.undo_ptr.read().unwrap();
        let mut undo_buffer = txn.undo_buffer.write().unwrap();
        undo_buffer.push(UndoEntry::new(delta, current.commit_ts, delta_ts.clone()));
        *entry.chain.undo_ptr.write().unwrap() = Some(UndoPtr::new(txn.txn_id(), undo_buffer.len() - 1));
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

        let entry = self.edges.entry(eid.clone()).or_insert_with(|| 
            VersionedEdge::with_modified_ts(edge, txn.txn_id())
        );

        // Conflict detection: Ensure the edge does not exist
        let current = entry.chain.current.read().unwrap();
        if current.commit_ts != txn.txn_id() && current.commit_ts > txn.start_ts() {
            return Err(StorageError::EdgeAlreadyExists(eid.to_string()));
        }

        // Record the edge creation in the transaction
        let delta = DeltaOp::DelEdge(eid);
        let delta_ts = entry.chain.undo_ptr.read().unwrap();
        let mut undo_buffer = txn.undo_buffer.write().unwrap();
        undo_buffer.push(UndoEntry::new(delta, current.commit_ts, delta_ts.clone()));
        *entry.chain.undo_ptr.write().unwrap() = Some(UndoPtr::new(txn.txn_id(), undo_buffer.len() - 1));

        Ok(eid)
    }

    /// Deletes a vertex from the graph within a transaction.
    fn delete_vertex(&self, txn: &MemTransaction, vid: VertexId) -> StorageResult<()> {
        // Atomically retrieve the versioned vertex (check existence).
        let entry = self
            .vertices
            .get(&vid)
            .ok_or(StorageError::VertexNotFound(vid.to_string()))?;

        delete_entity!(self, entry, txn, Vertex, CreateVertex, VertexNotFound);

        Ok(())
    }

    /// Deletes an edge from the graph within a transaction.
    fn delete_edge(&self, txn: &MemTransaction, eid: EdgeId) -> StorageResult<()> {
        // Atomically retrieve the versioned edge (check existence).
        let entry = self
            .edges
            .get(&eid)
            .ok_or(StorageError::EdgeNotFound(eid.to_string()))?;

        delete_entity!(self, entry, txn, Edge, CreateEdge, EdgeNotFound);

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

        update_properties!(self, vid, entry, txn, indices, props, SetVertexProps);

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

        update_properties!(self, eid, entry, txn, indices, props, SetEdgeProps);

        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use common::datatype::value::PropertyValue;
    use {Edge, Vertex};

    use super::*;
    use crate::{model::properties::PropertyStore, storage::StorageTransaction};

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
        let txn1 = graph.begin_transaction(IsolationLevel::Serializable);

        let v1 = create_vertex_alice();
        let vid1 = graph.create_vertex(&txn1, v1.clone()).unwrap();
        let _ = txn1.commit().unwrap();

        let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
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
            assert!(adj.len() == 1 && adj[0].end_ts == CommitTimestamp::max_commit_ts());
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
