use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use crate::error::{StorageError, StorageResult};
use crate::storage::{Direction, Graph, MutGraph, StorageTransaction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TransactionId(u64);

/// Vertex
#[derive(Clone, Debug, PartialEq)]
pub struct Vertex {
    pub id: u64,
}

/// Edge
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Edge {
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

/// Vertex with version information
#[derive(Debug, Clone)]
struct VersionedVertex {
    versions: BTreeMap<TransactionId, Vertex>,
    deleted: bool,
}

impl VersionedVertex {
    fn new() -> Self {
        Self {
            versions: BTreeMap::new(),
            deleted: false,
        }
    }
}

/// Edge with version information
#[derive(Debug, Clone)]
struct VersionedEdge {
    versions: BTreeMap<TransactionId, Edge>,
    deleted: bool,
}

impl VersionedEdge {
    fn new() -> Self {
        Self {
            versions: BTreeMap::new(),
            deleted: false,
        }
    }
}

/// MVCC Transaction
pub struct MvccGraphStorage {
    // TransactionID which represents the latest timestamp
    tx_id_counter: AtomicU64,
    // Vertex table
    vertices: RwLock<HashMap<u64, Arc<RwLock<VersionedVertex>>>>,
    // Edge table
    edges: RwLock<HashMap<u64, Arc<RwLock<VersionedEdge>>>>,
    // Adjacency list
    adjacency_forward: RwLock<HashMap<u64, Vec<Adjacency>>>,
    adjacency_reversed: RwLock<HashMap<u64, Vec<Adjacency>>>,
}

/// MVCC Transaction instance
pub struct MvccTransaction {
    storage: Arc<MvccGraphStorage>,
    tx_id: TransactionId,
    snapshot_time: TransactionId,
    // Local transaction modifications
    vertex_updates: RwLock<HashMap<u64, Vertex>>,
    edge_updates: RwLock<HashMap<u64, Edge>>,
    deleted_vertices: RwLock<HashSet<u64>>,
    deleted_edges: RwLock<HashSet<u64>>,
}

impl StorageTransaction for MvccTransaction {
    fn commit(self) -> StorageResult<()> {
        // Acquiring the global wirte locks
        let mut vertices = self.storage.vertices.write().unwrap();
        let mut edges = self.storage.edges.write().unwrap();
        
        // Validate the write conflicts
        for (id, _) in self.vertex_updates.read().unwrap().iter() {
            if let Some(v) = vertices.get(id) {
                let v_guard = v.read().unwrap();
                if v_guard.versions.keys().any(|&v_tx| v_tx > self.snapshot_time) {
                    return Err(StorageError::TransactionError(format!("VertexID: {}, write conflict.", id)));
                }
            }
        }
        
        for (id, vertex) in self.vertex_updates.write().unwrap().drain() {
            let entry = vertices.entry(id).or_insert_with(|| 
                Arc::new(RwLock::new(VersionedVertex::new()))
            );
            entry.write().unwrap().versions.insert(self.tx_id, vertex);
        }

        let mut adj_forward = self.storage.adjacency_forward.write().unwrap();
        let mut adj_reversed = self.storage.adjacency_reversed.write().unwrap();
        
        // delete the committed vertices
        let deleted_vertices = self.deleted_vertices.read().unwrap().clone();
        for vid in deleted_vertices.iter() {
            // @TODO more elegent deletion mechanism
            // vertices.remove(vid);
            // adj_forward.remove(vid);
            // adj_reversed.remove(vid);
            vertices.entry(*vid).and_modify(|v| {
                v.write().unwrap().deleted = true;
            });
        }
        
        for (id, edge) in self.edge_updates.write().unwrap().drain() {
            let entry = edges.entry(id).or_insert_with(|| 
                Arc::new(RwLock::new(VersionedEdge::new()))
            );
            entry.write().unwrap().versions.insert(self.tx_id, edge);
            // Update the forward adjacency
            adj_forward.entry(edge.from)
            .and_modify(|v| v.push(Adjacency {
                edge_id: edge.id,
                vertex_id: edge.to,
            }))
            .or_insert_with(|| vec![Adjacency {
                edge_id: edge.id,
                vertex_id: edge.to,
            }]);

            // Update the reversed adjacency
            adj_reversed.entry(edge.to)
                .and_modify(|v| v.push(Adjacency {
                    edge_id: edge.id,
                    vertex_id: edge.from,
                }))
                .or_insert_with(|| vec![Adjacency {
                    edge_id: edge.id,
                    vertex_id: edge.from,
                }]);
        }
        
        for eid in self.deleted_edges.read().unwrap().iter() {
            if let Some(edge) = edges.get(eid) {
                let mut edge_guard = edge.write().unwrap();
                edge_guard.deleted = true;
                // if let Some(latest) = edge_guard.versions.values().last() {
                    // if let Some(adjs) = adj_forward.get_mut(&latest.from) {
                    //     adjs.retain(|a| a.edge_id != *eid);
                    // }
                    // if let Some(adjs) = adj_reversed.get_mut(&latest.to) {
                    //     adjs.retain(|a| a.edge_id != *eid);
                    // }
                // }
            }
        }
 
        Ok(())
    }

    /// Drop the transaction content
    fn abort(self) -> StorageResult<()> {
        Ok(())
    }
}

impl Graph for MvccGraphStorage {
    type Transaction = MvccTransaction;
    type VertexID = u64;
    type EdgeID = u64;
    type Vertex = Vertex;
    type Edge = Edge;
    type Adjacency = Adjacency;
    type VertexIter = Box<dyn Iterator<Item = Vertex>>;
    type EdgeIter = Box<dyn Iterator<Item = Edge>>;
    type AdjacencyIter = Box<dyn Iterator<Item = Adjacency>>;

    fn get_vertex(&self, txn: &Self::Transaction, id: u64) -> StorageResult<Option<Vertex>> {
        // Check transaction local modification
        if let Some(v) = txn.vertex_updates.read().unwrap().get(&id) {
            return Ok(Some(v.clone()));
        }

        // Check whether vertex has been deleted
        if txn.deleted_vertices.read().unwrap().contains(&id) {
            return Err(StorageError::VertexNotFound(id.to_string()));
        }
        
        // Check global graph data
        let vertices = self.vertices.read().unwrap();
        if let Some(v_arc) = vertices.get(&id) {
            let v_guard = v_arc.read().unwrap();
            // Find the latest version that is less than or equal to snapshot_time
            if let Some((_, version)) = v_guard.versions.range(..=txn.snapshot_time).next_back() {
                return Ok(Some(version.clone()));
            }
        }
        
        Err(StorageError::TransactionError(format!("test")))
    }

    fn neighbors(&self, txn: &Self::Transaction, id: u64, direction: Direction) -> StorageResult<Self::AdjacencyIter> {
        // Get the global adjacency list
        let global_adjs = match direction {
            Direction::Forward => &self.adjacency_forward,
            Direction::Reversed => &self.adjacency_reversed,
        }.read()
        .unwrap()
        .get(&id)
        .cloned()
        .unwrap_or_default();

        // Filter the deleted edges
        let deleted_edges = txn.deleted_edges.read().unwrap();
        let filtered_global: Vec<Adjacency> = global_adjs
            .into_iter()
            .filter(|adj| !deleted_edges.contains(&adj.edge_id))
            .collect();

        // Resolve the local updated adjacency
        let local_edges: Vec<Adjacency> = txn.edge_updates.read().unwrap()
            .values()
            .filter(|e| match direction {
                Direction::Forward => e.from == id,
                Direction::Reversed => e.to == id,
            })
            .map(|e| Adjacency {
                edge_id: e.id,
                vertex_id: match direction {
                    Direction::Forward => e.to,
                    Direction::Reversed => e.from,
                },
            })
            .collect();

        // Merge the neighbor results
        Ok(Box::new(filtered_global.into_iter().chain(local_edges.into_iter())))
    }
    
    fn get_edge(&self, txn: &Self::Transaction, id: Self::EdgeID) -> StorageResult<Option<Self::Edge>> {
        // 1. Check transaction-local modifications
        if let Some(edge) = txn.edge_updates.read().unwrap().get(&id) {
            return Ok(Some(edge.clone()));
        }
    
        // 2. Check transaction delete markers
        if txn.deleted_edges.read().unwrap().contains(&id) {
            return Ok(None);
        }
    
        // 3. Check global storage
        let edges = self.edges.read().unwrap();
        if let Some(edge_arc) = edges.get(&id) {
            let edge_guard = edge_arc.read().unwrap();
            // Find the latest version less than or equal to the snapshot time
            if let Some((_, edge)) = edge_guard.versions.range(..=txn.snapshot_time).next_back() {
                return Ok(Some(edge.clone()));
            }
        }
    
        // 4. Edge does not exist
        Ok(None)
    }
    
    fn vertices(&self, txn: &Self::Transaction) -> StorageResult<Self::VertexIter> {
        // 1. Get global vertex iterator
        let global_vertices = self.vertices.read().unwrap();
        let global_iter = global_vertices.values().filter_map(move |v_arc| {
            let v_guard = v_arc.read().unwrap();
            // Find the latest version less than or equal to the snapshot time
            v_guard.versions
                .range(..=txn.snapshot_time)
                .next_back()
                .map(|(_, v)| v.clone())
        });
    
        // 2. Get transaction-local added vertices
        let local_vertices: Vec<Vertex> = txn.vertex_updates.read().unwrap().values().cloned().collect();
    
        // 3. Filter out deleted vertices within the transaction
        let deleted_vertices = txn.deleted_vertices.read().unwrap();
        let filtered_global: Vec<Vertex> = global_iter.filter(move |v| !deleted_vertices.contains(&v.id)).collect();
    
        // 4. Merge global and local vertices
        Ok(Box::new(filtered_global.into_iter().chain(local_vertices)))
    }
    
    fn edges(&self, txn: &Self::Transaction) -> StorageResult<Self::EdgeIter> {
        // 1. Get global edge iterator
        let global_edges = self.edges.read().unwrap();
        let global_iter = global_edges.values().filter_map(move |e_arc| {
            let e_guard = e_arc.read().unwrap();
            // Find the latest version less than or equal to the snapshot time
            e_guard.versions
                .range(..=txn.snapshot_time)
                .next_back()
                .map(|(_, e)| e.clone())
        });
    
        // 2. Get transaction-local added edges
        let local_edges: Vec<Edge> = txn.edge_updates.read().unwrap().values().cloned().collect();
    
        // 3. Filter out deleted edges within the transaction
        let deleted_edges = txn.deleted_edges.read().unwrap();
        let filtered_global: Vec<Edge> = global_iter.filter(move |e| !deleted_edges.contains(&e.id)).collect();
    
        // 4. Merge global and local edges
        Ok(Box::new(filtered_global.into_iter().chain(local_edges.into_iter())))
    }
}

impl MutGraph for MvccGraphStorage {
    fn add_vertex(&self, txn: &Self::Transaction, vertex: Vertex) -> StorageResult<()> {
        let mut updates = txn.vertex_updates.write().unwrap();
        if updates.contains_key(&vertex.id) {
            return Err(StorageError::TransactionError(format!("test")));
        }
        
        // Check the global storage
        let vertices = self.vertices.read().unwrap();
        if let Some(v_arc) = vertices.get(&vertex.id) {
            let v_guard = v_arc.read().unwrap();
            if v_guard.versions.range(..=txn.snapshot_time).next_back().is_some() {
                return Err(StorageError::TransactionError(format!("test")));
            }
        }
        
        updates.insert(vertex.id, vertex);
        Ok(())
    }
    
    fn add_edge(&self, txn: &Self::Transaction, edge: Edge) -> StorageResult<()> {
        // 1. Check if the edge ID already exists (including transaction-local modifications)
        let edge_id = edge.id;
        if txn.edge_updates.read().unwrap().contains_key(&edge_id) {
            return Err(StorageError::TransactionError(format!("Edge already exists")));
        }
    
        // 2. Check if the edge exists in the global storage
        let edges = self.edges.read().unwrap();
        if let Some(edge_arc) = edges.get(&edge_id) {
            let edge_guard = edge_arc.read().unwrap();
            // Check if there is an un-deleted version
            if edge_guard.versions.range(..=txn.snapshot_time).next_back().is_some() {
                return Err(StorageError::TransactionError(format!("Edge already exists")));
            }
        }
    
        // 3. Check if the start and end vertices of the edge exist
        let vertices = self.vertices.read().unwrap();
        let from_exists = vertices.contains_key(&edge.from)
            || txn.vertex_updates.read().unwrap().contains_key(&edge.from);
        let to_exists = vertices.contains_key(&edge.to)
            || txn.vertex_updates.read().unwrap().contains_key(&edge.to);
    
        if !from_exists || !to_exists {
            return Err(StorageError::VertexNotFound(format!("Vertex is not found")));
        }
    
        // 4. Add the edge to the transaction-local modifications
        txn.edge_updates.write().unwrap().insert(edge_id, edge);
    
        Ok(())
    }
}

impl MvccGraphStorage {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            tx_id_counter: AtomicU64::new(1),
            vertices: RwLock::new(HashMap::new()),
            edges: RwLock::new(HashMap::new()),
            adjacency_forward: RwLock::new(HashMap::new()),
            adjacency_reversed: RwLock::new(HashMap::new()),
        })
    }

    pub fn begin_transaction(self: &Arc<Self>) -> MvccTransaction {
        let tx_id = TransactionId(self.tx_id_counter.fetch_add(1, Ordering::SeqCst));
        MvccTransaction {
            storage: self.clone(),
            tx_id,
            snapshot_time: tx_id,
            vertex_updates: RwLock::new(HashMap::new()),
            edge_updates: RwLock::new(HashMap::new()),
            deleted_edges: RwLock::new(HashSet::new()),
            deleted_vertices: RwLock::new(HashSet::new()),
        }
    }
}