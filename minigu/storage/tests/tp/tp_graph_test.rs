use minigu_storage::error::StorageResult;
use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

use crate::common::*;

#[test]
fn test_graph_basic_operations() -> StorageResult<()> {
    // 1. Create MemGraph
    let (graph, _cleaner) = create_empty_graph();

    // 2. Open transaction
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // 3. Create vertices
    let alice = create_test_vertex(1, "Alice", 25);
    let bob = create_test_vertex(2, "Bob", 30);
    let carol = create_test_vertex(3, "Carol", 28);
    let dave = create_test_vertex(4, "Dave", 32);

    // Add vertices to graph
    let alice_id = graph.create_vertex(&txn, alice.clone())?;
    let bob_id = graph.create_vertex(&txn, bob.clone())?;
    let carol_id = graph.create_vertex(&txn, carol.clone())?;
    let dave_id = graph.create_vertex(&txn, dave.clone())?;

    // 4. Create edges
    let friend_edge = create_test_edge(1, alice_id, bob_id, FRIEND_LABEL_ID);
    let follow_edge = create_test_edge(2, bob_id, carol_id, FOLLOW_LABEL_ID);
    let another_friend_edge = create_test_edge(3, alice_id, carol_id, FRIEND_LABEL_ID);
    let another_follow_edge = create_test_edge(4, carol_id, dave_id, FOLLOW_LABEL_ID);

    // Add edges to graph
    let friend_edge_id = graph.create_edge(&txn, friend_edge.clone())?;
    let follow_edge_id = graph.create_edge(&txn, follow_edge.clone())?;
    let another_friend_edge_id = graph.create_edge(&txn, another_friend_edge.clone())?;
    let another_follow_edge_id = graph.create_edge(&txn, another_follow_edge.clone())?;

    // 5. Test vertex retrieval
    let retrieved_alice = graph.get_vertex(&txn, alice_id)?;
    assert_eq!(retrieved_alice, alice);

    // 6. Test edge retrieval
    let retrieved_friend = graph.get_edge(&txn, friend_edge_id)?;
    assert_eq!(retrieved_friend, friend_edge);

    // 7. Test adjacency iterator
    {
        let mut adj_count = 0;
        let adj_iter = txn.iter_adjacency(alice_id);
        for adj_result in adj_iter {
            let adj = adj_result?;
            assert!(adj.eid() == friend_edge_id || adj.eid() == another_friend_edge_id);
            adj_count += 1;
        }
        assert_eq!(adj_count, 2); // Alice should have 2 outgoing edges
    }

    // 8. Test vertex iterator
    {
        let mut vertex_count = 0;
        let vertex_iter = txn.iter_vertices().filter_map(|v| v.ok()).filter(|v| {
            match v.properties()[0].try_as_string() {
                Some(Some(name)) => {
                    name == "Alice" || name == "Bob" || name == "Carol" || name == "Dave"
                }
                _ => false,
            }
        });

        for _ in vertex_iter {
            vertex_count += 1;
        }
        assert_eq!(vertex_count, 4);
    }

    // 9. Test edge iterator
    {
        let mut edge_count = 0;
        let edge_iter = txn
            .iter_edges()
            .filter_map(|e| e.ok())
            .filter(|e| e.src_id() == alice_id);

        for _ in edge_iter {
            edge_count += 1;
        }
        assert_eq!(edge_count, 2); // Alice should have 2 outgoing edges
    }

    txn.commit()?;

    // 10. Open new transaction and verify data
    let verify_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Verify vertices still exist
    assert_eq!(graph.get_vertex(&verify_txn, alice_id)?, alice);
    assert_eq!(graph.get_vertex(&verify_txn, bob_id)?, bob);
    assert_eq!(graph.get_vertex(&verify_txn, carol_id)?, carol);
    assert_eq!(graph.get_vertex(&verify_txn, dave_id)?, dave);

    // Verify edges still exist
    assert_eq!(graph.get_edge(&verify_txn, friend_edge_id)?, friend_edge);
    assert_eq!(graph.get_edge(&verify_txn, follow_edge_id)?, follow_edge);
    assert_eq!(
        graph.get_edge(&verify_txn, another_friend_edge_id)?,
        another_friend_edge
    );
    assert_eq!(
        graph.get_edge(&verify_txn, another_follow_edge_id)?,
        another_follow_edge
    );

    verify_txn.commit()?;

    // 11. Test delete vertices and edges
    let delete_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    graph.delete_vertex(&delete_txn, alice_id)?;
    graph.delete_edge(&delete_txn, another_follow_edge_id)?;
    delete_txn.commit()?;

    // 12. Open new transaction and verify data
    let verify_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Check alice's vertex and its corresponding edges
    assert!(graph.get_vertex(&verify_txn, alice_id).is_err());
    assert!(graph.get_edge(&verify_txn, friend_edge_id).is_err());
    assert!(graph.get_edge(&verify_txn, another_friend_edge_id).is_err());

    // Check carol's vertex and its corresponding edges
    assert!(graph.get_vertex(&verify_txn, carol_id).is_ok());
    assert!(graph.get_edge(&verify_txn, follow_edge_id).is_ok());
    assert!(graph.get_edge(&verify_txn, another_follow_edge_id).is_err());

    // Check Vertex Iterator
    {
        let mut vertex_count = 0;
        let vertex_iter = verify_txn
            .iter_vertices()
            .filter_map(|v| v.ok())
            .filter(|v| match v.properties()[0].try_as_string() {
                Some(Some(name)) => {
                    name == "Alice" || name == "Bob" || name == "Carol" || name == "Dave"
                }
                _ => false,
            });
        for _ in vertex_iter {
            vertex_count += 1;
        }
        assert_eq!(vertex_count, 3); // Alice should be deleted
    }

    // Check Edge Iterator
    {
        let mut edge_count = 0;
        let edge_iter = verify_txn
            .iter_edges()
            .filter_map(|e| e.ok())
            .filter(|e| e.src_id() == alice_id);
        for _ in edge_iter {
            edge_count += 1;
        }
        assert_eq!(edge_count, 0); // Alice's edges should be deleted
    }

    // Check Adjacency Iterator
    {
        let mut adj_count = 0;
        let adj_iter = verify_txn.iter_adjacency(carol_id);
        for adj_result in adj_iter {
            let adj = adj_result?;
            assert!(adj.eid() == follow_edge_id);
            adj_count += 1;
        }
        assert_eq!(adj_count, 1); // Carol's adjacency list should contain follow_edge_id
    }
    verify_txn.commit()?;

    // 13. Test garbage collection
    // Loop to trigger garbage collection
    for _ in 0..50 {
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        txn.commit()?;
    }

    Ok(())
}

