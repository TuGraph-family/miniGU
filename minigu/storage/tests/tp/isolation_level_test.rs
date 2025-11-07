use std::thread;

use minigu_common::value::ScalarValue;
use minigu_storage::model::edge::Edge;
use minigu_storage::model::properties::PropertyRecord;
use minigu_storage::model::vertex::Vertex;
use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

use crate::common::*;

// ========== DIRTY READ TESTS ==========

#[test]
fn test_serializable_prevents_dirty_read_vertex() {
    let (graph, _cleaner) = create_test_graph();

    // Transaction 1 reads the vertex
    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let alice_v1 = graph.get_vertex(&txn1, 1).unwrap();
    assert_eq!(alice_v1.properties()[1], ScalarValue::Int32(Some(25)));

    // Transaction 2 modifies the vertex but does not commit
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    graph
        .set_vertex_property(&txn2, 1, vec![1], vec![ScalarValue::Int32(Some(26))])
        .unwrap();

    // Transaction 1 tries to read the vertex
    let alice_v2 = graph.get_vertex(&txn1, 1).unwrap();
    assert_eq!(alice_v2.properties()[1], ScalarValue::Int32(Some(25))); // Should see original value

    assert!(txn2.commit().is_ok());
    assert!(txn1.commit().is_err()); // Should fail due to read-write conflict
}

#[test]
fn test_serializable_prevents_dirty_read_edge() {
    let (graph, _cleaner) = create_test_graph();

    // Transaction 1 reads the edge
    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let edge_v1 = graph.get_edge(&txn1, 1).unwrap();
    assert_eq!(
        edge_v1.properties()[0],
        ScalarValue::String(Some("2024-01-01".to_string()))
    );

    // Transaction 2 modifies the edge but does not commit
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    graph
        .set_edge_property(&txn2, 1, vec![0], vec![ScalarValue::String(Some(
            "2024-02-01".to_string(),
        ))])
        .unwrap();

    let edge_v2 = graph.get_edge(&txn1, 1).unwrap();
    assert_eq!(
        edge_v2.properties()[0],
        ScalarValue::String(Some("2024-01-01".to_string()))
    );

    assert!(txn2.commit().is_ok());
    assert!(txn1.commit().is_err()); // Should fail due to read-write conflict
}

#[test]
fn test_serializable_prevents_dirty_read_new_vertex() {
    let (graph, _cleaner) = create_test_graph();

    // Transaction 1 reads vertex with vid 3
    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    assert!(graph.get_vertex(&txn1, 3).is_err()); // Should not exist

    // Transaction 2 creates a new vertex but does not commit
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let carol = Vertex::new(
        3,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            ScalarValue::String(Some("Carol".to_string())),
            ScalarValue::Int32(Some(28)),
        ]),
    );
    graph.create_vertex(&txn2, carol).unwrap();

    assert!(graph.get_vertex(&txn1, 3).is_err());

    assert!(txn2.commit().is_ok());
    assert!(txn1.commit().is_err()); // Should fail due to read-write conflict
}

// ========== NON-REPEATABLE READ TESTS ==========

#[test]
fn test_serializable_prevents_non_repeatable_read() {
    let (graph, _cleaner) = create_test_graph();

    // Transaction 1 reads the vertex
    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let alice_v1 = graph.get_vertex(&txn1, 1).unwrap();
    assert_eq!(alice_v1.properties()[1], ScalarValue::Int32(Some(25)));

    // Transaction 2 modifies and commits
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    graph
        .set_vertex_property(&txn2, 1, vec![1], vec![ScalarValue::Int32(Some(26))])
        .unwrap();
    txn2.commit().unwrap(); // Commit the change

    // Second read should return the same value as the first read
    let alice_v2 = graph.get_vertex(&txn1, 1).unwrap();
    assert_eq!(alice_v2.properties()[1], ScalarValue::Int32(Some(25)));

    assert!(txn1.commit().is_err()); // Should fail due to read-write conflict
}

#[test]
fn test_serializable_prevents_non_repeatable_read_edge() {
    let (graph, _cleaner) = create_test_graph();

    // Transaction 1 reads the edge
    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let edge_v1 = graph.get_edge(&txn1, 1).unwrap();
    assert_eq!(
        edge_v1.properties()[0],
        ScalarValue::String(Some("2024-01-01".to_string()))
    );

    // Transaction 2 modifies the edge and commits
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    graph
        .set_edge_property(&txn2, 1, vec![0], vec![ScalarValue::String(Some(
            "2024-02-01".to_string(),
        ))])
        .unwrap();
    txn2.commit().unwrap();

    // Second read should return the same value as the first read
    let edge_v2 = graph.get_edge(&txn1, 1).unwrap();
    assert_eq!(
        edge_v2.properties()[0],
        ScalarValue::String(Some("2024-01-01".to_string()))
    );

    assert!(txn1.commit().is_err()); // Should fail due to read-write conflict
}

