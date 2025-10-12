use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};

use crossbeam_skiplist::SkipMap;
use minigu_common::types::{EdgeId, VertexId};
use minigu_transaction::{
    GraphTxnManager, Timestamp, Transaction, global_timestamp_generator,
    global_transaction_id_generator,
};

use super::memory_graph::MemoryGraph;
use super::transaction::{IsolationLevel, MemTransaction, UndoEntry};
use crate::common::DeltaOp;
use crate::common::model::edge::{Edge, Neighbor};
use crate::common::wal::StorageWal;
use crate::common::wal::graph_wal::{Operation, RedoEntry};
use crate::error::{StorageError, StorageResult, TransactionError};

const GC_TRIGGER_THRESHOLD: usize = 50;

/// A manager for managing transactions.
pub struct MemTxnManager {
    /// Weak reference to the graph to avoid circular references
    pub(super) graph: Weak<MemoryGraph>,
    /// Active transactions' txn.
    pub(super) active_txns: SkipMap<Timestamp, Arc<MemTransaction>>,
    /// All transactions, running or committed.
    pub(super) committed_txns: SkipMap<Timestamp, Arc<MemTransaction>>,
    /// Commit lock to enforce serial commit order
    pub(super) commit_lock: Mutex<()>,
    pub(super) latest_commit_ts: AtomicU64,
    /// The watermark is the minimum start timestamp of the active transactions.
    /// If there is no active transaction, the watermark is the latest commit timestamp.
    watermark: AtomicU64,
    /// Last garbage collection timestamp
    last_gc_ts: AtomicU64,
}

impl Default for MemTxnManager {
    fn default() -> Self {
        Self {
            graph: Weak::new(),
            active_txns: SkipMap::new(),
            committed_txns: SkipMap::new(),
            commit_lock: Mutex::new(()),
            latest_commit_ts: AtomicU64::new(0),
            watermark: AtomicU64::new(0),
            last_gc_ts: AtomicU64::new(0),
        }
    }
}

impl GraphTxnManager for MemTxnManager {
    type Error = StorageError;
    type GraphContext = MemoryGraph;
    type Transaction = MemTransaction;

    fn begin_transaction(
        &self,
        isolation_level: IsolationLevel,
    ) -> Result<Arc<Self::Transaction>, Self::Error> {
        self.begin_transaction_at(None, None, isolation_level, false)
    }

    fn finish_transaction(&self, txn: &Self::Transaction) -> Result<(), Self::Error> {
        let txn_entry = self.active_txns.remove(&txn.txn_id());
        if let Some(txn_arc) = txn_entry {
            // Check if the transaction has been committed (by checking if it has a commit_ts)
            if let Some(commit_ts) = txn.commit_ts() {
                self.committed_txns
                    .insert(commit_ts, txn_arc.value().clone());
            }
            self.update_watermark();

            // Trigger GC if threshold is reached
            if self.committed_txns.len() >= GC_TRIGGER_THRESHOLD {
                if let Some(graph) = self.graph.upgrade() {
                    let _ = self.garbage_collect(&graph);
                }
            }

            return Ok(());
        }

        Err(StorageError::Transaction(
            TransactionError::TransactionNotFound(format!("{:?}", txn.txn_id())),
        ))
    }

    fn garbage_collect(&self, graph: &Self::GraphContext) -> Result<(), Self::Error> {
        let min_read_ts = self.low_watermark().raw();
        let mut expired_txns = Vec::new();
        let mut expired_undo_entries = Vec::new();

        // Step 1: Collect expired transactions and their undo entries
        for entry in self.committed_txns.iter() {
            if entry.key().raw() > min_read_ts {
                break;
            }

            expired_txns.push(entry.value().clone());

            // Collect undo entries for graph data cleanup
            for undo_entry in entry.value().undo_buffer().read().unwrap().iter() {
                expired_undo_entries.push(undo_entry.clone());
            }
        }

        // Step 2: Clean up graph data based on expired undo entries
        self.cleanup_graph_data(graph, expired_undo_entries)?;

        // Step 3: Remove expired transactions from tracking
        for txn in expired_txns {
            if let Some(commit_ts) = txn.commit_ts() {
                self.committed_txns.remove(&commit_ts);
            }
        }

        // Step 4: Update last GC timestamp
        let current_ts = global_timestamp_generator().current();
        self.last_gc_ts.store(current_ts.raw(), Ordering::SeqCst);

        Ok(())
    }

