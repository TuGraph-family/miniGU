// checkpoint.rs
// Implementation of checkpoint mechanism for MemoryGraph
//
// This module provides functionality to create and restore checkpoints of a MemoryGraph.
// A checkpoint represents a consistent snapshot of the graph state at a specific point in time.
// It can be used for backup, recovery, or state transfer purposes.

use std::collections::HashMap;
use std::sync::Arc;

use minigu_common::types::{EdgeId, VertexId};
use minigu_transaction::Timestamp;
use serde::{Deserialize, Serialize};

use super::memory_graph::{AdjacencyContainer, MemoryGraph, VersionedEdge, VersionedVertex};
use crate::common::model::edge::{Edge, Neighbor};
use crate::common::model::vertex::Vertex;
use crate::error::StorageResult;

/// Represents a checkpoint of a MemoryGraph at a specific point in time.
///
/// A GraphCheckpoint contains:
/// 1. Metadata about the checkpoint (timestamp, LSN, etc.)
/// 2. Serialized vertices and edges
/// 3. Adjacency list information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphCheckpoint {
    /// Metadata about the checkpoint
    pub metadata: CheckpointMetadata,

    /// Serialized vertices (current version only, no history)
    pub vertices: HashMap<VertexId, SerializedVertex>,

    /// Serialized edges (current version only, no history)
    pub edges: HashMap<EdgeId, SerializedEdge>,

    /// Serialized adjacency list
    pub adjacency_list: HashMap<VertexId, SerializedAdjacency>,
}

/// Metadata about a checkpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointMetadata {
    /// Timestamp when the checkpoint was created
    pub timestamp: u64,

    /// Log sequence number (LSN) at the time of checkpoint
    pub lsn: u64,

    /// Latest commit timestamp at the time of checkpoint
    pub latest_commit_ts: u64,

    /// Checkpoint format version
    pub version: u32,
}

/// Serialized representation of a vertex
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedVertex {
    /// The vertex data
    pub data: Vertex,

    /// Commit timestamp of the vertex
    pub commit_ts: Timestamp,
}

/// Serialized representation of an edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedEdge {
    /// The edge data
    pub data: Edge,

    /// Commit timestamp of the edge
    pub commit_ts: Timestamp,
}

/// Serialized representation of adjacency information for a vertex
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedAdjacency {
    /// Outgoing edges from this vertex
    pub outgoing: Vec<(EdgeId, VertexId)>,

    /// Incoming edges to this vertex
    pub incoming: Vec<(EdgeId, VertexId)>,
}

impl GraphCheckpoint {
    /// Creates a new `GraphCheckpoint` from the current in-memory state of a [`MemoryGraph`].
    ///
    /// This method captures a consistent snapshot of the graph, including:
    /// - The metadata (timestamp, LSN, latest commit timestamp, etc.)
    /// - All vertices and edges (current version only)
    /// - The full adjacency list (both outgoing and incoming edges)
    ///
    /// # Arguments
    ///
    /// * `graph` - A reference-counted pointer to the in-memory [`MemoryGraph`] to be checkpointed.
    ///
    /// # Returns
    ///
    /// A fully materialized `GraphCheckpoint` containing the graph's current state.
    ///
    /// # Panics
    ///
    /// This function may panic if:
    /// - System time is earlier than UNIX_EPOCH (highly unlikely)
    /// - Lock poisoning occurs on internal vertex/edge RwLocks (only if previous panic occurred)
    pub fn new(graph: &Arc<MemoryGraph>) -> Self {
        // Get current LSN (not next_lsn - we don't want to consume an LSN for checkpoint)
        // The checkpoint represents the state "up to and including" the current LSN
        let lsn = graph.persistence.current_lsn();

        // Create metadata
        let metadata = CheckpointMetadata {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            lsn,
            latest_commit_ts: graph
                .txn_manager
                .latest_commit_ts
                .load(std::sync::atomic::Ordering::SeqCst),
            version: 1, // Initial version
        };

        // Serialize vertices
        let mut vertices = HashMap::new();
        for entry in graph.vertices.iter() {
            let versioned_vertex = entry.value();
            let current = versioned_vertex.chain.current.read().unwrap();

            vertices.insert(
                *entry.key(),
                SerializedVertex {
                    data: current.data.clone(),
                    commit_ts: current.commit_ts,
                },
            );
        }

        // Serialize edges
        let mut edges = HashMap::new();
        for entry in graph.edges.iter() {
            let versioned_edge = entry.value();
            let current = versioned_edge.chain.current.read().unwrap();

            edges.insert(
                *entry.key(),
                SerializedEdge {
                    data: current.data.clone(),
                    commit_ts: current.commit_ts,
                },
            );
        }

        // Serialize adjacency list
        let mut adjacency_list = HashMap::new();
        for entry in graph.adjacency_list.iter() {
            let vertex_id = *entry.key();
            let adj_container = entry.value();

            let mut outgoing = Vec::new();
            for neighbor in adj_container.outgoing().iter() {
                outgoing.push((neighbor.value().eid(), neighbor.value().neighbor_id()));
            }

            let mut incoming = Vec::new();
            for neighbor in adj_container.incoming().iter() {
                incoming.push((neighbor.value().eid(), neighbor.value().neighbor_id()));
            }

            adjacency_list.insert(vertex_id, SerializedAdjacency { outgoing, incoming });
        }

        Self {
            metadata,
            vertices,
            edges,
            adjacency_list,
        }
    }