// ========== PHANTOM READ TESTS ==========

#[test]
fn test_serializable_prevents_phantom_read_vertices() {
    let (graph, _cleaner) = create_test_graph();

    // Transaction 1 reads vertices within a certain age range
    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let iter1 = txn1
        .iter_vertices()
        .filter_map(|v| v.ok())
        .filter(|v| match v.properties()[1] {
            ScalarValue::Int32(Some(age)) => (25..=30).contains(&age),
            _ => false,
        });
    let count1: usize = iter1.count();
    assert_eq!(count1, 2); // Alice (25) and Bob (30)

    // Transaction 2 inserts a new vertex that fits the criteria
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let carol = Vertex::new(
        3,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            ScalarValue::String(Some("Carol".to_string())),
            ScalarValue::Int32(Some(27)),
        ]),
    );
    graph.create_vertex(&txn2, carol).unwrap();
    txn2.commit().unwrap();

    // Second query, should return the same result (prevent phantom read)
    let iter2 = txn1
        .iter_vertices()
        .filter_map(|v| v.ok())
        .filter(|v| match v.properties()[1] {
            ScalarValue::Int32(Some(age)) => (25..=30).contains(&age),
            _ => false,
        });
    let count2: usize = iter2.count();
    assert_eq!(count2, 2); // Still 2 results, Carol is not visible

    txn1.abort().unwrap();
}

#[test]
fn test_serializable_prevents_phantom_read_edges() {
    let (graph, _cleaner) = create_test_graph();

    // Transaction 1 reads edges of a specific type (e.g., FRIEND)
    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let iter1 = txn1
        .iter_edges()
        .filter_map(|e| e.ok())
        .filter(|e| e.label_id() == FRIEND_LABEL_ID);
    let count1: usize = iter1.count();
    assert_eq!(count1, 1);

    // Transaction 2 inserts a new FRIEND edge
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let new_friend_edge = Edge::new(
        2,
        2,
        1,
        FRIEND_LABEL_ID,
        PropertyRecord::new(vec![ScalarValue::String(Some("2024-03-01".to_string()))]),
    );
    graph.create_edge(&txn2, new_friend_edge).unwrap();
    txn2.commit().unwrap();

    // Should return the same result (prevent phantom read)
    let iter2 = txn1
        .iter_edges()
        .filter_map(|e| e.ok())
        .filter(|e| e.label_id() == FRIEND_LABEL_ID);
    let count2: usize = iter2.count();
    assert_eq!(count2, 1);

    txn1.abort().unwrap();
}

// ========== WRITE-WRITE CONFLICT TESTS ==========

#[test]
fn test_serializable_write_write_conflict_vertex() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Transaction 1 modifies the vertex
    graph
        .set_vertex_property(&txn1, 1, vec![1], vec![ScalarValue::Int32(Some(26))])
        .unwrap();

    // Transaction 2 tries to modify the same vertex, should fail
    assert!(
        graph
            .set_vertex_property(&txn2, 1, vec![1], vec![ScalarValue::Int32(Some(27))])
            .is_err()
    );

    txn1.commit().unwrap();
    txn2.abort().unwrap();
}

#[test]
fn test_serializable_write_write_conflict_edge() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Transaction 1 modifies the edge
    graph
        .set_edge_property(&txn1, 1, vec![0], vec![ScalarValue::String(Some(
            "2024-02-01".to_string(),
        ))])
        .unwrap();

    // Transaction 2 tries to modify the same edge, should fail
    assert!(
        graph
            .set_edge_property(&txn2, 1, vec![0], vec![ScalarValue::String(Some(
                "2024-03-01".to_string(),
            ))])
            .is_err()
    );

    txn1.commit().unwrap();
    txn2.abort().unwrap();
}

// ========== DELETE OPERATION TESTS ==========

#[test]
fn test_serializable_delete_vertex_conflict() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Transaction 1 modifies the vertex
    graph
        .set_vertex_property(&txn1, 1, vec![1], vec![ScalarValue::Int32(Some(26))])
        .unwrap();

    // Transaction 2 tries to delete the same vertex, should fail
    assert!(graph.delete_vertex(&txn2, 1).is_err());

    txn1.commit().unwrap();
    txn2.abort().unwrap();
}

#[test]
fn test_serializable_delete_edge_conflict() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Transaction 1 modifies the edge
    graph
        .set_edge_property(&txn1, 1, vec![0], vec![ScalarValue::String(Some(
            "2024-02-01".to_string(),
        ))])
        .unwrap();

    // Transaction 2 tries to delete the same edge, should fail
    assert!(graph.delete_edge(&txn2, 1).is_err());

    txn1.commit().unwrap();
    txn2.abort().unwrap();
}