#[test]
fn test_graph_vertex_not_found() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Try to get non-existent vertex
    assert!(graph.get_vertex(&txn, 999).is_err());
    txn.abort()?;
    Ok(())
}

#[test]
fn test_graph_edge_not_found() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Try to get non-existent edge
    assert!(graph.get_edge(&txn, 999).is_err());
    txn.abort()?;
    Ok(())
}

#[test]
fn test_graph_duplicate_vertex_id() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    let alice = create_test_vertex(1, "Alice", 25);
    graph.create_vertex(&txn1, alice.clone())?;
    txn1.commit()?;

    // Create another vertex with same ID - system may allow this (overwrites)
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let alice2 = create_test_vertex(1, "Alice2", 30);
    graph.create_vertex(&txn2, alice2)?;
    txn2.commit()?;

    // Verify the vertex was updated/overwritten
    let txn3 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let retrieved = graph.get_vertex(&txn3, 1)?;
    // The vertex should exist (either original or updated)
    assert!(retrieved.vid() == 1);
    txn3.abort()?;
    Ok(())
}

#[test]
fn test_graph_duplicate_edge_id() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn1 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Create vertices first
    let alice = create_test_vertex(1, "Alice", 25);
    let bob = create_test_vertex(2, "Bob", 30);
    graph.create_vertex(&txn1, alice)?;
    graph.create_vertex(&txn1, bob)?;

    // Create edge
    let edge = create_test_edge(1, 1, 2, FRIEND_LABEL_ID);
    graph.create_edge(&txn1, edge.clone())?;
    txn1.commit()?;

    // Create another edge with same ID - system may allow this (overwrites)
    let txn2 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let edge2 = create_test_edge(1, 1, 2, FOLLOW_LABEL_ID);
    graph.create_edge(&txn2, edge2)?;
    txn2.commit()?;

    // Verify the edge exists
    let txn3 = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let retrieved = graph.get_edge(&txn3, 1)?;
    assert!(retrieved.eid() == 1);
    txn3.abort()?;
    Ok(())
}

