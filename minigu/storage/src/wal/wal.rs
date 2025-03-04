use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write, BufReader, BufWriter, Cursor},
    path::Path,
};
use bytes::BytesMut;
use postcard;
use crate::wal::{
    error::WalError, 
    entry::WalEntry,
};

const LEN_PREFIX_SIZE: usize = 4;
const BUF_WRITER_CAPACITY: usize = 1024 * 1024;

pub trait WalTrait {
    type Error;
    type Entry;
    type Iter: WalEntryIteratorTrait<Error = Self::Error, Entry = Self::Entry>;

    /// Opens a file (supports both read and write)
    fn open(path: impl AsRef<Path>) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Writes a single entry.
    fn write(&mut self, entry: &Self::Entry) -> Result<(), Self::Error>;

    /// Writes multiple entries.
    fn write_all(&mut self, entries: &[Self::Entry]) -> Result<(), Self::Error>;

    /// Reads all entries from the file and deserializes them one by one.
    fn read(&mut self) -> Result<Vec<Self::Entry>, Self::Error>;

    /// Creates a new reader iterator to traverse all entries.
    /// To avoid interfering with the internal BufReader, this method creates a separate iterator by cloning the file handle.
    fn new_reader(&self) -> Result<Self::Iter, Self::Error>;
}

pub trait WalEntryIteratorTrait {
    type Error;
    type Entry;

    /// Read the next entry.
    /// Returns Ok(None) when the end of the file is reached.
    fn next(&mut self) -> Result<Option<Self::Entry>, Self::Error>;
}

pub struct StorageWal {
    writer: BufWriter<File>,
    reader: BufReader<File>,
}

impl WalTrait for StorageWal {
    type Error = WalError;
    type Entry = WalEntry;
    type Iter = StorageWalEntryIterator;

    fn open(path: impl AsRef<Path>) -> Result<Self, WalError> {
        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(path)
            .map_err(WalError::Io)?;
        
        let writer = BufWriter::with_capacity(BUF_WRITER_CAPACITY, file.try_clone().map_err(WalError::Io)?);
        let reader = BufReader::new(file.try_clone().map_err(WalError::Io)?);
        Ok(Self { writer, reader })
    }

    fn write(&mut self, entry: &Self::Entry) -> Result<(), WalError> {
        let bytes = postcard::to_allocvec(entry).map_err(WalError::Serialize)?;
        let len = bytes.len() as u32;
        self.writer.write_all(&len.to_le_bytes()).map_err(WalError::Io)?;
        self.writer.write_all(&bytes).map_err(WalError::Io)?;
        self.writer.flush().map_err(WalError::Io)?;
        Ok(())
    }

    fn write_all(&mut self, entries: &[Self::Entry]) -> Result<(), WalError> {
        let mut buffer = BytesMut::new();
        for entry in entries {
            let bytes = postcard::to_allocvec(entry).map_err(WalError::Serialize)?;
            let len = bytes.len() as u32;
            buffer.extend_from_slice(&len.to_le_bytes());
            buffer.extend_from_slice(&bytes);
        }
        self.writer.write_all(&buffer).map_err(WalError::Io)?;
        self.writer.flush().map_err(WalError::Io)?;
        Ok(())
    }

    fn read(&mut self) -> Result<Vec<Self::Entry>, WalError> {
        self.reader.seek(SeekFrom::Start(0)).map_err(WalError::Io)?;
        let mut buffer = Vec::new();
        self.reader.read_to_end(&mut buffer).map_err(WalError::Io)?;

        let mut entries = Vec::new();
        let mut cursor = Cursor::new(&buffer);

        while (cursor.position() as usize) < buffer.len() {
            let mut len_bytes = [0u8; LEN_PREFIX_SIZE];
            if let Err(_) = cursor.read_exact(&mut len_bytes) {
                break;
            }
            let len = u32::from_le_bytes(len_bytes) as usize;
            let mut entry_bytes = vec![0; len];
            cursor.read_exact(&mut entry_bytes).map_err(|_| {
                WalError::Deserialize(postcard::Error::DeserializeUnexpectedEnd)
            })?;
            let entry: WalEntry = postcard::from_bytes(&entry_bytes).map_err(WalError::Deserialize)?;
            if entry.validate() {
                entries.push(entry);
            }
        }
        Ok(entries)
    }

    fn new_reader(&self) -> Result<StorageWalEntryIterator, WalError> {
        let file = self.reader.get_ref().try_clone().map_err(WalError::Io)?;
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(0)).map_err(WalError::Io)?;
        Ok(StorageWalEntryIterator { reader })
    }
}

pub struct StorageWalEntryIterator {
    reader: BufReader<File>,
}

impl WalEntryIteratorTrait for StorageWalEntryIterator {
    type Error = WalError;
    type Entry = WalEntry;