#[test]
fn test_serializable_read_deleted_vertex() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // First read of the vertex
    let alice = graph.get_vertex(&txn1, 1).unwrap();
    assert_eq!(
        alice.properties()[0],
        ScalarValue::String(Some("Alice".to_string()))
    );

    // Transaction 2 deletes the vertex
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    graph.delete_vertex(&txn2, 1).unwrap();
    txn2.commit().unwrap();

    // Transaction 1 should still see the vertex
    let alice_again = graph.get_vertex(&txn1, 1).unwrap();
    assert_eq!(
        alice_again.properties()[0],
        ScalarValue::String(Some("Alice".to_string()))
    );

    txn1.abort().unwrap();
}

#[test]
fn test_serializable_read_deleted_edge() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // First read of the edge
    let friend_edge = graph.get_edge(&txn1, 1).unwrap();
    assert_eq!(
        friend_edge.properties()[0],
        ScalarValue::String(Some("2024-01-01".to_string()))
    );

    // Transaction 2 deletes the edge
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    graph.delete_edge(&txn2, 1).unwrap();
    txn2.commit().unwrap();

    // Transaction 1 should still see the edge
    let friend_edge_again = graph.get_edge(&txn1, 1).unwrap();
    assert_eq!(
        friend_edge_again.properties()[0],
        ScalarValue::String(Some("2024-01-01".to_string()))
    );

    txn1.abort().unwrap();
}

// ========== ADJACENCY LIST TESTS ==========

#[test]
fn test_serializable_adjacency_consistency() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Read Alice's adjacency list
    let adj_iter1 = txn1.iter_adjacency(1);
    let count1 = adj_iter1.count();
    assert_eq!(count1, 1); // Alice has one outgoing edge to Bob

    // Transaction 2 modifies the graph
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let carol = Vertex::new(
        3,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            ScalarValue::String(Some("Carol".to_string())),
            ScalarValue::Int32(Some(28)),
        ]),
    );
    graph.create_vertex(&txn2, carol).unwrap();

    let new_edge = Edge::new(
        2,
        1,
        3,
        FOLLOW_LABEL_ID,
        PropertyRecord::new(vec![ScalarValue::String(Some("2024-04-01".to_string()))]),
    );
    graph.create_edge(&txn2, new_edge).unwrap();
    txn2.commit().unwrap();

    // Transaction 1 reads adjacency list again, should be consistent
    let adj_iter2 = txn1.iter_adjacency(1);
    let count2 = adj_iter2.count();
    assert_eq!(count2, 1); // Still 1 edge

    txn1.abort().unwrap();
}

// ========== COMPLEX SCENARIO TESTS ==========

#[test]
fn test_serializable_complex_transaction_scenario() {
    let (graph, _cleaner) = create_test_graph();

    // Simulate a complex social network scenario
    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Transaction 1: Count Alice's friends
    let friends_count_1 = txn1
        .iter_adjacency_outgoing(1)
        .filter_map(|adj| adj.ok())
        .filter(|adj| adj.label_id() == FRIEND_LABEL_ID)
        .count();
    assert_eq!(friends_count_1, 1);

    // Transaction 2: Concurrently add a new friend
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let david = Vertex::new(
        4,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            ScalarValue::String(Some("David".to_string())),
            ScalarValue::Int32(Some(32)),
        ]),
    );
    graph.create_vertex(&txn2, david).unwrap();

    let friend_edge = Edge::new(
        3,
        1,
        4,
        FRIEND_LABEL_ID,
        PropertyRecord::new(vec![ScalarValue::String(Some("2024-05-01".to_string()))]),
    );
    graph.create_edge(&txn2, friend_edge).unwrap();
    txn2.commit().unwrap();

    // Transaction 1 counts again, should be consistent
    let friends_count_2 = txn1
        .iter_adjacency_outgoing(1)
        .filter_map(|adj| adj.ok())
        .filter(|adj| adj.label_id() == FRIEND_LABEL_ID)
        .count();
    assert_eq!(friends_count_2, 1); // Should still be 1

    txn1.abort().unwrap();
}

// ========== ROLLBACK TESTS ==========

#[test]
fn test_rollback_vertex_creation() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    let carol = Vertex::new(
        3,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            ScalarValue::String(Some("Carol".to_string())),
            ScalarValue::Int32(Some(28)),
        ]),
    );
    graph.create_vertex(&txn1, carol).unwrap();

    // Rollback transaction
    txn1.abort().unwrap();

    // Verify the vertex does not exist
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    assert!(graph.get_vertex(&txn2, 3).is_err());
    txn2.abort().unwrap();
}