    /// This method reconstructs an in-memory graph by replaying the serialized state
    /// stored in the checkpoint, including:
    /// - Metadata (log sequence number and latest commit timestamp)
    /// - All current vertices and edges (no historical versions)
    /// - The full adjacency list (outgoing/incoming connections)
    ///
    /// This method is typically used during system recovery, state rehydration,
    /// or startup bootstrapping from the latest persisted checkpoint.
    ///
    /// # Arguments
    ///
    /// * `checkpoint_config` - Configuration options for the graph's checkpoint behavior.
    /// * `wal_config` - Configuration for initializing the graph's write-ahead log (WAL) system.
    ///
    /// # Returns
    ///
    /// A fully reconstructed [`Arc<MemoryGraph>`] containing the state at the time of checkpoint
    /// creation.
    pub fn restore(&self, graph: &Arc<MemoryGraph>) -> StorageResult<()> {
        // Set the next LSN to checkpoint LSN + 1
        // The checkpoint represents state "up to and including" checkpoint.lsn,
        // so the next available LSN is checkpoint.lsn + 1
        graph.persistence.set_next_lsn(self.metadata.lsn + 1);

        // Set the latest commit timestamp
        graph.txn_manager.latest_commit_ts.store(
            self.metadata.latest_commit_ts,
            std::sync::atomic::Ordering::SeqCst,
        );

        // Restore vertices
        for (vid, serialized_vertex) in &self.vertices {
            let versioned_vertex = VersionedVertex::new(serialized_vertex.data.clone());
            // Set the commit timestamp
            let mut current = versioned_vertex.chain.current.write().unwrap();
            current.commit_ts = serialized_vertex.commit_ts;
            drop(current);

            graph.vertices.insert(*vid, versioned_vertex);
        }

        // Restore edges
        for (eid, serialized_edge) in &self.edges {
            let versioned_edge = VersionedEdge::new(serialized_edge.data.clone());
            // Set the commit timestamp
            let mut current = versioned_edge.chain.current.write().unwrap();
            current.commit_ts = serialized_edge.commit_ts;
            drop(current);

            graph.edges.insert(*eid, versioned_edge);
        }

        // Restore adjacency list
        for (vid, serialized_adjacency) in &self.adjacency_list {
            let adjacency_container = AdjacencyContainer::new();

            // Restore outgoing edges
            for (edge_id, dst_id) in &serialized_adjacency.outgoing {
                let edge = graph.edges.get(edge_id).unwrap();
                let label_id = edge.chain.current.read().unwrap().data.label_id();
                adjacency_container
                    .outgoing()
                    .insert(Neighbor::new(label_id, *dst_id, *edge_id));
            }

            // Restore incoming edges
            for (edge_id, src_id) in &serialized_adjacency.incoming {
                let edge = graph.edges.get(edge_id).unwrap();
                let label_id = edge.chain.current.read().unwrap().data.label_id();
                adjacency_container
                    .incoming()
                    .insert(Neighbor::new(label_id, *src_id, *edge_id));
            }

            graph.adjacency_list.insert(*vid, adjacency_container);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use minigu_common::types::VertexId;
    use minigu_common::value::ScalarValue;
    use minigu_transaction::{GraphTxnManager, IsolationLevel};

    use super::*;
    use crate::tp::memory_graph;

    #[test]
    fn test_checkpoint_creation() {
        // Create a graph with mock data
        let graph = memory_graph::tests::mock_graph();

        // Create checkpoint
        let checkpoint = GraphCheckpoint::new(&graph);

        // Verify checkpoint contents
        assert!(checkpoint.vertices.len() == 4);
        assert!(checkpoint.edges.len() == 4);

        let alice_vid: VertexId = VertexId::from(1u64);
        // Verify vertex data
        let alice_serialized = checkpoint.vertices.get(&alice_vid).unwrap();
        assert_eq!(alice_serialized.data.vid(), alice_vid);
        assert_eq!(
            alice_serialized.data.properties()[0],
            ScalarValue::String(Some("Alice".to_string()))
        );

        // Verify adjacency list
        let alice_adj = checkpoint.adjacency_list.get(&alice_vid).unwrap();
        assert!(alice_adj.outgoing.len() == 2);
        assert!(alice_adj.incoming.len() == 1);
    }

    #[test]
    fn test_checkpoint_restore() {
        // Create a graph with mock data
        let original_graph = memory_graph::tests::mock_graph();

        // Create checkpoint
        let checkpoint = GraphCheckpoint::new(&original_graph);

        // Create a new empty graph to restore into
        let restored_graph = memory_graph::tests::mock_empty_graph();

        // Restore graph from checkpoint
        checkpoint.restore(&restored_graph).unwrap();

        let origin_txn = original_graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        let restore_txn = restored_graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        // Check vertices
        let original_alice = original_graph
            .get_vertex(&origin_txn, VertexId::from(1u64))
            .unwrap();
        let restored_alice = restored_graph
            .get_vertex(&restore_txn, VertexId::from(1u64))
            .unwrap();
        assert_eq!(original_alice.vid(), restored_alice.vid());
        assert_eq!(original_alice.properties(), restored_alice.properties());

        let original_bob = original_graph
            .get_vertex(&origin_txn, VertexId::from(2u64))
            .unwrap();
        let restored_bob = restored_graph
            .get_vertex(&restore_txn, VertexId::from(2u64))
            .unwrap();
        assert_eq!(original_bob.vid(), restored_bob.vid());
        assert_eq!(original_bob.properties(), restored_bob.properties());

        // Check edges
        let original_friend_edge = original_graph
            .get_edge(&origin_txn, VertexId::from(1u64))
            .unwrap();
        let restored_friend_edge = restored_graph
            .get_edge(&restore_txn, VertexId::from(1u64))
            .unwrap();
        assert_eq!(original_friend_edge.eid(), restored_friend_edge.eid());
        assert_eq!(
            original_friend_edge.properties(),
            restored_friend_edge.properties()
        );

        let original_follow_edge = original_graph
            .get_edge(&origin_txn, VertexId::from(3u64))
            .unwrap();
        let restored_follow_edge = restored_graph
            .get_edge(&restore_txn, VertexId::from(3u64))
            .unwrap();
        assert_eq!(original_follow_edge.eid(), restored_follow_edge.eid());
        assert_eq!(
            original_follow_edge.properties(),
            restored_follow_edge.properties()
        );

        // Check adjacency list
        let original_alice_adj = original_graph
            .adjacency_list
            .get(&VertexId::from(1u64))
            .unwrap();
        let restored_alice_adj = restored_graph
            .adjacency_list
            .get(&VertexId::from(1u64))
            .unwrap();
        assert_eq!(
            original_alice_adj.outgoing.len(),
            restored_alice_adj.outgoing.len()
        );
        assert_eq!(
            original_alice_adj.incoming.len(),
            restored_alice_adj.incoming.len()
        );
    }
}
