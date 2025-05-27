use std::error::Error;
use std::result;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BindError {
    #[error("Schema not specified")]
    SchemaNotSpecified,
    #[error("Schema not found {0:?}")]
    SchemaNotFound(String),
    #[error("Procedure not found {0:?}")]
    ProcedureNotFound(String),
    #[error("not support operation {0:?}")]
    NotSupported(String),
    #[error(transparent)]
    External(#[from] Box<dyn Error>),
}

pub type BindResult<T> = result::Result<T, BindError>;
