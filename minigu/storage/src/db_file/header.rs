//! Database file header implementation.
//!
//! The header is located at the beginning of the database file and contains
//! metadata about the file format, version, and region offsets.

use std::io::{Read, Seek, SeekFrom, Write};

use crc32fast::Hasher;
use serde::{Deserialize, Serialize};

use super::error::{DbFileError, DbFileResult};

/// Magic number identifying a miniGU database file.
pub const MAGIC: [u8; 8] = *b"MINIGU\x00\x00";

/// Current file format version.
pub const CURRENT_VERSION: u32 = 1;

/// Minimum header size in bytes.
/// The header uses self-describing length, but is always at least 256 bytes.
pub const HEADER_SIZE: u64 = 256;

/// Flags for database file features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DbFileFlags {
    bits: u32,
}

impl DbFileFlags {
    /// Checkpoint region contains valid data.
    pub const HAS_CHECKPOINT: u32 = 1 << 0;
    /// WAL region contains valid data.
    pub const HAS_WAL: u32 = 1 << 1;
    /// No special flags.
    pub const NONE: u32 = 0;

    /// Creates a new flags instance.
    pub fn new(bits: u32) -> Self {
        Self { bits }
    }

    /// Returns the raw bits.
    pub fn bits(&self) -> u32 {
        self.bits
    }

    /// Checks if a flag is set.
    pub fn has(&self, flag: u32) -> bool {
        self.bits & flag != 0
    }

    /// Sets a flag.
    pub fn set(&mut self, flag: u32) {
        self.bits |= flag;
    }

    /// Clears a flag.
    pub fn clear(&mut self, flag: u32) {
        self.bits &= !flag;
    }
}

/// Database file header.
///
/// This structure is stored at the beginning of the database file and contains
/// all metadata needed to locate and validate the checkpoint and WAL regions.
///
/// # Header Layout (Fixed 256 Bytes)
///
/// | Offset | Size | Type | Field | Description |
/// |--------|------|------|-------|-------------|
/// | 0 | 8 | `[u8; 8]` | `magic` | Magic bytes "MINIGU\0\0" |
/// | 8 | 4 | `u32` | `version` | File format version (current: 1) |
/// | 12 | 4 | `u32` | `header_size` | Total header size (256 bytes) |
/// | 16 | 4 | `u32` | `flags` | Feature flags (bitmask) |
/// | 20 | 8 | `u64` | `checkpoint_offset` | Offset to start of checkpoint region |
/// | 28 | 8 | `u64` | `checkpoint_length` | Length of valid checkpoint data |
/// | 36 | 8 | `u64` | `wal_offset` | Offset to start of WAL region |
/// | 44 | 8 | `u64` | `wal_length` | Length of valid WAL data |
/// | 52 | 8 | `u64` | `last_lsn` | Max LSN contained in the file |
/// | 60 | 8 | `u64` | `last_commit_ts` | Timestamp of last committed txn |
/// | 68 | 4 | `u32` | `header_crc` | CRC32 of fields 0..68 |
/// | 72 | 184 | `[u8]` | - | Padding (zero-filled) |
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbFileHeader {
    /// Magic number (must be "MINIGU\0\0").
    pub magic: [u8; 8],
    /// File format version.
    pub version: u32,
    /// Header size in bytes (for self-describing length).
    pub header_size: u32,
    /// Feature flags.
    pub flags: DbFileFlags,
    /// Offset of the checkpoint region from file start.
    pub checkpoint_offset: u64,
    /// Length of the checkpoint region in bytes.
    pub checkpoint_length: u64,
    /// Offset of the WAL region from file start.
    pub wal_offset: u64,
    /// Length of the WAL region in bytes (used portion).
    pub wal_length: u64,
    /// Last LSN written to the WAL.
    pub last_lsn: u64,
    /// Last commit timestamp.
    pub last_commit_ts: u64,
    /// CRC32 checksum of the header (excluding this field).
    pub header_crc: u32,
}

impl Default for DbFileHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl DbFileHeader {
    /// Creates a new header with default values.
    pub fn new() -> Self {
        Self {
            magic: MAGIC,
            version: CURRENT_VERSION,
            header_size: HEADER_SIZE as u32,
            flags: DbFileFlags::default(),
            checkpoint_offset: HEADER_SIZE,
            checkpoint_length: 0,
            wal_offset: HEADER_SIZE,
            wal_length: 0,
            last_lsn: 0,
            last_commit_ts: 0,
            header_crc: 0,
        }
    }

    /// Validates the magic number.
    pub fn validate_magic(&self) -> DbFileResult<()> {
        if self.magic != MAGIC {
            return Err(DbFileError::InvalidMagic);
        }
        Ok(())
    }

    /// Validates the version.
    pub fn validate_version(&self) -> DbFileResult<()> {
        if self.version > CURRENT_VERSION {
            return Err(DbFileError::UnsupportedVersion(
                self.version,
                CURRENT_VERSION,
            ));
        }
        Ok(())
    }

    /// Computes the CRC32 checksum of the header (excluding the crc field).
    fn compute_crc(&self) -> u32 {
        let mut hasher = Hasher::new();
        hasher.update(&self.magic);
        hasher.update(&self.version.to_le_bytes());
        hasher.update(&self.header_size.to_le_bytes());
        hasher.update(&self.flags.bits().to_le_bytes());
        hasher.update(&self.checkpoint_offset.to_le_bytes());
        hasher.update(&self.checkpoint_length.to_le_bytes());
        hasher.update(&self.wal_offset.to_le_bytes());
        hasher.update(&self.wal_length.to_le_bytes());
        hasher.update(&self.last_lsn.to_le_bytes());
        hasher.update(&self.last_commit_ts.to_le_bytes());
        hasher.finalize()
    }

