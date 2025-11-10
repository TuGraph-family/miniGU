use miette::Diagnostic;
use minigu_catalog::error::CatalogError;
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
