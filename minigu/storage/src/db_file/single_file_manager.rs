//! Single-file database recovery and management.
//!
//! This module provides integration between the `DbFile` single-file format
//! and the `MemoryGraph` for recovery and persistence operations.

use std::path::{Path, PathBuf};

use super::core::DbFile;
use super::error::{DbFileError, DbFileResult};
use super::header::DbFileFlags;
use crate::common::wal::graph_wal::RedoEntry;
use crate::tp::checkpoint::GraphCheckpoint;

/// Configuration for single-file database operations.
#[derive(Debug, Clone)]
pub struct SingleFileConfig {
    /// Path to the database file.
    pub db_path: PathBuf,
    /// Whether to create the file if it doesn't exist.
    pub create_if_missing: bool,
    /// Automatic checkpoint interval in number of WAL entries (0 = disabled).
    pub auto_checkpoint_entries: usize,
}

impl SingleFileConfig {
    /// Creates a new configuration with the given database path.
    pub fn new<P: AsRef<Path>>(db_path: P) -> Self {
        Self {
            db_path: db_path.as_ref().to_path_buf(),
            create_if_missing: true,
            auto_checkpoint_entries: 0,
        }
    }

    /// Sets whether to create the file if it doesn't exist.
    pub fn with_create_if_missing(mut self, create: bool) -> Self {
        self.create_if_missing = create;
        self
    }

    /// Sets the automatic checkpoint interval.
    pub fn with_auto_checkpoint(mut self, entries: usize) -> Self {
        self.auto_checkpoint_entries = entries;
        self
    }
}

/// Manager for single-file database operations.
///
/// This struct wraps a `DbFile` and provides higher-level operations
/// for recovery, checkpointing, and WAL management.
pub struct SingleFileManager {
    /// The underlying database file.
    db_file: DbFile,
    /// Configuration.
    config: SingleFileConfig,
    /// Number of WAL entries since last checkpoint.
    entries_since_checkpoint: usize,
}

impl SingleFileManager {
    /// Opens or creates a single-file database.
    pub fn open(config: SingleFileConfig) -> DbFileResult<Self> {
        let db_file = if config.db_path.exists() {
            DbFile::open(&config.db_path)?
        } else if config.create_if_missing {
            DbFile::create(&config.db_path)?
        } else {
            return Err(DbFileError::InvalidFile(format!(
                "Database file not found: {}",
                config.db_path.display()
            )));
        };

        Ok(Self {
            db_file,
            config,
            entries_since_checkpoint: 0,
        })
    }

    /// Returns the path to the database file.
    pub fn path(&self) -> &Path {
        self.db_file.path()
    }

    /// Returns the last LSN in the database.
    pub fn last_lsn(&self) -> u64 {
        self.db_file.last_lsn()
    }

    /// Returns the last commit timestamp.
    pub fn last_commit_ts(&self) -> u64 {
        self.db_file.last_commit_ts()
    }

    /// Checks if the database has a checkpoint.
    pub fn has_checkpoint(&self) -> bool {
        self.db_file.header().flags.has(DbFileFlags::HAS_CHECKPOINT)
    }

    /// Checks if the database has WAL entries.
    pub fn has_wal(&self) -> bool {
        self.db_file.header().flags.has(DbFileFlags::HAS_WAL)
    }

    /// Reads the checkpoint from the database.
    pub fn read_checkpoint(&mut self) -> DbFileResult<Option<GraphCheckpoint>> {
        self.db_file.read_checkpoint()
    }

    /// Reads all WAL entries from the database.
    pub fn read_wal_entries(&mut self) -> DbFileResult<Vec<RedoEntry>> {
        self.db_file.read_wal_entries()
    }

    /// Appends a WAL entry to the database.
    pub fn append_wal(&mut self, entry: &RedoEntry) -> DbFileResult<()> {
        self.db_file.append_wal(entry)?;
        self.entries_since_checkpoint += 1;
        Ok(())
    }

    /// Writes a checkpoint to the database.
    ///
    /// This will reset the WAL region and update the checkpoint.
    pub fn write_checkpoint(&mut self, checkpoint: &GraphCheckpoint) -> DbFileResult<()> {
        self.db_file.write_checkpoint(checkpoint)?;
        self.entries_since_checkpoint = 0;
        Ok(())
    }

    /// Flushes buffered data to disk.
    pub fn flush(&mut self) -> DbFileResult<()> {
        self.db_file.flush()
    }

    /// Syncs all data and metadata to disk.
    pub fn sync_all(&mut self) -> DbFileResult<()> {
        self.db_file.sync_all()
    }

    /// Returns the number of WAL entries since the last checkpoint.
    pub fn entries_since_checkpoint(&self) -> usize {
        self.entries_since_checkpoint
    }