    /// Updates the CRC field with the computed checksum.
    pub fn update_crc(&mut self) {
        self.header_crc = self.compute_crc();
    }

    /// Validates the CRC checksum.
    pub fn validate_crc(&self) -> DbFileResult<()> {
        let computed = self.compute_crc();
        if computed != self.header_crc {
            return Err(DbFileError::HeaderChecksumMismatch {
                expected: self.header_crc,
                actual: computed,
            });
        }
        Ok(())
    }

    /// Writes the header to a writer.
    pub fn write_to<W: Write + Seek>(&self, writer: &mut W) -> DbFileResult<()> {
        writer.seek(SeekFrom::Start(0))?;

        // Write fields in order
        writer.write_all(&self.magic)?;
        writer.write_all(&self.version.to_le_bytes())?;
        writer.write_all(&self.header_size.to_le_bytes())?;
        writer.write_all(&self.flags.bits().to_le_bytes())?;
        writer.write_all(&self.checkpoint_offset.to_le_bytes())?;
        writer.write_all(&self.checkpoint_length.to_le_bytes())?;
        writer.write_all(&self.wal_offset.to_le_bytes())?;
        writer.write_all(&self.wal_length.to_le_bytes())?;
        writer.write_all(&self.last_lsn.to_le_bytes())?;
        writer.write_all(&self.last_commit_ts.to_le_bytes())?;
        writer.write_all(&self.header_crc.to_le_bytes())?;

        // Pad to header_size
        let written = 8 + 4 + 4 + 4 + 8 + 8 + 8 + 8 + 8 + 8 + 4; // 72 bytes
        let padding = self.header_size as usize - written;
        writer.write_all(&vec![0u8; padding])?;

        Ok(())
    }

    /// Reads the header from a reader.
    pub fn read_from<R: Read + Seek>(reader: &mut R) -> DbFileResult<Self> {
        reader.seek(SeekFrom::Start(0))?;

        let mut magic = [0u8; 8];
        reader.read_exact(&mut magic)?;

        let mut buf4 = [0u8; 4];
        let mut buf8 = [0u8; 8];

        reader.read_exact(&mut buf4)?;
        let version = u32::from_le_bytes(buf4);

        reader.read_exact(&mut buf4)?;
        let header_size = u32::from_le_bytes(buf4);

        reader.read_exact(&mut buf4)?;
        let flags = DbFileFlags::new(u32::from_le_bytes(buf4));

        reader.read_exact(&mut buf8)?;
        let checkpoint_offset = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let checkpoint_length = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let wal_offset = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let wal_length = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let last_lsn = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let last_commit_ts = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf4)?;
        let header_crc = u32::from_le_bytes(buf4);

        let header = Self {
            magic,
            version,
            header_size,
            flags,
            checkpoint_offset,
            checkpoint_length,
            wal_offset,
            wal_length,
            last_lsn,
            last_commit_ts,
            header_crc,
        };

        // Validate
        header.validate_magic()?;
        header.validate_version()?;
        header.validate_crc()?;

        Ok(header)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_header_roundtrip() {
        let mut header = DbFileHeader::new();
        header.checkpoint_offset = 256;
        header.checkpoint_length = 1024;
        header.wal_offset = 1280;
        header.wal_length = 512;
        header.last_lsn = 42;
        header.last_commit_ts = 100;
        header.flags.set(DbFileFlags::HAS_CHECKPOINT);
        header.flags.set(DbFileFlags::HAS_WAL);
        header.update_crc();

        let mut buf = vec![0u8; HEADER_SIZE as usize];
        let mut cursor = Cursor::new(&mut buf);
        header.write_to(&mut cursor).unwrap();

        cursor.set_position(0);
        let read_header = DbFileHeader::read_from(&mut cursor).unwrap();

        assert_eq!(read_header.magic, MAGIC);
        assert_eq!(read_header.version, CURRENT_VERSION);
        assert_eq!(read_header.checkpoint_offset, 256);
        assert_eq!(read_header.checkpoint_length, 1024);
        assert_eq!(read_header.wal_offset, 1280);
        assert_eq!(read_header.wal_length, 512);
        assert_eq!(read_header.last_lsn, 42);
        assert_eq!(read_header.last_commit_ts, 100);
        assert!(read_header.flags.has(DbFileFlags::HAS_CHECKPOINT));
        assert!(read_header.flags.has(DbFileFlags::HAS_WAL));
    }

    #[test]
    fn test_invalid_magic() {
        let mut header = DbFileHeader::new();
        header.magic = *b"INVALID\x00";
        header.update_crc();

        let mut buf = vec![0u8; HEADER_SIZE as usize];
        let mut cursor = Cursor::new(&mut buf);
        header.write_to(&mut cursor).unwrap();

        cursor.set_position(0);
        let result = DbFileHeader::read_from(&mut cursor);
        assert!(matches!(result, Err(DbFileError::InvalidMagic)));
    }

    #[test]
    fn test_crc_validation() {
        let mut header = DbFileHeader::new();
        header.update_crc();

        let mut buf = vec![0u8; HEADER_SIZE as usize];
        let mut cursor = Cursor::new(&mut buf);
        header.write_to(&mut cursor).unwrap();

        // Corrupt a byte
        buf[20] ^= 0xFF;

        cursor = Cursor::new(&mut buf);
        let result = DbFileHeader::read_from(&mut cursor);
        assert!(matches!(
            result,
            Err(DbFileError::HeaderChecksumMismatch { .. })
        ));
    }
}
