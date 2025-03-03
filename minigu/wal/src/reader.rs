use std::{error::Error, path::Path};
use prost::Message;
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};
use crate::wal::LogEntry;

pub struct WalReader {
    reader: BufReader<File>,
}

impl WalReader {
    /// open a file for reading
    pub async fn open(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let file = File::open(path).await?;
        Ok(Self {
            reader: BufReader::new(file),
        })
    }

    /// read all log entries from file
    pub async fn read_all(&mut self) -> Result<Vec<LogEntry>, Box<dyn Error>> {
        let mut buffer = Vec::new();
        self.reader.read_to_end(&mut buffer).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let mut cursor = std::io::Cursor::new(&buffer);
        let mut entries = Vec::new();

        loop {
            match LogEntry::decode_length_delimited(&mut cursor) {
                Ok(entry) => {
                    if entry.validate() {
                        entries.push(entry);
                    }
                }
                Err(_) => {
                    // TODO: handle error
                    break;
                }
            }
            if buffer.is_empty() { break; }
        }

        Ok(entries)
    }
}