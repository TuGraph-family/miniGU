//! Core database file implementation.
//!
//! This module provides the main `DbFile` struct for managing a single
//! database file containing both checkpoint and WAL regions.

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crc32fast::Hasher;

use super::error::{DbFileError, DbFileResult};
use super::header::{DbFileFlags, DbFileHeader};
use crate::common::wal::LogRecord;
use crate::common::wal::graph_wal::RedoEntry;
use crate::tp::checkpoint::GraphCheckpoint;

/// Record header size: 4 bytes length + 4 bytes CRC.
const RECORD_HEADER_SIZE: usize = 8;

/// A single database file containing checkpoint and WAL regions.
///
/// # Physical Layout
///
/// The file is structured as follows:
///
/// ```text
/// +----------------+--------------------------+-----------------------+
/// |  Header (256B) |  Checkpoint Region (Var) |    WAL Region (Var)   |
/// +----------------+--------------------------+-----------------------+
/// |                |                          |                       |
/// 0               256            header.wal_offset          EOF (FileSize)
/// ```
///
/// - **Header**: Fixed 256 bytes containing metadata and region offsets.
/// - **Checkpoint Region**: Stores the periodic snapshot of the graph.
///   - Located at `header.checkpoint_offset`.
///   - Size given by `header.checkpoint_length`.
/// - **WAL Region**: Stores Write-Ahead Log entries appended since the last checkpoint.
///   - Located at `header.wal_offset` (immediately follows checkpoint).
///   - Size given by `header.wal_length`.
///
/// When a new checkpoint is written:
/// 1. The old checkpoint is overwritten.
/// 2. `wal_offset` is updated to point after the new checkpoint.
/// 3. The effective WAL length is reset to 0 (truncation).
///
/// This struct manages all I/O operations for this unified database file format,
/// including:
/// - Creating new database files
/// - Opening existing database files
/// - Reading/writing checkpoint data
/// - Appending/reading WAL entries
/// - File compaction after checkpoint
pub struct DbFile {
    /// The underlying file handle.
    file: File,
    /// Path to the database file.
    path: PathBuf,
    /// Cached header.
    header: DbFileHeader,
}

impl DbFile {
    /// Creates a new database file at the specified path.
    ///
    /// This will fail if the file already exists.
    pub fn create<P: AsRef<Path>>(path: P) -> DbFileResult<Self> {
        let path = path.as_ref().to_path_buf();

        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)?;

        let mut header = DbFileHeader::new();
        header.update_crc();

        let mut db_file = Self { file, path, header };
        db_file.write_header()?;
        db_file.file.sync_all()?;

