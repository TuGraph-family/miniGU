use std::collections::LinkedList;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use dashmap::{DashMap, DashSet};

use crate::error::{StorageError, StorageResult};
use crate::storage::{Direction, Graph, MutGraph, StorageTransaction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TxnId(u64);

/// Vertex
#[derive(Clone, Debug, PartialEq)]
pub struct Vertex {
    // Transaction information of the versioned vertex
    begin_ts: TxnId,
    end_ts: TxnId,
    // The internal vertex data
    pub id: u64,
}

/// Edge
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Edge {
    // Transaction information of the versioned edges
    begin_ts: TxnId,
    end_ts: TxnId,
    // The internal edge data
    id: u64,
    from: u64,
    to: u64,
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
    versions: RwLock<LinkedList<Vertex>>,
}

impl VersionedVertex {
    fn new() -> Self {
        Self {
            versions: RwLock::new(LinkedList::new()),
        }
    }

    fn get_visible(&self, ts: TxnId) -> Option<Vertex> {
        self.versions
            .read()
            .unwrap()
            .iter()
            .find(|v| v.begin_ts <= ts && ts <= v.end_ts)
            .cloned()
    }

    fn visit_front<T: FnOnce(&mut Vertex)>(&self, visitor: T) -> StorageResult<()> {
        match self.versions.write().unwrap().front_mut().map(visitor) {
            Some(()) => Ok(()),
            None => Err(StorageError::VertexNotFound(
                "Vertex {}is not found".to_string(),
            )),
        }
    }

    fn insert_version(&self, version: Vertex) -> StorageResult<()> {
        self.versions.write().unwrap().push_front(version);
        Ok(())
    }
}

/// Edge with version information
#[derive(Debug)]
struct VersionedEdge {
    versions: RwLock<LinkedList<Edge>>,
}

impl VersionedEdge {
    /// Get the latest visible version of the edge at or before the given timestamp
    fn new() -> Self {
        Self {
            versions: RwLock::new(LinkedList::new()),
        }
    }

    fn get_visible(&self, ts: TxnId) -> Option<Edge> {
        self.versions
            .read()
            .unwrap()
            .iter()
            .find(|v| v.begin_ts <= ts && ts <= v.end_ts)
            .cloned()
    }

    fn visit_front<T: FnOnce(&mut Edge)>(&self, visitor: T) -> StorageResult<()> {
        match self.versions.write().unwrap().front_mut().map(visitor) {
            Some(()) => Ok(()),
            None => Err(StorageError::VertexNotFound(
                "Vertex is not found".to_string(),
            )),
        }
    }

    fn insert_version(&self, version: Edge) {
        self.versions.write().unwrap().push_front(version);
    }
}

/// MVCC Transaction
pub struct MvccGraphStorage {
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
    Insert(Vertex),
}

enum UpdateEdgeOp {
    Delete,
    Insert(Edge),
}

/// MVCC Transaction instance
pub struct MvccTransaction {
    txn_id: TxnId,
    storage: Arc<MvccGraphStorage>,
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
                    if vv.end_ts < self.txn_id {
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
                    if ve.end_ts < self.txn_id {
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
            }?;
        }

        for (id, op) in self.edge_updates.into_iter() {
            let entry = self
                .storage
                .edges
                .entry(id)
                .or_insert_with(|| Arc::new(VersionedEdge::new()));
            match op {
                UpdateEdgeOp::Delete => entry.visit_front(|v| v.end_ts = commit_ts).unwrap(),
                UpdateEdgeOp::Insert(e) => {
                    entry.insert_version(e);
                    if let Some(mut v) = self.storage.adjacency_forward.get_mut(&e.from) {
                        v.push(Adjacency::new(e.id, e.to));
                        v.sort_by_key(|s| s.vertex_id);
                    }
                    if let Some(mut v) = self.storage.adjacency_reversed.get_mut(&e.to) {
                        v.push(Adjacency::new(e.id, e.from));
                        v.sort_by_key(|s| s.vertex_id);
                    }
                }
            };
        }

        Ok(())
    }

    /// Drop the transaction content
    fn abort(self) -> StorageResult<()> {
        Ok(())
    }
}

