// graph_wal.rs
// Minimal standalone Write‑Ahead Log (WAL) implementation for an **in‑memory graph database**.
//
// Log record layout (little‑endian):
// ┌────────────┬────────────┬───────────┐
// │ u32 len    │ u32 crc32  │ payload…  │
// └────────────┴────────────┴───────────┘
// - `len`    : number of bytes in payload
// - `crc32`  : checksum of payload for corruption detection
//
// Basic API:
//   let mut wal = GraphWal::open("graph.wal")?;
//   wal.append(&serialized_bytes)?;
//   wal.flush()?; // durable after crash
//
//   for rec in GraphWal::iter("graph.wal")? {
//       let bytes = rec?;
//       ... // apply to in‑memory state
//   }
//
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use crc32fast::Hasher;
use serde::{Deserialize, Serialize};

use super::{LogRecord, StorageWal};
use crate::common::transaction::{DeltaOp, IsolationLevel, Timestamp};
use crate::error::{StorageError, StorageResult, WalError};

const HEADER_SIZE: usize = 8; // 4 bytes length + 4 bytes crc32

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedoEntry {
    pub lsn: u64,                  // Log sequence number
    pub txn_id: Timestamp,         // Transaction ID
    pub iso_level: IsolationLevel, // Isolation level
    pub op: Operation,             // Operation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    BeginTransaction(Timestamp),  // transaction start timestamp
    CommitTransaction(Timestamp), // transaction commit timestamp
    AbortTransaction,
    Delta(DeltaOp), // Delta operation
}

impl LogRecord for RedoEntry {
    fn to_bytes(&self) -> StorageResult<Vec<u8>> {
        postcard::to_allocvec(self)
            .map_err(|e| StorageError::Wal(WalError::SerializationFailed(e.to_string())))
    }

    fn from_bytes(bytes: Vec<u8>) -> StorageResult<Self> {
        postcard::from_bytes(&bytes)
            .map_err(|e| StorageError::Wal(WalError::DeserializationFailed(e.to_string())))
    }
}

/// Write‑ahead log in append‑only mode, tailored for an in‑memory graph store.
pub struct GraphWal {
    pub file: BufWriter<File>,
    pub path: PathBuf,
}

impl StorageWal for GraphWal {
    type Record = RedoEntry;

    type LogIterator = impl Iterator<Item = StorageResult<Self::Record>>;

