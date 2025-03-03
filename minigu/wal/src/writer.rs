use tokio::{
    fs::{OpenOptions, File},
    io::{AsyncWriteExt, BufWriter},
};
use std::path::Path;
use prost::Message;
use crate::wal::LogEntry;

pub struct WalWriter {
    writer: BufWriter<File>,
}

impl WalWriter {
    /// open a file for writing
    pub async fn open(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .await?;
        Ok(Self {
            writer: BufWriter::new(file),
        })
    }

    /// append log entries to file
    pub async fn append(&mut self, entries: &[LogEntry]) -> Result<(), std::io::Error> {
        for entry in entries {
            let bytes: Vec<u8> = entry.encode_length_delimited_to_vec();
            self.writer.write_all(&bytes).await?;
        }
        self.writer.flush().await?;
        Ok(())
    }
}