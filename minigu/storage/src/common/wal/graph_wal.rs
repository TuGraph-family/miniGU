//! WAL entry definitions for the graph database.
//!
//! This module defines the core WAL entry types used by the persistence layer.
//! The actual WAL implementation is in `DbFilePersistence` (single-file format).

use minigu_transaction::{IsolationLevel, Timestamp};
use serde::{Deserialize, Serialize};

use super::LogRecord;
use crate::common::DeltaOp;
use crate::error::{StorageError, StorageResult, WalError};

/// A redo log entry representing a transaction operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedoEntry {
    pub lsn: u64,                  // Log sequence number
    pub txn_id: Timestamp,         // Transaction ID
    pub iso_level: IsolationLevel, // Isolation level
    pub op: Operation,             // Operation
}

/// Operation types that can be logged in the WAL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    BeginTransaction(Timestamp),  // transaction start timestamp
    CommitTransaction(Timestamp), // transaction commit timestamp
    AbortTransaction,
    Delta(DeltaOp), // Delta operation
}

impl LogRecord for RedoEntry {
    fn to_bytes(&self) -> StorageResult<Vec<u8>> {
        postcard::to_allocvec(self)
            .map_err(|e| StorageError::Wal(WalError::SerializationFailed(e.to_string())))
    }

    fn from_bytes(bytes: Vec<u8>) -> StorageResult<Self> {
        postcard::from_bytes(&bytes)
            .map_err(|e| StorageError::Wal(WalError::DeserializationFailed(e.to_string())))
    }
}
