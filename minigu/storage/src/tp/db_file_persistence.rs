//! DbFile-backed persistence implementation.
//!
//! This module provides `DbFilePersistence`, which implements `PersistenceProvider`
//! using the single-file database format (`.minigu`).

use std::sync::RwLock;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::common::wal::graph_wal::RedoEntry;
use crate::db_file::{SingleFileConfig, SingleFileManager};
use crate::error::{StorageError, StorageResult, WalError};
use crate::tp::checkpoint::GraphCheckpoint;
use crate::tp::persistence::PersistenceProvider;

/// DbFile-backed persistence provider.
///
/// This implementation stores all WAL and Checkpoint data in a single
/// `.minigu` file, using the integrated database file format.
pub struct DbFilePersistence {
    /// The underlying single-file manager.
    manager: RwLock<SingleFileManager>,
    /// Atomic LSN counter for thread-safe access.
    next_lsn: AtomicU64,
}

impl DbFilePersistence {
    /// Creates a new DbFilePersistence from a SingleFileManager.
    pub fn new(manager: SingleFileManager) -> Self {
        let last_lsn = manager.last_lsn();
        Self {
            manager: RwLock::new(manager),
            next_lsn: AtomicU64::new(last_lsn + 1),
        }
    }

    /// Opens or creates a database file at the given path.
    pub fn open<P: AsRef<std::path::Path>>(path: P) -> StorageResult<Self> {
        let config = SingleFileConfig::new(path);
        let manager = SingleFileManager::open(config)?;
        Ok(Self::new(manager))
    }

    /// Creates a new database file at the given path.
    /// Returns an error if the file already exists.
    pub fn create<P: AsRef<std::path::Path>>(path: P) -> StorageResult<Self> {
        let path_ref = path.as_ref();

        // Check if file already exists
        if path_ref.exists() {
            return Err(StorageError::Wal(WalError::Io(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("Database file already exists: {:?}", path_ref),
            ))));
        }

        let config = SingleFileConfig::new(path).with_create_if_missing(true);
        let manager = SingleFileManager::open(config)?;
        Ok(Self::new(manager))
    }
}

impl PersistenceProvider for DbFilePersistence {
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
        self.manager.write().unwrap().append_wal(entry)?;
        Ok(())
    }

    fn flush_wal(&self) -> StorageResult<()> {
        self.manager.write().unwrap().flush()?;
        Ok(())
    }

    fn read_wal_entries(&self) -> StorageResult<Vec<RedoEntry>> {
        let entries = self.manager.write().unwrap().read_wal_entries()?;
        Ok(entries)
    }

    fn truncate_wal_until(&self, min_lsn: u64) -> StorageResult<usize> {
        let count = self
            .manager
            .write()
            .unwrap()
            .db_file_mut()
            .truncate_wal_until(min_lsn)?;
        Ok(count)
    }

    fn write_checkpoint(&self, checkpoint: &GraphCheckpoint) -> StorageResult<()> {
        self.manager.write().unwrap().write_checkpoint(checkpoint)?;
        Ok(())
    }

    fn read_checkpoint(&self) -> StorageResult<Option<GraphCheckpoint>> {
        let checkpoint = self.manager.write().unwrap().read_checkpoint()?;
        Ok(checkpoint)
    }

    fn has_checkpoint(&self) -> bool {
        self.manager.read().unwrap().has_checkpoint()
    }

    fn sync_all(&self) -> StorageResult<()> {
        self.manager.write().unwrap().sync_all()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use minigu_transaction::{IsolationLevel, Timestamp};
    use serial_test::serial;

    use super::*;
    use crate::common::DeltaOp;
    use crate::common::wal::graph_wal::Operation;

    fn test_dir() -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("db_file_persistence_test_{}", std::process::id()));
        path
    }

    fn cleanup(path: &std::path::Path) {
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    #[serial]
    fn test_db_file_persistence_basic() {
        let base = test_dir();
        cleanup(&base);
        fs::create_dir_all(&base).unwrap();

        let db_path = base.join("test.minigu");
        let persistence = DbFilePersistence::open(&db_path).unwrap();

        // Test LSN management
        assert_eq!(persistence.current_lsn(), 1);
        assert_eq!(persistence.next_lsn(), 1);
        assert_eq!(persistence.next_lsn(), 2);
        assert_eq!(persistence.current_lsn(), 3);

        persistence.set_next_lsn(100);
        assert_eq!(persistence.current_lsn(), 100);

        cleanup(&base);
    }

    #[test]
    #[serial]
    fn test_db_file_persistence_wal() {
        let base = test_dir();
        cleanup(&base);
        fs::create_dir_all(&base).unwrap();

        let db_path = base.join("test.minigu");

        {
            let persistence = DbFilePersistence::open(&db_path).unwrap();

            for i in 1..=5 {
                let entry = RedoEntry {
                    lsn: persistence.next_lsn(),
                    txn_id: Timestamp::with_ts(100 + i),
                    iso_level: IsolationLevel::Serializable,
                    op: Operation::Delta(DeltaOp::DelVertex(i)),
                };
                persistence.append_wal(&entry).unwrap();
            }
            persistence.sync_all().unwrap();
        }

        // Reopen and verify
        {
            let persistence = DbFilePersistence::open(&db_path).unwrap();
            let entries = persistence.read_wal_entries().unwrap();
            assert_eq!(entries.len(), 5);
        }

        cleanup(&base);
    }
    #[test]
    #[serial]
    fn test_db_file_persistence_concurrent() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        let base = test_dir();
        cleanup(&base);
        fs::create_dir_all(&base).unwrap();

        let db_path = base.join("test_concurrent.minigu");
        let persistence = Arc::new(DbFilePersistence::open(&db_path).unwrap());

        let num_threads = 4;
        let ops_per_thread = 50;
        let barrier = Arc::new(Barrier::new(num_threads));

        let mut handles = vec![];

        for t_id in 0..num_threads {
            let persistence = persistence.clone();
            let barrier = barrier.clone();
            handles.push(thread::spawn(move || {
                barrier.wait();
                for i in 0..ops_per_thread {
                    let entry = RedoEntry {
                        lsn: persistence.next_lsn(), // concurrent atomic LSN generation
                        txn_id: Timestamp::with_ts(1000 + (t_id as u64) * 1000 + i),
                        iso_level: IsolationLevel::Serializable,
                        op: Operation::Delta(DeltaOp::DelVertex(i)),
                    };
                    persistence.append_wal(&entry).unwrap(); // concurrent write
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        persistence.sync_all().unwrap();

        // Reopen and verify total count
        {
            let persistence = DbFilePersistence::open(&db_path).unwrap();
            let entries = persistence.read_wal_entries().unwrap();
            assert_eq!(entries.len(), num_threads * ops_per_thread as usize);

            // Verify LSN uniqueness (optional but good sanity check)
            let mut lsns: Vec<u64> = entries.iter().map(|e| e.lsn).collect();
            lsns.sort_unstable();
            lsns.dedup();
            assert_eq!(lsns.len(), num_threads * ops_per_thread as usize);
        }

        cleanup(&base);
    }
}
