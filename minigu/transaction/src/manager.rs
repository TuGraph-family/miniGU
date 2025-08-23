//! Transaction manager trait and related functionality
//!
//! This module defines the core transaction manager interface that handles
//! transaction lifecycle management, watermarking, and garbage collection.

use std::sync::Arc;

use crate::transaction::Transaction;

/// Trait for transaction managers supporting MVCC operations.
/// This trait abstracts the core functionality needed for managing transactions
/// across different storage implementations.
///
/// Note: Timestamp and transaction ID generation is handled by global generators
/// accessible via `global_timestamp_generator()` and `global_transaction_id_generator()`.
pub trait GraphTxnManager {
    /// The transaction type that this manager handles
    type Transaction: Transaction;
    /// The graph/storage context type
    type GraphContext;
    /// The error type for operations
    type Error;

    /// Begin a new transaction and return it.
    /// This creates a new transaction, adds it to the active transaction set and updates
    /// watermarks.
    fn begin_transaction(&self) -> Result<Arc<Self::Transaction>, Self::Error>;

    /// Unregister a transaction when it completes (commits or aborts).
    /// This removes the transaction from active set, updates watermarks,
    /// and may trigger garbage collection.
    fn finish_transaction(&self, txn: &Self::Transaction) -> Result<(), Self::Error>;

    /// Perform garbage collection to clean up expired transaction data.
    /// This typically includes removing old transaction records and cleaning up
    /// version chains that are no longer visible to any active transaction.
    fn garbage_collect(&self, graph: &Self::GraphContext) -> Result<(), Self::Error>;
}
