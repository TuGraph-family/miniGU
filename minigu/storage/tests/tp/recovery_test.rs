use std::fs;
use std::path::PathBuf;

use minigu_common::value::ScalarValue;
use minigu_storage::tp::MemoryGraph;
use minigu_storage::tp::checkpoint::GraphCheckpoint;
use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};
use serial_test::serial;

use crate::common::*;

fn test_db_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("recovery_test_{}.minigu", std::process::id()));
    path
}

fn cleanup(path: &PathBuf) {
    if path.exists() {
        let _ = fs::remove_file(path);
    }
}

#[test]
#[serial]
fn test_recovery_from_wal() {
    let db_path = test_db_path();
    cleanup(&db_path);
    // 1. Create a graph backed by a file
    {
        let graph = MemoryGraph::with_db_file(&db_path).unwrap();

        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        // Create Alice
        let alice = create_test_vertex(1, "Alice", 25);
        graph.create_vertex(&txn, alice).unwrap();

        txn.commit().unwrap();

        // Ensure data is synced to disk
        graph.persistence().sync_all().unwrap();
    } // Graph is dropped here

    // 2. Re-open graph from the same file (Recovery)
    {
        let graph = MemoryGraph::with_db_file(&db_path).unwrap();

        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        // Verify Alice exists
        let alice = graph.get_vertex(&txn, 1).unwrap();
        assert_eq!(alice.vid(), 1);
        assert_eq!(
            alice.properties()[0],
            ScalarValue::String(Some("Alice".to_string()))
        );
    }

    cleanup(&db_path);
}

#[test]
#[serial]
fn test_recovery_from_checkpoint_and_wal() {
    let db_path = test_db_path();
    cleanup(&db_path);

    // 1. Create graph and add some data (Checkpoint part)
    {
        let graph = MemoryGraph::with_db_file(&db_path).unwrap();
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        // Create Alice and Bob
        let alice = create_test_vertex(1, "Alice", 25);
        let bob = create_test_vertex(2, "Bob", 30);
        graph.create_vertex(&txn, alice).unwrap();
        graph.create_vertex(&txn, bob).unwrap();

        txn.commit().unwrap();

        // Create a checkpoint
        // Note: Checkpoint creation usually involves locking and truncating WAL
        // We simulate it by manually creating a checkpoint and writing it.
        // In a real scenario, a background thread or a manager would do this.
        let checkpoint = GraphCheckpoint::new(&graph);
        graph.persistence().write_checkpoint(&checkpoint).unwrap();
        graph
            .persistence()
            .truncate_wal_until(checkpoint.metadata.lsn)
            .unwrap();
        graph.persistence().sync_all().unwrap();

        // 2. Add more data (WAL part)
        let txn2 = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        // Create Carol (will be in WAL but not checkpoint)
        let carol = create_test_vertex(3, "Carol", 22);
        graph.create_vertex(&txn2, carol).unwrap();

        txn2.commit().unwrap();
        graph.persistence().sync_all().unwrap();
    } // Graph dropped

    // 3. Recover
    {
        let graph = MemoryGraph::with_db_file(&db_path).unwrap();

        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        // Verify Alice and Bob (from checkpoint)
        let alice = graph.get_vertex(&txn, 1).unwrap();
        assert_eq!(alice.vid(), 1);

        let bob = graph.get_vertex(&txn, 2).unwrap();
        assert_eq!(bob.vid(), 2);

        // Verify Carol (from WAL)
        let carol = graph.get_vertex(&txn, 3).unwrap();
        assert_eq!(carol.vid(), 3);
    }

    cleanup(&db_path);
}

