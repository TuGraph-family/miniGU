use minigu_common::TimestampError;
use thiserror::Error;

pub type CatalogTxnResult<T> = Result<T, CatalogTxnError>;

#[derive(Error, Debug)]
pub enum CatalogTxnError {
    #[error("timestamp error: {0}")]
    Timestamp(#[from] TimestampError),
    #[error("already exists: {key}")]
    AlreadyExists { key: String },
    #[error("not found: {key}")]
    NotFound { key: String },
    #[error("write conflict on key: {key}")]
    WriteConflict { key: String },
    #[error("illegal state: {reason}")]
    IllegalState { reason: String },
    #[error("referential integrity violation: {reason}")]
    ReferentialIntegrity { reason: String },
}
