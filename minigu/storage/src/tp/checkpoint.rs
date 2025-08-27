// checkpoint.rs
// Implementation of checkpoint mechanism for MemoryGraph
//
// This module provides functionality to create and restore checkpoints of a MemoryGraph.
// A checkpoint represents a consistent snapshot of the graph state at a specific point in time.
// It can be used for backup, recovery, or state transfer purposes.

use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use crc32fast::Hasher;
use minigu_common::types::{EdgeId, VertexId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::memory_graph::{AdjacencyContainer, MemoryGraph, VersionedEdge, VersionedVertex};
use crate::common::model::edge::{Edge, Neighbor};
use crate::common::model::vertex::Vertex;
use crate::common::transaction::Timestamp;
use crate::common::wal::StorageWal;
use crate::common::wal::graph_wal::WalManagerConfig;
use crate::error::{CheckpointError, StorageError, StorageResult};

// @TODO: Consider making this configurable via
// CheckpointManagerConfig instead of a hardcoded constant.
const DEFAULT_CHECKPOINT_PREFIX: &str = "checkpoint";
const MAX_CHECKPOINTS: usize = 5;
const AUTO_CHECKPOINT_INTERVAL_SECS: u64 = 30;
const DEFAULT_CHECKPOINT_TIMEOUT_SECS: u64 = 30;
const DEFAULT_CHECKPOINT_DIR_NAME: &str = ".checkpoint";

/// Represents a checkpoint of a MemoryGraph at a specific point in time.
///
/// A GraphCheckpoint contains:
/// 1. Metadata about the checkpoint (timestamp, LSN, etc.)
/// 2. Serialized vertices and edges
/// 3. Adjacency list information
#[derive(Debug, Serialize, Deserialize)]
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
    /// It does **not** include historical versions of vertices or edges—only the
    /// latest committed state is serialized.
    ///
    /// This checkpoint can later be saved to disk using [`GraphCheckpoint::save_to_file`],
    /// and used for recovery via [`GraphCheckpoint::restore`] or the checkpoint manager.
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
        // Get current LSN
        let lsn = graph.wal_manager.next_lsn();

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

            vertices.insert(*entry.key(), SerializedVertex {
                data: current.data.clone(),
                commit_ts: current.commit_ts,
            });
        }

        // Serialize edges
        let mut edges = HashMap::new();
        for entry in graph.edges.iter() {
            let versioned_edge = entry.value();
            let current = versioned_edge.chain.current.read().unwrap();

            edges.insert(*entry.key(), SerializedEdge {
                data: current.data.clone(),
                commit_ts: current.commit_ts,
            });
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

    /// Saves the checkpoint to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> StorageResult<()> {
        let file =
            File::create(path).map_err(|e| StorageError::Checkpoint(CheckpointError::Io(e)))?;

        let mut writer = BufWriter::new(file);

        // Serialize the checkpoint
        let serialized = postcard::to_allocvec(self).map_err(|e| {
            StorageError::Checkpoint(CheckpointError::SerializationFailed(e.to_string()))
        })?;

        // Calculate checksum
        let mut hasher = Hasher::new();
        hasher.update(&serialized);
        let checksum = hasher.finalize();

        // Write length and checksum
        let len = serialized.len() as u32;
        writer
            .write_all(&len.to_le_bytes())
            .map_err(|e| StorageError::Checkpoint(CheckpointError::Io(e)))?;
        writer
            .write_all(&checksum.to_le_bytes())
            .map_err(|e| StorageError::Checkpoint(CheckpointError::Io(e)))?;

        // Write serialized data
        writer
            .write_all(&serialized)
            .map_err(|e| StorageError::Checkpoint(CheckpointError::Io(e)))?;

        // Flush to ensure data is written
        writer
            .flush()
            .map_err(|e| StorageError::Checkpoint(CheckpointError::Io(e)))?;

        Ok(())
    }

    /// Loads a checkpoint from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> StorageResult<Self> {
        let file =
            File::open(path).map_err(|e| StorageError::Checkpoint(CheckpointError::Io(e)))?;

        let mut reader = BufReader::new(file);

        // Read length and checksum
        let mut len_bytes = [0u8; 4];
        reader
            .read_exact(&mut len_bytes)
            .map_err(|e| StorageError::Checkpoint(CheckpointError::Io(e)))?;
        let len = u32::from_le_bytes(len_bytes) as usize;

        let mut checksum_bytes = [0u8; 4];
        reader
            .read_exact(&mut checksum_bytes)
            .map_err(|e| StorageError::Checkpoint(CheckpointError::Io(e)))?;
        let checksum = u32::from_le_bytes(checksum_bytes);

        // Read serialized data
        let mut serialized = vec![0u8; len];
        reader
            .read_exact(&mut serialized)
            .map_err(|e| StorageError::Checkpoint(CheckpointError::Io(e)))?;

        // Verify checksum
        let mut hasher = Hasher::new();
        hasher.update(&serialized);
        if hasher.finalize() != checksum {
            return Err(StorageError::Checkpoint(CheckpointError::ChecksumMismatch));
        }

        // Deserialize
        postcard::from_bytes(&serialized).map_err(|e| {
            StorageError::Checkpoint(CheckpointError::DeserializationFailed(e.to_string()))
        })
    }

    /// Restores a new [`MemoryGraph`] instance from this checkpoint snapshot.
    ///
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
    pub fn restore(
        &self,
        checkpoint_config: CheckpointManagerConfig,
        wal_config: WalManagerConfig,
    ) -> StorageResult<Arc<MemoryGraph>> {
        let graph = MemoryGraph::with_config_fresh(checkpoint_config, wal_config);

        // Set the LSN to the checkpoint's LSN
        graph.wal_manager.set_next_lsn(self.metadata.lsn);

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

        Ok(graph)
    }
}

