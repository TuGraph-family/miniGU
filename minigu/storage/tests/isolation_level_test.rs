mod common;
use std::thread;
use std::time::Duration;

use common::*;
use minigu_common::datatype::value::PropertyValue;
use minigu_storage::model::edge::Edge;
use minigu_storage::model::properties::PropertyRecord;
use minigu_storage::model::vertex::Vertex;
use minigu_storage::{Graph, IsolationLevel, MutGraph, StorageTransaction};

// ========== DIRTY READ TESTS ==========

#[test]
fn test_serializable_prevents_dirty_read_vertex() {
    let (graph, _cleaner) = create_test_graph();

    // Transaction 1 modifies the vertex but does not commit
    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
    graph
        .set_vertex_property(&txn1, 1, vec![1], vec![PropertyValue::Int(26)])
        .unwrap();

    // Transaction 2 tries to read uncommitted changes
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
    // Should not be able to read uncommitted data under Serializable isolation level
    assert!(graph.get_vertex(&txn2, 1).is_err());

    txn1.commit().unwrap();
    txn2.abort().unwrap();
}

#[test]
fn test_serializable_prevents_dirty_read_edge() {
    let (graph, _cleaner) = create_test_graph();

    // Transaction 1 modifies the edge but does not commit
    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
    graph
        .set_edge_property(&txn1, 1, vec![0], vec![PropertyValue::String(
            "2024-02-01".into(),
        )])
        .unwrap();

    // Transaction 2 tries to read uncommitted changes
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
    // Should not be able to read uncommitted edge modification
    assert!(graph.get_edge(&txn2, 1).is_err());

    txn1.commit().unwrap();
    txn2.abort().unwrap();
}

#[test]
fn test_serializable_prevents_dirty_read_new_vertex() {
    let (graph, _cleaner) = create_test_graph();

    // Transaction 1 creates a new vertex but does not commit
    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
    let carol = Vertex::new(
        3,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            PropertyValue::String("Carol".into()),
            PropertyValue::Int(28),
        ]),
    );
    graph.create_vertex(&txn1, carol).unwrap();

    // Transaction 2 tries to read uncommitted new vertex
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
    assert!(graph.get_vertex(&txn2, 3).is_err());

    txn1.abort().unwrap();
    txn2.abort().unwrap();
}

// ========== NON-REPEATABLE READ TESTS ==========

#[test]
fn test_serializable_prevents_non_repeatable_read() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);

    // First read
    let alice_v1 = graph.get_vertex(&txn1, 1).unwrap();
    assert_eq!(alice_v1.properties()[1], PropertyValue::Int(25));

    // Another transaction modifies and commits
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
    graph
        .set_vertex_property(&txn2, 1, vec![1], vec![PropertyValue::Int(26)])
        .unwrap();
    txn2.commit().unwrap();

    // Second read, should return the same value (snapshot isolation)
    let alice_v2 = graph.get_vertex(&txn1, 1).unwrap();
    assert_eq!(alice_v2.properties()[1], PropertyValue::Int(25));

    txn1.abort().unwrap();
}

#[test]
fn test_serializable_prevents_non_repeatable_read_edge() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);

    // First read边
    let edge_v1 = graph.get_edge(&txn1, 1).unwrap();
    assert_eq!(
        edge_v1.properties()[0],
        PropertyValue::String("2024-01-01".into())
    );

    // Another transaction modifies the edge and commits
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
    graph
        .set_edge_property(&txn2, 1, vec![0], vec![PropertyValue::String(
            "2024-02-01".into(),
        )])
        .unwrap();
    txn2.commit().unwrap();

    // Second read, should return the same value
    let edge_v2 = graph.get_edge(&txn1, 1).unwrap();
    assert_eq!(
        edge_v2.properties()[0],
        PropertyValue::String("2024-01-01".into())
    );

    txn1.abort().unwrap();
}

// ========== PHANTOM READ TESTS ==========

#[test]
fn test_serializable_prevents_phantom_read_vertices() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);

    // First query: Find people aged 25-30
    let iter1 = txn1.iter_vertices().filter_map(|v| v.ok()).filter(|v| {
        let age = v.properties()[1].as_int().unwrap();
        *age >= 25 && *age <= 30
    });
    let count1: usize = iter1.count();
    assert_eq!(count1, 2); // Alice (25) 和 Bob (30)

    // Another transaction inserts a new vertex
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
    let carol = Vertex::new(
        3,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            PropertyValue::String("Carol".into()),
            PropertyValue::Int(27),
        ]),
    );
    graph.create_vertex(&txn2, carol).unwrap();
    txn2.commit().unwrap();

    // Second query, should return the same result (prevent phantom read)
    let iter2 = txn1.iter_vertices().filter_map(|v| v.ok()).filter(|v| {
        let age = v.properties()[1].as_int().unwrap();
        *age >= 25 && *age <= 30
    });
    let count2: usize = iter2.count();
    assert_eq!(count2, 2); // Still 2 results, Carol is not visible

    txn1.abort().unwrap();
}