        Ok(db_file)
    }

    /// Opens an existing database file.
    pub fn open<P: AsRef<Path>>(path: P) -> DbFileResult<Self> {
        let path = path.as_ref().to_path_buf();

        let file = OpenOptions::new().read(true).write(true).open(&path)?;

        let mut db_file = Self {
            file,
            path,
            header: DbFileHeader::new(),
        };

        db_file.header = db_file.read_header()?;

        Ok(db_file)
    }

    /// Opens an existing database file or creates a new one if it doesn't exist.
    pub fn open_or_create<P: AsRef<Path>>(path: P) -> DbFileResult<Self> {
        let path = path.as_ref();
        if path.exists() {
            Self::open(path)
        } else {
            Self::create(path)
        }
    }

    /// Returns the path to the database file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns a reference to the current header.
    pub fn header(&self) -> &DbFileHeader {
        &self.header
    }

    /// Writes the header to the file.
    fn write_header(&mut self) -> DbFileResult<()> {
        self.header.write_to(&mut self.file)?;
        Ok(())
    }

    /// Reads the header from the file.
    fn read_header(&mut self) -> DbFileResult<DbFileHeader> {
        DbFileHeader::read_from(&mut self.file)
    }

    /// Reads the checkpoint from the file, if one exists.
    pub fn read_checkpoint(&mut self) -> DbFileResult<Option<GraphCheckpoint>> {
        if !self.header.flags.has(DbFileFlags::HAS_CHECKPOINT) {
            return Ok(None);
        }

        if self.header.checkpoint_length == 0 {
            return Ok(None);
        }

        self.file
            .seek(SeekFrom::Start(self.header.checkpoint_offset))?;

        // Read length and checksum
        let mut len_bytes = [0u8; 4];
        self.file.read_exact(&mut len_bytes)?;
        let len = u32::from_le_bytes(len_bytes) as usize;

        let mut crc_bytes = [0u8; 4];
        self.file.read_exact(&mut crc_bytes)?;
        let stored_crc = u32::from_le_bytes(crc_bytes);

        // Read payload
        let mut payload = vec![0u8; len];
        self.file.read_exact(&mut payload)?;

        // Verify checksum
        let mut hasher = Hasher::new();
        hasher.update(&payload);
        let computed_crc = hasher.finalize();

        if computed_crc != stored_crc {
            return Err(DbFileError::CheckpointChecksumMismatch);
        }

        // Deserialize
        let checkpoint: GraphCheckpoint = postcard::from_bytes(&payload)
            .map_err(|e| DbFileError::Deserialization(e.to_string()))?;

        Ok(Some(checkpoint))
    }

    /// Writes a checkpoint to the file.
    ///
    /// This will overwrite any existing checkpoint. After writing a checkpoint,
    /// the WAL region is reset (old entries are discarded).
    pub fn write_checkpoint(&mut self, checkpoint: &GraphCheckpoint) -> DbFileResult<()> {
        // Serialize the checkpoint
        let payload = postcard::to_allocvec(checkpoint)
            .map_err(|e| DbFileError::Serialization(e.to_string()))?;

        // Compute checksum
        let mut hasher = Hasher::new();
        hasher.update(&payload);
        let crc = hasher.finalize();

        let len = payload.len() as u32;
        let total_size = RECORD_HEADER_SIZE + payload.len();

        // Write checkpoint at the designated offset
        self.file
            .seek(SeekFrom::Start(self.header.checkpoint_offset))?;
        self.file.write_all(&len.to_le_bytes())?;
        self.file.write_all(&crc.to_le_bytes())?;
        self.file.write_all(&payload)?;

        // Update header
        self.header.checkpoint_length = total_size as u64;
        self.header.flags.set(DbFileFlags::HAS_CHECKPOINT);

        // WAL region starts right after checkpoint
        self.header.wal_offset = self.header.checkpoint_offset + total_size as u64;
        self.header.wal_length = 0; // Reset WAL after checkpoint
        self.header.flags.clear(DbFileFlags::HAS_WAL);

        self.header.last_lsn = checkpoint.metadata.lsn;
        self.header.last_commit_ts = checkpoint.metadata.latest_commit_ts;

        self.header.update_crc();
        self.write_header()?;

        // Sync to ensure durability
        self.file.sync_all()?;

        Ok(())
    }

    /// Appends a WAL entry to the file.
    ///
    /// This method ensures crash safety by:
    /// 1. Writing WAL data to disk
    /// 2. **Syncing WAL data first (fsync #1)**
    /// 3. Updating header with new wal_length and last_lsn
    /// 4. **Syncing header (fsync #2)**
    ///
    /// This double-sync approach guarantees that the header will NEVER point to
    /// WAL data that hasn't been durably written, even if the OS reorders page flushes.
    ///
    /// Performance note: This adds 2 fsync calls per WAL entry. For better performance
    /// in batch scenarios, consider implementing a batch WAL API in the future.
    pub fn append_wal(&mut self, entry: &RedoEntry) -> DbFileResult<()> {
        // Serialize the entry
        let payload = entry
            .to_bytes()
            .map_err(|e| DbFileError::Serialization(e.to_string()))?;

        // Compute checksum
        let mut hasher = Hasher::new();
        hasher.update(&payload);
        let crc = hasher.finalize();

        let len = payload.len() as u32;
        let entry_size = RECORD_HEADER_SIZE + payload.len();

        // Seek to end of WAL region
        let write_offset = self.header.wal_offset + self.header.wal_length;
        self.file.seek(SeekFrom::Start(write_offset))?;

        // Write entry
        self.file.write_all(&len.to_le_bytes())?;
        self.file.write_all(&crc.to_le_bytes())?;
        self.file.write_all(&payload)?;

        // CRITICAL: Sync WAL data BEFORE updating header
        // This ensures that if we crash after updating the header,
        // the WAL data it points to is guaranteed to be on disk
        self.file.sync_data()?;

        // Update header
        self.header.wal_length += entry_size as u64;
        self.header.flags.set(DbFileFlags::HAS_WAL);
        self.header.last_lsn = entry.lsn;

        self.header.update_crc();
        self.write_header()?;

        // Sync header to ensure it's durable
        self.file.sync_data()?;

        Ok(())
    }

    /// Flushes buffered data and syncs to disk.
    pub fn flush(&mut self) -> DbFileResult<()> {
        self.file.sync_data()?;
        Ok(())
    }

    /// Syncs all data and metadata to disk.
    pub fn sync_all(&mut self) -> DbFileResult<()> {
        self.file.sync_all()?;
        Ok(())
    }

    /// Reads all WAL entries from the file.
    ///
    /// This method includes defensive checks for crash recovery:
    /// - If a short read is encountered, it logs a warning and stops reading
    /// - If the last entry's LSN is less than header.last_lsn, it logs a warning
    pub fn read_wal_entries(&mut self) -> DbFileResult<Vec<RedoEntry>> {
        if !self.header.flags.has(DbFileFlags::HAS_WAL) {
            return Ok(Vec::new());
        }

        if self.header.wal_length == 0 {
            return Ok(Vec::new());
        }

        let mut entries = Vec::new();
        let wal_end = self.header.wal_offset + self.header.wal_length;
        let mut current_offset = self.header.wal_offset;

        while current_offset < wal_end {
            self.file.seek(SeekFrom::Start(current_offset))?;

            // Read length and checksum
            let mut len_bytes = [0u8; 4];
            match self.file.read_exact(&mut len_bytes) {
                Ok(_) => {}
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    // WAL truncated - header claims more data than actually exists
                    // This can happen if a crash occurred between writing WAL data and syncing
                    eprintln!(
                        "WARNING: WAL truncated at offset {}, expected {} more bytes. \
                         This may indicate incomplete write before crash.",
                        current_offset,
                        wal_end - current_offset
                    );
                    break;
                }
                Err(e) => return Err(e.into()),
            }
            let len = u32::from_le_bytes(len_bytes) as usize;

            let mut crc_bytes = [0u8; 4];
            self.file.read_exact(&mut crc_bytes)?;
            let stored_crc = u32::from_le_bytes(crc_bytes);

            // Read payload
            let mut payload = vec![0u8; len];
            self.file.read_exact(&mut payload)?;

            // Verify checksum
            let mut hasher = Hasher::new();
            hasher.update(&payload);
            let computed_crc = hasher.finalize();

            if computed_crc != stored_crc {
                return Err(DbFileError::WalChecksumMismatch {
                    offset: current_offset,
                });
            }

            // Deserialize
            let entry = RedoEntry::from_bytes(payload)
                .map_err(|e| DbFileError::Deserialization(e.to_string()))?;

            entries.push(entry);
            current_offset += (RECORD_HEADER_SIZE + len) as u64;
        }

        // Sort by LSN
        entries.sort_by_key(|e| e.lsn);

        // Defensive check: verify last entry LSN matches header
        if let Some(last_entry) = entries.last() {
            if last_entry.lsn < self.header.last_lsn {
                eprintln!(
                    "WARNING: Last WAL entry LSN {} < header.last_lsn {}. \
                     This may indicate data loss from incomplete write.",
                    last_entry.lsn, self.header.last_lsn
                );
            }
        } else if self.header.last_lsn > 0 {
            eprintln!(
                "WARNING: No WAL entries found but header.last_lsn = {}",
                self.header.last_lsn
            );
        }

        Ok(entries)
    }

    /// Returns the last LSN in the WAL.
    pub fn last_lsn(&self) -> u64 {
        self.header.last_lsn
    }

    /// Returns the last commit timestamp.
    pub fn last_commit_ts(&self) -> u64 {
        self.header.last_commit_ts
    }

    /// Updates the last commit timestamp in the header.
    pub fn set_last_commit_ts(&mut self, ts: u64) -> DbFileResult<()> {
        self.header.last_commit_ts = ts;
        self.header.update_crc();
        self.write_header()?;
        Ok(())
    }

    /// Truncates WAL entries with LSN less than the given minimum LSN.
    ///
    /// This is useful for WAL compaction after a checkpoint. The method:
    /// 1. Reads all current WAL entries
    /// 2. Filters to keep only entries with LSN >= min_lsn
    /// 3. Rewrites the WAL region with retained entries
    ///
    /// # Arguments
    ///
    /// * `min_lsn` - Minimum LSN to keep. Entries with LSN < min_lsn are removed.
    ///
    /// # Returns
    ///
    /// The number of entries that were removed.
    pub fn truncate_wal_until(&mut self, min_lsn: u64) -> DbFileResult<usize> {
        // Read all current entries
        let all_entries = self.read_wal_entries()?;
        let original_count = all_entries.len();

        // Filter to keep only entries >= min_lsn
        let retained: Vec<_> = all_entries
            .into_iter()
            .filter(|e| e.lsn >= min_lsn)
            .collect();

        let removed_count = original_count - retained.len();

        if removed_count == 0 {
            return Ok(0);
        }

        // Reset WAL region
        self.header.wal_length = 0;
        if retained.is_empty() {
            self.header.flags.clear(DbFileFlags::HAS_WAL);
        }

        self.header.update_crc();
        self.write_header()?;

        // Rewrite retained entries
        for entry in &retained {
            self.append_wal(entry)?;
        }

        self.sync_all()?;

        Ok(removed_count)
    }

    /// Returns database file statistics.
    pub fn stats(&self) -> DbFileStats {
        DbFileStats {
            header_size: self.header.header_size as u64,
            checkpoint_offset: self.header.checkpoint_offset,
            checkpoint_length: self.header.checkpoint_length,
            wal_offset: self.header.wal_offset,
            wal_length: self.header.wal_length,
            last_lsn: self.header.last_lsn,
            last_commit_ts: self.header.last_commit_ts,
            has_checkpoint: self.header.flags.has(DbFileFlags::HAS_CHECKPOINT),
            has_wal: self.header.flags.has(DbFileFlags::HAS_WAL),
        }
    }
}

