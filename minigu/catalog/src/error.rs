use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
#[derive(Clone)]
pub enum Error {
    #[error("schema not exists {0:?}")]
    SchemaNotExists(String),
    #[error("schema already exists {0:?}")]
    SchemaAlreadyExists(String),
    #[error("graph type not exists {0:?}")]
    GraphTypeNotExists(String),
    #[error("graph type already exists {0:?}")]
    GraphTypeAlreadyExists(String),
    #[error("graph not exists {0:?}")]
    GraphNotExists(String),
    #[error("graph already exists {0:?}")]
    GraphAlreadyExists(String),
    #[error("procedure {0} not exists")]
    ProcedureNotExists(String),
    #[error("node type not exists {0:?}")]
    NodeTypeNotExists(String),
    #[error("edge type not exists {0:?}")]
    EdgeTypeNotExists(String),
    #[error("invalid field {0:?}")]
    InvalidField(String),
    #[error("duplicate field name {0:?}")]
    DuplicateField(String),
    #[error("not support operation {0:?}")]
    NotSupported(String),
    #[error("tmp error")]
    ErrorCur
}