//! Stress tests and integration tests for the db_file module.
//!
//! These tests verify:
//! - Large data volume handling
//! - Crash recovery simulation
//! - Data integrity under various conditions

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use minigu_transaction::{IsolationLevel, Timestamp};
use serial_test::serial;

use crate::common::DeltaOp;
use crate::common::wal::graph_wal::{Operation, RedoEntry};
use crate::db_file::DbFile;

static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_test_dir(test_name: &str) -> PathBuf {
    let counter = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let mut path = std::env::temp_dir();
    path.push(format!(
        "minigu_stress_{}_{}_{}",
        test_name,
        std::process::id(),
        counter
    ));
    path
}

fn cleanup(path: &std::path::Path) {
    let _ = fs::remove_dir_all(path);
}

/// Test writing and reading a large number of WAL entries.
/// Skip this test because it takes too long.
#[test]
#[serial]
#[ignore]
fn test_large_wal_volume() {
    let base = unique_test_dir("large_wal");
    cleanup(&base);
    fs::create_dir_all(&base).unwrap();

    let db_path = base.join("large_wal.minigu");
    let num_entries = 10_000;

    // Write many entries
    {
        let mut db = DbFile::create(&db_path).unwrap();

        for i in 1..=num_entries {
            let entry = RedoEntry {
                lsn: i as u64,
                txn_id: Timestamp::with_ts(1000 + i as u64),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::DelVertex(i as u64)),
            };
            db.append_wal(&entry).unwrap();

            // Periodic flush to simulate real usage
            if i % 1000 == 0 {
                db.flush().unwrap();
            }
        }
        db.sync_all().unwrap();
    }

    // Read and verify all entries
    {
        let mut db = DbFile::open(&db_path).unwrap();
        let entries = db.read_wal_entries().unwrap();

        assert_eq!(entries.len(), num_entries);

        // Verify entries are in order
        for (i, entry) in entries.iter().enumerate() {
            assert_eq!(entry.lsn, (i + 1) as u64);
        }
    }

    cleanup(&base);
}

/// Test crash recovery - simulate crash by not syncing and reopening.
#[test]
#[serial]
fn test_crash_recovery_no_sync() {
    let base = unique_test_dir("crash_recovery");
    cleanup(&base);
    fs::create_dir_all(&base).unwrap();

    let db_path = base.join("crash_recovery.minigu");
    let synced_entries = 50;
    let unsynced_entries = 10;

    // Write entries, sync some, then "crash" (drop without syncing)
    {
        let mut db = DbFile::create(&db_path).unwrap();

        // Write and sync first batch
        for i in 1..=synced_entries {
            let entry = RedoEntry {
                lsn: i as u64,
                txn_id: Timestamp::with_ts(1000 + i as u64),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::DelVertex(i as u64)),
            };
            db.append_wal(&entry).unwrap();
        }
        db.sync_all().unwrap();

        // Write second batch without sync (simulates crash)
        for i in (synced_entries + 1)..=(synced_entries + unsynced_entries) {
            let entry = RedoEntry {
                lsn: i as u64,
                txn_id: Timestamp::with_ts(1000 + i as u64),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::DelVertex(i as u64)),
            };
            db.append_wal(&entry).unwrap();
        }
        // Note: Not calling sync_all() here to simulate crash
    }

    // Reopen and verify - should recover at least the synced entries
    {
        let mut db = DbFile::open(&db_path).unwrap();
        let entries = db.read_wal_entries().unwrap();

        // We should have at least the synced entries
        // (buffered entries may or may not be present depending on OS behavior)
        assert!(entries.len() >= synced_entries);

        // Verify the entries we do have are valid
        for (i, entry) in entries.iter().enumerate() {
            assert_eq!(entry.lsn, (i + 1) as u64);
        }
    }

    cleanup(&base);
}

/// Test reopening database file multiple times.
#[test]
#[serial]
fn test_multiple_reopen() {
    let base = unique_test_dir("reopen");
    cleanup(&base);
    fs::create_dir_all(&base).unwrap();

    let db_path = base.join("reopen.minigu");
    let entries_per_session = 100;
    let num_sessions = 10;

    for session in 0..num_sessions {
        let mut db = if session == 0 {
            DbFile::create(&db_path).unwrap()
        } else {
            DbFile::open(&db_path).unwrap()
        };

        // Verify existing entries
        let existing = db.read_wal_entries().unwrap();
        assert_eq!(existing.len(), session * entries_per_session);

        // Add more entries
        let start_lsn = (session * entries_per_session + 1) as u64;
        for i in 0..entries_per_session {
            let entry = RedoEntry {
                lsn: start_lsn + i as u64,
                txn_id: Timestamp::with_ts(1000 + start_lsn + i as u64),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::DelVertex(start_lsn + i as u64)),
            };
            db.append_wal(&entry).unwrap();
        }
        db.sync_all().unwrap();
    }

    // Final verification
    {
        let mut db = DbFile::open(&db_path).unwrap();
        let entries = db.read_wal_entries().unwrap();
        assert_eq!(entries.len(), num_sessions * entries_per_session);

        // Verify all entries are sequential
        for (i, entry) in entries.iter().enumerate() {
            assert_eq!(entry.lsn, (i + 1) as u64);
        }
    }

    cleanup(&base);
}

/// Test file size growth with various entry sizes.
#[test]
#[serial]
fn test_variable_entry_sizes() {
    use minigu_common::value::ScalarValue;

    use crate::common::SetPropsOp;

    let base = unique_test_dir("variable_size");
    cleanup(&base);
    fs::create_dir_all(&base).unwrap();

    let db_path = base.join("variable_size.minigu");
    let num_entries = 100;

    {
        let mut db = DbFile::create(&db_path).unwrap();

        for i in 1..=num_entries {
            // Create entries with varying payload sizes
            let props: Vec<ScalarValue> = (0..i)
                .map(|j| ScalarValue::String(format!("value_{}_prop_{}", i, j).into()))
                .collect();

            let entry = RedoEntry {
                lsn: i as u64,
                txn_id: Timestamp::with_ts(1000 + i as u64),
                iso_level: IsolationLevel::Serializable,
                op: Operation::Delta(DeltaOp::SetVertexProps(
                    i as u64,
                    SetPropsOp {
                        indices: (0..i).collect(),
                        props,
                    },
                )),
            };
            db.append_wal(&entry).unwrap();
        }
        db.sync_all().unwrap();
    }

    // Verify
    {
        let mut db = DbFile::open(&db_path).unwrap();

        let entries = db.read_wal_entries().unwrap();
        assert_eq!(entries.len(), num_entries);
    }

    cleanup(&base);
}
