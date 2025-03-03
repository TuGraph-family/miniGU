use tokio::runtime::Runtime;
use wal::{wal::{LogEntry, Operation, InsertVertexData, Properties}, writer::WalWriter, reader::WalReader};
use prost::Message;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::tempdir;
    use std::path::PathBuf;

    #[test]
    fn test_log_entry_creation() {
        let properties: Properties = Properties {
            properties: {
                let mut map = HashMap::new();
                map.insert(1, "20".to_string());
                map.insert(2, "alice".to_string());
                map
            },
        };
        let insert_vertex_data = InsertVertexData {
            id: 123,
            properties: Some(properties),
        };
        let data = insert_vertex_data.encode_to_vec();
        let entry = LogEntry::new(1, Operation::InsertVertex, data.clone());
        assert_eq!(entry.tx_id, 1);
        assert_eq!(entry.op, Operation::InsertVertex as i32);
        assert_eq!(entry.data, data);
        assert!(entry.validate());
    }

    #[test]
    fn test_log_entry_validation() {
        let data = vec![1, 2, 3];
        let mut entry = LogEntry::new(1, Operation::InsertVertex, data);
        assert!(entry.validate());
        entry.checksum = 0;
        assert!(!entry.validate());
    }

    #[test]
    fn test_wal_writer_append() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let temp_dir = tempdir().expect("failed to create temp dir");
            let file_path: PathBuf = temp_dir.path().join("test_wal.log");

            let mut writer = WalWriter::open(&file_path).await.unwrap();
            let entry = LogEntry::new(1, Operation::InsertVertex, vec![1, 2, 3]);
            writer.append(&[entry]).await.unwrap();
        });
    }

    #[test]
    fn test_wal_reader_read_all() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let temp_dir = tempdir().expect("failed to create temp dir");
            let file_path: PathBuf = temp_dir.path().join("test_wal.log");

            let mut writer = WalWriter::open(&file_path).await.unwrap();
            let entry1 = LogEntry::new(1, Operation::InsertVertex, vec![1, 2, 3]);
            let entry2 = LogEntry::new(2, Operation::InsertEdge, vec![4, 5, 6]);
            writer.append(&[entry1, entry2]).await.unwrap();

            let mut reader = WalReader::open(&file_path).await.unwrap();
            let entries = reader.read_all().await.unwrap();
            assert_eq!(entries[0].tx_id, 1);
            assert_eq!(entries[1].tx_id, 2);
        });
    }

    #[test]
    fn test_wal_writer_and_reader_integration() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let temp_dir = tempdir().expect("failed to create temp dir");
            let file_path: PathBuf = temp_dir.path().join("test_wal.log");

            let mut writer = WalWriter::open(&file_path).await.unwrap();
            let entry1 = LogEntry::new(1, Operation::InsertVertex, vec![1, 2, 3]);
            let entry2 = LogEntry::new(2, Operation::InsertEdge, vec![4, 5, 6]);
            writer.append(&[entry1, entry2]).await.unwrap();

            let mut reader = WalReader::open(&file_path).await.unwrap();
            let entries = reader.read_all().await.unwrap();
            assert_eq!(entries[0].tx_id, 1);
            assert_eq!(entries[0].op, Operation::InsertVertex as i32);
            assert_eq!(entries[1].tx_id, 2);
            assert_eq!(entries[1].op, Operation::InsertEdge as i32);
        });
    }
}