/// Represents a checkpoint entry in the checkpoint manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointEntry {
    /// Unique identifier for the checkpoint
    pub id: String,

    /// Path to the checkpoint file
    pub path: PathBuf,

    /// Metadata about the checkpoint
    pub metadata: CheckpointMetadata,

    /// Description of the checkpoint (optional)
    pub description: Option<String>,

    /// Creation time of the checkpoint
    pub created_at: u64,
}

/// Configuration for the checkpoint manager
#[derive(Debug, Clone)]
pub struct CheckpointManagerConfig {
    /// Directory where checkpoints are stored
    pub checkpoint_dir: PathBuf,

    /// Maximum number of checkpoints to keep (0 means unlimited)
    pub max_checkpoints: usize,

    /// Automatic checkpoint interval in seconds (0 means disabled)
    pub auto_checkpoint_interval_secs: u64,

    /// Prefix for checkpoint filenames
    pub checkpoint_prefix: String,

    /// Timeout for waiting for active transactions to complete (in seconds)
    pub transaction_timeout_secs: u64,
}

fn default_checkpoint_dir() -> PathBuf {
    let dir = env::current_dir().unwrap();
    dir.join(DEFAULT_CHECKPOINT_DIR_NAME)
}

impl Default for CheckpointManagerConfig {
    fn default() -> Self {
        Self {
            checkpoint_dir: default_checkpoint_dir(),
            max_checkpoints: MAX_CHECKPOINTS,
            auto_checkpoint_interval_secs: AUTO_CHECKPOINT_INTERVAL_SECS,
            checkpoint_prefix: DEFAULT_CHECKPOINT_PREFIX.to_string(),
            transaction_timeout_secs: DEFAULT_CHECKPOINT_TIMEOUT_SECS,
        }
    }
}

/// Manages checkpoint creation, storage, and recovery for a [`MemoryGraph`].
///
/// The `CheckpointManager` is responsible for handling persistent snapshots of the graph
/// at specific points in time. It supports both manual and automatic checkpointing,
/// enforces retention policies, and enables full recovery of the graph state.
///
/// # Features
///
/// - Creates checkpoints on demand or at regular intervals.
/// - Lists and loads available checkpoints from disk.
/// - Restores a [`MemoryGraph`] from a given checkpoint file.
/// - Applies a retention policy to limit the number of stored checkpoints.
/// - Integrates with WAL (Write-Ahead Log) for consistent recovery.
pub struct CheckpointManager {
    /// Configuration for the checkpoint manager
    config: CheckpointManagerConfig,

    /// Reference to the graph being checkpointed
    graph: Arc<MemoryGraph>,

    /// Map of checkpoint ID to checkpoint entry
    checkpoints: HashMap<String, CheckpointEntry>,

