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
    #[error("procedure {0} already exists")]
    ProcedureAlreadyExists(String),
    #[error("procedure {0} not exists")]
    ProcedureNotExists(String),
    #[error("node type not exists {0:?}")]
    VertexTypeNotExists(String),
    #[error("edge type not exists {0:?}")]
    EdgeTypeNotExists(String),
    #[error("edge type already exists {0:?}")]
    EdgeTypeAlreadyExists(String),
    #[error("property {0:?} not exists")]
    PropertyNotExists(String),
    #[error("property {0:?} already exists")]
    PropertyAlreadyExists(String),
    #[error("not support operation {0:?}")]
    NotSupported(String),
    #[error("tmp error")]
    ErrorCur
}