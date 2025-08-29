//! Transaction trait and related functionality
//!
//! This module defines the core transaction interface and related types
//! for database transactions.

use serde::{Deserialize, Serialize};

use crate::timestamp::Timestamp;

/// Isolation level for transactions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IsolationLevel {
    /// Snapshot isolation - reads see a consistent snapshot
    Snapshot,
    /// Serializable isolation - full serializability
    Serializable,
}

/// Trait defining the core operations that all transactions must support.
/// This trait abstracts the fundamental transaction behavior across different
/// storage implementations.
pub trait Transaction: Send + Sync {
    /// The error type for transaction operations
    type Error;

    /// Get the transaction ID
    fn txn_id(&self) -> Timestamp;

    /// Get the start timestamp of the transaction
    fn start_ts(&self) -> Timestamp;

    /// Get the commit timestamp of the transaction
    fn commit_ts(&self) -> Option<Timestamp>;

    /// Get the isolation level of the transaction
    fn isolation_level(&self) -> &IsolationLevel;

    /// Commit the transaction, returning the commit timestamp on success
    fn commit(&self) -> Result<Timestamp, Self::Error>;

    /// Abort the transaction and rollback all changes
    fn abort(&self) -> Result<(), Self::Error>;
}
