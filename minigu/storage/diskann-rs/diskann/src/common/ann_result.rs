// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.
use std::alloc::LayoutError;
use std::array::TryFromSliceError;
use std::num::TryFromIntError;

/// Result
#[allow(clippy::upper_case_acronyms)]
pub type ANNResult<T> = Result<T, ANNError>;

/// DiskANN Error
/// ANNError is `Send` (i.e., safe to send across threads)
#[allow(clippy::upper_case_acronyms, clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum ANNError {
    /// Index construction and search error
    #[error("IndexError: {err}")]
    IndexError { err: String },

    /// Index configuration error
    #[error("IndexConfigError: {parameter} is invalid, err={err}")]
    IndexConfigError { parameter: String, err: String },

    /// Integer conversion error
    #[error("TryFromIntError: {err}")]
    TryFromIntError {
        #[from]
        err: TryFromIntError,
    },

    /// IO error
    #[error("IOError: {err}")]
    IOError {
        #[from]
        err: std::io::Error,
    },

    /// Layout error in memory allocation
    #[error("MemoryAllocLayoutError: {err}")]
    MemoryAllocLayoutError {
        #[from]
        err: LayoutError,
    },

    /// PoisonError which can be returned whenever a lock is acquired
    /// Both Mutexes and RwLocks are poisoned whenever a thread fails while the lock is held
    #[error("LockPoisonError: {err}")]
    LockPoisonError { err: String },

    /// DiskIOAlignmentError which can be returned when calling windows API CreateFileA for the disk
    /// index file fails.
    #[error("DiskIOAlignmentError: {err}")]
    DiskIOAlignmentError { err: String },

    /// IOQueueError which can be returned when we call windows API CreateIoQueue for the disk index
    /// file fails.
    #[error("IOQueueError: {err}")]
    IOQueueError { err: String },

    // PQ construction error
    // Error happened when we construct PQ pivot or PQ compressed table
    #[error("PQError: {err}")]
    PQError { err: String },

    /// Array conversion error
    #[error("Error try creating array from slice: {err}")]
    TryFromSliceError {
        #[from]
        err: TryFromSliceError,
    },

    /// KMeans error
    #[error("KMeansError: {err}")]
    KMeansError { err: String },
}

impl ANNError {
    pub fn log_index_config_error(parameter: String, err: String) -> Self {
        ANNError::IndexConfigError { parameter, err }
    }

    pub fn log_index_error(err: String) -> Self {
        ANNError::IndexError { err }
    }

    pub fn log_lock_poison_error(err: String) -> Self {
        ANNError::LockPoisonError { err }
    }

    pub fn log_pq_error(err: String) -> Self {
        ANNError::PQError { err }
    }

    pub fn log_io_error(err: std::io::Error) -> Self {
        ANNError::IOError { err }
    }

    pub fn log_disk_io_alignment_error(err: String) -> Self {
        ANNError::DiskIOAlignmentError { err }
    }

    pub fn log_io_queue_error(err: String) -> Self {
        ANNError::IOQueueError { err }
    }

    pub fn log_kmeans_error(err: String) -> Self {
        ANNError::KMeansError { err }
    }
}