#[test]
fn test_rollback_edge_creation() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    let follow_edge = Edge::new(
        2,
        2,
        1,
        FOLLOW_LABEL_ID,
        PropertyRecord::new(vec![ScalarValue::String(Some("2024-06-01".to_string()))]),
    );
    graph.create_edge(&txn1, follow_edge).unwrap();

    // Rollback transaction
    txn1.abort().unwrap();

    // Verify the edge does not exist
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    assert!(graph.get_edge(&txn2, 2).is_err());
    txn2.abort().unwrap();
}

#[test]
fn test_rollback_property_update() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Modify property
    graph
        .set_vertex_property(&txn1, 1, vec![1], vec![ScalarValue::Int32(Some(99))])
        .unwrap();

    // Rollback transaction
    txn1.abort().unwrap();

    // Verify the property has not changed
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let alice = graph.get_vertex(&txn2, 1).unwrap();
    assert_eq!(alice.properties()[1], ScalarValue::Int32(Some(25))); // Original value
    txn2.abort().unwrap();
}

// ========== PERFORMANCE AND STRESS TESTS ==========

#[test]
fn test_concurrent_transactions_stress() {
    let (graph, _cleaner) = create_test_graph();

    let graph_clone = graph.clone();

    // Create multiple concurrent transactions
    let handle1 = thread::spawn(move || {
        for i in 0..10 {
            let txn = graph_clone
                .txn_manager()
                .begin_transaction(IsolationLevel::Serializable)
                .unwrap();
            let vertex = Vertex::new(
                100 + i,
                PERSON_LABEL_ID,
                PropertyRecord::new(vec![
                    ScalarValue::String(Some(format!("User{}", i))),
                    ScalarValue::Int32(Some(20 + i as i32)),
                ]),
            );
            if graph_clone.create_vertex(&txn, vertex).is_ok() {
                let _ = txn.commit();
            } else {
                let _ = txn.abort();
            }
        }
    });

    let graph_clone2 = graph.clone();
    let handle2 = thread::spawn(move || {
        for i in 0..10 {
            let txn = graph_clone2
                .txn_manager()
                .begin_transaction(IsolationLevel::Serializable)
                .unwrap();
            if graph_clone2
                .set_vertex_property(&txn, 1, vec![1], vec![ScalarValue::Int32(Some(30 + i))])
                .is_ok()
            {
                let _ = txn.commit();
            } else {
                let _ = txn.abort();
            }
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // Verify the graph is still consistent
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let alice = graph.get_vertex(&txn, 1).unwrap();
    assert!(alice.properties()[1].try_as_int32().unwrap().unwrap() >= 25);
    txn.abort().unwrap();
}

// ========== READ-ONLY TRANSACTION TESTS ==========
#[test]
fn test_read_only_transaction_consistency_under_concurrent_writes() {
    let (graph, _cleaner) = create_test_graph();

    // Start a read-only transaction to establish a consistent snapshot
    let read_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Read initial state
    let initial_alice = graph.get_vertex(&read_txn, 1).unwrap();
    let initial_bob = graph.get_vertex(&read_txn, 2).unwrap();
    let initial_edge = graph.get_edge(&read_txn, 1).unwrap();

    assert_eq!(initial_alice.properties()[1], ScalarValue::Int32(Some(25)));
    assert_eq!(initial_bob.properties()[1], ScalarValue::Int32(Some(30)));
    assert_eq!(
        initial_edge.properties()[0],
        ScalarValue::String(Some("2024-01-01".to_string()))
    );

    let graph_clone1 = graph.clone();
    let graph_clone2 = graph.clone();
    let graph_clone3 = graph.clone();

    // Concurrent writer 1: Update Alice's age multiple times
    let handle1 = thread::spawn(move || {
        for i in 0..5 {
            let write_txn = graph_clone1
                .txn_manager()
                .begin_transaction(IsolationLevel::Serializable)
                .unwrap();
            if graph_clone1
                .set_vertex_property(&write_txn, 1, vec![1], vec![ScalarValue::Int32(Some(
                    26 + i,
                ))])
                .is_ok()
            {
                let _ = write_txn.commit();
            } else {
                let _ = write_txn.abort();
            }
        }
    });

    // Concurrent writer 2: Update Bob's age multiple times
    let handle2 = thread::spawn(move || {
        for i in 0..5 {
            let write_txn = graph_clone2
                .txn_manager()
                .begin_transaction(IsolationLevel::Serializable)
                .unwrap();
            if graph_clone2
                .set_vertex_property(&write_txn, 2, vec![1], vec![ScalarValue::Int32(Some(
                    31 + i,
                ))])
                .is_ok()
            {
                let _ = write_txn.commit();
            } else {
                let _ = write_txn.abort();
            }
        }
    });

    // Concurrent writer 3: Update edge properties and create new vertices
    let handle3 = thread::spawn(move || {
        for i in 0..3 {
            let write_txn = graph_clone3
                .txn_manager()
                .begin_transaction(IsolationLevel::Serializable)
                .unwrap();

            // Update edge property
            if graph_clone3
                .set_edge_property(&write_txn, 1, vec![0], vec![ScalarValue::String(Some(
                    format!("2024-0{}-01", i + 2),
                ))])
                .is_ok()
            {
                // Create new vertex
                let new_vertex = Vertex::new(
                    10 + i as u64,
                    PERSON_LABEL_ID,
                    PropertyRecord::new(vec![
                        ScalarValue::String(Some(format!("User{}", i))),
                        ScalarValue::Int32(Some(20 + i)),
                    ]),
                );
                if graph_clone3.create_vertex(&write_txn, new_vertex).is_ok() {
                    let _ = write_txn.commit();
                } else {
                    let _ = write_txn.abort();
                }
            } else {
                let _ = write_txn.abort();
            }
        }
    });

    // Read-only transaction should see consistent snapshot throughout
    let mid_alice = graph.get_vertex(&read_txn, 1).unwrap();
    let mid_bob = graph.get_vertex(&read_txn, 2).unwrap();
    let mid_edge = graph.get_edge(&read_txn, 1).unwrap();

    // Values should be identical to initial reads (consistent snapshot)
    assert_eq!(mid_alice.properties()[1], ScalarValue::Int32(Some(25)));
    assert_eq!(mid_bob.properties()[1], ScalarValue::Int32(Some(30)));
    assert_eq!(
        mid_edge.properties()[0],
        ScalarValue::String(Some("2024-01-01".to_string()))
    );

    // Wait for all writers to complete
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();

    // Final reads should still be consistent with initial snapshot
    let final_alice = graph.get_vertex(&read_txn, 1).unwrap();
    let final_bob = graph.get_vertex(&read_txn, 2).unwrap();
    let final_edge = graph.get_edge(&read_txn, 1).unwrap();

    assert_eq!(final_alice.properties()[1], ScalarValue::Int32(Some(25)));
    assert_eq!(final_bob.properties()[1], ScalarValue::Int32(Some(30)));
    assert_eq!(
        final_edge.properties()[0],
        ScalarValue::String(Some("2024-01-01".to_string()))
    );

    // New vertices created by writers should not be visible
    assert!(graph.get_vertex(&read_txn, 10).is_err());
    assert!(graph.get_vertex(&read_txn, 11).is_err());
    assert!(graph.get_vertex(&read_txn, 12).is_err());

    // Vertex count should remain consistent
    let initial_count = read_txn.iter_vertices().filter_map(|v| v.ok()).count();
    let final_count = read_txn.iter_vertices().filter_map(|v| v.ok()).count();
    assert_eq!(initial_count, final_count);

    read_txn.abort().unwrap();

    // Verify that changes are visible in a new transaction
    let verify_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let updated_alice = graph.get_vertex(&verify_txn, 1).unwrap();
    let updated_bob = graph.get_vertex(&verify_txn, 2).unwrap();

    // Should see the updated values now
    assert!(
        updated_alice.properties()[1]
            .try_as_int32()
            .unwrap()
            .unwrap()
            > 25
    );
    assert!(updated_bob.properties()[1].try_as_int32().unwrap().unwrap() > 30);

    verify_txn.abort().unwrap();
}

// ========== TRANSACTION INTERRUPTION AND RECOVERY TESTS ==========

#[test]
fn test_transaction_panic_during_vertex_creation() {
    let (graph, _cleaner) = create_test_graph();

    // Record initial state
    let initial_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let initial_vertex_count = initial_txn.iter_vertices().filter_map(|v| v.ok()).count();
    initial_txn.abort().unwrap();

    // Create a transaction that will panic
    let graph_clone = graph.clone();
    let panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let txn = graph_clone
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        // Create a vertex
        let vertex = Vertex::new(
            100,
            PERSON_LABEL_ID,
            PropertyRecord::new(vec![
                ScalarValue::String(Some("PanicVertex".to_string())),
                ScalarValue::Int32(Some(99)),
            ]),
        );
        graph_clone.create_vertex(&txn, vertex).unwrap();

        // Simulate panic before commit
        panic!("Simulated panic during transaction");
    }));

    // Assert that panic happened
    assert!(panic_result.is_err());

    // Verify graph consistency - should not create new vertex
    let verify_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let final_vertex_count = verify_txn.iter_vertices().filter_map(|v| v.ok()).count();
    assert_eq!(initial_vertex_count, final_vertex_count);

    // Assert that panic vertex does not exist
    assert!(graph.get_vertex(&verify_txn, 100).is_err());
    verify_txn.abort().unwrap();
}