    /// Last automatic checkpoint time
    last_auto_checkpoint: Option<SystemTime>,

    // Lock to ensure no transaction is trying to
    // update the graph while we are creating a checkpoint
    pub(super) checkpoint_lock: RwLock<()>,
}

impl CheckpointManager {
    /// Creates a new checkpoint manager for the given graph
    pub fn new(graph: Arc<MemoryGraph>, config: CheckpointManagerConfig) -> StorageResult<Self> {
        // Create checkpoint directory if it doesn't exist
        fs::create_dir_all(&config.checkpoint_dir)
            .map_err(|e| StorageError::Checkpoint(CheckpointError::Io(e)))?;

        let mut manager = Self {
            config,
            graph,
            checkpoints: HashMap::new(),
            last_auto_checkpoint: None,
            checkpoint_lock: RwLock::new(()),
        };

        // Load existing checkpoints
        manager.load_existing_checkpoints()?;

        Ok(manager)
    }

    /// Loads existing checkpoints from the checkpoint directory
    fn load_existing_checkpoints(&mut self) -> StorageResult<()> {
        let entries = fs::read_dir(&self.config.checkpoint_dir)
            .map_err(|e| StorageError::Checkpoint(CheckpointError::Io(e)))?;

        for entry in entries {
            let entry = entry.map_err(|e| StorageError::Checkpoint(CheckpointError::Io(e)))?;
            let path = entry.path();

            // Skip non-files and files that don't start with our prefix
            if !path.is_file()
                || !path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| name.starts_with(&self.config.checkpoint_prefix))
                    .unwrap_or(false)
            {
                continue;
            }

            // Try to load the checkpoint metadata
            match self.load_checkpoint_entry(&path) {
                Ok(entry) => {
                    self.checkpoints.insert(entry.id.clone(), entry);
                }
                Err(e) => {
                    // Log error but continue with other checkpoints
                    eprintln!("Failed to load checkpoint at {:?}: {:?}", path, e);
                }
            }
        }