    /// Checks if an automatic checkpoint should be triggered.
    pub fn should_auto_checkpoint(&self) -> bool {
        self.config.auto_checkpoint_entries > 0
            && self.entries_since_checkpoint >= self.config.auto_checkpoint_entries
    }

    /// Returns a mutable reference to the underlying DbFile.
    pub fn db_file_mut(&mut self) -> &mut DbFile {
        &mut self.db_file
    }

    /// Returns a reference to the underlying DbFile.
    pub fn db_file(&self) -> &DbFile {
        &self.db_file
    }
}

/// Recovery data loaded from a single-file database.
#[derive(Debug)]
pub struct RecoveryData {
    /// The checkpoint, if one exists.
    pub checkpoint: Option<GraphCheckpoint>,
    /// WAL entries to replay.
    pub wal_entries: Vec<RedoEntry>,
    /// The last LSN from the file.
    pub last_lsn: u64,
}

impl RecoveryData {
    /// Loads recovery data from a single-file database.
    pub fn load(manager: &mut SingleFileManager) -> DbFileResult<Self> {
        let checkpoint = manager.read_checkpoint()?;
        let checkpoint_lsn = checkpoint.as_ref().map(|c| c.metadata.lsn).unwrap_or(0);

        let all_entries = manager.read_wal_entries()?;

        // Filter WAL entries to only include those >= checkpoint LSN
        let wal_entries: Vec<_> = all_entries
            .into_iter()
            .filter(|e| e.lsn >= checkpoint_lsn)
            .collect();

        Ok(Self {
            checkpoint,
            wal_entries,
            last_lsn: manager.last_lsn(),
        })
    }

    /// Returns true if there is data to recover.
    pub fn has_data(&self) -> bool {
        self.checkpoint.is_some() || !self.wal_entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use minigu_transaction::{IsolationLevel, Timestamp};
    use serial_test::serial;

    use super::*;
    use crate::common::DeltaOp;
    use crate::common::wal::graph_wal::Operation;

    fn test_dir() -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("single_file_test_{}", std::process::id()));
        path
    }

    fn cleanup(path: &Path) {
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    #[serial]
    fn test_single_file_manager_create() {
        let base = test_dir();
        cleanup(&base);
        fs::create_dir_all(&base).unwrap();

        let db_path = base.join("test.minigu");
        let config = SingleFileConfig::new(&db_path);

        let manager = SingleFileManager::open(config).unwrap();
        assert_eq!(manager.last_lsn(), 0);
        assert!(!manager.has_checkpoint());
        assert!(!manager.has_wal());

        cleanup(&base);
    }

    #[test]
    #[serial]
    fn test_single_file_manager_wal() {
        let base = test_dir();
        cleanup(&base);
        fs::create_dir_all(&base).unwrap();

        let db_path = base.join("test.minigu");
        let config = SingleFileConfig::new(&db_path);

        {
            let mut manager = SingleFileManager::open(config.clone()).unwrap();

            for i in 1..=10 {
                let entry = RedoEntry {
                    lsn: i,
                    txn_id: Timestamp::with_ts(100 + i),
                    iso_level: IsolationLevel::Serializable,
                    op: Operation::Delta(DeltaOp::DelVertex(i)),
                };
                manager.append_wal(&entry).unwrap();
            }
            manager.sync_all().unwrap();

            assert_eq!(manager.entries_since_checkpoint(), 10);
            assert!(manager.has_wal());
        }

        // Reopen and verify
        {
            let mut manager = SingleFileManager::open(config).unwrap();
            let entries = manager.read_wal_entries().unwrap();
            assert_eq!(entries.len(), 10);
        }

        cleanup(&base);
    }

    #[test]
    #[serial]
    fn test_recovery_data_load() {
        let base = test_dir();
        cleanup(&base);
        fs::create_dir_all(&base).unwrap();

        let db_path = base.join("test.minigu");
        let config = SingleFileConfig::new(&db_path);

        {
            let mut manager = SingleFileManager::open(config.clone()).unwrap();

            for i in 1..=5 {
                let entry = RedoEntry {
                    lsn: i,
                    txn_id: Timestamp::with_ts(100 + i),
                    iso_level: IsolationLevel::Serializable,
                    op: Operation::Delta(DeltaOp::DelVertex(i)),
                };
                manager.append_wal(&entry).unwrap();
            }
            manager.sync_all().unwrap();
        }

        // Load recovery data
        {
            let mut manager = SingleFileManager::open(config).unwrap();
            let recovery = RecoveryData::load(&mut manager).unwrap();

            assert!(recovery.has_data());
            assert!(recovery.checkpoint.is_none());
            assert_eq!(recovery.wal_entries.len(), 5);
        }

        cleanup(&base);
    }
}