#[test]
fn test_transaction_panic_during_property_update() {
    let (graph, _cleaner) = create_test_graph();

    // Record initial age of Alice
    let initial_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let initial_alice = graph.get_vertex(&initial_txn, 1).unwrap();
    let initial_age = initial_alice.properties()[1].clone();
    initial_txn.abort().unwrap();

    // Create a transaction that will panic
    let graph_clone = graph.clone();
    let panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let txn = graph_clone
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        // Update Alice's age
        graph_clone
            .set_vertex_property(&txn, 1, vec![1], vec![ScalarValue::Int32(Some(999))])
            .unwrap();

        // Simulate panic before commit
        panic!("Simulated panic during property update");
    }));

    // Assert that panic happened
    assert!(panic_result.is_err());

    // Verify Alice's age did not change
    let verify_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let final_alice = graph.get_vertex(&verify_txn, 1).unwrap();
    assert_eq!(final_alice.properties()[1], initial_age);
    verify_txn.abort().unwrap();
}

#[test]
fn test_transaction_panic_during_deletion() {
    let (graph, _cleaner) = create_test_graph();

    // Assert that Bob exists
    let initial_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    assert!(graph.get_vertex(&initial_txn, 2).is_ok());
    initial_txn.abort().unwrap();

    // Create a transaction that will panic
    let graph_clone = graph.clone();
    let panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let txn = graph_clone
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        // Delete Bob
        graph_clone.delete_vertex(&txn, 2).unwrap();

        // Simulate panic before commit
        panic!("Simulated panic during deletion");
    }));

    // Assert that panic happened
    assert!(panic_result.is_err());

    // Verify that Bob still exists
    let verify_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    assert!(graph.get_vertex(&verify_txn, 2).is_ok());
    verify_txn.abort().unwrap();
}