    /// Open existing log or create a new one at `path`.
    fn open<P: AsRef<Path>>(path: P) -> StorageResult<Self> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent).map_err(|e| StorageError::Wal(WalError::Io(e)))?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .read(true)
            .open(&path)
            .map_err(|e| StorageError::Wal(WalError::Io(e)))?;

        // Seek to the end of the file
        file.seek(SeekFrom::End(0))
            .map_err(|e| StorageError::Wal(WalError::Io(e)))?;

        Ok(Self {
            file: BufWriter::new(file),
            path: path.as_ref().to_path_buf(),
        })
    }

    /// Append a record and buffer it. Call `flush` to fsync.
    ///
    /// This method performs the following steps to ensure atomicity and data integrity:
    ///
    /// - Serializes the input `record` into a byte payload.
    /// - Computes a CRC32 checksum of the payload for integrity validation.
    /// - Captures the current file position to allow rollback in case of failure.
    /// - Constructs the full binary record in the format: `[length (4 bytes)] [checksum (4 bytes)]
    ///   [payload]`.
    /// - Attempts to write the entire record in a single operation.
    ///
    /// If the write fails, the file is truncated back to the original position
    /// to prevent partial or corrupted writes.
    ///
    /// # Errors
    ///
    /// Returns a [`StorageError::Wal`] variant if any I/O operation fails during
    /// seeking, writing, or truncating.
    fn append(&mut self, record: &Self::Record) -> StorageResult<()> {
        let payload = record.to_bytes()?;
        let mut hasher = Hasher::new();
        hasher.update(&payload);
        let checksum = hasher.finalize();
        let len = payload.len() as u32;

        // Get current position before writing to restore on error
        let original_pos = self
            .file
            .stream_position()
            .map_err(|e| StorageError::Wal(WalError::Io(e)))?;

        // Write all data in one operation for atomicity
        let mut data = Vec::with_capacity(HEADER_SIZE + payload.len());
        data.extend_from_slice(&len.to_le_bytes());
        data.extend_from_slice(&checksum.to_le_bytes());
        data.extend_from_slice(&payload);

        match self.file.write_all(&data) {
            Ok(_) => Ok(()),
            Err(e) => {
                // On error, truncate back to original position
                self.file
                    .seek(SeekFrom::Start(original_pos))
                    .map_err(|e| StorageError::Wal(WalError::Io(e)))?;
                self.file
                    .get_ref()
                    .set_len(original_pos)
                    .map_err(|e| StorageError::Wal(WalError::Io(e)))?;
                Err(StorageError::Wal(WalError::Io(e)))
            }
        }
    }

    /// Flush internal buffer and fsync to guarantee durability.
    fn flush(&mut self) -> StorageResult<()> {
        // Flush to buffer pool of OS but don't guarantee durability
        self.file
            .flush()
            .map_err(|e| StorageError::Wal(WalError::Io(e)))?;
        // So, we should then flush wal to disk
        self.file
            .get_ref()
            .sync_data()
            .map_err(|e| StorageError::Wal(WalError::Io(e)))
    }

    /// Returns an iterator over WAL (Write-Ahead Log) records in the file.
    ///
    /// This method creates a new reader by cloning the underlying file and seeks
    /// to the beginning. It then constructs a generator that lazily reads records
    /// one by one, verifying their integrity using checksums.
    ///
    /// Each record is expected to follow the format:
    /// `[length (4 bytes)] [checksum (4 bytes)] [payload (variable length)]`
    ///
    /// The iterator will:
    /// - Read the fixed-size header to extract the record length and checksum.
    /// - Read the payload of the given length.
    /// - Verify the CRC32 checksum against the payload.
    /// - Deserialize the payload into a `LogRecord`.
    ///
    /// On encountering EOF, the iteration ends gracefully. If any I/O or checksum
    /// error occurs, the iterator yields a `StorageError`.
    ///
    /// # Errors
    ///
    /// Returns a [`StorageError::Wal`] if cloning the file or seeking fails during setup.
    /// The iterator itself yields `Result<LogRecord, StorageError>` for each entry.
    fn iter(&self) -> StorageResult<Self::LogIterator> {
        let mut reader = self
            .file
            .get_ref()
            .try_clone()
            .map_err(|e| StorageError::Wal(WalError::Io(e)))?;
        // Seek to the beginning of the file
        reader
            .seek(std::io::SeekFrom::Start(0))
            .map_err(|e| StorageError::Wal(WalError::Io(e)))?;

        Ok(gen move {
            const LEN_OFFSET: usize = 0;
            const LEN_SIZE: usize = 4;
            const CHECKSUM_OFFSET: usize = 4;
            const CHECKSUM_SIZE: usize = 4;
            let mut header = [0u8; HEADER_SIZE];
            loop {
                if let Err(e) = reader.read_exact(&mut header) {
                    // Normal EOF – stop iteration
                    if e.kind() == io::ErrorKind::UnexpectedEof {
                        return;
                    }
                    yield Err(StorageError::Wal(WalError::Io(e)));
                    continue;
                }

                let len = u32::from_le_bytes(
                    header[LEN_OFFSET..LEN_OFFSET + LEN_SIZE]
                        .try_into()
                        .unwrap(),
                ) as usize;
                let checksum = u32::from_le_bytes(
                    header[CHECKSUM_OFFSET..CHECKSUM_OFFSET + CHECKSUM_SIZE]
                        .try_into()
                        .unwrap(),
                );

                let mut payload = vec![0u8; len];
                if let Err(e) = reader.read_exact(&mut payload) {
                    yield Err(StorageError::Wal(WalError::Io(e)));
                    continue;
                }

                let mut hasher = Hasher::new();
                hasher.update(&payload);
                if hasher.finalize() != checksum {
                    yield Err(StorageError::Wal(WalError::ChecksumMismatch));
                    continue;
                }

                yield LogRecord::from_bytes(payload);
            }
        })
    }

    /// Reads and returns all WAL (Write-Ahead Log) records from the file in order.
    ///
    /// This method invokes [`Self::iter`] to create a streaming iterator over all
    /// log entries. It collects all successfully parsed records into a vector,
    /// sorts them by their `lsn` (Log Sequence Number), and returns the sorted list.
    ///
    /// This is typically used during recovery to load and replay the full log
    /// content in a consistent order.
    ///
    /// # Errors
    ///
    /// Returns a [`StorageError::Wal`] if reading any log entry or initializing
    /// the iterator fails. Any corrupt record (e.g., checksum mismatch or I/O error)
    /// will cause early termination with an error.
    fn read_all(&self) -> StorageResult<Vec<Self::Record>> {
        let iter = self.iter()?;
        let mut records = Vec::new();
        for entry in iter {
            records.push(entry?);
        }
        records.sort_by_key(|entry| entry.lsn);
        Ok(records)
    }
}

