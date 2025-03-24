use std::sync::{Arc, Mutex, OnceLock, RwLock};

use common::datatype::types::{EdgeId, VertexId};
use dashmap::{DashMap, DashSet};

use crate::{error::{StorageError, StorageResult}, model::{edge::Edge, vertex::Vertex}, storage::StorageTransaction, transaction::{CommitTimestamp, DeltaOp, IsolationLevel, SetPropsOp, UndoEntry, UndoPtr}};

use super::memory_graph::{MemGraph, VersionedAdjEntry};

pub struct MemTxnManager {
    /// Active transactions' start timestamps.
    pub(super) active_txns: DashSet<CommitTimestamp>, 
    /// All transactions, running or committed.
    pub(super) txn_map: DashMap<CommitTimestamp, Arc<MemTransaction>>,
    /// Commit lock to enforce serial commit order
    pub(super) commit_lock: Mutex<()>,
}

impl MemTxnManager {
    /// Create a new MemTxnManager
    pub fn new() -> Self {
        Self {
            active_txns: DashSet::new(),
            txn_map: DashMap::new(),
            commit_lock: Mutex::new(()),
        }
    }

    /// Register a new transaction.
    pub fn register(&self, txn: Arc<MemTransaction>) {
        self.txn_map.insert(txn.txn_id, txn.clone());
    }

    /// Unregister a transaction.
    pub fn remove(&self, txn_id: CommitTimestamp) -> StorageResult<()> {
        self.txn_map.remove(&txn_id);
        Ok(())
    }

    /// Start a garbage collection of expired transactions.
    pub fn garbage_collect(&self) -> StorageResult<()> {
        todo!()
    }
}

pub struct MemTransaction {
    graph: Arc<MemGraph>, // Reference to the associated in-memory graph

    // ---- Transaction Config ----
    isolation_level: IsolationLevel, // Isolation level of the transaction

    // ---- Timestamp management ----
    /// Start timestamp assigned when the transaction begins
    start_ts: CommitTimestamp, 
    commit_ts: OnceLock<CommitTimestamp>, // Commit timestamp assigned upon committing
    txn_id: CommitTimestamp,      // Unique transaction identifier

    // ---- Read sets ----
    pub(super) vertex_reads: DashSet<VertexId>, // Set of vertices read by this transaction
    pub(super) edge_reads: DashSet<EdgeId>,     // Set of edges read by this transaction

    // ---- Write sets ----
    pub(super) vertex_writes: DashSet<VertexId>, // Set of vertices written by this transaction
    pub(super) edge_writes: DashSet<EdgeId>,     // Set of edges written by this transaction

    // ---- Undo logs ----
    // pub(super) vertex_undos: DashMap<VertexId, Vec<UndoEntry<Vertex>>>, // Vertex undo logs
    // pub(super) edge_undos: DashMap<EdgeId, Vec<UndoEntry<Edge>>>,       // Edge undo logs
    pub(super) undo_buffer: RwLock<Vec<UndoEntry>>,
}