#[test]
fn test_serializable_prevents_phantom_read_edges() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);

    // First query: Find all FRIEND type edges
    let iter1 = txn1
        .iter_edges()
        .filter_map(|e| e.ok())
        .filter(|e| e.label_id() == FRIEND_LABEL_ID);
    let count1: usize = iter1.count();
    assert_eq!(count1, 1);

    // Another transaction inserts a new FRIEND edge
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
    let new_friend_edge = Edge::new(
        2,
        2,
        1,
        FRIEND_LABEL_ID,
        PropertyRecord::new(vec![PropertyValue::String("2024-03-01".into())]),
    );
    graph.create_edge(&txn2, new_friend_edge).unwrap();
    txn2.commit().unwrap();

    // Should return the same result (prevent phantom read)
    let iter2 = txn1
        .iter_edges()
        .filter_map(|e| e.ok())
        .filter(|e| e.label_id() == FRIEND_LABEL_ID);
    let count2: usize = iter2.count();
    assert_eq!(count2, 1); // Still 1 result

    txn1.abort().unwrap();
}

// ========== WRITE-WRITE CONFLICT TESTS ==========

#[test]
fn test_serializable_write_write_conflict_vertex() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);

    // Transaction 1 modifies the vertex
    graph
        .set_vertex_property(&txn1, 1, vec![1], vec![PropertyValue::Int(26)])
        .unwrap();

    // Transaction 2 tries to modify the same vertex, should fail
    assert!(
        graph
            .set_vertex_property(&txn2, 1, vec![1], vec![PropertyValue::Int(27)])
            .is_err()
    );

    txn1.commit().unwrap();
    txn2.abort().unwrap();
}

#[test]
fn test_serializable_write_write_conflict_edge() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);

    // Transaction 1 modifies the edge
    graph
        .set_edge_property(&txn1, 1, vec![0], vec![PropertyValue::String(
            "2024-02-01".into(),
        )])
        .unwrap();

    // Transaction 2 tries to modify the same edge, should fail
    assert!(
        graph
            .set_edge_property(&txn2, 1, vec![0], vec![PropertyValue::String(
                "2024-03-01".into()
            )])
            .is_err()
    );

    txn1.commit().unwrap();
    txn2.abort().unwrap();
}

// ========== DELETE OPERATION TESTS ==========

#[test]
fn test_serializable_delete_vertex_conflict() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);

    // Transaction 1 modifies the vertex
    graph
        .set_vertex_property(&txn1, 1, vec![1], vec![PropertyValue::Int(26)])
        .unwrap();

    // Transaction 2 tries to delete the same vertex, should fail
    assert!(graph.delete_vertex(&txn2, 1).is_err());

    txn1.commit().unwrap();
    txn2.abort().unwrap();
}

#[test]
fn test_serializable_delete_edge_conflict() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);

    // Transaction 1 modifies the edge
    graph
        .set_edge_property(&txn1, 1, vec![0], vec![PropertyValue::String(
            "2024-02-01".into(),
        )])
        .unwrap();

    // Transaction 2 tries to delete the same edge, should fail
    assert!(graph.delete_edge(&txn2, 1).is_err());

    txn1.commit().unwrap();
    txn2.abort().unwrap();
}

#[test]
fn test_serializable_read_deleted_vertex() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);

    // First read of the vertex
    let alice = graph.get_vertex(&txn1, 1).unwrap();
    assert_eq!(alice.properties()[0], PropertyValue::String("Alice".into()));

    // Transaction 2 deletes the vertex
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
    graph.delete_vertex(&txn2, 1).unwrap();
    txn2.commit().unwrap();

    // Transaction 1 should still see the vertex (snapshot isolation)
    let alice_again = graph.get_vertex(&txn1, 1).unwrap();
    assert_eq!(
        alice_again.properties()[0],
        PropertyValue::String("Alice".into())
    );

    txn1.abort().unwrap();
}

#[test]
fn test_serializable_read_deleted_edge() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);

    // First read of the edge
    let friend_edge = graph.get_edge(&txn1, 1).unwrap();
    assert_eq!(
        friend_edge.properties()[0],
        PropertyValue::String("2024-01-01".into())
    );

    // Transaction 2 deletes the edge
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
    graph.delete_edge(&txn2, 1).unwrap();
    txn2.commit().unwrap();

    // Transaction 1 should still see the edge (snapshot isolation)
    let friend_edge_again = graph.get_edge(&txn1, 1).unwrap();
    assert_eq!(
        friend_edge_again.properties()[0],
        PropertyValue::String("2024-01-01".into())
    );

    txn1.abort().unwrap();
}

