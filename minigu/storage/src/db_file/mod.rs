//! Single database file module for miniGU.
//!
//! This module provides functionality to integrate WAL and Checkpoint data
//! into a single database file (`.minigu`), supporting:
//! - Fixed file header with magic number, version, and region offsets
//! - Checkpoint region for graph state snapshots
//! - WAL region for write-ahead log entries
//! - Crash recovery from a single file
//! - High-level manager for recovery and persistence
//!
//! # File Layout
//!
//! ```text
//! ┌────────────────────────────────────────────────────────┐
//! │                  FILE HEADER                           │
//! │  (self-describing length, minimum 256 bytes)           │
//! ├────────────────────────────────────────────────────────┤
//! │                 CHECKPOINT REGION                      │
//! │  [len:4B][crc:4B][GraphCheckpoint payload...]          │
//! ├────────────────────────────────────────────────────────┤
//! │                    WAL REGION                          │
//! │  [len:4B][crc:4B][RedoEntry 1...]                      │
//! │  [len:4B][crc:4B][RedoEntry 2...]                      │
//! │  ...                                                   │
//! └────────────────────────────────────────────────────────┘
//! ```

pub mod core;
pub mod error;
pub mod header;
pub mod single_file_manager;

#[cfg(test)]
mod stress_tests;

pub use core::{DbFile, DbFileStats};

pub use error::DbFileError;
pub use header::{CURRENT_VERSION, DbFileFlags, DbFileHeader, HEADER_SIZE, MAGIC};
pub use single_file_manager::{RecoveryData, SingleFileConfig, SingleFileManager};
