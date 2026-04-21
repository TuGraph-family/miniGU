use serde::{Deserialize, Serialize};

/// Isolation level for transactions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IsolationLevel {
    /// Snapshot isolation - reads see a consistent snapshot
    Snapshot,
    /// Serializable isolation - full serializability
    Serializable,
}