impl Graph for MvccGraphStorage {
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
        if let Some(v) = txn.vertex_updates.get(&id) {
            return match v.value() {
                UpdateVertexOp::Delete => Ok(None),
                UpdateVertexOp::Insert(v) => Ok(Some(v.clone())),
            };
        }

        // Check global graph data
        if let Some(v) = self.vertices.get(&id) {
            txn.vertex_read.insert(id);
            Ok(v.get_visible(txn.txn_id))
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
            Direction::Forward => &self.adjacency_forward.get(&id),
            Direction::Reversed => {
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
                        adjs.push(Adjacency::new(e.id, e.to))
                    } else {
                        adjs.push(Adjacency::new(e.id, e.from))
                    }
                }
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
                UpdateEdgeOp::Insert(e) => Some(*e),
            });
        }

        // 2. Check global graph data
        if let Some(e) = self.edges.get(&id) {
            txn.edge_read.insert(id);
            Ok(e.get_visible(txn.txn_id))
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
            .filter_map(|v| v.value().get_visible(txn.txn_id))
            .collect::<Vec<_>>();

        for v_update in &txn.vertex_updates {
            match v_update.value() {
                UpdateVertexOp::Delete => {
                    if let Ok(i) = vertices.binary_search_by(|v| v.id.cmp(v_update.key())) {
                        vertices.remove(i);
                    }
                }
                UpdateVertexOp::Insert(v) => vertices.push(v.clone()),
            }
        }

        Ok(Box::new(vertices.into_iter()))
    }

    fn edges(&self, txn: &Self::Transaction) -> StorageResult<Self::EdgeIter> {
        // brute force version

        let mut edges = self
            .edges
            .iter()
            .filter_map(|e| e.value().get_visible(txn.txn_id))
            .collect::<Vec<_>>();

        for e_update in &txn.edge_updates {
            match e_update.value() {
                UpdateEdgeOp::Delete => {
                    if let Ok(i) = edges.binary_search_by(|e| e.id.cmp(e_update.key())) {
                        edges.remove(i);
                    }
                }
                UpdateEdgeOp::Insert(e) => edges.push(*e),
            }
        }

        Ok(Box::new(edges.into_iter()))
    }
}

impl MutGraph for MvccGraphStorage {
    fn create_vertex(&self, txn: &Self::Transaction, vertex: Vertex) -> StorageResult<()> {
        let mut vertex = vertex;
        vertex.begin_ts = txn.txn_id;

        if let Some(txn_v) = txn.vertex_updates.get(&vertex.id) {
            match txn_v.value() {
                UpdateVertexOp::Delete => {
                    txn.vertex_updates.entry(vertex.id).and_modify(|v| {
                        *v = UpdateVertexOp::Insert(vertex);
                    });
                }
                UpdateVertexOp::Insert(v) => {
                    return Err(StorageError::TransactionError(format!(
                        "Vertex {} has been inserted",
                        v.id
                    )));
                }
            };
        } else {
            txn.vertex_updates
                .insert(vertex.id, UpdateVertexOp::Insert(vertex));
        }

        Ok(())
    }

    fn create_edge(&self, txn: &Self::Transaction, edge: Edge) -> StorageResult<()> {
        let mut edge = edge;
        edge.begin_ts = txn.txn_id;

        if let Some(txn_v) = txn.edge_updates.get(&edge.id) {
            match txn_v.value() {
                UpdateEdgeOp::Delete => {
                    txn.edge_updates.entry(edge.id).and_modify(|e| {
                        *e = UpdateEdgeOp::Insert(edge);
                    });
                }
                UpdateEdgeOp::Insert(e) => {
                    return Err(StorageError::EdgeNotFound(format!(
                        "Edge {} has been inserted",
                        e.id
                    )));
                }
            };
        } else {
            txn.edge_updates.insert(edge.id, UpdateEdgeOp::Insert(edge));
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
                .entry(v.id)
                .and_modify(|v| *v = UpdateVertexOp::Delete)
                .or_insert(UpdateVertexOp::Delete);
        }

        Ok(())
    }

