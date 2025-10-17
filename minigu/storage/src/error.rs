use std::io;
use std::num::NonZeroU32;

use minigu_transaction::TimestampError;
use thiserror::Error;
pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Transaction error: {0}")]
    Transaction(#[from] TransactionError),
    #[error("VertexNotFoundError: {0}")]
    VertexNotFound(#[from] VertexNotFoundError),
    #[error("EdgeNotFoundError: {0}")]
    EdgeNotFound(#[from] EdgeNotFoundError),
    #[error("Schema error: {0}")]
    Schema(#[from] SchemaError),
    #[error("WAL error: {0}")]
    Wal(#[from] WalError),
    #[error("Checkpoint error: {0}")]
    Checkpoint(#[from] CheckpointError),
    #[error("Vector index error: {0}")]
    VectorIndex(#[from] VectorIndexError),
    #[error("Feature not supported: {0}")]
    NotSupported(String),
}

#[derive(Error, Debug)]
pub enum WalError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Data corruption: checksum mismatch")]
    ChecksumMismatch,
    #[error("Invalid record format: {0}")]
    InvalidFormat(String),
    #[error("Record deserialization failed: {0}")]
    DeserializationFailed(String),
    #[error("Record serialization failed: {0}")]
    SerializationFailed(String),
}

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("Write-Read conflict: {0}")]
    WriteReadConflict(String),
    #[error("Read-Write conflict: {0}")]
    ReadWriteConflict(String),
    #[error("Write-Write conflict: {0}")]
    WriteWriteConflict(String),
    #[error("Version not visible: {0}")]
    VersionNotVisible(String),
    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),
    #[error("Transaction already committed: {0}")]
    TransactionAlreadyCommitted(String),
    #[error("Invalid state: {0}")]
    InvalidState(String),
    #[error("Timestamp error: {0}")]
    Timestamp(#[from] TimestampError),
}

#[derive(Error, Debug)]
pub enum VertexNotFoundError {
    #[error("Vertex {0} not found")]
    VertexNotFound(String),
    #[error("Vertex {0} is tombstone")]
    VertexTombstone(String),
}

#[derive(Error, Debug)]
pub enum EdgeNotFoundError {
    #[error("Edge {0} not found")]
    EdgeNotFound(String),
    #[error("Edge {0} is tombstone")]
    EdgeTombstone(String),
}

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("Vertex schema already exists")]
    VertexSchemaAlreadyExists,
    #[error("Edge schema already exists")]
    EdgeSchemaAlreadyExists,
    #[error("Vertex schema not found")]
    VertexSchemaNotFound,
    #[error("Edge schema not found")]
    EdgeSchemaNotFound,
}

#[derive(Error, Debug)]
pub enum CheckpointError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Data corruption: checksum mismatch")]
    ChecksumMismatch,
    #[error("Checkpoint serialization failed: {0}")]
    SerializationFailed(String),
    #[error("Checkpoint deserialization failed: {0}")]
    DeserializationFailed(String),
    #[error("Invalid checkpoint format: {0}")]
    InvalidFormat(String),
    #[error("Checkpoint not found: {0}")]
    CheckpointNotFound(String),
    #[error("Checkpoint directory error: {0}")]
    DirectoryError(String),
    #[error("Timeout waiting for active transactions to complete")]
    Timeout,
}

#[derive(Error, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum VectorIndexError {
    #[error("DiskANN error: {0}")]
    DiskANN(#[from] diskann::common::ANNError),
    #[error("Index not found: {0}")]
    IndexNotFound(String),
    #[error("Invalid vector dimension: expected {expected}, got {actual}")]
    InvalidDimension { expected: usize, actual: usize },
    #[error("Data conversion error: {0}")]
    DataConversion(String),
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
    #[error("Invalid index type: {0}")]
    InvalidIndexType(String),
    #[error("Index configuration error: {0}")]
    Configuration(String),
    #[error("Index build error: {0}")]
    BuildError(String),
    #[error("Search error: {0}")]
    SearchError(String),
    #[error("ID mapping error: {0}")]
    IdMappingError(String),
    #[error("Vector ID {vector_id} not found in mapping")]
    VectorIdNotFound { vector_id: u32 },
    #[error("Node ID {node_id} not found in mapping")]
    NodeIdNotFound { node_id: u64 },
    #[error("Duplicate node ID {node_id} in input vectors")]
    DuplicateNodeId { node_id: u64 },
    #[error("Empty vector dataset provided")]
    EmptyDataset,
    #[error("Temporary file error: {0}")]
    TempFileError(String),
    #[error("Index not built yet")]
    IndexNotBuilt,
    #[error("Invalid search parameters: {0}")]
    InvalidSearchParams(String),
    #[error("Invalid build parameters: {0}")]
    InvalidBuildParams(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error(
        "VectorId {vector_id} exceeds u32::MAX, cannot be used with DiskANN which requires u32 vector IDs"
    )]
    VectorIdOverflow { vector_id: u64 },
    #[error(
        "Capacity exceeded: current {current} + new vectors would exceed max capacity {max_capacity}"
    )]
    CapacityExceeded { current: usize, max_capacity: usize },
    #[error("NotSupported: {0}")]
    NotSupported(String),
    #[error("Invalid bitmap length: expected {expected}, got {got}")]
    InvalidBitmapLength { expected: usize, got: usize },
    #[error("Filter error: {0}")]
    FilterError(String),
    #[error("Vector index already exists for label {label_id} and property {property_id}")]
    IndexAlreadyExists {
        label_id: NonZeroU32,
        property_id: u32,
    },
}