// ========== ADJACENCY LIST TESTS ==========

#[test]
fn test_serializable_adjacency_consistency() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);

    // Read Alice's adjacency list
    let adj_iter1 = txn1.iter_adjacency(1);
    let count1 = adj_iter1.count();
    assert_eq!(count1, 1); // Alice has one outgoing edge to Bob

    // Transaction 2 modifies the graph
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
    let carol = Vertex::new(
        3,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            PropertyValue::String("Carol".into()),
            PropertyValue::Int(28),
        ]),
    );
    graph.create_vertex(&txn2, carol).unwrap();

    let new_edge = Edge::new(
        2,
        1,
        3,
        FOLLOW_LABEL_ID,
        PropertyRecord::new(vec![PropertyValue::String("2024-04-01".into())]),
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
    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);

    // Transaction 1: Count Alice's friends
    let friends_count_1 = txn1
        .iter_adjacency_outgoing(1)
        .filter_map(|adj| adj.ok())
        .filter(|adj| adj.label_id() == FRIEND_LABEL_ID)
        .count();
    assert_eq!(friends_count_1, 1);

    // Transaction 2: Concurrently add a new friend
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
    let david = Vertex::new(
        4,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            PropertyValue::String("David".into()),
            PropertyValue::Int(32),
        ]),
    );
    graph.create_vertex(&txn2, david).unwrap();

    let friend_edge = Edge::new(
        3,
        1,
        4,
        FRIEND_LABEL_ID,
        PropertyRecord::new(vec![PropertyValue::String("2024-05-01".into())]),
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

    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);

    let carol = Vertex::new(
        3,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            PropertyValue::String("Carol".into()),
            PropertyValue::Int(28),
        ]),
    );
    graph.create_vertex(&txn1, carol).unwrap();

    // Rollback transaction
    txn1.abort().unwrap();

    // Verify the vertex does not exist
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
    assert!(graph.get_vertex(&txn2, 3).is_err());
    txn2.abort().unwrap();
}

#[test]
fn test_rollback_edge_creation() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);

    let follow_edge = Edge::new(
        2,
        2,
        1,
        FOLLOW_LABEL_ID,
        PropertyRecord::new(vec![PropertyValue::String("2024-06-01".into())]),
    );
    graph.create_edge(&txn1, follow_edge).unwrap();

    // Rollback transaction
    txn1.abort().unwrap();

    // Verify the edge does not exist
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
    assert!(graph.get_edge(&txn2, 2).is_err());
    txn2.abort().unwrap();
}

#[test]
fn test_rollback_property_update() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);

    // Modify property
    graph
        .set_vertex_property(&txn1, 1, vec![1], vec![PropertyValue::Int(99)])
        .unwrap();

    // Rollback transaction
    txn1.abort().unwrap();

    // Verify the property has not changed
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
    let alice = graph.get_vertex(&txn2, 1).unwrap();
    assert_eq!(alice.properties()[1], PropertyValue::Int(25)); // Original value
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
            let txn = graph_clone.begin_transaction(IsolationLevel::Serializable);
            let vertex = Vertex::new(
                100 + i,
                PERSON_LABEL_ID,
                PropertyRecord::new(vec![
                    PropertyValue::String(format!("User{}", i)),
                    PropertyValue::Int(20 + i as i32),
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
            let txn = graph_clone2.begin_transaction(IsolationLevel::Serializable);
            if graph_clone2
                .set_vertex_property(&txn, 1, vec![1], vec![PropertyValue::Int(30 + i)])
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
    let txn = graph.begin_transaction(IsolationLevel::Serializable);
    let alice = graph.get_vertex(&txn, 1).unwrap();
    assert!(alice.properties()[1].as_int().unwrap() >= &25);
    txn.abort().unwrap();
}

#[test]
fn test_long_running_transaction() {
    let (graph, _cleaner) = create_test_graph();

    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);

    // Read initial state
    let alice_initial = graph.get_vertex(&txn1, 1).unwrap();
    assert_eq!(alice_initial.properties()[1], PropertyValue::Int(25));

    // Simulate long-running transaction
    thread::sleep(Duration::from_millis(100));

    // Other transaction modifies data
    let txn2 = graph.begin_transaction(IsolationLevel::Serializable);
    graph
        .set_vertex_property(&txn2, 1, vec![1], vec![PropertyValue::Int(35)])
        .unwrap();
    txn2.commit().unwrap();

    // Long transaction still sees a consistent snapshot
    let alice_later = graph.get_vertex(&txn1, 1).unwrap();
    assert_eq!(alice_later.properties()[1], PropertyValue::Int(25));

    txn1.abort().unwrap();
}
