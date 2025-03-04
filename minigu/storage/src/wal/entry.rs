use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Operation {
    BeginTx,
    CommitTx,
    InsertVertex,
    InsertEdge,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalEntry {
    // Transaction ID
    pub tx_id: u64,
    // Operation type
    pub op: Operation,
    // Byte array that stores the serialized data of the operation.
    pub data: Vec<u8>,
    // Checksum value
    pub checksum: u32,
    // Timestamp of the entry   
    pub timestamp: i64, 
}

impl WalEntry {
    pub fn new(tx_id: u64, op: Operation, data: Vec<u8>) -> Self {
        let mut entry = Self {
            tx_id,
            op,
            data,
            checksum: 0,    // Initialize to zero
            timestamp: chrono::Utc::now().timestamp(),
        };
        entry.checksum = entry.calculate_checksum();
        entry
    }

    fn calculate_checksum(&self) -> u32 {
        let mut temp = self.clone();
        temp.checksum = 0;
        let bytes = postcard::to_allocvec(&temp).unwrap(); 
        crc32fast::hash(&bytes)
    }

    pub fn validate(&self) -> bool {
        self.checksum == self.calculate_checksum()
    }
}