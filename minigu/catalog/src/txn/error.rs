use std::error::Error;

use thiserror::Error;

/// Error types related to Catalog transactions.
#[derive(Error, Debug)]
pub enum CatalogTxnError {
    /// Target already exists (pre-check during write).
    #[error("catalog already exists: {key}")]
    AlreadyExists { key: String },

    /// Target not found (pre-check during write).
    #[error("catalog not found: {key}")]
    NotFound { key: String },

    /// Write conflict (e.g., concurrent creation/deletion of an object with the same name).
    #[error("catalog write conflict on key: {key}")]
    WriteConflict { key: String },

    /// Illegal transaction state (e.g., double commit/abort, inconsistent state machine, etc.).
    #[error("illegal transaction state: {reason}")]
    IllegalState { reason: String },

    /// Visibility error (e.g., read view invisibility, failed version selection, etc.).
    #[error("visibility error: {reason}")]
    Visibility { reason: String },

    /// GC-related error (e.g., inconsistency or internal error during garbage collection).
    #[error("gc error: {reason}")]
    GcError { reason: String },

    /// Referential integrity violation (e.g., deleting a referenced object, or creating/replacing
    /// an object with missing dependencies).
    #[error("referential integrity error: {reason}")]
    ReferentialIntegrity { reason: String },

    /// Timestamp/transaction ID generation error (propagated).
    #[error(transparent)]
    Timestamp(#[from] minigu_transaction::TimestampError),

    /// Other external error (propagated).
    #[error(transparent)]
    External(#[from] Box<dyn Error + Send + Sync + 'static>),

    /// Serializable conflict: a key read by this transaction was modified by another
    /// transaction after this transaction's snapshot (start_ts).
    #[error("serialization conflict on key: {key}")]
    SerializationConflict { key: String },
}

/// Unified result type for Catalog transaction operations.
pub type CatalogTxnResult<T> = Result<T, CatalogTxnError>;