    fn low_watermark(&self) -> Timestamp {
        Timestamp::with_ts(self.watermark.load(Ordering::Acquire))
    }
}

impl MemTxnManager {
    /// Create a new MemTxnManager
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the graph reference after construction
    pub fn set_graph(&mut self, graph: &Arc<MemoryGraph>) {
        self.graph = Arc::downgrade(graph);
    }

    /// Begin a new transaction with specified parameters
    pub fn begin_transaction_at(
        &self,
        txn_id: Option<Timestamp>,
        start_ts: Option<Timestamp>,
        isolation_level: IsolationLevel,
        skip_wal: bool,
    ) -> Result<Arc<MemTransaction>, StorageError> {
        let graph = self.graph.upgrade().ok_or_else(|| {
            StorageError::Transaction(TransactionError::InvalidState(
                "Graph reference is no longer valid".to_string(),
            ))
        })?;

        // Update the counters
        let txn_id = if let Some(txn_id) = txn_id {
            global_transaction_id_generator()
                .update_if_greater(txn_id)
                .map_err(TransactionError::Timestamp)?;
            txn_id
        } else {
            global_transaction_id_generator()
                .next()
                .map_err(TransactionError::Timestamp)?
        };
        let start_ts = if let Some(start_ts) = start_ts {
            global_timestamp_generator()
                .update_if_greater(start_ts)
                .map_err(TransactionError::Timestamp)?;
            start_ts
        } else {
            global_timestamp_generator()
                .next()
                .map_err(TransactionError::Timestamp)?
        };

        // Acquire the checkpoint lock to prevent new transactions from being created
        // while we are creating a checkpoint
        let _checkpoint_lock = graph
            .checkpoint_manager
            .as_ref()
            .unwrap()
            .checkpoint_lock
            .read()
            .unwrap();

        // Create the transaction
        let txn = Arc::new(MemTransaction::with_memgraph(
            graph.clone(),
            txn_id,
            start_ts,
            isolation_level,
        ));
        self.active_txns.insert(txn.txn_id(), txn.clone());
        self.update_watermark();

        // Write `Operation::BeginTransaction` to WAL,
        // unless the function is called when recovering from WAL
        if !skip_wal {
            let wal_entry = RedoEntry {
                lsn: graph.wal_manager.next_lsn(),
                txn_id: txn.txn_id(),
                iso_level: *txn.isolation_level(),
                op: Operation::BeginTransaction(txn.start_ts()),
            };
            graph
                .wal_manager
                .wal()
                .write()
                .unwrap()
                .append(&wal_entry)
                .unwrap();
        }

        Ok(txn)
    }

    /// Update the watermark based on currently active transactions.
    /// The watermark represents the minimum timestamp that any active transaction
    /// can see, which is crucial for determining what data can be garbage collected.
    fn update_watermark(&self) {
        let min_ts = self
            .active_txns
            .front()
            .map(|v| v.value().start_ts().raw())
            .unwrap_or(self.latest_commit_ts.load(Ordering::Acquire))
            .max(self.low_watermark().raw());
        self.watermark.store(min_ts, Ordering::SeqCst);
    }

