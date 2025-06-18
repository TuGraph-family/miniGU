use std::error::Error;

use miette::Diagnostic;
use minigu_common::error::NotImplemented;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum ExecutionError {
    #[error("arrow error")]
    Arrow(#[from] arrow::error::ArrowError),

    #[error(transparent)]
    Custom(#[from] Box<dyn Error + Send + Sync + 'static>),

    #[error(transparent)]
    #[diagnostic(transparent)]
    NotImplemented(#[from] NotImplemented),
}

pub type ExecutionResult<T> = Result<T, ExecutionError>;
