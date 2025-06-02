use minigu_catalog::error::CatalogError;
use minigu_common::error::NotImplemented;
use smol_str::SmolStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BindError {
    #[error("catalog error")]
    Catalog(#[from] CatalogError),

    #[error("not a directory: {0}")]
    NotDirectory(String),

    #[error("not a schema: {0}")]
    NotSchema(String),

    #[error("no such directory or schema at: {0}")]
    DirectoryOrSchemaNotFound(String),

    #[error("current schema is not specified")]
    CurrentSchemaNotSpecified,

    #[error("home schema is not specified")]
    HomeSchemaNotSpecified,

    #[error("procedure not found: {0}")]
    ProcedureNotFound(SmolStr),

    #[error("too many objects: {0:?}")]
    InvalidObjectReference(Vec<SmolStr>),

    #[error(transparent)]
    NotImplemented(#[from] NotImplemented),
}

pub type BindResult<T> = std::result::Result<T, BindError>;

pub(crate) fn not_implemented<T>(feature: String, issue: Option<u32>) -> BindResult<T> {
    Err(BindError::NotImplemented(NotImplemented::new(
        feature,
        issue.into(),
    )))
}
