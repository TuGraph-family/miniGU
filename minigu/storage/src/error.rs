use thiserror::Error;

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Transaction Error: {0}")]
    TransactionError(String),
    #[error("Vertex {0} not found")]
    VertexNotFound(String),
    #[error("Edge {0} not found")]
    EdgeNotFound(String),
}
