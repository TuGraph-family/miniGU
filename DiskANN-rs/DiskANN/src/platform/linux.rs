// Copyright (c) Microsoft Corporation.
// Copyright (c) 2025 MiniGU. All rights reserved.
//
// Licensed under the MIT License. See DiskANN-rs/LICENSE for license information.
//
// Linux platform implementation adapted from Microsoft's file_handle.rs (Windows version):
// - Simplified FileHandle using standard library (File, OpenOptions) instead of winapi
// - Supports Read-only mode for DiskANN index file access
use std::fs::{File, OpenOptions};
use std::io;

#[derive(Debug, Default)]
pub struct FileHandle(Option<File>);

#[derive(Debug, Clone, Copy)]
pub enum AccessMode {
    Read,
}

#[derive(Debug, Clone, Copy)]
pub enum ShareMode {
    Read,
}

impl FileHandle {
    pub fn new(path: &str, _access: AccessMode, _share: ShareMode) -> io::Result<Self> {
        let file = OpenOptions::new().read(true).open(path)?;
        Ok(FileHandle(Some(file)))
    }

    #[allow(clippy::expect_used)]
    pub fn file(&self) -> &File {
        self.0
            .as_ref()
            .expect("FileHandle should always contain a file")
    }
}
