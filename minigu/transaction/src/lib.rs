//! Common transaction infrastructure for minigu database system.
//!
//! This module provides shared transaction-related structures and utilities
//! that are used across both the catalog and storage layers.

use std::sync::Weak;

pub mod error;
pub mod manager;
pub mod timestamp;
pub mod transaction;

pub use error::TimestampError;
// Re-export commonly used types
pub use manager::GraphTxnManager;
pub use timestamp::{
    GlobalTimestampGenerator, Timestamp, TransactionIdGenerator, global_timestamp_generator,
    global_transaction_id_generator, init_global_timestamp_generator,
    init_global_transaction_id_generator,
};
pub use transaction::{IsolationLevel, Transaction};

/// A generic undo log entry for multi-version concurrency control.
/// This abstraction can be used by both storage and catalog layers.
///
/// Type parameter `T` represents the type of delta operation (e.g., DeltaOp for storage, CatalogOp
/// for catalog)
#[derive(Debug, Clone)]
pub struct UndoEntry<T> {
    /// The delta operation of the undo entry
    delta: T,
    /// The timestamp when this version was created
    timestamp: Timestamp,
    /// Pointer to the next undo entry in the undo buffer
    next: UndoPtr<T>,
}

/// Weak pointer to an undo entry, used to build undo chains
pub type UndoPtr<T> = Weak<UndoEntry<T>>;

impl<T> UndoEntry<T> {
    /// Create a new UndoEntry
    pub fn new(delta: T, timestamp: Timestamp, next: UndoPtr<T>) -> Self {
        Self {
            delta,
            timestamp,
            next,
        }
    }

    /// Get the delta operation of the undo entry
    pub fn delta(&self) -> &T {
        &self.delta
    }

    /// Get the timestamp of the undo entry
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    /// Get the next undo pointer in the chain
    pub fn next(&self) -> UndoPtr<T> {
        self.next.clone()
    }
}
