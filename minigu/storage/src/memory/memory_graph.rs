use std::collections::LinkedList;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use common::datatype::value::PropertyValue;
use dashmap::{DashMap, DashSet};

use super::memory_iters::{MemAdjIter, MemEdgeIter, MemVertexIter};
use crate::error::{StorageError, StorageResult};
use crate::model::edge::{Direction, Edge};
use crate::model::vertex::Vertex;
use crate::storage::{Graph, MutGraph, StorageTransaction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct TxnId(u64);

impl TxnId {
    fn is_updated(&self) -> bool {
        self.0 != u64::MAX
    }

    fn empty() -> Self {
        Self(0)
    }

    fn max() -> Self {
        Self(u64::MAX)
    }
}

/// Vertex
#[derive(Clone, Debug)]
pub struct VVertex {
    // Transaction information of the versioned vertex
    begin_ts: TxnId,
    end_ts: TxnId,
    // The internal vertex data
    inner: Vertex,
}

impl VVertex {
    fn new(vertex: Vertex) -> Self {
        Self {
            begin_ts: TxnId::empty(),
            end_ts: TxnId::empty(),
            inner: vertex,
        }
    }

    fn vid(&self) -> u64 {
        self.inner.vid()
    }

    pub fn inner(&self) -> &Vertex {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Vertex {
        &mut self.inner
    }
}

impl PartialEq for VVertex {
    fn eq(&self, other: &Self) -> bool {
        self.inner.vid() == other.inner.vid()
    }
}

/// Edge
#[derive(Clone, Debug)]
pub struct VEdge {
    // Transaction information of the versioned edges
    begin_ts: TxnId,
    end_ts: TxnId,
    // The internal edge data
    inner: Edge,
}

impl VEdge {
    fn new(edge: Edge) -> Self {
        Self {
            begin_ts: TxnId::empty(),
            end_ts: TxnId::empty(),
            inner: edge,
        }
    }

    fn edge_id(&self) -> u64 {
        self.inner.eid
    }

    fn dst_id(&self) -> u64 {
        self.inner.dst_id
    }

    fn source_id(&self) -> u64 {
        self.inner.source_id
    }

    pub(super) fn inner(&self) -> &Edge {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Edge {
        &mut self.inner
    }
}

/// Adjacency
#[derive(Clone, Debug)]
pub struct Adjacency {
    pub edge_id: u64,
    pub vertex_id: u64,
}

impl Adjacency {
    fn new(eid: u64, vid: u64) -> Self {
        Self {
            edge_id: eid,
            vertex_id: vid,
        }
    }
}

/// Vertex with version information
#[derive(Debug)]
pub(crate) struct VersionedVertex {
    versions: RwLock<LinkedList<VVertex>>,
}

impl VersionedVertex {
    fn new() -> Self {
        Self {
            versions: RwLock::new(LinkedList::new()),
        }
    }

    pub fn get_visible(&self, ts: TxnId) -> Option<VVertex> {
        self.versions
            .read()
            .unwrap()
            .iter()
            .find(|v| v.begin_ts <= ts && ts <= v.end_ts)
            .cloned()
    }

    fn front(&self) -> Option<VVertex> {
        self.versions.read().unwrap().front().cloned()
    }

    fn visit_front<T: FnOnce(&mut VVertex)>(&self, visitor: T) -> StorageResult<()> {
        match self.versions.write().unwrap().front_mut().map(visitor) {
            Some(()) => Ok(()),
            None => Err(StorageError::VertexNotFound(
                "Vertex {}is not found".to_string(),
            )),
        }
    }

    fn insert_version(&self, version: VVertex) -> StorageResult<()> {
        self.versions.write().unwrap().push_front(version);
        Ok(())
    }
}

/// Edge with version information
#[derive(Debug)]
pub(super) struct VersionedEdge {
    versions: RwLock<LinkedList<VEdge>>,
}

impl VersionedEdge {
    /// Get the latest visible version of the edge at or before the given timestamp
    fn new() -> Self {
        Self {
            versions: RwLock::new(LinkedList::new()),
        }
    }

    pub(super) fn get_visible(&self, ts: TxnId) -> Option<VEdge> {
        self.versions
            .read()
            .unwrap()
            .iter()
            .find(|v| v.begin_ts <= ts && ts <= v.end_ts)
            .cloned()
    }

    fn front(&self) -> Option<VEdge> {
        self.versions.read().unwrap().front().cloned()
    }

    fn visit_front<T: FnOnce(&mut VEdge)>(&self, visitor: T) -> StorageResult<()> {
        match self.versions.write().unwrap().front_mut().map(visitor) {
            Some(()) => Ok(()),
            None => Err(StorageError::VertexNotFound(
                "Vertex is not found".to_string(),
            )),
        }
    }

    fn insert_version(&self, version: VEdge) -> StorageResult<()> {
        self.versions.write().unwrap().push_front(version);
        Ok(())
    }
}

/// MVCC Transaction
pub struct MemGraph {
    // TransactionID which represents the latest timestamp
    tx_id_counter: AtomicU64,
    // Vertex table
    pub(super) vertices: DashMap<u64, Arc<VersionedVertex>>,
    // Edge table
    pub(super) edges: DashMap<u64, Arc<VersionedEdge>>,
    // Adjacency list
    pub(super) adjacency_out: DashMap<u64, Vec<Adjacency>>,
    pub(super) adjacency_in: DashMap<u64, Vec<Adjacency>>,
    // Allow only one writer for committing
    commit_mutex: Mutex<()>,
}

pub(super) enum UpdateVertexOp {
    Delete,
    Insert(VVertex),
    SetProps((Vec<usize>, Vec<PropertyValue>)),
}

pub(super) enum UpdateEdgeOp {
    Delete,
    Insert(VEdge),
    SetProps((Vec<usize>, Vec<PropertyValue>)),
}

/// MVCC Transaction instance
pub struct MemTransaction {
    txn_id: TxnId,
    pub(super) storage: Arc<MemGraph>,
    // Transaction read set
    pub(super) vertex_read: DashSet<u64>,
    pub(super) edge_read: DashSet<u64>,
    // Local transaction modifications
    pub(super) vertex_updates: DashMap<u64, UpdateVertexOp>,
    pub(super) edge_updates: DashMap<u64, UpdateEdgeOp>,
}

impl MemTransaction {
    #[allow(dead_code)]
    pub(super) fn id(&self) -> TxnId {
        self.txn_id
    }
}

impl StorageTransaction for MemTransaction {
    fn commit(self) -> StorageResult<()> {
        // Acquiring the commit lock
        let _commit_guard = self.storage.commit_mutex.lock().unwrap();

        // Validate the read-write conflicts
        for vid in self.vertex_read.into_iter() {
            if let Some(rv) = self.storage.vertices.get(&vid) {
                if let Some(vv) = rv.get_visible(self.txn_id) {
                    if vv.end_ts.is_updated() {
                        return Err(StorageError::TransactionError(format!(
                            "Read-write conflict happens for vertex {} and version {:?}",
                            vid, self.txn_id
                        )));
                    } else {
                        // without conflicts
                    }
                } else {
                    return Err(StorageError::VertexNotFound(format!(
                        "Vertex {} with timestamp {:?} is invisible",
                        vid, self.txn_id
                    )));
                }
            }
        }
        for eid in self.edge_read.into_iter() {
            if let Some(re) = self.storage.edges.get(&eid) {
                if let Some(ve) = re.get_visible(self.txn_id) {
                    if ve.end_ts.is_updated() {
                        return Err(StorageError::EdgeNotFound(format!(
                            "Read-write conflict happens for edge {} and version {:?}",
                            eid, self.txn_id
                        )));
                    }
                } else {
                    // without conflicts
                }
            } else {
                return Err(StorageError::VertexNotFound(format!(
                    "Edge {} with timestamp {:?} is invisible",
                    eid, self.txn_id
                )));
            }
        }

        // Acquire the commit timestamp to perform the modifications
        let commit_ts = self.storage.acquire_commit_ts();

        for (id, op) in self.vertex_updates.into_iter() {
            let entry = self
                .storage
                .vertices
                .entry(id)
                .or_insert_with(|| Arc::new(VersionedVertex::new()));
            match op {
                UpdateVertexOp::Delete => entry.visit_front(|v| v.end_ts = commit_ts),
                UpdateVertexOp::Insert(v) => entry.insert_version(v),
                UpdateVertexOp::SetProps((indices, props)) => {
                    if let Some(mut vertex) = entry.front() {
                        vertex.inner_mut().set_props(&indices, props);
                        entry.insert_version(vertex)
                    } else {
                        Err(StorageError::VertexNotFound(format!(
                            "Vertex {} is not found",
                            id
                        )))
                    }
                }
            }?;
        }

        for (id, op) in self.edge_updates.into_iter() {
            let entry = self
                .storage
                .edges
                .entry(id)
                .or_insert_with(|| Arc::new(VersionedEdge::new()));
            match op {
                UpdateEdgeOp::Delete => entry.visit_front(|v| v.end_ts = commit_ts),
                UpdateEdgeOp::Insert(e) => {
                    if let Some(mut v) = self.storage.adjacency_out.get_mut(&e.source_id()) {
                        v.push(Adjacency::new(e.edge_id(), e.dst_id()));
                        v.sort_by_key(|s| s.vertex_id);
                    }
                    if let Some(mut v) = self.storage.adjacency_in.get_mut(&e.dst_id()) {
                        v.push(Adjacency::new(e.edge_id(), e.source_id()));
                        v.sort_by_key(|s| s.vertex_id);
                    }
                    entry.insert_version(e)
                }
                UpdateEdgeOp::SetProps((indices, props)) => {
                    if let Some(mut edge) = entry.front() {
                        edge.inner_mut().set_props(&indices, props);
                        entry.insert_version(edge)
                    } else {
                        Err(StorageError::EdgeNotFound(format!(
                            "Edge {} is not found",
                            id
                        )))
                    }
                }
            }?;
        }

        Ok(())
    }

    /// Drop the transaction content
    fn abort(self) -> StorageResult<()> {
        Ok(())
    }
}

impl Graph for MemGraph {
    type Adjacency = Adjacency;
    type AdjacencyIter<'a> = MemAdjIter<'a>;
    type Edge = Edge;
    type EdgeID = u64;
    type EdgeIter<'a> = MemEdgeIter<'a>;
    type Transaction = MemTransaction;
    type Vertex = Vertex;
    type VertexID = u64;
    type VertexIter<'a> = MemVertexIter<'a>;

    fn get_vertex(&self, txn: &Self::Transaction, id: u64) -> StorageResult<Option<Vertex>> {
        // Check transaction local modification
        let mut set_props: Option<(Vec<usize>, Vec<PropertyValue>)> = None;
        if let Some(v) = txn.vertex_updates.get(&id) {
            match v.value() {
                UpdateVertexOp::Delete => return Ok(None),
                UpdateVertexOp::Insert(v) => return Ok(Some(v.inner().clone())),
                UpdateVertexOp::SetProps((indices, props)) => {
                    set_props = Some((indices.clone(), props.clone()))
                }
            };
        }

        // Check global graph data
        if let Some(v) = self.vertices.get(&id) {
            txn.vertex_read.insert(id);
            if let Some((indices, props)) = set_props {
                Ok(v.get_visible(txn.txn_id).map(|v| {
                    let mut vertex = v.inner().clone();
                    vertex.set_props(&indices, props);
                    vertex
                }))
            } else {
                Ok(v.get_visible(txn.txn_id).map(|v| v.inner().clone()))
            }
        } else {
            Err(StorageError::VertexNotFound(format!(
                "Vertex {} is not found",
                id
            )))
        }
    }

    fn neighbors<'a>(
        &self,
        txn: &'a Self::Transaction,
        id: u64,
        direction: Direction,
    ) -> StorageResult<Self::AdjacencyIter<'a>> {
        MemAdjIter::new(txn, id, direction)
    }

    fn get_edge(
        &self,
        txn: &Self::Transaction,
        id: Self::EdgeID,
    ) -> StorageResult<Option<Self::Edge>> {
        // 1. Check transaction-local modifications
        if let Some(edge) = txn.edge_updates.get(&id) {
            return Ok(match edge.value() {
                UpdateEdgeOp::Delete => None,
                UpdateEdgeOp::Insert(e) => Some(e.inner().clone()),
                UpdateEdgeOp::SetProps(_) => todo!(),
            });
        }

        // 2. Check global graph data
        if let Some(e) = self.edges.get(&id) {
            txn.edge_read.insert(id);
            Ok(e.get_visible(txn.txn_id).map(|v| v.inner().clone()))
        } else {
            Err(StorageError::EdgeNotFound(format!(
                "Edge {} is not found",
                id
            )))
        }
    }

    fn vertices<'a>(&self, txn: &'a Self::Transaction) -> StorageResult<Self::VertexIter<'a>> {
        Ok(MemVertexIter::new(txn))
    }

    fn edges<'a>(&self, txn: &'a Self::Transaction) -> StorageResult<Self::EdgeIter<'a>> {
        Ok(MemEdgeIter::new(txn))
    }
}

