// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.
#![warn(missing_docs)]

//! Disk graph storage

use std::sync::{Arc, Mutex};

use crate::common::{ANNError, ANNResult};
use crate::model::{AlignedRead, IOContext, LinuxAlignedFileReader};

/// Graph storage for disk index
/// One thread has one storage instance
pub struct DiskGraphStorage {
    /// Disk graph reader
    disk_graph_reader: Arc<Mutex<LinuxAlignedFileReader>>,

    /// IO Context
    io_context: Arc<IOContext>,
}

impl DiskGraphStorage {
    /// Create a new DiskGraphStorage instance
    pub fn new(
        disk_graph_reader: Arc<Mutex<LinuxAlignedFileReader>>,
        io_context: Arc<IOContext>,
    ) -> ANNResult<Self> {
        Ok(Self {
            disk_graph_reader,
            io_context,
        })
    }

    /// Get the IO context
    pub fn get_ctx(&self) -> &IOContext {
        &self.io_context
    }

    /// Read disk graph data
    pub fn read<T>(&self, read_requests: &mut [AlignedRead<T>], ctx: &IOContext) -> ANNResult<()> {
        self.disk_graph_reader
            .lock()
            .map_err(|e| ANNError::log_index_error(format!("Mutex poisoned: {e}")))?
            .read(read_requests, ctx)
    }
}