// ========== ADVANCED WRITE-WRITE CONFLICT TESTS ==========

#[test]
fn test_serializable_concurrent_create_same_vertex() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    let carol1 = Vertex::new(
        3,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            ScalarValue::String(Some("Carol1".to_string())),
            ScalarValue::Int32(Some(28)),
        ]),
    );
    let carol2 = Vertex::new(
        3,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            ScalarValue::String(Some("Carol2".to_string())),
            ScalarValue::Int32(Some(29)),
        ]),
    );

    // Both transactions try to create vertex with same ID
    assert!(graph.create_vertex(&txn1, carol1).is_ok());
    assert!(graph.create_vertex(&txn2, carol2).is_err());

    txn1.commit().unwrap();
    txn2.abort().unwrap();
}

#[test]
fn test_serializable_concurrent_create_same_edge() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    let edge1 = Edge::new(
        2,
        1,
        2,
        FOLLOW_LABEL_ID,
        PropertyRecord::new(vec![ScalarValue::String(Some("2024-01-01".to_string()))]),
    );
    let edge2 = Edge::new(
        2,
        1,
        2,
        FOLLOW_LABEL_ID,
        PropertyRecord::new(vec![ScalarValue::String(Some("2024-02-01".to_string()))]),
    );

    assert!(graph.create_edge(&txn1, edge1).is_ok());
    assert!(graph.create_edge(&txn2, edge2).is_err());

    txn1.commit().unwrap();
    txn2.abort().unwrap();
}

#[test]
fn test_serializable_update_after_delete_conflict() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Transaction 1 deletes vertex
    graph.delete_vertex(&txn1, 1).unwrap();

    // Transaction 2 tries to update the same vertex
    assert!(
        graph
            .set_vertex_property(&txn2, 1, vec![1], vec![ScalarValue::Int32(Some(100))])
            .is_err()
    );

    txn1.commit().unwrap();
    txn2.abort().unwrap();
}

#[test]
fn test_serializable_delete_after_update_conflict() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Transaction 1 updates vertex
    graph
        .set_vertex_property(&txn1, 1, vec![1], vec![ScalarValue::Int32(Some(100))])
        .unwrap();

    // Transaction 2 tries to delete the same vertex
    assert!(graph.delete_vertex(&txn2, 1).is_err());

    txn1.commit().unwrap();
    txn2.abort().unwrap();
}

// ========== MULTIPLE PROPERTY UPDATE TESTS ==========

#[test]
fn test_serializable_multiple_property_updates() {
    let (graph, _cleaner) = create_test_graph();

    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Update multiple properties in sequence
    graph
        .set_vertex_property(&txn, 1, vec![1], vec![ScalarValue::Int32(Some(26))])
        .unwrap();
    graph
        .set_vertex_property(&txn, 1, vec![0], vec![ScalarValue::String(Some(
            "Alicia".to_string(),
        ))])
        .unwrap();

    let alice = graph.get_vertex(&txn, 1).unwrap();
    assert_eq!(
        alice.properties()[0],
        ScalarValue::String(Some("Alicia".to_string()))
    );
    assert_eq!(alice.properties()[1], ScalarValue::Int32(Some(26)));

    txn.commit().unwrap();
}