impl MutGraph for MemGraph {
    fn create_vertex(&self, txn: &Self::Transaction, vertex: Vertex) -> StorageResult<()> {
        let mut vertex = VVertex::new(vertex);
        vertex.begin_ts = txn.txn_id;
        vertex.end_ts = TxnId::max();

        if let Some(txn_v) = txn.vertex_updates.get(&vertex.vid()) {
            match txn_v.value() {
                UpdateVertexOp::Delete => {
                    txn.vertex_updates.entry(vertex.vid()).and_modify(|v| {
                        *v = UpdateVertexOp::Insert(vertex);
                    });
                }
                UpdateVertexOp::SetProps(_) | UpdateVertexOp::Insert(_) => {
                    return Err(StorageError::TransactionError(format!(
                        "Vertex {} has been inserted",
                        vertex.vid()
                    )));
                }
            };
        } else {
            txn.vertex_updates
                .insert(vertex.vid(), UpdateVertexOp::Insert(vertex));
        }

        Ok(())
    }

    fn create_edge(&self, txn: &Self::Transaction, edge: Edge) -> StorageResult<()> {
        let mut edge = VEdge::new(edge);
        edge.begin_ts = txn.txn_id;
        edge.end_ts = TxnId::max();

        if let Some(txn_v) = txn.edge_updates.get(&edge.edge_id()) {
            match txn_v.value() {
                UpdateEdgeOp::Delete => {
                    txn.edge_updates.entry(edge.edge_id()).and_modify(|e| {
                        *e = UpdateEdgeOp::Insert(edge);
                    });
                }
                UpdateEdgeOp::SetProps(_) | UpdateEdgeOp::Insert(_) => {
                    return Err(StorageError::EdgeNotFound(format!(
                        "Edge {} has been inserted",
                        edge.edge_id(),
                    )));
                }
            };
        } else {
            txn.edge_updates
                .insert(edge.edge_id(), UpdateEdgeOp::Insert(edge));
        }

        Ok(())
    }

