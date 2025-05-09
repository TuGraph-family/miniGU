use std::io;

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