impl MemTransaction {
    pub(super) fn with_memgraph(
        graph: Arc<MemGraph>,
        txn_id: CommitTimestamp,
        start_ts: CommitTimestamp,
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
            vertex_writes: DashSet::new(),
            edge_writes: DashSet::new(),
            // vertex_undos: DashMap::new(),
            // edge_undos: DashMap::new(),
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

    /// Invalidates adjacency entries when a vertex is deleted.
    /// This marks all outgoing and incoming edges as deleted by updating their end timestamps.
    fn invalidate_adjacency_by_vid(
        &self,
        vid: u64,
        commit_ts: CommitTimestamp,
    ) -> StorageResult<()> {
        let max_commit_ts = CommitTimestamp::max_commit_ts();

        // Invalidate outgoing edges
        if let Some(mut entries) = self.graph.adjacency_out.get_mut(&vid) {
            for entry in entries.iter_mut() {
                if entry.end_ts() == max_commit_ts {
                    entry.end_ts = commit_ts;
                }
            }
        }

        // Invalidate incoming edges
        if let Some(mut entries) = self.graph.adjacency_in.get_mut(&vid) {
            for entry in entries.iter_mut() {
                if entry.end_ts() == max_commit_ts {
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

    /// Returns the start timestamp of the transaction.
    pub fn start_ts(&self) -> CommitTimestamp {
        self.start_ts
    }

    /// Returns the transaction ID.
    pub fn txn_id(&self) -> CommitTimestamp {
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
    pub fn graph(&self) -> &Arc<MemGraph> {
        &self.graph
    }

    /// Returns the isolution level
    pub fn isolation_level(&self) -> &IsolationLevel {
        &self.isolation_level
    }

    /// Reconstructs a specific version of a Vertex based on the undo chain and a target timestamp
    pub(super) fn apply_deltas_for_vertex(&self, undo_ptr: UndoPtr, base_vertex: &mut Vertex, target_ts: CommitTimestamp) -> StorageResult<()> {
        let mut current = undo_ptr;
        
        while let Some(entry) = self.graph.txn_manager.txn_map.get(&current.txn_id()) {

            let undo_buffer = entry.undo_buffer.read().unwrap();
            let undo_entry = &undo_buffer[current.entry_offset()];
            
            match undo_entry.delta() {
                DeltaOp::CreateVertex(original) => *base_vertex = original.clone(),
                DeltaOp::SetVertexProps(_, SetPropsOp { indices, props }) => {
                                    base_vertex.set_props(indices, props.clone());
                                }
                DeltaOp::DelVertex(_) => {
                    base_vertex.is_tombstone = true;
                }
                _ => unreachable!("Unreachable delta op for a vertex"),
            }
            
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

    /// Reconstructs a specific version of an Edge based on the undo chain and a target timestamp
    pub(super) fn apply_deltas_for_edge(&self, undo_ptr: UndoPtr, base_edge: &mut Edge, target_ts: CommitTimestamp) -> StorageResult<()> {
        let mut current = undo_ptr;
        
        while let Some(entry) = self.graph.txn_manager.txn_map.get(&current.txn_id()) {
            let undo_buffer = entry.undo_buffer.read().unwrap();
            let undo_entry = &undo_buffer[current.entry_offset()];
            
            match undo_entry.delta() {
                DeltaOp::CreateEdge(original) => *base_edge = original.clone(),
                DeltaOp::SetEdgeProps(_, SetPropsOp { indices, props }) => {
                    base_edge.set_props(indices, props.clone());
                },
                DeltaOp::DelEdge(_) => {
                    base_edge.is_tombstone = true;
                },
                _ => unreachable!("Unreachable delta op for an edge"),
            }
            
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
    type CommitTimestamp = CommitTimestamp;

    /// Commits the transaction, applying all changes atomically.
    /// Ensures serializability, updates version chains, and manages adjacency lists.
    fn commit(&self) -> StorageResult<CommitTimestamp> {
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
        let commit_ts = CommitTimestamp::new_commit_ts();
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
                    DeltaOp::AddInEdge(edge) => {
                        // 处理添加入边操作
                        update_commit_ts!(self, edges, &edge.eid());
                        // 更新邻接列表
                        self.update_adjacency_by_eid(&edge.eid(), commit_ts)?;
                    },
                    DeltaOp::AddOutEdge(edge) => {
                        // 处理添加出边操作
                        update_commit_ts!(self, edges, &edge.eid());
                        // 更新邻接列表
                        self.update_adjacency_by_eid(&edge.eid(), commit_ts)?;
                    },
                    DeltaOp::RemoveInEdge(eid) => {
                        // 处理移除入边操作
                        update_commit_ts!(self, edges, eid);
                        // 使邻接列表条目失效
                        self.invalidate_adjacency_by_eid(eid, commit_ts)?;
                    },
                    DeltaOp::RemoveOutEdge(eid) => {
                        // 处理移除出边操作
                        update_commit_ts!(self, edges, eid);
                        // 使邻接列表条目失效
                        self.invalidate_adjacency_by_eid(eid, commit_ts)?;
                    },
                }
            }
        }

        // Step 5: Clean up transaction state.
        self.graph.txn_manager.remove(self.txn_id())?;
        Ok(commit_ts)
    }

    /// Aborts the transaction, rolling back all changes.
    fn abort(&self) -> StorageResult<()> {
        // 获取 undo_buffer 的只读锁
        let undo_entries: Vec<_> = self.undo_buffer.write().unwrap().drain(..).collect();
        
        // 处理所有 undo 条目
        for undo_entry in undo_entries.into_iter() {
            let commit_ts = undo_entry.timestamp();
            let next = undo_entry.next();
            match undo_entry.delta() {
                DeltaOp::CreateVertex(vertex) => {
                    // 对于新创建的顶点，直接从图中移除或标记为已删除
                    let vid = vertex.vid();
                    if let Some(entry) = self.graph.vertices.get(&vid) {
                        let mut current = entry.chain.current.write().unwrap();
                        if current.commit_ts == self.txn_id() {
                            // 如果是当前事务创建的，标记为已删除
                            current.data = vertex.clone();
                            current.data.is_tombstone = false;
                            current.commit_ts = commit_ts; 
                            *entry.chain.undo_ptr.write().unwrap() = next;
                        }
                    }
                }
                DeltaOp::CreateEdge(edge) => {
                    // 对于新创建的边，直接从图中移除或标记为已删除
                    let eid = edge.eid();
                    if let Some(entry) = self.graph.edges.get(&eid) {
                        let mut current = entry.chain.current.write().unwrap();
                        if current.commit_ts == self.txn_id() {
                            // 如果是当前事务创建的，标记为已删除
                            current.data = edge.clone();
                            current.data.is_tombstone = false;
                            current.commit_ts = commit_ts;
                            *entry.chain.undo_ptr.write().unwrap() = next;
                        }
                    }
                },
                DeltaOp::SetVertexProps(vid, SetPropsOp { indices, props }) => {
                    // 对于属性修改，需要根据 undo_entry 的 entity_id 确定是顶点还是边
                    // 恢复顶点属性
                    if let Some(entry) = self.graph.vertices.get(&vid) {
                        let mut current = entry.chain.current.write().unwrap();
                        if current.commit_ts == self.txn_id() {
                            // 恢复属性
                            current.data.set_props(indices, props.clone());
                            // 更新 undo 指针指向前一个版本
                            *entry.chain.undo_ptr.write().unwrap() = next;
                        }
                    }
                },
                DeltaOp::SetEdgeProps(eid, SetPropsOp { indices, props }) => {
                    // 恢复边属性
                    if let Some(entry) = self.graph.edges.get(&eid) {
                        let mut current = entry.chain.current.write().unwrap();
                        if current.commit_ts == self.txn_id() {
                            // 恢复属性
                            current.data.set_props(indices, props.clone());
                            // 更新 undo 指针指向前一个版本
                            *entry.chain.undo_ptr.write().unwrap() = next;
                        }
                    }
                }
                DeltaOp::DelVertex(vid) => {
                    // 恢复顶点
                    if let Some(entry) = self.graph.vertices.get(&vid) {
                        let mut current = entry.chain.current.write().unwrap();
                        if current.commit_ts == self.txn_id() {
                            // 恢复删除标记
                            current.data.is_tombstone = false;
                            // 更新 undo 指针指向前一个版本
                            *entry.chain.undo_ptr.write().unwrap() = next;
                        }
                    }
                },
                DeltaOp::DelEdge(eid) => {
                    // 恢复边
                    if let Some(entry) = self.graph.edges.get(&eid) {
                        let mut current = entry.chain.current.write().unwrap();
                        if current.commit_ts == self.txn_id() {
                            // 恢复删除标记
                            current.data.is_tombstone = false;
                            // 更新 undo 指针指向前一个版本
                            *entry.chain.undo_ptr.write().unwrap() = next;
                        }
                    }
                }
                DeltaOp::AddLabel(_) => todo!(),
                DeltaOp::RemoveLabel(_) => todo!(),
                DeltaOp::AddInEdge(edge) => todo!(),
                DeltaOp::AddOutEdge(edge) => todo!(),
                DeltaOp::RemoveInEdge(eid) => todo!(),
                DeltaOp::RemoveOutEdge(eid) => todo!(),
            }
        }
        
        // 从事务管理器中移除事务
        self.graph.txn_manager.remove(self.txn_id())?;
        Ok(())
    }
}