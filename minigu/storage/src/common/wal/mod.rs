pub mod graph_wal;

use std::path::Path;

use crate::error::StorageResult;

/// Trait for records that can be stored in a write-ahead log
pub trait LogRecord: Clone + std::fmt::Debug {
    /// Convert the record to bytes for storage
    fn to_bytes(&self) -> StorageResult<Vec<u8>>;

    /// Create a record from bytes
    fn from_bytes(bytes: Vec<u8>) -> StorageResult<Self>
    where
        Self: Sized;
}

/// Trait defining a Write-Ahead Log (WAL) interface.
///
/// A WAL is an append-only log used to ensure durability of operations
/// before they are applied to the main data structure.
pub trait StorageWal {
    type Record: LogRecord;

    /// The type of iterator returned when reading the log
    type LogIterator: Iterator<Item = StorageResult<Self::Record>>;

    /// Open existing log or create a new one at the specified path.
    fn open<P: AsRef<Path>>(path: P) -> StorageResult<Self>
    where
        Self: Sized;

    /// Append a record to the log and buffer it.
    fn append(&mut self, record: &Self::Record) -> StorageResult<()>;

    /// Flush internal buffer and fsync to guarantee durability.
    fn flush(&mut self) -> StorageResult<()>;

    /// Return an iterator that replays the log from the current instance.
    fn iter(&self) -> StorageResult<Self::LogIterator>;

    /// Read all records from the log
    fn read_all(&self) -> StorageResult<Vec<Self::Record>>;
}