impl GraphWal {
    /// Truncates the WAL (Write-Ahead Log) file to remove entries with LSN less than `min_lsn`.
    ///
    /// This is typically used during log compaction or checkpointing, to discard
    /// obsolete log entries that are no longer needed for recovery.
    ///
    /// The truncation process involves:
    /// - Reading all existing log records using [`Self::read_all`].
    /// - Filtering out records with `lsn < min_lsn`.
    /// - Flushing and deleting the original WAL file.
    /// - Rewriting a new WAL file with the retained records via [`Self::append`].
    /// - Replacing the current file handle with the newly opened WAL file.
    ///
    /// # Errors
    ///
    /// Returns a [`StorageError::Wal`] if any I/O operation fails during reading,
    /// deletion, creation, writing, or flushing.
    pub fn truncate_until(&mut self, min_lsn: u64) -> StorageResult<()> {
        // Read all current records
        let entries = self.read_all()?;

        // Filter and keep only records with LSN >= min_lsn
        let retained: Vec<_> = entries.into_iter().filter(|e| e.lsn >= min_lsn).collect();

        // Close old file writer (drop old writer to ensure exclusive access)
        self.flush()?; // Ensure old writer flushes all data
        let file_path = self.path.clone();

        // Rewrite file: first delete original file
        fs::remove_file(&file_path).map_err(|e| StorageError::Wal(WalError::Io(e)))?;

        // Create a new WAL file
        let mut new_wal = GraphWal::open(&file_path)?;

        // Write retained records back to file using existing append method
        for record in retained {
            new_wal.append(&record)?;
        }

        // Ensure all data is flushed to disk
        new_wal.flush()?;

        // Update self.file to the new file handle
        self.file = new_wal.file;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct WalManagerConfig {
    pub wal_path: PathBuf,
}

fn default_wal_path() -> PathBuf {
    let tmp_dir = temp_dir::TempDir::new().unwrap();
    let path = tmp_dir.path().join("minigu-wal.log");
    tmp_dir.leak();
    path
}

impl Default for WalManagerConfig {
    fn default() -> Self {
        Self {
            wal_path: default_wal_path(),
        }
    }
}

pub struct WalManager {
    pub(super) wal: Arc<RwLock<GraphWal>>,
    pub(super) next_lsn: AtomicU64,
    pub(super) wal_path: PathBuf,
}

impl WalManager {
    pub fn new(config: WalManagerConfig) -> Self {
        let path = config.wal_path;
        Self {
            wal: Arc::new(RwLock::new(GraphWal::open(&path).unwrap())),
            next_lsn: AtomicU64::new(0),
            wal_path: path.to_path_buf(),
        }
    }

    pub fn next_lsn(&self) -> u64 {
        self.next_lsn.fetch_add(1, Ordering::SeqCst)
    }

    pub fn set_next_lsn(&self, lsn: u64) {
        self.next_lsn.store(lsn, Ordering::SeqCst);
    }

    pub fn wal(&self) -> &Arc<RwLock<GraphWal>> {
        &self.wal
    }

    pub fn truncate_until(&self, lsn: u64) -> StorageResult<()> {
        self.wal.write().unwrap().truncate_until(lsn)
    }

    pub fn path(&self) -> &Path {
        &self.wal_path
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use minigu_common::value::ScalarValue;
    use serial_test::serial;

    use super::*;
    use crate::common::transaction::{DeltaOp, SetPropsOp, Timestamp};

    fn temp_wal_path() -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("test_wal_{}.log", std::process::id()));
        path
    }

    fn cleanup(path: &Path) {
        let _ = fs::remove_file(path);
    }

    #[test]
    #[serial]
    fn test_walentry_serialization() {
        // Create a WalEntry with SetVertexProps operation
        let txn_id = Timestamp(100);
        let delta = DeltaOp::SetVertexProps(42, SetPropsOp {
            indices: vec![0, 1],
            props: vec![
                ScalarValue::Int32(10.into()),
                ScalarValue::String("test".to_string().into()),
            ],
        });
        let entry = RedoEntry {
            lsn: 0,
            txn_id,
            iso_level: IsolationLevel::Serializable,
            op: Operation::Delta(delta),
        };

        // Serialize and deserialize
        let bytes = entry.to_bytes().unwrap();
        let deserialized = RedoEntry::from_bytes(bytes).unwrap();

        // Verify the deserialized entry matches the original
        assert_eq!(deserialized.lsn, 0);
        assert_eq!(deserialized.txn_id.0, 100);
        match &deserialized.op {
            Operation::Delta(DeltaOp::SetVertexProps(vid, SetPropsOp { indices, props })) => {
                assert_eq!(*vid, 42);
                assert_eq!(*indices, vec![0, 1]);
                assert_eq!(*props, vec![
                    ScalarValue::Int32(10.into()),
                    ScalarValue::String("test".to_string().into())
                ]);
            }
            _ => panic!("Expected Delta(SetVertexProps) operation"),
        }
    }

    #[test]
    #[serial]
    fn test_walentry_append_and_read() {
        let path = temp_wal_path();
        cleanup(&path);

        // Create and write WalEntry records
        {
            let mut wal = GraphWal::open(&path).unwrap();

            // Entry 1: Delete vertex 42
            let entry1 = RedoEntry {
                lsn: 1,
                txn_id: Timestamp(100),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::DelVertex(42)),
            };
            wal.append(&entry1).unwrap();

            // Entry 2: Delete edge 24
            let entry2 = RedoEntry {
                lsn: 2,
                txn_id: Timestamp(101),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::DelEdge(24)),
            };
            wal.append(&entry2).unwrap();

            wal.flush().unwrap();
        }