        Ok(())
    }

    /// Loads a checkpoint entry from a file
    fn load_checkpoint_entry(&self, path: &Path) -> StorageResult<CheckpointEntry> {
        // Load the checkpoint to get its metadata
        let checkpoint = GraphCheckpoint::load_from_file(path)?;

        // Extract the ID from the filename
        let id = path
            .file_stem()
            .and_then(|name| name.to_str())
            .map(|name| name.trim_start_matches(&self.config.checkpoint_prefix))
            .map(|name| name.trim_start_matches('_').to_string())
            .ok_or_else(|| {
                StorageError::Checkpoint(CheckpointError::InvalidFormat(
                    "Invalid checkpoint filename".to_string(),
                ))
            })?;

        let timestamp = checkpoint.metadata.timestamp;

        Ok(CheckpointEntry {
            id,
            path: path.to_path_buf(),
            metadata: checkpoint.metadata,
            description: None, // No description stored in the file currently
            created_at: timestamp,
        })
    }

    /// Creates a new checkpoint
    pub fn create_checkpoint(&mut self, description: Option<String>) -> StorageResult<String> {
        let checkpoint;
        {
            // Acquire the checkpoint lock
            let _lock = self.checkpoint_lock.write().unwrap();

            // Wait for active transactions to complete
            self.wait_for_transaction_quiescence()?;

            // Create a new checkpoint
            checkpoint = GraphCheckpoint::new(&self.graph);

            // Truncate the WAL, keeping only entries after the checkpoint's LSN
            self.graph
                .wal_manager
                .truncate_until(checkpoint.metadata.lsn)?;
        }

        // Generate a unique ID for the checkpoint
        let id = Uuid::new_v4().to_string();

        // Create the checkpoint file path
        let filename = format!("{}_{}.bin", self.config.checkpoint_prefix, id);
        let path = self.config.checkpoint_dir.join(filename);

        // Save the checkpoint to file
        checkpoint.save_to_file(&path)?;

        // Create and store the checkpoint entry
        let entry = CheckpointEntry {
            id: id.clone(),
            path,
            metadata: checkpoint.metadata.clone(),
            description,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };

        self.checkpoints.insert(id.clone(), entry);

        // Update last auto checkpoint time
        self.last_auto_checkpoint = Some(SystemTime::now());

        // Apply retention policy
        self.apply_retention_policy()?;

        Ok(id)
    }

    fn wait_for_transaction_quiescence(&self) -> StorageResult<()> {
        // Wait for active transactions to complete
        let start_time = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(self.config.transaction_timeout_secs);
        while !self.graph.txn_manager.active_txns.is_empty() {
            if start_time.elapsed() > timeout {
                return Err(StorageError::Checkpoint(CheckpointError::Timeout));
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        Ok(())
    }

    /// Lists all available checkpoints
    pub fn list_checkpoints(&self) -> Vec<&CheckpointEntry> {
        // Sort by creation time (newest first)
        let mut checkpoints: Vec<&CheckpointEntry> = self.checkpoints.values().collect();
        checkpoints.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        checkpoints
    }

    /// Gets a specific checkpoint by ID
    pub fn get_checkpoint(&self, id: &str) -> StorageResult<&CheckpointEntry> {
        self.checkpoints.get(id).ok_or_else(|| {
            StorageError::Checkpoint(CheckpointError::CheckpointNotFound(id.to_string()))
        })
    }

    /// Loads a checkpoint by ID
    pub fn load_checkpoint(&self, id: &str) -> StorageResult<GraphCheckpoint> {
        let entry = self.get_checkpoint(id)?;
        GraphCheckpoint::load_from_file(&entry.path)
    }

    /// Restores the graph from a checkpoint
    pub fn restore_from_checkpoint(
        &self,
        id: &str,
        checkpoint_config: CheckpointManagerConfig,
        wal_config: WalManagerConfig,
    ) -> StorageResult<Arc<MemoryGraph>> {
        let checkpoint = self.load_checkpoint(id)?;

        // Restore the graph
        checkpoint.restore(checkpoint_config, wal_config)
    }

    /// Deletes a checkpoint by ID
    pub fn delete_checkpoint(&mut self, id: &str) -> StorageResult<()> {
        let entry = self.checkpoints.remove(id).ok_or_else(|| {
            StorageError::Checkpoint(CheckpointError::CheckpointNotFound(id.to_string()))
        })?;

        // Delete the file
        fs::remove_file(&entry.path)
            .map_err(|e| StorageError::Checkpoint(CheckpointError::Io(e)))?;

        Ok(())
    }

    /// Applies the retention policy (keeps only the N most recent checkpoints)
    fn apply_retention_policy(&mut self) -> StorageResult<()> {
        if self.config.max_checkpoints == 0 || self.checkpoints.len() <= self.config.max_checkpoints
        {
            return Ok(());
        }

        // Sort checkpoints by creation time (oldest first)
        let mut checkpoints: Vec<(String, u64)> = self
            .checkpoints
            .iter()
            .map(|(id, entry)| (id.clone(), entry.created_at))
            .collect();

        checkpoints.sort_by_key(|(_, time)| *time);

        // Delete oldest checkpoints that exceed the limit
        let to_delete = checkpoints.len() - self.config.max_checkpoints;
        for (id, _) in checkpoints.into_iter().take(to_delete) {
            self.delete_checkpoint(&id)?;
        }

        Ok(())
    }

    /// Checks if an automatic checkpoint should be created
    pub fn check_auto_checkpoint(&mut self) -> StorageResult<Option<String>> {
        if self.config.auto_checkpoint_interval_secs == 0 {
            return Ok(None);
        }

        let now = SystemTime::now();
        let should_checkpoint = match self.last_auto_checkpoint {
            Some(last) => now
                .duration_since(last)
                .map(|duration| duration.as_secs() >= self.config.auto_checkpoint_interval_secs)
                .unwrap_or(true),
            None => true,
        };

        if should_checkpoint {
            let description = Some(format!("Auto checkpoint at {}", chrono::Local::now()));
            let id = self.create_checkpoint(description)?;
            Ok(Some(id))
        } else {
            Ok(None)
        }
    }
}

impl MemoryGraph {
    /// Recovers a [`MemoryGraph`] by loading the latest checkpoint and replaying WAL entries.
    ///
    /// This method implements a two-phase recovery process:
    ///
    /// 1. **Checkpoint-based Recovery**   If a valid checkpoint exists in the configured directory,
    ///    the graph is restored from it, and all WAL entries with LSN ≥ checkpoint LSN are applied
    ///    to reach the latest consistent state.
    ///
    /// 2. **WAL-only Recovery**   If no checkpoint is found, the graph is initialized empty and
    ///    recovered solely from WAL entries.
    ///
    /// # Returns
    ///
    /// A fully recovered [`Arc<MemoryGraph>`] containing the most recent state reconstructed
    /// from persisted checkpoints and logs.
    pub fn recover_from_checkpoint_and_wal(
        checkpoint_config: CheckpointManagerConfig,
        wal_config: WalManagerConfig,
    ) -> StorageResult<Arc<Self>> {
        // Create checkpoint directory if it doesn't exist
        fs::create_dir_all(&checkpoint_config.checkpoint_dir)
            .map_err(|e| StorageError::Checkpoint(CheckpointError::Io(e)))?;

        // Find the most recent checkpoint
        let checkpoint_path = Self::find_most_recent_checkpoint(&checkpoint_config)?;

        // If no checkpoint found, create a new empty graph
        if checkpoint_path.is_none() {
            let graph = Self::with_config_fresh(checkpoint_config.clone(), wal_config.clone());
            graph.recover_from_wal()?;
            return Ok(graph);
        }

        // Restore from checkpoint
        let checkpoint = GraphCheckpoint::load_from_file(checkpoint_path.unwrap())?;
        let checkpoint_lsn = checkpoint.metadata.lsn;
        let graph = checkpoint.restore(checkpoint_config, wal_config)?;

        // Read WAL entries with LSN >= checkpoint_lsn
        let all_entries = graph.wal_manager.wal().read().unwrap().read_all()?;

        let new_entries: Vec<_> = all_entries
            .into_iter()
            .filter(|entry| entry.lsn >= checkpoint_lsn)
            .collect();

        // Apply new WAL entries
        if !new_entries.is_empty() {
            graph.apply_wal_entries(new_entries)?;
        }

        Ok(graph)
    }

    /// Finds the most recent checkpoint in the checkpoint directory
    fn find_most_recent_checkpoint(
        config: &CheckpointManagerConfig,
    ) -> StorageResult<Option<PathBuf>> {
        let entries = match fs::read_dir(&config.checkpoint_dir) {
            Ok(entries) => entries,
            Err(e) => return Err(StorageError::Checkpoint(CheckpointError::Io(e))),
        };

        let mut latest_checkpoint: Option<(PathBuf, SystemTime)> = None;

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => return Err(StorageError::Checkpoint(CheckpointError::Io(e))),
            };

            let path = entry.path();

            // Skip non-files and files that don't start with our prefix
            if !path.is_file()
                || !path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| name.starts_with(&config.checkpoint_prefix))
                    .unwrap_or(false)
            {
                continue;
            }

            // Get file metadata to check modification time
            let metadata = match fs::metadata(&path) {
                Ok(metadata) => metadata,
                Err(e) => return Err(StorageError::Checkpoint(CheckpointError::Io(e))),
            };

            let modified = match metadata.modified() {
                Ok(time) => time,
                Err(e) => return Err(StorageError::Checkpoint(CheckpointError::Io(e))),
            };

            // Update latest checkpoint if this one is newer
            if let Some((_, latest_time)) = &latest_checkpoint {
                if modified > *latest_time {
                    latest_checkpoint = Some((path, modified));
                }
            } else {
                latest_checkpoint = Some((path, modified));
            }
        }

        Ok(latest_checkpoint.map(|(path, _)| path))
    }

    /// Creates a checkpoint using the checkpoint manager
    pub fn create_managed_checkpoint(&self, description: Option<String>) -> StorageResult<String> {
        match &self.checkpoint_manager {
            Some(manager) => {
                // Need to get a mutable reference to the manager
                // This is safe because we're only modifying the manager's internal state
                let manager_ptr = manager as *const CheckpointManager as *mut CheckpointManager;
                unsafe { (*manager_ptr).create_checkpoint(description) }
            }
            None => Err(StorageError::Checkpoint(
                crate::error::CheckpointError::DirectoryError(
                    "No checkpoint manager configured".to_string(),
                ),
            )),
        }
    }

    /// Checks if an automatic checkpoint should be created
    pub fn check_auto_checkpoint(&self) -> StorageResult<Option<String>> {
        match &self.checkpoint_manager {
            Some(manager) => {
                // Need to get a mutable reference to the manager
                // This is safe because we're only modifying the manager's internal state
                let manager_ptr = manager as *const CheckpointManager as *mut CheckpointManager;
                unsafe { (*manager_ptr).check_auto_checkpoint() }
            }
            None => Ok(None), // No checkpoint manager, so no auto checkpoint
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Seek;
    use std::{env, fs};

    use minigu_common::value::ScalarValue;

    use super::*;
    use crate::error::CheckpointError;
    use crate::tp::memory_graph;
    use crate::tp::transaction::IsolationLevel;

    fn get_temp_file_path(prefix: &str) -> std::path::PathBuf {
        env::temp_dir().join(format!("{}_{}.bin", prefix, std::process::id()))
    }

    #[test]
    fn test_checkpoint_creation() {
        // Create a graph with mock data
        let (graph, _cleaner) = memory_graph::tests::mock_graph();

        // Create checkpoint
        let checkpoint = GraphCheckpoint::new(&graph);

        // Verify checkpoint contents
        assert!(checkpoint.vertices.len() == 4);
        assert!(checkpoint.edges.len() == 4);

        let alice_vid: VertexId = 1;
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
    fn test_checkpoint_save_and_load() {
        // Create a graph with mock data
        let (graph, _cleaner) = memory_graph::tests::mock_graph();

        // Create and save checkpoint
        let checkpoint_path = get_temp_file_path("checkpoint_save_load");
        let checkpoint = GraphCheckpoint::new(&graph);
        checkpoint.save_to_file(&checkpoint_path).unwrap();

        // Load checkpoint
        let loaded_checkpoint = GraphCheckpoint::load_from_file(&checkpoint_path).unwrap();

        // Verify loaded checkpoint has same number of elements
        assert_eq!(loaded_checkpoint.vertices.len(), checkpoint.vertices.len());
        assert_eq!(loaded_checkpoint.edges.len(), checkpoint.edges.len());
        assert_eq!(
            loaded_checkpoint.adjacency_list.len(),
            checkpoint.adjacency_list.len()
        );

        // Clean up
        fs::remove_file(checkpoint_path).unwrap();
    }

    #[test]
    fn test_checkpoint_restore() {
        // Create a graph with mock data
        let checkpoint_config = memory_graph::tests::mock_checkpoint_config();
        let wal_config = memory_graph::tests::mock_wal_config();
        let (original_graph, _cleaner) = memory_graph::tests::mock_graph_with_config(
            checkpoint_config.clone(),
            wal_config.clone(),
        );

        // Create checkpoint
        let checkpoint = GraphCheckpoint::new(&original_graph);

        // Restore graph from checkpoint
        let restored_graph = checkpoint.restore(checkpoint_config, wal_config).unwrap();

        let origin_txn = original_graph.begin_transaction(IsolationLevel::Serializable);
        let restore_txn = restored_graph.begin_transaction(IsolationLevel::Serializable);

        // Check vertices
        let original_alice = original_graph.get_vertex(&origin_txn, 1).unwrap();
        let restored_alice = restored_graph.get_vertex(&restore_txn, 1).unwrap();
        assert_eq!(original_alice.vid(), restored_alice.vid());
        assert_eq!(original_alice.properties(), restored_alice.properties());

        let original_bob = original_graph.get_vertex(&origin_txn, 2).unwrap();
        let restored_bob = restored_graph.get_vertex(&restore_txn, 2).unwrap();
        assert_eq!(original_bob.vid(), restored_bob.vid());
        assert_eq!(original_bob.properties(), restored_bob.properties());

        // Check edges
        let original_friend_edge = original_graph.get_edge(&origin_txn, 1).unwrap();
        let restored_friend_edge = restored_graph.get_edge(&restore_txn, 1).unwrap();
        assert_eq!(original_friend_edge.eid(), restored_friend_edge.eid());
        assert_eq!(
            original_friend_edge.properties(),
            restored_friend_edge.properties()
        );

        let original_follow_edge = original_graph.get_edge(&origin_txn, 3).unwrap();
        let restored_follow_edge = restored_graph.get_edge(&restore_txn, 3).unwrap();
        assert_eq!(original_follow_edge.eid(), restored_follow_edge.eid());
        assert_eq!(
            original_follow_edge.properties(),
            restored_follow_edge.properties()
        );

        // Check adjacency list
        let original_alice_adj = original_graph.adjacency_list.get(&1).unwrap();
        let restored_alice_adj = restored_graph.adjacency_list.get(&1).unwrap();
        assert_eq!(
            original_alice_adj.outgoing.len(),
            restored_alice_adj.outgoing.len()
        );
        assert_eq!(
            original_alice_adj.incoming.len(),
            restored_alice_adj.incoming.len()
        );
    }

    #[test]
    fn test_checkpoint_with_corrupted_file() {
        // Create a graph with mock data
        let checkpoint_config = memory_graph::tests::mock_checkpoint_config();
        let wal_config = memory_graph::tests::mock_wal_config();
        let (graph, _cleaner) =
            memory_graph::tests::mock_graph_with_config(checkpoint_config, wal_config.clone());

        // Create and save checkpoint
        let checkpoint_path = get_temp_file_path("checkpoint_corrupted");
        let checkpoint = GraphCheckpoint::new(&graph);
        checkpoint.save_to_file(&checkpoint_path).unwrap();

        // Corrupt the file
        {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .open(&checkpoint_path)
                .unwrap();
            file.seek(std::io::SeekFrom::Start(8)).unwrap(); // Skip length and checksum
            file.write_all(&[0, 0, 0, 0]).unwrap(); // Write some garbage
        }

        // Try to load the corrupted checkpoint
        let result = GraphCheckpoint::load_from_file(&checkpoint_path);
        assert!(result.is_err());

        // Verify it's a checksum error
        match result {
            Err(StorageError::Checkpoint(CheckpointError::ChecksumMismatch)) => {}
            _ => panic!("Expected checksum mismatch error"),
        }
    }

    #[test]
    #[ignore]
    fn test_checkpoint_manager() {
        // Create a graph with mock data
        let checkpoint_config = memory_graph::tests::mock_checkpoint_config();
        let wal_config = memory_graph::tests::mock_wal_config();
        let (graph, _cleaner) = memory_graph::tests::mock_graph_with_config(
            checkpoint_config.clone(),
            wal_config.clone(),
        );

        // Create a checkpoint manager
        let mut manager = CheckpointManager::new(graph.clone(), checkpoint_config.clone()).unwrap();

        // Create 5 checkpoints
        let mut checkpoint_ids = Vec::new();
        for i in 0..5 {
            let description = Some(format!("Test checkpoint {}", i));
            let id = manager.create_checkpoint(description).unwrap();
            // sleep for 1 second to make sure the created_at time is different
            std::thread::sleep(std::time::Duration::from_millis(1000));
            checkpoint_ids.push(id);
        }

        // Verify we only have 3 checkpoints (due to retention policy)
        let checkpoints = manager.list_checkpoints();
        assert_eq!(checkpoints.len(), 3);

        // Verify the oldest 2 checkpoints were deleted
        assert!(!manager.checkpoints.contains_key(&checkpoint_ids[0]));
        assert!(!manager.checkpoints.contains_key(&checkpoint_ids[1]));

        // Verify the newest 3 checkpoints are still there
        assert!(manager.checkpoints.contains_key(&checkpoint_ids[2]));
        assert!(manager.checkpoints.contains_key(&checkpoint_ids[3]));
        assert!(manager.checkpoints.contains_key(&checkpoint_ids[4]));

        // Load a checkpoint
        let checkpoint = manager.load_checkpoint(&checkpoint_ids[4]).unwrap();
        assert_eq!(checkpoint.vertices.len(), 4); // Should have 4 vertices from mock graph

        // Restore from a checkpoint
        let restored_graph = manager
            .restore_from_checkpoint(&checkpoint_ids[4], checkpoint_config, wal_config)
            .unwrap();

        // Verify the restored graph has the same data
        let original_txn = graph.begin_transaction(IsolationLevel::Serializable);
        let restored_txn = restored_graph.begin_transaction(IsolationLevel::Serializable);

        // Check vertices
        let original_alice = graph.get_vertex(&original_txn, 1).unwrap();
        let restored_alice = restored_graph.get_vertex(&restored_txn, 1).unwrap();
        assert_eq!(original_alice.vid(), restored_alice.vid());

        // Delete a checkpoint
        manager.delete_checkpoint(&checkpoint_ids[4]).unwrap();
        assert!(!manager.checkpoints.contains_key(&checkpoint_ids[4]));
    }
}
