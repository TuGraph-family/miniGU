//! Timestamp management for MVCC transactions
//!
//! This module provides timestamp and transaction ID generation for multi-version
//! concurrency control (MVCC) operations across the minigu database system.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};

use serde::{Deserialize, Serialize};

use crate::error::TimestampError;

/// Represents a commit timestamp used for multi-version concurrency control (MVCC).
/// It can either represent a transaction ID which starts from 1 << 63,
/// or a commit timestamp which starts from 0. So, we can determine a timestamp is
/// a transaction ID if the highest bit is set to 1, or a commit timestamp if the highest bit is 0.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub struct Timestamp(u64);

impl Timestamp {
    /// The start of the transaction ID range.
    pub const TXN_ID_START: u64 = 1 << 63;

    /// Create timestamp by a given commit ts
    pub fn with_ts(timestamp: u64) -> Self {
        Self(timestamp)
    }

    /// Returns the maximum possible commit timestamp.
    pub fn max_commit_ts() -> Self {
        Self(u64::MAX & !Self::TXN_ID_START)
    }

    /// Returns true if the timestamp is a transaction ID.
    pub fn is_txn_id(&self) -> bool {
        self.raw() & Self::TXN_ID_START != 0
    }

    /// Returns true if the timestamp is a commit timestamp.
    pub fn is_commit_ts(&self) -> bool {
        self.raw() & Self::TXN_ID_START == 0
    }

    /// Returns the raw value of the timestamp.
    pub fn raw(&self) -> u64 {
        self.0
    }
}

/// Global timestamp generator for MVCC version control
pub struct GlobalTimestampGenerator {
    counter: AtomicU64,
}

impl GlobalTimestampGenerator {
    /// Create a new timestamp generator
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(1),
        }
    }

    /// Create a new timestamp generator with a starting value
    pub fn with_start(start: u64) -> Self {
        Self {
            counter: AtomicU64::new(start),
        }
    }

    /// Generate the next timestamp
    pub fn next(&self) -> Result<Timestamp, TimestampError> {
        let mut cur = self.counter.load(Ordering::SeqCst);
        loop {
            if cur >= Timestamp::max_commit_ts().raw() {
                return Err(TimestampError::CommitTsOverflow(cur));
            }
            match self.counter.compare_exchange_weak(
                cur,
                cur + 1,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return Ok(Timestamp::with_ts(cur)),
                Err(actual) => cur = actual,
            }
        }
    }

    /// Get the current timestamp without incrementing
    pub fn current(&self) -> Timestamp {
        Timestamp::with_ts(self.counter.load(Ordering::SeqCst))
    }

    /// Update the counter if the given timestamp is greater than the current value
    pub fn update_if_greater(&self, ts: Timestamp) -> Result<(), TimestampError> {
        if !ts.is_commit_ts() {
            return Err(TimestampError::WrongDomainCommit(ts.raw()));
        }
        if ts.raw() >= Timestamp::max_commit_ts().raw() {
            return Err(TimestampError::CommitTsOverflow(ts.raw()));
        }
        self.counter.fetch_max(ts.raw() + 1, Ordering::SeqCst);
        Ok(())
    }
}

impl Default for GlobalTimestampGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Transaction ID generator
pub struct TransactionIdGenerator {
    counter: AtomicU64,
}

impl TransactionIdGenerator {
    /// Create a new transaction ID generator
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(Timestamp::TXN_ID_START + 1),
        }
    }

    /// Create a new transaction ID generator with a starting value
    pub fn with_start(start: u64) -> Self {
        Self {
            counter: AtomicU64::new(start),
        }
    }

    /// Generate the next transaction ID
    pub fn next(&self) -> Result<Timestamp, TimestampError> {
        let mut cur = self.counter.load(Ordering::SeqCst);
        loop {
            if cur == u64::MAX {
                return Err(TimestampError::TxnIdOverflow(cur));
            }
            match self.counter.compare_exchange_weak(
                cur,
                cur + 1,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return Ok(Timestamp::with_ts(cur)),
                Err(actual) => cur = actual,
            }
        }
    }

    /// Update the counter if the given transaction ID is greater than the current value
    pub fn update_if_greater(&self, txn_id: Timestamp) -> Result<(), TimestampError> {
        if !txn_id.is_txn_id() {
            return Err(TimestampError::WrongDomainTxnId(txn_id.raw()));
        }
        if txn_id.raw() == u64::MAX {
            return Err(TimestampError::TxnIdOverflow(txn_id.raw()));
        }
        self.counter.fetch_max(txn_id.raw() + 1, Ordering::SeqCst);
        Ok(())
    }
}

