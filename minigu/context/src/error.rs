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

    #[error("schema path error")]
    SchemaPathInvalid,

    #[error("graph not exists{0}")]
    GraphNotExists(String),
}

pub type SessionResult<T> = std::result::Result<T, Error>;

#[derive(Debug, Error, Diagnostic)]
pub enum IndexCatalogError {
    #[error(transparent)]
    Catalog(#[from] CatalogError),
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error("vector index name already exists: {0}")]
    NameAlreadyExists(String),
}

pub type IndexCatalogResult<T> = std::result::Result<T, IndexCatalogError>;