#[test]
fn test_serializable_batch_property_updates() {
    let (graph, _cleaner) = create_test_graph();

    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Update multiple properties at once
    graph
        .set_vertex_property(&txn, 1, vec![0, 1], vec![
            ScalarValue::String(Some("Alicia".to_string())),
            ScalarValue::Int32(Some(26)),
        ])
        .unwrap();

    let alice = graph.get_vertex(&txn, 1).unwrap();
    assert_eq!(
        alice.properties()[0],
        ScalarValue::String(Some("Alicia".to_string()))
    );
    assert_eq!(alice.properties()[1], ScalarValue::Int32(Some(26)));

    txn.commit().unwrap();
}

// ========== CROSS-VERTEX TRANSACTION TESTS ==========

#[test]
fn test_serializable_multi_vertex_consistency() {
    let (graph, _cleaner) = create_test_graph();

    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Update multiple vertices in one transaction
    graph
        .set_vertex_property(&txn, 1, vec![1], vec![ScalarValue::Int32(Some(26))])
        .unwrap();
    graph
        .set_vertex_property(&txn, 2, vec![1], vec![ScalarValue::Int32(Some(31))])
        .unwrap();

    txn.commit().unwrap();

    // Verify both updates are visible
    let verify_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let alice = graph.get_vertex(&verify_txn, 1).unwrap();
    let bob = graph.get_vertex(&verify_txn, 2).unwrap();
    assert_eq!(alice.properties()[1], ScalarValue::Int32(Some(26)));
    assert_eq!(bob.properties()[1], ScalarValue::Int32(Some(31)));
    verify_txn.abort().unwrap();
}

#[test]
fn test_serializable_multi_edge_consistency() {
    let (graph, _cleaner) = create_test_graph();

    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Create multiple edges
    let edge2 = Edge::new(
        2,
        2,
        1,
        FOLLOW_LABEL_ID,
        PropertyRecord::new(vec![ScalarValue::String(Some("2024-02-01".to_string()))]),
    );
    let edge3 = Edge::new(
        3,
        1,
        2,
        FOLLOW_LABEL_ID,
        PropertyRecord::new(vec![ScalarValue::String(Some("2024-03-01".to_string()))]),
    );

    graph.create_edge(&txn, edge2).unwrap();
    graph.create_edge(&txn, edge3).unwrap();

    txn.commit().unwrap();

    // Verify both edges exist
    let verify_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    assert!(graph.get_edge(&verify_txn, 2).is_ok());
    assert!(graph.get_edge(&verify_txn, 3).is_ok());
    verify_txn.abort().unwrap();
}

// ========== EDGE DIRECTION TESTS ==========

#[test]
fn test_serializable_bidirectional_edge_consistency() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Check outgoing edges from Alice
    let outgoing_count = txn1
        .iter_adjacency_outgoing(1)
        .filter_map(|adj| adj.ok())
        .count();
    assert_eq!(outgoing_count, 1);

    // Check incoming edges to Bob
    let incoming_count = txn1
        .iter_adjacency_incoming(2)
        .filter_map(|adj| adj.ok())
        .count();
    assert_eq!(incoming_count, 1);

    // Transaction 2 adds new edge
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let new_edge = Edge::new(
        2,
        2,
        1,
        FRIEND_LABEL_ID,
        PropertyRecord::new(vec![ScalarValue::String(Some("2024-04-01".to_string()))]),
    );
    graph.create_edge(&txn2, new_edge).unwrap();
    txn2.commit().unwrap();

    // Transaction 1 should still see consistent view
    let outgoing_count2 = txn1
        .iter_adjacency_outgoing(1)
        .filter_map(|adj| adj.ok())
        .count();
    assert_eq!(outgoing_count2, 1);

    txn1.abort().unwrap();
}

// ========== VERTEX-EDGE RELATIONSHIP TESTS ==========

#[test]
fn test_serializable_delete_vertex_cascades_edges() {
    let (graph, _cleaner) = create_test_graph();

    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Delete Alice
    graph.delete_vertex(&txn, 1).unwrap();
    txn.commit().unwrap();

    // Verify edge is also deleted
    let verify_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    assert!(graph.get_edge(&verify_txn, 1).is_err());
    verify_txn.abort().unwrap();
}

#[test]
fn test_serializable_orphaned_edge_prevention() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Delete Bob (target vertex)
    graph.delete_vertex(&txn1, 2).unwrap();
    txn1.commit().unwrap();

    // Verify edge from Alice to Bob is deleted
    let verify_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    assert!(graph.get_edge(&verify_txn, 1).is_err());
    verify_txn.abort().unwrap();
}

// ========== EMPTY TRANSACTION TESTS ==========

#[test]
fn test_empty_transaction_commit() {
    let (graph, _cleaner) = create_test_graph();

    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    assert!(txn.commit().is_ok());
}

#[test]
fn test_empty_transaction_abort() {
    let (graph, _cleaner) = create_test_graph();

    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    assert!(txn.abort().is_ok());
}