    /// Clean up graph data based on expired undo entries
    fn cleanup_graph_data(
        &self,
        graph: &MemoryGraph,
        expired_entries: Vec<Arc<UndoEntry>>,
    ) -> StorageResult<()> {
        let mut expired_edges: HashMap<EdgeId, Edge> = HashMap::new();

        // Analyze expired undo entries to determine what needs cleanup
        for undo_entry in expired_entries {
            match undo_entry.delta() {
                // DeltaOp::CreateEdge means the edge was deleted in this transaction
                DeltaOp::CreateEdge(edge) => {
                    expired_edges.insert(edge.eid(), edge.without_properties());
                }
                DeltaOp::DelEdge(eid) => {
                    expired_edges.remove(eid);
                }
                _ => {}
            }
        }

        // Clean up expired edges and vertices
        for (_, edge) in expired_edges {
            self.cleanup_expired_edge(graph, &edge)?;
        }

        Ok(())
    }

    /// Clean up a single expired edge and related data
    fn cleanup_expired_edge(&self, graph: &MemoryGraph, edge: &Edge) -> StorageResult<()> {
        // Helper macro to check if entity is tombstone
        macro_rules! is_tombstone {
            ($collection:ident, $id:expr) => {
                graph
                    .$collection
                    .get($id)
                    .map(|v| v.value().chain.current.read().unwrap().data.is_tombstone)
                    .unwrap_or(false)
            };
        }

        let src_tombstone = is_tombstone!(vertices, &edge.src_id());
        let dst_tombstone = is_tombstone!(vertices, &edge.dst_id());
        let edge_tombstone = is_tombstone!(edges, &edge.eid());

        // Clean up tombstone vertices and their adjacencies
        if src_tombstone {
            self.remove_vertex_and_adjacencies(graph, edge.src_id());
        }
        if dst_tombstone {
            self.remove_vertex_and_adjacencies(graph, edge.dst_id());
        }

        // Clean up tombstone edges
        if edge_tombstone {
            graph.edges.remove(&edge.eid());
            self.remove_edge_from_adjacency(graph, edge);
        }

        Ok(())
    }

    /// Remove vertex and all its adjacency relationships
    fn remove_vertex_and_adjacencies(&self, graph: &MemoryGraph, vid: VertexId) {
        graph.vertices.remove(&vid);

        let mut neighbors_to_update = Vec::new();

        if let Some(adj_container) = graph.adjacency_list.get(&vid) {
            // Collect neighbors that need adjacency updates
            for adj in adj_container.incoming().iter() {
                neighbors_to_update.push((
                    adj.neighbor_id(),
                    Neighbor::new(adj.label_id(), vid, adj.eid()),
                    true, // remove from outgoing
                ));
            }
            for adj in adj_container.outgoing().iter() {
                neighbors_to_update.push((
                    adj.neighbor_id(),
                    Neighbor::new(adj.label_id(), vid, adj.eid()),
                    false, // remove from incoming
                ));
            }
        }

        // Update neighbor adjacencies
        for (neighbor_vid, neighbor_entry, is_outgoing) in neighbors_to_update {
            graph.adjacency_list.entry(neighbor_vid).and_modify(|adj| {
                if is_outgoing {
                    adj.outgoing().remove(&neighbor_entry);
                } else {
                    adj.incoming().remove(&neighbor_entry);
                }
            });
        }

        // Remove the vertex's adjacency list
        graph.adjacency_list.remove(&vid);
    }

    /// Remove edge from adjacency lists
    fn remove_edge_from_adjacency(&self, graph: &MemoryGraph, edge: &Edge) {
        let src_neighbor = Neighbor::new(edge.label_id(), edge.dst_id(), edge.eid());
        let dst_neighbor = Neighbor::new(edge.label_id(), edge.src_id(), edge.eid());

        graph.adjacency_list.entry(edge.src_id()).and_modify(|adj| {
            adj.outgoing().remove(&src_neighbor);
        });

        graph.adjacency_list.entry(edge.dst_id()).and_modify(|adj| {
            adj.incoming().remove(&dst_neighbor);
        });
    }
}
