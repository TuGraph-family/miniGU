//! Error types for the database file module.

use std::io;

use thiserror::Error;

/// Errors that can occur when working with database files.
#[derive(Error, Debug)]
pub enum DbFileError {
    /// The file does not have a valid miniGU magic number.
    #[error("Invalid magic number: expected 'MINIGU\\0\\0'")]
    InvalidMagic,

    /// The file version is not supported by this version of miniGU.
    #[error("Unsupported file version: {0} (current: {1})")]
    UnsupportedVersion(u32, u32),

    /// The header checksum does not match.
    #[error("Header checksum mismatch: expected {expected:#010x}, got {actual:#010x}")]
    HeaderChecksumMismatch { expected: u32, actual: u32 },

    /// The checkpoint region checksum does not match.
    #[error("Checkpoint checksum mismatch")]
    CheckpointChecksumMismatch,

    /// The WAL entry checksum does not match.
    #[error("WAL entry checksum mismatch at offset {offset}")]
    WalChecksumMismatch { offset: u64 },

    /// The file is truncated or corrupted.
    #[error("File is truncated: expected at least {expected} bytes, got {actual}")]
    FileTruncated { expected: u64, actual: u64 },

    /// An I/O error occurred.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// Serialization error.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Deserialization error.
    #[error("Deserialization error: {0}")]
    Deserialization(String),

    /// The file is not a valid database file.
    #[error("Not a valid database file: {0}")]
    InvalidFile(String),

    /// Checkpoint not found in the file.
    #[error("No checkpoint found in database file")]
    NoCheckpoint,
}

/// Result type for database file operations.
pub type DbFileResult<T> = Result<T, DbFileError>;