#[test]
#[serial]
fn test_checkpoint_lsn_consistency() {
    let db_path = test_db_path();
    cleanup(&db_path);

    let checkpoint_lsn: u64;
    let lsn_before_checkpoint: u64;
    let lsn_after_checkpoint: u64;
    let final_lsn: u64;
    let initial_lsn: u64;

    // Phase 1: Create graph, perform operations, and create checkpoint
    {
        let graph = MemoryGraph::with_db_file(&db_path).unwrap();

        // Initial LSN should be 1 (starts from 0, incremented to 1)
        initial_lsn = graph.persistence().current_lsn();
        assert_eq!(initial_lsn, 1, "Initial LSN should be 1");

        // Transaction 1: Create Alice
        let txn1 = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        let alice = create_test_vertex(1, "Alice", 25);
        graph.create_vertex(&txn1, alice).unwrap();
        txn1.commit().unwrap();

        // Transaction 2: Create Bob
        let txn2 = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        let bob = create_test_vertex(2, "Bob", 30);
        graph.create_vertex(&txn2, bob).unwrap();
        txn2.commit().unwrap();

        // Get LSN before checkpoint
        lsn_before_checkpoint = graph.persistence().current_lsn();
        println!("LSN before checkpoint: {}", lsn_before_checkpoint);

        // LSN should have increased (BeginTxn + CreateVertex + CommitTxn for each transaction)
        // Transaction 1: LSN 1 (BeginTxn), LSN 2 (CreateVertex), LSN 3 (CommitTxn)
        // Transaction 2: LSN 4 (BeginTxn), LSN 5 (CreateVertex), LSN 6 (CommitTxn)
        // So current_lsn should be 7 (next available LSN)
        assert!(
            lsn_before_checkpoint > initial_lsn,
            "LSN should increase after transactions"
        );

        // Create checkpoint
        let checkpoint = GraphCheckpoint::new(&graph);
        checkpoint_lsn = checkpoint.metadata.lsn;

        println!("Checkpoint LSN: {}", checkpoint_lsn);

        // Checkpoint LSN should be the current LSN at the time of checkpoint creation
        // current_lsn() returns the NEXT LSN to be assigned, so checkpoint captures
        // the state "up to and including current_lsn"
        assert_eq!(
            checkpoint_lsn, lsn_before_checkpoint,
            "Checkpoint LSN should equal current_lsn (next available LSN)"
        );

        // Write checkpoint to persistence
        graph.persistence().write_checkpoint(&checkpoint).unwrap();
        graph
            .persistence()
            .truncate_wal_until(checkpoint_lsn)
            .unwrap();
        graph.persistence().sync_all().unwrap();

        // LSN after checkpoint should remain the same (checkpoint doesn't consume LSN)
        lsn_after_checkpoint = graph.persistence().current_lsn();
        assert_eq!(
            lsn_after_checkpoint, lsn_before_checkpoint,
            "LSN should not change after checkpoint creation"
        );

        // Transaction 3: Create Carol (after checkpoint)
        let txn3 = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        let carol = create_test_vertex(3, "Carol", 22);
        graph.create_vertex(&txn3, carol).unwrap();
        txn3.commit().unwrap();

        // Get final LSN
        final_lsn = graph.persistence().current_lsn();
        println!("Final LSN after Carol: {}", final_lsn);

        // LSN should continue to increase monotonically
        assert!(
            final_lsn > lsn_after_checkpoint,
            "LSN should continue to increase after checkpoint"
        );

        graph.persistence().sync_all().unwrap();
    } // Graph dropped

    // Phase 2: Recover from DB file and verify LSN values
    {
        let recovered_graph = MemoryGraph::with_db_file(&db_path).unwrap();

        // After recovery, next_lsn should be set to final_lsn
        let recovered_lsn = recovered_graph.persistence().current_lsn();
        println!("Recovered LSN: {}", recovered_lsn);

        assert_eq!(
            recovered_lsn, final_lsn,
            "Recovered LSN should match the final LSN from before shutdown"
        );

        // Verify checkpoint was loaded correctly
        let loaded_checkpoint = recovered_graph.persistence().read_checkpoint().unwrap();
        assert!(
            loaded_checkpoint.is_some(),
            "Checkpoint should exist after recovery"
        );

        let loaded_checkpoint = loaded_checkpoint.unwrap();
        assert_eq!(
            loaded_checkpoint.metadata.lsn, checkpoint_lsn,
            "Loaded checkpoint LSN should match original checkpoint LSN"
        );

        // Verify all vertices exist
        let txn = recovered_graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        let alice = recovered_graph.get_vertex(&txn, 1).unwrap();
        assert_eq!(alice.vid(), 1);
        assert_eq!(
            alice.properties()[0],
            ScalarValue::String(Some("Alice".to_string()))
        );

        let bob = recovered_graph.get_vertex(&txn, 2).unwrap();
        assert_eq!(bob.vid(), 2);
        assert_eq!(
            bob.properties()[0],
            ScalarValue::String(Some("Bob".to_string()))
        );

        let carol = recovered_graph.get_vertex(&txn, 3).unwrap();
        assert_eq!(carol.vid(), 3);
        assert_eq!(
            carol.properties()[0],
            ScalarValue::String(Some("Carol".to_string()))
        );

        txn.commit().unwrap();

        // Perform a new transaction after recovery to verify LSN continues correctly
        let txn_new = recovered_graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        let david = create_test_vertex(4, "David", 28);
        recovered_graph.create_vertex(&txn_new, david).unwrap();
        txn_new.commit().unwrap();

        let lsn_after_new_txn = recovered_graph.persistence().current_lsn();
        println!("LSN after new transaction: {}", lsn_after_new_txn);

        // LSN should continue to increase monotonically after recovery
        assert!(
            lsn_after_new_txn > recovered_lsn,
            "LSN should continue to increase monotonically after recovery"
        );

        // Verify strict monotonicity across the entire sequence
        assert!(initial_lsn < lsn_before_checkpoint);
        assert!(lsn_before_checkpoint <= lsn_after_checkpoint);
        assert!(lsn_after_checkpoint < final_lsn);
        assert_eq!(final_lsn, recovered_lsn);
        assert!(recovered_lsn < lsn_after_new_txn);

        println!(
            "LSN sequence verified: {} < {} <= {} < {} == {} < {}",
            initial_lsn,
            lsn_before_checkpoint,
            lsn_after_checkpoint,
            final_lsn,
            recovered_lsn,
            lsn_after_new_txn
        );
    }

    cleanup(&db_path);
}