    fn delete_vertices(
        &self,
        txn: &Self::Transaction,
        vertices: Vec<Self::Vertex>,
    ) -> StorageResult<()> {
        for v in vertices {
            txn.vertex_updates
                .entry(v.vid())
                .and_modify(|v| *v = UpdateVertexOp::Delete)
                .or_insert(UpdateVertexOp::Delete);
        }

        Ok(())
    }

    fn delete_edges(&self, txn: &Self::Transaction, edges: Vec<Self::Edge>) -> StorageResult<()> {
        for e in edges {
            txn.edge_updates
                .entry(e.eid)
                .and_modify(|v| *v = UpdateEdgeOp::Delete)
                .or_insert(UpdateEdgeOp::Delete);
        }

        Ok(())
    }

    fn set_vertex_property(
        &self,
        txn: &Self::Transaction,
        vid: u64,
        indices: Vec<usize>,
        props: Vec<common::datatype::value::PropertyValue>,
    ) -> StorageResult<()> {
        txn.vertex_read.insert(vid);
        if let Some(mut v) = txn.vertex_updates.get_mut(&vid) {
            match v.value_mut() {
                UpdateVertexOp::Delete => {
                    return Err(StorageError::TransactionError(format!(
                        "Set the props of a deleted vertex {:}.",
                        vid
                    )));
                }
                UpdateVertexOp::Insert(vvertex) => vvertex.inner_mut().set_props(&indices, props),
                UpdateVertexOp::SetProps((pindices, pprops)) => {
                    *pindices = indices;
                    *pprops = props;
                }
            }
        } else {
            txn.vertex_updates
                .insert(vid, UpdateVertexOp::SetProps((indices, props)));
        }

        Ok(())
    }