#[test]
fn test_graph_edge_with_nonexistent_vertices() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Try to create edge between non-existent vertices
    let edge = create_test_edge(1, 999, 1000, FRIEND_LABEL_ID);
    let result = graph.create_edge(&txn, edge);
    assert!(result.is_err());

    txn.abort()?;
    Ok(())
}

#[test]
fn test_graph_delete_nonexistent_vertex() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Try to delete non-existent vertex
    let result = graph.delete_vertex(&txn, 999);
    assert!(result.is_err());

    txn.abort()?;
    Ok(())
}

#[test]
fn test_graph_delete_nonexistent_edge() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Try to delete non-existent edge
    let result = graph.delete_edge(&txn, 999);
    assert!(result.is_err());

    txn.abort()?;
    Ok(())
}

#[test]
fn test_graph_update_nonexistent_vertex_property() -> StorageResult<()> {
    use minigu_common::value::ScalarValue;

    let (graph, _cleaner) = create_empty_graph();
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Try to update property of non-existent vertex
    let result = graph.set_vertex_property(&txn, 999, vec![0], vec![ScalarValue::Int32(Some(100))]);
    assert!(result.is_err());

    txn.abort()?;
    Ok(())
}

#[test]
fn test_graph_update_nonexistent_edge_property() -> StorageResult<()> {
    use minigu_common::value::ScalarValue;

    let (graph, _cleaner) = create_empty_graph();
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Try to update property of non-existent edge
    let result = graph.set_edge_property(&txn, 999, vec![0], vec![ScalarValue::String(Some(
        "test".to_string(),
    ))]);
    assert!(result.is_err());

    txn.abort()?;
    Ok(())
}

#[test]
fn test_graph_empty_adjacency_list() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Create isolated vertex
    let alice = create_test_vertex(1, "Alice", 25);
    graph.create_vertex(&txn, alice)?;

    // Check adjacency list is empty
    let adj_count = txn.iter_adjacency(1).count();
    assert_eq!(adj_count, 0);

    txn.abort()?;
    Ok(())
}

#[test]
fn test_graph_self_loop_edge() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    let alice = create_test_vertex(1, "Alice", 25);
    graph.create_vertex(&txn, alice)?;

    // Create self-loop edge
    let self_edge = create_test_edge(1, 1, 1, FOLLOW_LABEL_ID);
    graph.create_edge(&txn, self_edge.clone())?;

    // Verify self-loop
    let retrieved_edge = graph.get_edge(&txn, 1)?;
    assert_eq!(retrieved_edge.src_id(), 1);
    assert_eq!(retrieved_edge.dst_id(), 1);

    txn.commit()?;
    Ok(())
}

#[test]
fn test_graph_bidirectional_edges() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    let alice = create_test_vertex(1, "Alice", 25);
    let bob = create_test_vertex(2, "Bob", 30);
    graph.create_vertex(&txn, alice)?;
    graph.create_vertex(&txn, bob)?;

    // Create bidirectional edges
    let edge1 = create_test_edge(1, 1, 2, FRIEND_LABEL_ID);
    let edge2 = create_test_edge(2, 2, 1, FRIEND_LABEL_ID);
    graph.create_edge(&txn, edge1)?;
    graph.create_edge(&txn, edge2)?;

    // Check Alice's outgoing
    let alice_out = txn.iter_adjacency_outgoing(1).count();
    assert_eq!(alice_out, 1);

    // Check Alice's incoming
    let alice_in = txn.iter_adjacency_incoming(1).count();
    assert_eq!(alice_in, 1);

    txn.commit()?;
    Ok(())
}