// ========== ITERATOR CONSISTENCY TESTS ==========

#[test]
fn test_vertex_iterator_snapshot_isolation() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Collect initial vertices
    let initial_vertices: Vec<_> = txn1
        .iter_vertices()
        .filter_map(|v| v.ok())
        .map(|v| v.vid())
        .collect();

    // Another transaction adds vertices
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    for i in 10..15 {
        let vertex = Vertex::new(
            i,
            PERSON_LABEL_ID,
            PropertyRecord::new(vec![
                ScalarValue::String(Some(format!("User{}", i))),
                ScalarValue::Int32(Some(20 + i as i32)),
            ]),
        );
        graph.create_vertex(&txn2, vertex).unwrap();
    }
    txn2.commit().unwrap();

    // Original transaction should see same vertices
    let final_vertices: Vec<_> = txn1
        .iter_vertices()
        .filter_map(|v| v.ok())
        .map(|v| v.vid())
        .collect();

    assert_eq!(initial_vertices.len(), final_vertices.len());
    txn1.abort().unwrap();
}

#[test]
fn test_edge_iterator_snapshot_isolation() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    let initial_edges: Vec<_> = txn1
        .iter_edges()
        .filter_map(|e| e.ok())
        .map(|e| e.eid())
        .collect();

    // Another transaction adds edges
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    for i in 10..15 {
        let edge = Edge::new(
            i,
            1,
            2,
            FOLLOW_LABEL_ID,
            PropertyRecord::new(vec![ScalarValue::String(Some(format!(
                "2024-{:02}-01",
                i - 8
            )))]),
        );
        if graph.create_edge(&txn2, edge).is_ok() {
            // Some may fail due to duplicate IDs, that's ok
        }
    }
    txn2.commit().unwrap();

    let final_edges: Vec<_> = txn1
        .iter_edges()
        .filter_map(|e| e.ok())
        .map(|e| e.eid())
        .collect();

    assert_eq!(initial_edges.len(), final_edges.len());
    txn1.abort().unwrap();
}

// ========== LABEL FILTER TESTS ==========

#[test]
fn test_filter_vertices_by_label() {
    let (graph, _cleaner) = create_test_graph();

    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    let person_count = txn
        .iter_vertices()
        .filter_map(|v| v.ok())
        .filter(|v| v.label_id == PERSON_LABEL_ID)
        .count();

    assert_eq!(person_count, 2); // Alice and Bob
    txn.abort().unwrap();
}

#[test]
fn test_filter_edges_by_label() {
    let (graph, _cleaner) = create_test_graph();

    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    let friend_edges = txn
        .iter_edges()
        .filter_map(|e| e.ok())
        .filter(|e| e.label_id() == FRIEND_LABEL_ID)
        .count();

    assert_eq!(friend_edges, 1);
    txn.abort().unwrap();
}

// ========== LARGE TRANSACTION TESTS ==========

#[test]
fn test_large_batch_vertex_creation() {
    let (graph, _cleaner) = create_test_graph();

    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Create 100 vertices
    for i in 100..200 {
        let vertex = Vertex::new(
            i,
            PERSON_LABEL_ID,
            PropertyRecord::new(vec![
                ScalarValue::String(Some(format!("User{}", i))),
                ScalarValue::Int32(Some((20 + (i % 50)) as i32)),
            ]),
        );
        graph.create_vertex(&txn, vertex).unwrap();
    }

    txn.commit().unwrap();

    // Verify all created
    let verify_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    for i in 100..200 {
        assert!(graph.get_vertex(&verify_txn, i).is_ok());
    }
    verify_txn.abort().unwrap();
}

#[test]
fn test_large_batch_edge_creation() {
    let (graph, _cleaner) = create_test_graph();

    // First create vertices
    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    for i in 100..110 {
        let vertex = Vertex::new(
            i,
            PERSON_LABEL_ID,
            PropertyRecord::new(vec![
                ScalarValue::String(Some(format!("User{}", i))),
                ScalarValue::Int32(Some(25)),
            ]),
        );
        graph.create_vertex(&txn1, vertex).unwrap();
    }
    txn1.commit().unwrap();

    // Then create edges
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    for i in 0..45 {
        let edge = Edge::new(
            100 + i,
            100 + (i % 10),
            100 + ((i + 1) % 10),
            FOLLOW_LABEL_ID,
            PropertyRecord::new(vec![ScalarValue::String(Some("2024-01-01".to_string()))]),
        );
        graph.create_edge(&txn2, edge).unwrap();
    }
    txn2.commit().unwrap();

    // Verify
    let verify_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let edge_count = verify_txn
        .iter_edges()
        .filter_map(|e| e.ok())
        .filter(|e| e.eid() >= 100)
        .count();
    assert_eq!(edge_count, 45);
    verify_txn.abort().unwrap();
}
