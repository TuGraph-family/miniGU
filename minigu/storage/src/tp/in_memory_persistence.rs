//! In-memory persistence implementation for testing.
//!
//! This module provides `InMemoryPersistence`, which implements `PersistenceProvider`
//! without any disk I/O. This is intended for unit tests that need to test
//! transaction semantics without caring about physical persistence.

use std::sync::RwLock;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::common::wal::graph_wal::RedoEntry;
use crate::error::StorageResult;
use crate::tp::checkpoint::GraphCheckpoint;
use crate::tp::persistence::PersistenceProvider;

/// In-memory persistence provider for testing.
///
/// This implementation stores all WAL entries and checkpoint data in memory,
/// without any disk I/O. It is designed for unit tests that need to test
/// transaction semantics without generating any files.
pub struct InMemoryPersistence {
    /// Atomic LSN counter.
    next_lsn: AtomicU64,
    /// In-memory WAL entries.
    wal_entries: RwLock<Vec<RedoEntry>>,
    /// In-memory checkpoint.
    checkpoint: RwLock<Option<GraphCheckpoint>>,
}

impl InMemoryPersistence {
    /// Creates a new in-memory persistence provider.
    pub fn new() -> Self {
        Self {
            next_lsn: AtomicU64::new(1),
            wal_entries: RwLock::new(Vec::new()),
            checkpoint: RwLock::new(None),
        }
    }
}

impl Default for InMemoryPersistence {
    fn default() -> Self {
        Self::new()
    }
}

impl PersistenceProvider for InMemoryPersistence {
    fn next_lsn(&self) -> u64 {
        self.next_lsn.fetch_add(1, Ordering::SeqCst)
    }

    fn set_next_lsn(&self, lsn: u64) {
        self.next_lsn.store(lsn, Ordering::SeqCst);
    }

    fn current_lsn(&self) -> u64 {
        self.next_lsn.load(Ordering::SeqCst)
    }

    fn append_wal(&self, entry: &RedoEntry) -> StorageResult<()> {
        self.wal_entries.write().unwrap().push(entry.clone());
        Ok(())
    }

    fn flush_wal(&self) -> StorageResult<()> {
        // No-op for in-memory
        Ok(())
    }

    fn read_wal_entries(&self) -> StorageResult<Vec<RedoEntry>> {
        let entries = self.wal_entries.read().unwrap().clone();
        Ok(entries)
    }

    fn truncate_wal_until(&self, min_lsn: u64) -> StorageResult<usize> {
        let mut entries = self.wal_entries.write().unwrap();
        let original_len = entries.len();
        entries.retain(|e| e.lsn >= min_lsn);
        Ok(original_len - entries.len())
    }

    fn write_checkpoint(&self, checkpoint: &GraphCheckpoint) -> StorageResult<()> {
        *self.checkpoint.write().unwrap() = Some(checkpoint.clone());
        Ok(())
    }

    fn read_checkpoint(&self) -> StorageResult<Option<GraphCheckpoint>> {
        Ok(self.checkpoint.read().unwrap().clone())
    }

    fn has_checkpoint(&self) -> bool {
        self.checkpoint.read().unwrap().is_some()
    }

    fn sync_all(&self) -> StorageResult<()> {
        // No-op for in-memory
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use minigu_transaction::{IsolationLevel, Timestamp};

    use super::*;
    use crate::common::DeltaOp;
    use crate::common::wal::graph_wal::Operation;

    #[test]
    fn test_in_memory_persistence_lsn() {
        let persistence = InMemoryPersistence::new();

        assert_eq!(persistence.current_lsn(), 1);
        assert_eq!(persistence.next_lsn(), 1);
        assert_eq!(persistence.next_lsn(), 2);
        assert_eq!(persistence.current_lsn(), 3);

        persistence.set_next_lsn(100);
        assert_eq!(persistence.current_lsn(), 100);
    }

    #[test]
    fn test_in_memory_persistence_wal() {
        let persistence = InMemoryPersistence::new();

        for i in 1..=5 {
            let entry = RedoEntry {
                lsn: persistence.next_lsn(),
                txn_id: Timestamp::with_ts(100 + i),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::DelVertex(i)),
            };
            persistence.append_wal(&entry).unwrap();
        }

        let entries = persistence.read_wal_entries().unwrap();
        assert_eq!(entries.len(), 5);

        // Test truncation
        let removed = persistence.truncate_wal_until(3).unwrap();
        assert_eq!(removed, 2); // LSN 1 and 2 removed

        let entries = persistence.read_wal_entries().unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].lsn, 3);
    }

    #[test]
    fn test_in_memory_persistence_checkpoint() {
        let persistence = InMemoryPersistence::new();

        assert!(!persistence.has_checkpoint());
        assert!(persistence.read_checkpoint().unwrap().is_none());

        // We can't easily create a GraphCheckpoint without a MemoryGraph,
        // so we just test the interface behavior here.
    }
}
