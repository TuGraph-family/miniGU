use serde::{Deserialize, Serialize};

/// Isolation level for transactions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IsolationLevel {
    /// Snapshot isolation - reads see a consistent snapshot
    Snapshot,
    /// Serializable isolation - full serializability
    Serializable,
}

/// Lock strategy for OLTP transactions.
///
/// `Pessimistic` performs eager conflict checks when applying writes. `Optimistic` defers conflict
/// detection to commit-time validation using write sets.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LockStrategy {
    Pessimistic,
    Optimistic,
}

/// Transaction behavior configuration shared by storage constructors.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct TxnOptions {
    pub default_lock: LockStrategy,
    pub default_isolation: IsolationLevel,
}

impl Default for TxnOptions {
    fn default() -> Self {
        Self {
            default_lock: LockStrategy::Pessimistic,
            default_isolation: IsolationLevel::Snapshot,
        }
    }
}
