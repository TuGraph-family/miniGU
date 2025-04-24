use std::{fmt, io};

use postcard;
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
    #[error("Wal error: {0}")]
    Wal(#[from] WalError),
}

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("Write-Read conflict: {0}")]
    WriteReadConflict(String),
    #[error("Read-Write conflict: {0}")]
    ReadWriteConflict(String),
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

#[derive(Debug)]
pub enum WalError {
    Io(io::Error),
    Serialize(postcard::Error),
    Deserialize(postcard::Error),
}

impl fmt::Display for WalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalError::Io(err) => write!(f, "I/O error: {}", err),
            WalError::Serialize(err) => write!(f, "Serialization error: {}", err),
            WalError::Deserialize(err) => write!(f, "Deserialization error: {}", err),
        }
    }
}

impl std::error::Error for WalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WalError::Io(err) => Some(err),
            WalError::Serialize(err) => Some(err),
            WalError::Deserialize(err) => Some(err),
        }
    }
}