    fn next(&mut self) -> Result<Option<Self::Entry>, WalError> {
        let mut len_bytes = [0u8; LEN_PREFIX_SIZE];
        // Attempt to read the length prefix; if EOF is encountered, return None.
        match self.reader.read_exact(&mut len_bytes) {
            Ok(_) => {},
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(WalError::Io(e)),
        }
        let len = u32::from_le_bytes(len_bytes) as usize;
        let mut entry_bytes = vec![0; len];
        self.reader.read_exact(&mut entry_bytes).map_err(|_| {
            WalError::Deserialize(postcard::Error::DeserializeUnexpectedEnd)
        })?;
        let entry: WalEntry = postcard::from_bytes(&entry_bytes).map_err(WalError::Deserialize)?;
        if entry.validate() {
            Ok(Some(entry))
        } else {
            // If validation fails, skip this entry and continue reading.
            Ok(None)
        }
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use super::{StorageWal, WalTrait, WalEntryIteratorTrait};
    use super::super::entry::{WalEntry, Operation};

    /// Test the basic write and read operations.
    #[test]
    fn test_wal_write_and_read() {
        let temp_path: PathBuf = std::env::temp_dir().join("wal_test.log");
        let _ = fs::remove_file(&temp_path);

        let mut wal = StorageWal::open(&temp_path).expect("Failed to open WAL");

        let entries = vec![
            WalEntry::new(1, Operation::BeginTx, b"begin".to_vec()),
            WalEntry::new(1, Operation::InsertVertex, b"vertex".to_vec()),
            WalEntry::new(1, Operation::CommitTx, b"commit".to_vec()),
        ];

        wal.write_all(&entries).expect("Failed to write entries");

        let read_entries = wal.read().expect("Failed to read entries");

        assert_eq!(read_entries.len(), entries.len(), "Mismatch in entry count");

        for (written, read) in entries.iter().zip(read_entries.iter()) {
            assert_eq!(written.tx_id, read.tx_id, "Transaction ID mismatch");
            assert_eq!(written.data, read.data, "Data mismatch");
            assert!(read.validate(), "Entry validation failed");
        }

        let _ = fs::remove_file(&temp_path);
    }

    /// Test writing single log entries and then reading them back.
    #[test]
    fn test_wal_single_write() {
        let temp_path: PathBuf = std::env::temp_dir().join("wal_single_write_test.log");
        let _ = fs::remove_file(&temp_path);

        let mut wal = StorageWal::open(&temp_path).expect("Failed to open WAL");

        let entry1 = WalEntry::new(10, Operation::BeginTx, b"start".to_vec());
        let entry2 = WalEntry::new(10, Operation::CommitTx, b"end".to_vec());
        wal.write(&entry1).expect("Failed to write entry1");
        wal.write(&entry2).expect("Failed to write entry2");

        let read_entries = wal.read().expect("Failed to read entries");
        assert_eq!(read_entries.len(), 2, "Expected 2 entries");

        assert_eq!(read_entries[0].tx_id, entry1.tx_id, "Entry1 Transaction ID mismatch");
        assert_eq!(read_entries[0].data, entry1.data, "Entry1 Data mismatch");
        assert_eq!(read_entries[1].tx_id, entry2.tx_id, "Entry2 Transaction ID mismatch");
        assert_eq!(read_entries[1].data, entry2.data, "Entry2 Data mismatch");

        let _ = fs::remove_file(&temp_path);
    }

    /// Test the iterator functionality to traverse log entries.
    #[test]
    fn test_wal_iterator() {
        let temp_path: PathBuf = std::env::temp_dir().join("wal_iterator_test.log");
        let _ = fs::remove_file(&temp_path);

        let mut wal = StorageWal::open(&temp_path).expect("Failed to open WAL");

        let entries = vec![
            WalEntry::new(20, Operation::BeginTx, b"init".to_vec()),
            WalEntry::new(20, Operation::InsertEdge, b"edge1".to_vec()),
            WalEntry::new(20, Operation::InsertEdge, b"edge2".to_vec()),
            WalEntry::new(20, Operation::CommitTx, b"finish".to_vec()),
        ];
        wal.write_all(&entries).expect("Failed to write entries");

        let mut iter = wal.new_reader().expect("Failed to create WAL iterator");
        let mut iterated_entries = Vec::new();

        while let Ok(Some(entry)) = iter.next() {
            iterated_entries.push(entry);
        }

        assert_eq!(iterated_entries.len(), entries.len(), "Iterator did not yield all entries");

        for (expected, actual) in entries.iter().zip(iterated_entries.iter()) {
            assert_eq!(expected.tx_id, actual.tx_id, "Transaction ID mismatch in iterator");
            assert_eq!(expected.data, actual.data, "Data mismatch in iterator");
        }

        let _ = fs::remove_file(&temp_path);
    }
}