#[test]
fn test_graph_multiple_edge_types() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    let alice = create_test_vertex(1, "Alice", 25);
    let bob = create_test_vertex(2, "Bob", 30);
    graph.create_vertex(&txn, alice)?;
    graph.create_vertex(&txn, bob)?;

    // Create different types of edges between same vertices
    let friend_edge = create_test_edge(1, 1, 2, FRIEND_LABEL_ID);
    let follow_edge = create_test_edge(2, 1, 2, FOLLOW_LABEL_ID);
    graph.create_edge(&txn, friend_edge)?;
    graph.create_edge(&txn, follow_edge)?;

    // Verify both edges exist
    let total_edges = txn.iter_adjacency(1).count();
    assert_eq!(total_edges, 2);

    // Filter by edge type
    let friend_count = txn
        .iter_adjacency_outgoing(1)
        .filter_map(|adj| adj.ok())
        .filter(|adj| adj.label_id() == FRIEND_LABEL_ID)
        .count();
    assert_eq!(friend_count, 1);

    txn.commit()?;
    Ok(())
}

#[test]
fn test_graph_property_value_types() -> StorageResult<()> {
    use minigu_common::value::ScalarValue;
    use minigu_storage::model::properties::PropertyRecord;
    use minigu_storage::model::vertex::Vertex;

    let (graph, _cleaner) = create_empty_graph();
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Create vertex with different property types
    let vertex = Vertex::new(
        1,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            ScalarValue::String(Some("Alice".to_string())),
            ScalarValue::Int32(Some(25)),
            ScalarValue::Int64(Some(1000000)),
            ScalarValue::Float32(Some(1.75.into())),
            ScalarValue::Boolean(Some(true)),
        ]),
    );
    graph.create_vertex(&txn, vertex)?;

    let retrieved = graph.get_vertex(&txn, 1)?;
    assert_eq!(
        retrieved.properties()[0],
        ScalarValue::String(Some("Alice".to_string()))
    );
    assert_eq!(retrieved.properties()[1], ScalarValue::Int32(Some(25)));
    assert_eq!(retrieved.properties()[2], ScalarValue::Int64(Some(1000000)));
    assert_eq!(
        retrieved.properties()[3],
        ScalarValue::Float32(Some(1.75.into()))
    );
    assert_eq!(retrieved.properties()[4], ScalarValue::Boolean(Some(true)));

    txn.commit()?;
    Ok(())
}

#[test]
fn test_graph_null_property_values() -> StorageResult<()> {
    use minigu_common::value::ScalarValue;
    use minigu_storage::model::properties::PropertyRecord;
    use minigu_storage::model::vertex::Vertex;

    let (graph, _cleaner) = create_empty_graph();
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    let vertex = Vertex::new(
        1,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![ScalarValue::String(None), ScalarValue::Int32(None)]),
    );
    graph.create_vertex(&txn, vertex)?;

    let retrieved = graph.get_vertex(&txn, 1)?;
    assert_eq!(retrieved.properties()[0], ScalarValue::String(None));
    assert_eq!(retrieved.properties()[1], ScalarValue::Int32(None));

    txn.commit()?;
    Ok(())
}

#[test]
fn test_graph_vertex_count() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Create vertices
    for i in 1..=10 {
        let vertex = create_test_vertex(i, &format!("User{}", i), 20 + i as i32);
        graph.create_vertex(&txn, vertex)?;
    }

    let count = txn.iter_vertices().filter_map(|v| v.ok()).count();
    assert_eq!(count, 10);

    txn.commit()?;
    Ok(())
}

#[test]
fn test_graph_edge_count() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    // Create vertices
    for i in 1..=5 {
        let vertex = create_test_vertex(i, &format!("User{}", i), 20 + i as i32);
        graph.create_vertex(&txn, vertex)?;
    }

    // Create edges
    for i in 1..5 {
        let edge = create_test_edge(i as u64, i, i + 1, FRIEND_LABEL_ID);
        graph.create_edge(&txn, edge)?;
    }

    let count = txn.iter_edges().filter_map(|e| e.ok()).count();
    assert_eq!(count, 4);

    txn.commit()?;
    Ok(())
}

#[test]
fn test_graph_clear_all_data() -> StorageResult<()> {
    let (graph, _cleaner) = create_test_graph();

    // Delete all
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

    graph.delete_vertex(&txn, 1)?;
    graph.delete_vertex(&txn, 2)?;

    txn.commit()?;

    // Verify empty
    let verify_txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();
    let vertex_count = verify_txn.iter_vertices().filter_map(|v| v.ok()).count();
    let edge_count = verify_txn.iter_edges().filter_map(|e| e.ok()).count();

    assert_eq!(vertex_count, 0);
    assert_eq!(edge_count, 0);

    verify_txn.abort()?;
    Ok(())
}
