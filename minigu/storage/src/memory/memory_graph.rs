use std::collections::LinkedList;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::u64;

use common::datatype::value::PropertyValue;
use dashmap::{DashMap, DashSet};

use crate::error::{StorageError, StorageResult};
use crate::model::vertex::Vertex;
use crate::storage::{Graph, MutGraph, StorageTransaction};
use crate::model::edge::{Direction, Edge};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TxnId(u64);

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

    fn inner(&self) -> &Vertex {
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

    fn inner(&self) -> &Edge {
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
struct VersionedVertex {
    versions: RwLock<LinkedList<VVertex>>,
}

impl VersionedVertex {
    fn new() -> Self {
        Self {
            versions: RwLock::new(LinkedList::new()),
        }
    }

    fn get_visible(&self, ts: TxnId) -> Option<VVertex> {
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
struct VersionedEdge {
    versions: RwLock<LinkedList<VEdge>>,
}

impl VersionedEdge {
    /// Get the latest visible version of the edge at or before the given timestamp
    fn new() -> Self {
        Self {
            versions: RwLock::new(LinkedList::new()),
        }
    }

    fn get_visible(&self, ts: TxnId) -> Option<VEdge> {
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
    vertices: DashMap<u64, Arc<VersionedVertex>>,
    // Edge table
    edges: DashMap<u64, Arc<VersionedEdge>>,
    // Adjacency list
    adjacency_forward: DashMap<u64, Vec<Adjacency>>,
    adjacency_reversed: DashMap<u64, Vec<Adjacency>>,
    // Allow only one writer for committing
    commit_mutex: Mutex<()>,
}

enum UpdateVertexOp {
    Delete,
    Insert(VVertex),
    SetProps((Vec<usize>, Vec<PropertyValue>))
}

enum UpdateEdgeOp {
    Delete,
    Insert(VEdge),
    SetProps((Vec<usize>, Vec<PropertyValue>))
}

/// MVCC Transaction instance
pub struct MvccTransaction {
    txn_id: TxnId,
    storage: Arc<MemGraph>,
    // Transaction read set
    vertex_read: DashSet<u64>,
    edge_read: DashSet<u64>,
    // Local transaction modifications
    vertex_updates: DashMap<u64, UpdateVertexOp>,
    edge_updates: DashMap<u64, UpdateEdgeOp>,
}

impl StorageTransaction for MvccTransaction {
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
                        Err(StorageError::VertexNotFound(format!("Vertex {} is not found", id)))
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
                    if let Some(mut v) = self.storage.adjacency_forward.get_mut(&e.source_id()) {
                        v.push(Adjacency::new(e.edge_id(), e.dst_id()));
                        v.sort_by_key(|s| s.vertex_id);
                    }
                    if let Some(mut v) = self.storage.adjacency_reversed.get_mut(&e.dst_id()) {
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
                        Err(StorageError::EdgeNotFound(format!("Edge {} is not found", id)))
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
    type AdjacencyIter = Box<dyn Iterator<Item = Adjacency>>;
    type Edge = Edge;
    type EdgeID = u64;
    type EdgeIter = Box<dyn Iterator<Item = Edge>>;
    type Transaction = MvccTransaction;
    type Vertex = Vertex;
    type VertexID = u64;
    type VertexIter = Box<dyn Iterator<Item = Vertex>>;

    fn get_vertex(&self, txn: &Self::Transaction, id: u64) -> StorageResult<Option<Vertex>> {
        // Check transaction local modification
        let mut set_props: Option<(Vec<usize>, Vec<PropertyValue>)> = None;
        if let Some(v) = txn.vertex_updates.get(&id) {
            match v.value() {
                UpdateVertexOp::Delete => return Ok(None),
                UpdateVertexOp::Insert(v) => return Ok(Some(v.inner().clone())),
                UpdateVertexOp::SetProps((indices, props)) => set_props = Some((indices.clone(), props.clone())),
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

    fn neighbors(
        &self,
        txn: &Self::Transaction,
        id: u64,
        direction: Direction,
    ) -> StorageResult<Self::AdjacencyIter> {
        // brute force version
        // clone the neighbors as a snapshot, and then remove or insert updated neighbors with the
        // txn_id

        // Get the global adjacency list
        let mut is_forward = true;
        let global_adjs = match direction {
            Direction::Out => &self.adjacency_forward.get(&id),
            Direction::In => {
                is_forward = false;
                &self.adjacency_reversed.get(&id)
            }
        };

        let mut adjs = match global_adjs {
            Some(adjs) => adjs.value().clone(),
            None => vec![],
        };
        for e in txn.edge_updates.iter() {
            match e.value() {
                UpdateEdgeOp::Delete => {
                    if let Ok(i) = adjs.binary_search_by(|v| v.edge_id.cmp(e.key())) {
                        adjs.remove(i);
                    }
                }
                UpdateEdgeOp::Insert(e) => {
                    if is_forward {
                        adjs.push(Adjacency::new(e.edge_id(), e.dst_id()))
                    } else {
                        adjs.push(Adjacency::new(e.edge_id(), e.source_id()))
                    }
                }
                UpdateEdgeOp::SetProps(_) => {}
            }
        }

        // Merge the neighbor results
        Ok(Box::new(adjs.into_iter()))
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

    fn vertices(&self, txn: &Self::Transaction) -> StorageResult<Self::VertexIter> {
        // brute force version

        let mut vertices = self
            .vertices
            .iter()
            .filter_map(|v| 
                v.value()
                .get_visible(txn.txn_id)
                .map(|v| v.inner().clone())
            )
            .collect::<Vec<_>>();

        for v_update in &txn.vertex_updates {
            match v_update.value() {
                UpdateVertexOp::Delete => {
                    if let Ok(i) = vertices.binary_search_by(|v| v.vid().cmp(v_update.key())) {
                        vertices.remove(i);
                    }
                }
                UpdateVertexOp::Insert(v) => vertices.push(v.inner().clone()),
                UpdateVertexOp::SetProps((indices, props)) => {
                    if let Ok(i) = vertices.binary_search_by(|v| v.vid().cmp(v_update.key())) {
                        vertices[i].set_props(indices, props.clone());
                    }
                }
            }
        }

        Ok(Box::new(vertices.into_iter()))
    }

    fn edges(&self, txn: &Self::Transaction) -> StorageResult<Self::EdgeIter> {
        // brute force version

        let mut edges = self
            .edges
            .iter()
            .filter_map(|e| 
                e.value()
                .get_visible(txn.txn_id)
                .map(|v| v.inner().clone())
            )
            .collect::<Vec<_>>();

        for e_update in &txn.edge_updates {
            match e_update.value() {
                UpdateEdgeOp::Delete => {
                    if let Ok(i) = edges.binary_search_by(|e| e.eid.cmp(e_update.key())) {
                        edges.remove(i);
                    }
                }
                UpdateEdgeOp::Insert(e) => edges.push(e.inner().clone()),
                UpdateEdgeOp::SetProps((indices, props)) => {
                    if let Ok(i) = edges.binary_search_by(|e| e.eid.cmp(e_update.key())) {
                        edges[i].set_props(indices, props.clone());
                    }
                }
            }
        }

        Ok(Box::new(edges.into_iter()))
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
                UpdateVertexOp::SetProps(_) |
                UpdateVertexOp::Insert(_) => {
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
                UpdateEdgeOp::SetProps(_) |
                UpdateEdgeOp::Insert(_) => {
                    return Err(StorageError::EdgeNotFound(format!(
                        "Edge {} has been inserted",
                        edge.edge_id(),
                    )));
                }
            };
        } else {
            txn.edge_updates.insert(edge.edge_id(), UpdateEdgeOp::Insert(edge));
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
    
    fn set_vertex_property(&self, txn: &Self::Transaction, vid: u64, indices: Vec<usize>, props: Vec<common::datatype::value::PropertyValue>) -> StorageResult<()> {
        txn.vertex_read.insert(vid);
        if let Some(mut v) = txn.vertex_updates.get_mut(&vid) {
            match v.value_mut() {
                UpdateVertexOp::Delete => return Err(StorageError::TransactionError(format!("Set the props of a deleted vertex {:}.", vid))),
                UpdateVertexOp::Insert(vvertex) => vvertex.inner_mut().set_props(&indices, props),
                UpdateVertexOp::SetProps((pindices, pprops)) => {
                    *pindices = indices;
                    *pprops = props;
                }
            }
        } else {
            txn.vertex_updates.insert(vid, UpdateVertexOp::SetProps((indices, props)));
        }

        Ok(())
    }
    
    fn set_edge_propoerty(&self, txn: &Self::Transaction, eid: u64, indices: Vec<usize>, props: Vec<common::datatype::value::PropertyValue>) -> StorageResult<()> {
        txn.edge_read.insert(eid);
        if let Some(mut e) = txn.edge_updates.get_mut(&eid) {
            match e.value_mut() {
                UpdateEdgeOp::Delete => return Err(StorageError::TransactionError(format!("Set the props of a deleted edge {:}.", eid))),
                UpdateEdgeOp::Insert(vedge) => vedge.inner_mut().set_props(&indices, props),
                UpdateEdgeOp::SetProps((pindices, pprops)) => {
                    *pindices = indices;
                    *pprops = props;
                }
            }
        } else {
            txn.edge_updates.insert(eid, UpdateEdgeOp::SetProps((indices, props)));
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
            adjacency_forward: DashMap::new(),
            adjacency_reversed: DashMap::new(),
            commit_mutex: Mutex::new(()),
        })
    }

    pub fn begin_transaction(self: &Arc<Self>) -> MvccTransaction {
        let tx_id = TxnId(self.tx_id_counter.fetch_add(1, Ordering::SeqCst));
        MvccTransaction {
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
    use crate::model::properties::PropertyStore;

    use super::*;

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

        let edge = Edge::new(1, 1, 2, 0, Direction::Out, PropertyStore::new(Vec::new()));

        storage
            .create_vertex(&txn, Vertex::new(1, 0, PropertyStore::new(Vec::new())))
            .unwrap();
        storage
            .create_vertex(&txn, Vertex::new(2, 0, PropertyStore::new(Vec::new())))
            .unwrap();
        storage.create_edge(&txn, edge).unwrap();
        let neighbors = storage
            .neighbors(&txn, 1, Direction::Out)
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors[0].vertex_id, 2);
        let neighbors = storage
            .neighbors(&txn, 2, Direction::In)
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors[0].vertex_id, 1);
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
        storage.delete_vertices(&txn3, vec![Vertex::new(1, 0, PropertyStore::new(Vec::new()))]).unwrap();
        txn3.commit().unwrap();

        // Commit transaction 2 should fail due to conflict
        let result = txn2.commit();
        assert!(result.is_err());
    }
}
