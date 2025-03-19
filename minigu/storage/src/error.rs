use thiserror::Error;

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Transaction Error: {0}")]
    TransactionError(String),
    #[error("Vertex {0} not found")]
    VertexNotFound(String),
    #[error("Vertex {0} already exists")]
    VertexAlreadyExists(String),
    #[error("Edge {0} already exists")]
    EdgeAlreadyExists(String),
    #[error("Edge {0} not found")]
    EdgeNotFound(String),
    #[error("Version {0} not found")]
    VersionNotFound(String),
    #[error("Iterator err: {0}")]
    IteratorError(String),
    #[error("Read Conflict: {0}")]
    ReadConflict(String),
    #[error("Write Conflict: {0}")]
    WriteConflict(String),
}
