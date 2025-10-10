// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.
#![allow(dead_code)] // Todo: Remove this when the disk index query code is complete.
use crate::common::ANNError;
use crate::platform::FileHandle;

// The IOContext struct for disk I/O. One for each thread.
#[allow(clippy::upper_case_acronyms)]
pub struct IOContext {
    pub status: Status,
    pub file_handle: FileHandle,
}

impl Default for IOContext {
    fn default() -> Self {
        IOContext {
            status: Status::ReadWait,
            file_handle: FileHandle::default(),
        }
    }
}

impl IOContext {
    pub fn new() -> Self {
        Self::default()
    }
}

pub enum Status {
    ReadWait,
    ReadSuccess,
    ReadFailed(ANNError),
    ProcessComplete,
}
