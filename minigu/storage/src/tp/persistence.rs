//! Persistence abstraction for TP storage layer.
//!
//! This module defines the `PersistenceProvider` trait which abstracts away
//! the physical storage details from the TP layer. This allows the graph
//! to work with different storage backends:
//! - `DbFilePersistence`: Single-file database format
//! - `InMemoryPersistence`: In-memory storage for testing

use crate::common::wal::graph_wal::RedoEntry;
use crate::error::StorageResult;
use crate::tp::checkpoint::GraphCheckpoint;

/// Abstraction for TP layer persistence operations.
///
/// The TP layer (transactions, MVCC, recovery) depends on this trait
/// instead of concrete WAL/Checkpoint implementations. This decouples
/// the transaction semantics from physical storage layout.
pub trait PersistenceProvider: Send + Sync {
    // ---- LSN Management ----

    /// Gets the next LSN and atomically increments the counter.
    fn next_lsn(&self) -> u64;

    /// Sets the next LSN (used during recovery).
    fn set_next_lsn(&self, lsn: u64);

    /// Gets the current LSN without incrementing.
    fn current_lsn(&self) -> u64;

    // ---- WAL Operations ----

    /// Appends a WAL entry.
    fn append_wal(&self, entry: &RedoEntry) -> StorageResult<()>;

    /// Flushes buffered WAL data to durable storage.
    fn flush_wal(&self) -> StorageResult<()>;

    /// Reads all WAL entries.
    fn read_wal_entries(&self) -> StorageResult<Vec<RedoEntry>>;

    /// Truncates WAL entries with LSN less than min_lsn.
    /// Returns the number of entries removed.
    fn truncate_wal_until(&self, min_lsn: u64) -> StorageResult<usize>;

    // ---- Checkpoint Operations ----

    /// Writes a checkpoint, replacing any existing one.
    fn write_checkpoint(&self, checkpoint: &GraphCheckpoint) -> StorageResult<()>;

    /// Reads the current checkpoint, if any.
    fn read_checkpoint(&self) -> StorageResult<Option<GraphCheckpoint>>;

    /// Checks if a checkpoint exists.
    fn has_checkpoint(&self) -> bool;

    // ---- Sync Operations ----

    /// Ensures all data is durably persisted (fsync).
    fn sync_all(&self) -> StorageResult<()>;
}