        // Read and verify WalEntry records
        {
            let wal = GraphWal::open(&path).unwrap();
            let entries: Vec<RedoEntry> =
                wal.iter().unwrap().collect::<Result<Vec<_>, _>>().unwrap();

            assert_eq!(entries.len(), 2);

            // Verify first entry
            assert_eq!(entries[0].lsn, 1);
            assert_eq!(entries[0].txn_id.0, 100);
            match &entries[0].op {
                Operation::Delta(DeltaOp::DelVertex(vid)) => assert_eq!(*vid, 42),
                _ => panic!("Expected Delta(DelVertex) operation"),
            }

            // Verify second entry
            assert_eq!(entries[1].lsn, 2);
            assert_eq!(entries[1].txn_id.0, 101);
            match &entries[1].op {
                Operation::Delta(DeltaOp::DelEdge(eid)) => assert_eq!(*eid, 24),
                _ => panic!("Expected Delta(DelEdge) operation"),
            }
        }

        cleanup(&path);
    }

    #[test]
    #[serial]
    fn test_walentry_recovery_sequence() {
        let path = temp_wal_path();
        cleanup(&path);

        // Create a sequence of operations in the WAL
        {
            let mut wal = GraphWal::open(&path).unwrap();

            // Transaction 1: Delete vertex 10
            let entry1 = RedoEntry {
                lsn: 1,
                txn_id: Timestamp(100),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::DelVertex(10)),
            };
            wal.append(&entry1).unwrap();

            // Transaction 2: Delete edge 20
            let entry2 = RedoEntry {
                lsn: 2,
                txn_id: Timestamp(101),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::DelEdge(20)),
            };
            wal.append(&entry2).unwrap();

            // Transaction 3: Delete vertex 30
            let entry3 = RedoEntry {
                lsn: 3,
                txn_id: Timestamp(102),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::DelVertex(30)),
            };
            wal.append(&entry3).unwrap();

            wal.flush().unwrap();
        }

        // Read and verify the recovery sequence
        {
            let wal = GraphWal::open(&path).unwrap();
            let entries = wal.iter().unwrap();

            // Process entries in sequence
            let mut deleted_vertices = Vec::new();
            let mut deleted_edges = Vec::new();

            for entry_result in entries {
                let entry = entry_result.unwrap();
                match &entry.op {
                    Operation::Delta(DeltaOp::DelVertex(vid)) => deleted_vertices.push(*vid),
                    Operation::Delta(DeltaOp::DelEdge(eid)) => deleted_edges.push(*eid),
                    _ => {}
                }
            }

            // Verify the recovery state
            assert_eq!(deleted_vertices, vec![10, 30]);
            assert_eq!(deleted_edges, vec![20]);
        }

        cleanup(&path);
    }

    #[test]
    #[serial]
    fn test_walentry_invalid_data() {
        let path = temp_wal_path();
        cleanup(&path);

        // Write a valid record
        {
            let mut wal = GraphWal::open(&path).unwrap();
            let entry = RedoEntry {
                lsn: 1,
                txn_id: Timestamp(100),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::DelVertex(42)),
            };
            wal.append(&entry).unwrap();
            wal.flush().unwrap();
        }

        // Append invalid data directly to the file
        {
            let mut file = OpenOptions::new().append(true).open(&path).unwrap();

            // Write invalid header (correct length but wrong checksum)
            let payload = vec![0u8; 20]; // Empty payload with correct structure
            let len = payload.len() as u32;
            let invalid_checksum = 12345u32;

            file.write_all(&len.to_le_bytes()).unwrap();
            file.write_all(&invalid_checksum.to_le_bytes()).unwrap();
            file.write_all(&payload).unwrap();
            file.sync_data().unwrap();
        }

        // Read records - should get one valid record and one error
        {
            let wal = GraphWal::open(&path).unwrap();
            let mut entries = wal.iter().unwrap();
            // First entry should be valid
            let first = entries.next().unwrap().unwrap();
            match &first.op {
                Operation::Delta(DeltaOp::DelVertex(vid)) => assert_eq!(*vid, 42),
                _ => panic!("Expected Delta(DelVertex) operation"),
            }

            // Second entry should be an error due to checksum mismatch
            let second = entries.next().unwrap();
            assert!(second.is_err());
            match second {
                Err(StorageError::Wal(WalError::ChecksumMismatch)) => {}
                _ => panic!("Expected checksum mismatch error"),
            }

            // No more entries
            assert!(entries.next().is_none());
        }

        cleanup(&path);
    }

    #[test]
    #[serial]
    fn test_read_all() {
        let path = temp_wal_path();
        cleanup(&path);

        // Create and write two entries
        {
            let mut wal = GraphWal::open(&path).unwrap();

            // Entry 1: Delete vertex 42
            let entry1 = RedoEntry {
                lsn: 1,
                txn_id: Timestamp(100),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::DelVertex(42)),
            };
            wal.append(&entry1).unwrap();

            // Entry 2: Delete edge 24
            let entry2 = RedoEntry {
                lsn: 2,
                txn_id: Timestamp(101),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::DelEdge(24)),
            };
            wal.append(&entry2).unwrap();

            wal.flush().unwrap();
        }

        // Read all entries at once using read_all
        {
            let wal = GraphWal::open(&path).unwrap();
            let entries = wal.read_all().unwrap();

            // Verify we got both entries in correct order
            assert_eq!(entries.len(), 2);

            // Verify first entry
            assert_eq!(entries[0].lsn, 1);
            assert_eq!(entries[0].txn_id.0, 100);
            match &entries[0].op {
                Operation::Delta(DeltaOp::DelVertex(vid)) => assert_eq!(*vid, 42),
                _ => panic!("Expected Delta(DelVertex) operation"),
            }

            // Verify second entry
            assert_eq!(entries[1].lsn, 2);
            assert_eq!(entries[1].txn_id.0, 101);
            match &entries[1].op {
                Operation::Delta(DeltaOp::DelEdge(eid)) => assert_eq!(*eid, 24),
                _ => panic!("Expected Delta(DelEdge) operation"),
            }
        }

        cleanup(&path);
    }

    #[test]
    #[serial]
    fn test_truncate_until() {
        let path = temp_wal_path();
        cleanup(&path);

        // Create and write entries with different LSNs
        {
            let mut wal = GraphWal::open(&path).unwrap();

            // Entry with LSN 1
            let entry1 = RedoEntry {
                lsn: 1,
                txn_id: Timestamp(100),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::DelVertex(10)),
            };
            wal.append(&entry1).unwrap();

            // Entry with LSN 2
            let entry2 = RedoEntry {
                lsn: 2,
                txn_id: Timestamp(101),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::DelVertex(20)),
            };
            wal.append(&entry2).unwrap();

            // Entry with LSN 3
            let entry3 = RedoEntry {
                lsn: 3,
                txn_id: Timestamp(102),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::DelVertex(30)),
            };
            wal.append(&entry3).unwrap();

            wal.flush().unwrap();

            let entries = wal.read_all().unwrap();
            assert_eq!(entries.len(), 3);

            // Truncate entries with LSN <= 2
            wal.truncate_until(2).unwrap();

            // Verify only entries with LSN >= 2 remain
            let entries = wal.read_all().unwrap();
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].lsn, 2);
            assert_eq!(entries[1].lsn, 3);
        }

        // Reopen the WAL and verify truncation persisted
        {
            let wal = GraphWal::open(&path).unwrap();
            let entries = wal.read_all().unwrap();
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].lsn, 2);
            assert_eq!(entries[1].lsn, 3);
        }

        cleanup(&path);
    }
}