impl Default for TransactionIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

// Global singleton instances
static GLOBAL_TIMESTAMP_GENERATOR: OnceLock<Arc<GlobalTimestampGenerator>> = OnceLock::new();
static GLOBAL_TRANSACTION_ID_GENERATOR: OnceLock<Arc<TransactionIdGenerator>> = OnceLock::new();

/// Get the global timestamp generator instance
pub fn global_timestamp_generator() -> Arc<GlobalTimestampGenerator> {
    GLOBAL_TIMESTAMP_GENERATOR
        .get_or_init(|| Arc::new(GlobalTimestampGenerator::new()))
        .clone()
}

/// Get the global transaction ID generator instance
pub fn global_transaction_id_generator() -> Arc<TransactionIdGenerator> {
    GLOBAL_TRANSACTION_ID_GENERATOR
        .get_or_init(|| Arc::new(TransactionIdGenerator::new()))
        .clone()
}

/// Initialize the global timestamp generator with a specific starting value
/// This should only be called once during system initialization
pub fn init_global_timestamp_generator(start: u64) -> Result<(), &'static str> {
    GLOBAL_TIMESTAMP_GENERATOR
        .set(Arc::new(GlobalTimestampGenerator::with_start(start)))
        .map_err(|_| "Global timestamp generator already initialized")
}

/// Initialize the global transaction ID generator with a specific starting value
/// This should only be called once during system initialization
pub fn init_global_transaction_id_generator(start: u64) -> Result<(), &'static str> {
    GLOBAL_TRANSACTION_ID_GENERATOR
        .set(Arc::new(TransactionIdGenerator::with_start(start)))
        .map_err(|_| "Global transaction ID generator already initialized")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_txn_id_detection() {
        let commit_ts = Timestamp::with_ts(100);
        assert!(commit_ts.is_commit_ts());
        assert!(!commit_ts.is_txn_id());

        let txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 100);
        assert!(!txn_id.is_commit_ts());
        assert!(txn_id.is_txn_id());
    }

    #[test]
    fn test_global_timestamp_generator() {
        let generator = GlobalTimestampGenerator::new();
        assert_eq!(generator.current().raw(), 1);

        let ts1 = generator.next().unwrap();
        assert_eq!(ts1.raw(), 1);
        assert_eq!(generator.current().raw(), 2);

        let ts2 = generator.next().unwrap();
        assert_eq!(ts2.raw(), 2);
        assert_eq!(generator.current().raw(), 3);
    }

    #[test]
    fn test_transaction_id_generator() {
        let generator = TransactionIdGenerator::new();

        let txn1 = generator.next().unwrap();
        assert!(txn1.is_txn_id());
        assert_eq!(txn1.raw(), Timestamp::TXN_ID_START + 1);

        let txn2 = generator.next().unwrap();
        assert!(txn2.is_txn_id());
        assert_eq!(txn2.raw(), Timestamp::TXN_ID_START + 2);
    }

    #[test]
    fn test_update_if_greater() {
        let ts_generator = GlobalTimestampGenerator::new();
        ts_generator
            .update_if_greater(Timestamp::with_ts(100))
            .unwrap();
        assert_eq!(ts_generator.current().raw(), 101);

        ts_generator
            .update_if_greater(Timestamp::with_ts(50))
            .unwrap();
        assert_eq!(ts_generator.current().raw(), 101); // Should not decrease

        let txn_generator = TransactionIdGenerator::new();
        txn_generator
            .update_if_greater(Timestamp::with_ts(Timestamp::TXN_ID_START + 100))
            .unwrap();
        let next = txn_generator.next().unwrap();
        assert_eq!(next.raw(), Timestamp::TXN_ID_START + 101);
    }

    #[test]
    fn test_global_singleton_instances() {
        // Test that global instances work correctly and are singletons
        let gen1 = global_timestamp_generator();
        let gen2 = global_timestamp_generator();

        // Generate timestamps from first reference
        let ts1 = gen1.next().unwrap();

        // Second reference should see the updated state
        let ts2 = gen2.next().unwrap();
        assert!(ts2.raw() > ts1.raw());

        // Test transaction ID generator singleton
        let txn_gen1 = global_transaction_id_generator();
        let txn_gen2 = global_transaction_id_generator();

        let txn1 = txn_gen1.next().unwrap();
        let txn2 = txn_gen2.next().unwrap();
        assert!(txn2.raw() > txn1.raw());
        assert!(txn1.is_txn_id());
        assert!(txn2.is_txn_id());
    }
}
