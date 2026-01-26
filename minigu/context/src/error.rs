use miette::Diagnostic;
use minigu_catalog::error::CatalogError;
use minigu_storage::error::StorageError;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("current schema not set yet")]
    CurrentSchemaNotSet,

    #[error("catalog error")]
    Catalog(#[from] CatalogError),

    #[error("current graph not set yet")]
    CurrentGraphNotSet,

    #[error("schema path error")]
    SchemaPathInvalid,

    #[error("graph not exists{0}")]
    GraphNotExists(String),

    #[error("internal error: {0}")]
    Internal(String),
}

pub type SessionResult<T> = std::result::Result<T, Error>;

#[derive(Debug, Error, Diagnostic)]
pub enum IndexCatalogError {
    #[error(transparent)]
    Catalog(#[from] CatalogError),

    #[error(transparent)]
    Storage(#[from] StorageError),

    #[error("vector index name already exists: {0}")]
    VectorIndexNameAlreadyExists(String),

    #[error("vector index {0} not found")]
    VectorIndexNotFound(String),

    #[error("vector index on label {0} property {1} already exists")]
    VectorIndexAlreadyExists(String, String),
}

pub type IndexCatalogResult<T> = std::result::Result<T, IndexCatalogError>;