    fn delete_edges(&self, txn: &Self::Transaction, edges: Vec<Self::Edge>) -> StorageResult<()> {
        for e in edges {
            txn.edge_updates
                .entry(e.id)
                .and_modify(|v| *v = UpdateEdgeOp::Delete)
                .or_insert(UpdateEdgeOp::Delete);
        }

        Ok(())
    }
}

impl MvccGraphStorage {
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
    use super::*;

    #[test]
    fn test_create_and_get_vertex() {
        let storage = MvccGraphStorage::new();
        let txn = storage.begin_transaction();

        let vertex = Vertex {
            begin_ts: TxnId(1),
            end_ts: TxnId(u64::MAX),
            id: 1,
        };

        storage.create_vertex(&txn, vertex.clone()).unwrap();
        let fetched_vertex = storage.get_vertex(&txn, 1).unwrap().unwrap();
        assert_eq!(fetched_vertex, vertex);
    }

    #[test]
    fn test_create_and_get_edge() {
        let storage = MvccGraphStorage::new();
        let txn = storage.begin_transaction();

        let edge = Edge {
            begin_ts: TxnId(1),
            end_ts: TxnId(u64::MAX),
            id: 1,
            from: 1,
            to: 2,
        };

        storage.create_edge(&txn, edge).unwrap();
        let fetched_edge = storage.get_edge(&txn, 1).unwrap().unwrap();
        assert_eq!(fetched_edge, edge);
    }

    #[test]
    fn test_delete_vertex() {
        let storage = MvccGraphStorage::new();
        let txn = storage.begin_transaction();

        let vertex = Vertex {
            begin_ts: TxnId(1),
            end_ts: TxnId(u64::MAX),
            id: 1,
        };

        storage.create_vertex(&txn, vertex.clone()).unwrap();
        storage.delete_vertices(&txn, vec![vertex]).unwrap();
        let fetched_vertex = storage.get_vertex(&txn, 1).unwrap();
        assert!(fetched_vertex.is_none());
    }

    #[test]
    fn test_delete_edge() {
        let storage = MvccGraphStorage::new();
        let txn = storage.begin_transaction();

        let edge = Edge {
            begin_ts: TxnId(1),
            end_ts: TxnId(u64::MAX),
            id: 1,
            from: 1,
            to: 2,
        };

        storage.create_edge(&txn, edge).unwrap();
        storage.delete_edges(&txn, vec![edge]).unwrap();
        let fetched_edge = storage.get_edge(&txn, 1).unwrap();
        assert!(fetched_edge.is_none());
    }

    #[test]
    fn test_neighbors() {
        let storage = MvccGraphStorage::new();
        let txn = storage.begin_transaction();

        let edge = Edge {
            begin_ts: TxnId(1),
            end_ts: TxnId(u64::MAX),
            id: 1,
            from: 1,
            to: 2,
        };

        storage
            .create_vertex(&txn, Vertex {
                begin_ts: TxnId(1),
                end_ts: TxnId(u64::MAX),
                id: 1,
            })
            .unwrap();
        storage
            .create_vertex(&txn, Vertex {
                begin_ts: TxnId(1),
                end_ts: TxnId(u64::MAX),
                id: 2,
            })
            .unwrap();
        storage.create_edge(&txn, edge).unwrap();
        let neighbors = storage
            .neighbors(&txn, 1, Direction::Forward)
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors[0].vertex_id, 2);
        let neighbors = storage
            .neighbors(&txn, 2, Direction::Reversed)
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors[0].vertex_id, 1);
    }

    #[test]
    fn test_transaction_commit() {
        let storage = MvccGraphStorage::new();
        let txn = storage.begin_transaction();

        let vertex = Vertex {
            begin_ts: TxnId(1),
            end_ts: TxnId(u64::MAX),
            id: 1,
        };

        storage.create_vertex(&txn, vertex.clone()).unwrap();
        txn.commit().unwrap();

        let new_txn = storage.begin_transaction();
        let fetched_vertex = storage.get_vertex(&new_txn, 1).unwrap().unwrap();
        assert_eq!(fetched_vertex, vertex);
    }
}