    fn set_edge_property(
        &self,
        txn: &Self::Transaction,
        eid: u64,
        indices: Vec<usize>,
        props: Vec<common::datatype::value::PropertyValue>,
    ) -> StorageResult<()> {
        txn.edge_read.insert(eid);
        if let Some(mut e) = txn.edge_updates.get_mut(&eid) {
            match e.value_mut() {
                UpdateEdgeOp::Delete => {
                    return Err(StorageError::TransactionError(format!(
                        "Set the props of a deleted edge {:}.",
                        eid
                    )));
                }
                UpdateEdgeOp::Insert(vedge) => vedge.inner_mut().set_props(&indices, props),
                UpdateEdgeOp::SetProps((pindices, pprops)) => {
                    *pindices = indices;
                    *pprops = props;
                }
            }
        } else {
            txn.edge_updates
                .insert(eid, UpdateEdgeOp::SetProps((indices, props)));
        }

        Ok(())
    }
}

impl MemGraph {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            tx_id_counter: AtomicU64::new(1),
            vertices: DashMap::new(),
            edges: DashMap::new(),
            adjacency_out: DashMap::new(),
            adjacency_in: DashMap::new(),
            commit_mutex: Mutex::new(()),
        })
    }

    pub fn begin_transaction(self: &Arc<Self>) -> MemTransaction {
        let tx_id = TxnId(self.tx_id_counter.fetch_add(1, Ordering::SeqCst));
        MemTransaction {
            txn_id: tx_id,
            storage: self.clone(),
            vertex_read: DashSet::new(),
            edge_read: DashSet::new(),
            vertex_updates: DashMap::new(),
            edge_updates: DashMap::new(),
        }
    }

    fn acquire_commit_ts(&self) -> TxnId {
        TxnId(self.tx_id_counter.fetch_add(1, Ordering::SeqCst))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iterators::{EdgeIterator, VertexIterator};
    use crate::model::properties::PropertyStore;

    #[test]
    fn test_create_and_get_vertex() {
        let storage = MemGraph::new();
        let txn = storage.begin_transaction();

        let vertex = Vertex::new(1, 0, PropertyStore::new(Vec::new()));

        storage.create_vertex(&txn, vertex.clone()).unwrap();
        let fetched_vertex = storage.get_vertex(&txn, 1).unwrap().unwrap();
        assert_eq!(fetched_vertex.vid(), vertex.vid());
    }

    #[test]
    fn test_create_and_get_edge() {
        let storage = MemGraph::new();
        let txn = storage.begin_transaction();

        let edge = Edge::new(1, 1, 2, 0, Direction::Out, PropertyStore::new(Vec::new()));

        storage.create_edge(&txn, edge.clone()).unwrap();
        let fetched_edge = storage.get_edge(&txn, 1).unwrap().unwrap();
        assert_eq!(fetched_edge.eid, edge.eid);
    }

    #[test]
    fn test_delete_vertex() {
        let storage = MemGraph::new();
        let txn = storage.begin_transaction();

        let vertex = Vertex::new(1, 0, PropertyStore::new(Vec::new()));

        storage.create_vertex(&txn, vertex.clone()).unwrap();
        storage.delete_vertices(&txn, vec![vertex.clone()]).unwrap();
        let fetched_vertex = storage.get_vertex(&txn, 1).unwrap();
        assert!(fetched_vertex.is_none());
    }

    #[test]
    fn test_delete_edge() {
        let storage = MemGraph::new();
        let txn = storage.begin_transaction();

        let edge = Edge::new(1, 1, 2, 0, Direction::Out, PropertyStore::new(Vec::new()));

        storage.create_edge(&txn, edge.clone()).unwrap();
        storage.delete_edges(&txn, vec![edge.clone()]).unwrap();
        let fetched_edge = storage.get_edge(&txn, 1).unwrap();
        assert!(fetched_edge.is_none());
    }

    #[test]
    fn test_neighbors() {
        let storage = MemGraph::new();
        let txn = storage.begin_transaction();

        // Create vertices
        let vertex1 = Vertex::new(1, 0, PropertyStore::new(Vec::new()));
        let vertex2 = Vertex::new(2, 0, PropertyStore::new(Vec::new()));
        storage.create_vertex(&txn, vertex1).unwrap();
        storage.create_vertex(&txn, vertex2).unwrap();

        // Create edge from vertex1 to vertex2
        let edge = Edge::new(1, 1, 2, 0, Direction::Out, PropertyStore::new(Vec::new()));
        storage.create_edge(&txn, edge.clone()).unwrap();

        // Test outgoing neighbors of vertex1
        let mut out_neighbors = storage.neighbors(&txn, 1, Direction::Out).unwrap();
        out_neighbors.next().unwrap();
        assert_eq!(out_neighbors.edge().unwrap().eid, 1, "Edge ID should match");
        assert_eq!(
            out_neighbors.edge().unwrap().dst_id,
            2,
            "Neighbor vertex ID should be 2"
        );
        out_neighbors.next().unwrap();
        assert!(out_neighbors.edge().is_none(), "No more neighbors");

        // Test incoming neighbors of vertex2
        let mut in_neighbors = storage.neighbors(&txn, 2, Direction::In).unwrap();
        in_neighbors.next().unwrap();
        assert_eq!(in_neighbors.edge().unwrap().eid, 1, "Edge ID should match");
        assert_eq!(
            in_neighbors.edge().unwrap().source_id,
            1,
            "Neighbor vertex ID should be 1"
        );

        // Test no outgoing neighbors for vertex2
        let mut no_out_neighbors = storage.neighbors(&txn, 2, Direction::Out).unwrap();
        no_out_neighbors.next().unwrap();
        assert!(
            no_out_neighbors.edge().is_none(),
            "Vertex2 should have no outgoing neighbors"
        );

        // Test after deleting the edge
        storage.delete_edges(&txn, vec![edge]).unwrap();
        let mut neighbors_after_delete = storage.neighbors(&txn, 1, Direction::Out).unwrap();
        neighbors_after_delete.next().unwrap();
        assert!(
            neighbors_after_delete.edge().is_none(),
            "No neighbors should exist after edge deletion"
        );
    }

    #[test]
    fn test_transaction_commit() {
        let storage = MemGraph::new();
        let txn = storage.begin_transaction();

        let vertex = Vertex::new(1, 0, PropertyStore::new(Vec::new()));

        storage.create_vertex(&txn, vertex.clone()).unwrap();
        txn.commit().unwrap();

        let new_txn = storage.begin_transaction();
        let fetched_vertex = storage.get_vertex(&new_txn, 1).unwrap().unwrap();
        assert_eq!(fetched_vertex.vid(), vertex.vid());
    }

    #[test]
    fn test_multi_transaction_commit_success() {
        let storage = MemGraph::new();

        // Transaction 1 creates a vertex
        let txn1 = storage.begin_transaction();
        let vertex1 = Vertex::new(1, 0, PropertyStore::new(Vec::new()));
        storage.create_vertex(&txn1, vertex1.clone()).unwrap();
        txn1.commit().unwrap();

        // Transaction 2 creates another vertex
        let txn2 = storage.begin_transaction();
        let vertex2 = Vertex::new(2, 0, PropertyStore::new(Vec::new()));
        storage.create_vertex(&txn2, vertex2.clone()).unwrap();
        txn2.commit().unwrap();

        // Verify both vertices are present
        let txn3 = storage.begin_transaction();
        let fetched_vertex1 = storage.get_vertex(&txn3, 1).unwrap().unwrap();
        let fetched_vertex2 = storage.get_vertex(&txn3, 2).unwrap().unwrap();
        assert_eq!(fetched_vertex1.vid(), vertex1.vid());
        assert_eq!(fetched_vertex2.vid(), vertex2.vid());
    }

    #[test]
    fn test_multi_transaction_commit_conflict() {
        let storage = MemGraph::new();

        // Transaction 1 creates a vertex
        let txn1 = storage.begin_transaction();
        let vertex1 = Vertex::new(1, 0, PropertyStore::new(Vec::new()));
        storage.create_vertex(&txn1, vertex1.clone()).unwrap();
        txn1.commit().unwrap();

        // Transaction 2 tries to read the vertex
        let txn2 = storage.begin_transaction();
        storage.get_vertex(&txn2, 1).unwrap();

        // Transaction 3 delete the vertex
        let txn3 = storage.begin_transaction();
        storage
            .delete_vertices(&txn3, vec![Vertex::new(
                1,
                0,
                PropertyStore::new(Vec::new()),
            )])
            .unwrap();
        txn3.commit().unwrap();

        // Commit transaction 2 should fail due to conflict
        let result = txn2.commit();
        assert!(result.is_err());
    }

    #[test]
    fn test_vertex_iterator() {
        let storage = MemGraph::new();
        let txn = storage.begin_transaction();

        // Test empty graph
        let mut empty_iter = storage.vertices(&txn).unwrap();
        empty_iter.next().unwrap();
        assert!(
            empty_iter.vertex().is_none(),
            "Empty graph should have no vertices"
        );

        // Create multiple vertices
        let vertex1 = Vertex::new(1, 0, PropertyStore::new(Vec::new()));
        let vertex2 = Vertex::new(2, 0, PropertyStore::new(Vec::new()));
        let vertex3 = Vertex::new(3, 0, PropertyStore::new(Vec::new()));

        storage.create_vertex(&txn, vertex1.clone()).unwrap();
        storage.create_vertex(&txn, vertex2.clone()).unwrap();
        storage.create_vertex(&txn, vertex3.clone()).unwrap();

        // Test vertex iteration
        let mut vertex_iter = storage.vertices(&txn).unwrap();
        let mut vertex_count = 0;
        let mut found_vids = Vec::new();
        vertex_iter.next().unwrap();
        while vertex_iter.vertex().is_some() {
            if let Some(vertex) = vertex_iter.vertex() {
                found_vids.push(vertex.vid());
                vertex_count += 1;
            }
            vertex_iter.next().unwrap();
        }

        assert_eq!(vertex_count, 3, "Should find all three vertices");
        assert!(found_vids.contains(&1));
        assert!(found_vids.contains(&2));
        assert!(found_vids.contains(&3));

        // Test iteration after deletion
        storage
            .delete_vertices(&txn, vec![vertex2.clone()])
            .unwrap();

        let mut after_delete_iter = storage.vertices(&txn).unwrap();
        let mut after_delete_count = 0;
        let mut after_delete_vids = Vec::new();
        after_delete_iter.next().unwrap();
        while after_delete_iter.vertex().is_some() {
            if let Some(vertex) = after_delete_iter.vertex() {
                after_delete_vids.push(vertex.vid());
                after_delete_count += 1;
            }
            after_delete_iter.next().unwrap();
        }

        assert_eq!(
            after_delete_count, 2,
            "Should find two vertices after deletion"
        );
        assert!(after_delete_vids.contains(&1));
        assert!(after_delete_vids.contains(&3));
        assert!(!after_delete_vids.contains(&2));
    }

    #[test]
    fn test_edge_iterator() {
        let storage = MemGraph::new();
        let txn = storage.begin_transaction();

        // Test empty graph
        let mut empty_iter = storage.edges(&txn).unwrap();
        empty_iter.next().unwrap();
        assert!(
            empty_iter.edge().is_none(),
            "Empty graph should have no edges"
        );

        // Create vertices and edges
        let vertex1 = Vertex::new(1, 0, PropertyStore::new(Vec::new()));
        let vertex2 = Vertex::new(2, 0, PropertyStore::new(Vec::new()));
        let vertex3 = Vertex::new(3, 0, PropertyStore::new(Vec::new()));

        storage.create_vertex(&txn, vertex1.clone()).unwrap();
        storage.create_vertex(&txn, vertex2.clone()).unwrap();
        storage.create_vertex(&txn, vertex3.clone()).unwrap();

        let edge1 = Edge::new(1, 1, 2, 0, Direction::Out, PropertyStore::new(Vec::new()));
        let edge2 = Edge::new(2, 2, 3, 0, Direction::Out, PropertyStore::new(Vec::new()));
        let edge3 = Edge::new(3, 1, 3, 0, Direction::Out, PropertyStore::new(Vec::new()));

        storage.create_edge(&txn, edge1.clone()).unwrap();
        storage.create_edge(&txn, edge2.clone()).unwrap();
        storage.create_edge(&txn, edge3.clone()).unwrap();

        // Test edge iteration
        let mut edge_iter = storage.edges(&txn).unwrap();
        let mut edge_count = 0;
        let mut found_eids = Vec::new();

        edge_iter.next().unwrap();
        while edge_iter.edge().is_some() {
            if let Some(edge) = edge_iter.edge() {
                found_eids.push(edge.eid);
                edge_count += 1;
            }
            edge_iter.next().unwrap();
        }

        assert_eq!(edge_count, 3, "Should find all three edges");
        assert!(found_eids.contains(&1));
        assert!(found_eids.contains(&2));
        assert!(found_eids.contains(&3));

        // Test iteration after deletion
        storage.delete_edges(&txn, vec![edge2.clone()]).unwrap();

        let mut after_delete_iter = storage.edges(&txn).unwrap();
        let mut after_delete_count = 0;
        let mut after_delete_eids = Vec::new();
        after_delete_iter.next().unwrap();
        while after_delete_iter.edge().is_some() {
            if let Some(edge) = after_delete_iter.edge() {
                after_delete_eids.push(edge.eid);
                after_delete_count += 1;
            }
            after_delete_iter.next().unwrap();
        }

        assert_eq!(
            after_delete_count, 2,
            "Should find two edges after deletion"
        );
        assert!(after_delete_eids.contains(&1));
        assert!(after_delete_eids.contains(&3));
        assert!(!after_delete_eids.contains(&2));
    }
}