/// Statistics about the database file.
#[derive(Debug, Clone)]
pub struct DbFileStats {
    /// Size of the header in bytes.
    pub header_size: u64,
    /// Offset of the checkpoint region.
    pub checkpoint_offset: u64,
    /// Length of the checkpoint region.
    pub checkpoint_length: u64,
    /// Offset of the WAL region.
    pub wal_offset: u64,
    /// Length of the WAL region.
    pub wal_length: u64,
    /// Last LSN in the WAL.
    pub last_lsn: u64,
    /// Last commit timestamp.
    pub last_commit_ts: u64,
    /// Whether a checkpoint exists.
    pub has_checkpoint: bool,
    /// Whether WAL entries exist.
    pub has_wal: bool,
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

    fn temp_db_path() -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("test_db_{}.minigu", std::process::id()));
        path
    }

    fn cleanup(path: &std::path::Path) {
        let _ = fs::remove_file(path);
    }

    #[test]
    #[serial]
    fn test_create_and_open() {
        let path = temp_db_path();
        cleanup(&path);

        // Create
        {
            let db = DbFile::create(&path).unwrap();
            assert_eq!(db.header().version, super::super::header::CURRENT_VERSION);
        }

        // Open
        {
            let db = DbFile::open(&path).unwrap();
            assert_eq!(db.header().version, super::super::header::CURRENT_VERSION);
        }

        cleanup(&path);
    }

    #[test]
    #[serial]
    fn test_wal_append_and_read() {
        let path = temp_db_path();
        cleanup(&path);

        {
            let mut db = DbFile::create(&path).unwrap();

            // Append entries
            for i in 1..=5 {
                let entry = RedoEntry {
                    lsn: i,
                    txn_id: Timestamp::with_ts(100 + i),
                    iso_level: IsolationLevel::Serializable,
                    op: Operation::Delta(DeltaOp::DelVertex(i)),
                };
                db.append_wal(&entry).unwrap();
            }
            db.flush().unwrap();

            // Read entries
            let entries = db.read_wal_entries().unwrap();
            assert_eq!(entries.len(), 5);
            for (i, entry) in entries.iter().enumerate() {
                assert_eq!(entry.lsn, (i + 1) as u64);
            }
        }

        cleanup(&path);
    }

    #[test]
    #[serial]
    fn test_open_or_create() {
        let path = temp_db_path();
        cleanup(&path);

        // Should create
        {
            let _db = DbFile::open_or_create(&path).unwrap();
            assert!(path.exists());
        }

        // Should open
        {
            let _db = DbFile::open_or_create(&path).unwrap();
        }

        cleanup(&path);
    }

    #[test]
    #[serial]
    fn test_truncate_wal_until() {
        let path = temp_db_path();
        cleanup(&path);

        {
            let mut db = DbFile::create(&path).unwrap();

            // Append 10 entries
            for i in 1..=10 {
                let entry = RedoEntry {
                    lsn: i,
                    txn_id: Timestamp::with_ts(100 + i),
                    iso_level: IsolationLevel::Serializable,
                    op: Operation::Delta(DeltaOp::DelVertex(i)),
                };
                db.append_wal(&entry).unwrap();
            }
            db.sync_all().unwrap();

            // Truncate entries with LSN < 6 (keep 6-10)
            let removed = db.truncate_wal_until(6).unwrap();
            assert_eq!(removed, 5);

            // Verify remaining entries
            let entries = db.read_wal_entries().unwrap();
            assert_eq!(entries.len(), 5);
            assert_eq!(entries[0].lsn, 6);
            assert_eq!(entries[4].lsn, 10);
        }

        cleanup(&path);
    }

    #[test]
    #[serial]
    fn test_stats() {
        let path = temp_db_path();
        cleanup(&path);

        {
            let mut db = DbFile::create(&path).unwrap();

            // Initial stats
            let stats = db.stats();
            assert_eq!(stats.header_size, 256);
            assert!(!stats.has_checkpoint);
            assert!(!stats.has_wal);

            // Add some WAL entries
            for i in 1..=3 {
                let entry = RedoEntry {
                    lsn: i,
                    txn_id: Timestamp::with_ts(100 + i),
                    iso_level: IsolationLevel::Serializable,
                    op: Operation::Delta(DeltaOp::DelVertex(i)),
                };
                db.append_wal(&entry).unwrap();
            }
            db.sync_all().unwrap();

            // Stats after WAL
            let stats = db.stats();
            assert!(stats.has_wal);
            assert!(stats.wal_length > 0);
            assert_eq!(stats.last_lsn, 3);
        }

        cleanup(&path);
    }
}
