use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("arrow error")]
    Arrow(#[from] arrow::error::ArrowError),

    #[error("external error")]
    External(#[from] Box<dyn Error>),
}

pub type ExecutionResult<T> = Result<T, ExecutionError>;
