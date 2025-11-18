use miette::Diagnostic;
use thiserror::Error;
use minigu_storage::error::StorageError;

#[derive(Debug, Error)]
pub enum GCardError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    BinCode(#[from] bincode::Error),

    #[error("invalid data: {0}")]
    InvalidData(String),

    #[error("edge not found: {0}")]
    EdgeNotFound(String),

    #[error("vertex not found: {0}")]
    VertexNotFound(String),

    #[error("invalid state: {0}")]
    InvalidState(String),

    #[error("storage error")]
    Storage(#[from] StorageError),
}

pub type GCardResult<T> = Result<T, GCardError>;