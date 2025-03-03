use prost::Message;
use crc32fast::hash as crc32;
use chrono::Utc;

// use include! to include the generated code
include!(concat!(env!("OUT_DIR"), "/wal.rs"));

impl LogEntry {
    /// create a new LogEntry
    pub fn new(tx_id: u64, op: Operation, data: Vec<u8>) -> Self {
        let timestamp = Utc::now().timestamp();
        let op = op as i32;
        let mut entry = Self {
            tx_id,
            op,
            data,
            checksum: 0,
            timestamp,
        };
        entry.checksum = entry.calculate_checksum();
        entry
    }

    /// caculate the checksum
    fn calculate_checksum(&self) -> u32 {
        let mut temp = self.clone();
        temp.checksum = 0;
        let bytes = temp.encode_to_vec();
        crc32(&bytes)
    }

    /// validate the checksum
    pub fn validate(&self) -> bool {
        self.checksum == self.calculate_checksum()
    }
}