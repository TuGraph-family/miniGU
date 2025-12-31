use std::num::NonZeroU32;
use std::sync::Arc;

use minigu_common::value::ScalarValue;
use minigu_storage::ap::olap_graph::{OlapEdge, OlapPropertyStore, OlapStorage, OlapVertex};
use minigu_storage::ap::transaction::MemTransaction;
use minigu_storage::ap::{MutOlapGraph, OlapGraph};
use minigu_storage::common::model::properties::PropertyRecord;
use minigu_storage::error::StorageError;
use minigu_transaction::{IsolationLevel, Timestamp};

fn make_storage() -> OlapStorage {
    super::ap_graph_test::mock_olap_graph(0)
}

/// Helper function to create test edges with different timestamps
fn create_test_edges(storage: &Arc<OlapStorage>, txn_id: Timestamp, edge_offset: u32) {
    // Create vertex first
    let txn = MemTransaction::new(storage.clone(), txn_id, txn_id, IsolationLevel::Snapshot);

    let _ = storage.create_vertex(
        &txn,
        OlapVertex {
            vid: 1,
            properties: PropertyRecord::default(),
            block_offset: 0,
        },
    );

    // Create edges
    // Edge 1: label 100, dst 10
    let _ = storage.create_edge_in_txn(
        &txn,
        OlapEdge {
            label_id: NonZeroU32::new(100 + edge_offset),
            src_id: 1,
            dst_id: 10,
            properties: OlapPropertyStore::default(),
        },
    );

    // Edge 2: label 101, dst 20
    let _ = storage.create_edge_in_txn(
        &txn,
        OlapEdge {
            label_id: NonZeroU32::new(101 + edge_offset),
            src_id: 1,
            dst_id: 20,
            properties: OlapPropertyStore::default(),
        },
    );

    // Edge 3: label 102, dst 30
    let _ = storage.create_edge_in_txn(
        &txn,
        OlapEdge {
            label_id: NonZeroU32::new(102 + edge_offset),
            src_id: 1,
            dst_id: 30,
            properties: OlapPropertyStore::default(),
        },
    );

    txn.commit_at(None).expect("Commit should succeed");
}

#[test]
fn test_ap_commit_replaces_txn_id() {
    let storage = make_storage();
    let arc_storage = Arc::new(storage);

    let base_txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START);

    let vertex_txn = MemTransaction::new(
        arc_storage.clone(),
        base_txn_id,
        base_txn_id,
        IsolationLevel::Snapshot,
    );
    let _ = arc_storage.create_vertex(
        &vertex_txn,
        OlapVertex {
            vid: 1,
            properties: PropertyRecord::default(),
            block_offset: 0,
        },
    );
    vertex_txn
        .commit_at(None)
        .expect("Vertex commit should succeed");

    let edge1_txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 1);
    let edge1_txn = MemTransaction::new(
        arc_storage.clone(),
        edge1_txn_id,
        edge1_txn_id,
        IsolationLevel::Snapshot,
    );
    let _ = arc_storage.create_edge_in_txn(
        &edge1_txn,
        OlapEdge {
            label_id: NonZeroU32::new(100),
            src_id: 1,
            dst_id: 42,
            properties: OlapPropertyStore::default(),
        },
    );
    edge1_txn
        .commit_at(None)
        .expect("Edge1 commit should succeed");

    let edge2_txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 2);
    let edge2_txn = MemTransaction::new(
        arc_storage.clone(),
        edge2_txn_id,
        edge2_txn_id,
        IsolationLevel::Snapshot,
    );

    let _ = arc_storage.create_edge_in_txn(
        &edge2_txn,
        OlapEdge {
            label_id: NonZeroU32::new(100),
            src_id: 1,
            dst_id: 42,
            properties: OlapPropertyStore::default(),
        },
    );

    let commit_ts = edge2_txn.commit_at(None).expect("commit should succeed");

    let edges = arc_storage.edges.read().unwrap();
    let block = edges.first().unwrap();

    assert_eq!(block.edges[0].commit_ts, commit_ts);
    assert_eq!(block.min_ts, edge1_txn_id);
    assert_eq!(block.max_ts, edge2_txn_id);
}

#[test]
fn test_iter_edges_at_ts_filters() {
    // Test 1: Basic visibility filtering with multiple timestamps
    let storage = make_storage();
    let arc_storage = Arc::new(storage);

    let target_ts_50 = Timestamp::with_ts(Timestamp::TXN_ID_START + 50);
    let txn50 = MemTransaction::new(
        arc_storage.clone(),
        target_ts_50,
        target_ts_50,
        IsolationLevel::Snapshot,
    );
    txn50.commit_at(None).expect("txn50 commit should succeed");

    // Create edges at 100 timestamps
    let txn_id1 = Timestamp::with_ts(Timestamp::TXN_ID_START + 100);
    create_test_edges(&arc_storage, txn_id1, 0);

    let target_ts_150 = Timestamp::with_ts(Timestamp::TXN_ID_START + 150);
    let txn150 = MemTransaction::new(
        arc_storage.clone(),
        target_ts_150,
        target_ts_150,
        IsolationLevel::Snapshot,
    );
    txn150
        .commit_at(None)
        .expect("txn150 commit should succeed");

    // Create edges at 200 timestamps
    let txn_id2 = Timestamp::with_ts(Timestamp::TXN_ID_START + 200);
    create_test_edges(&arc_storage, txn_id2, 100);

    let target_ts_250 = Timestamp::with_ts(Timestamp::TXN_ID_START + 250);
    let txn250 = MemTransaction::new(
        arc_storage.clone(),
        target_ts_250,
        target_ts_250,
        IsolationLevel::Snapshot,
    );
    txn250
        .commit_at(None)
        .expect("txn250 commit should succeed");

    // Test visibility at different target timestamps
    // At ts 50 (before any commits) - should see nothing
    let iter50 = arc_storage.iter_edges_at_ts(&txn50).unwrap();
    let mut count50 = 0;
    for _result in iter50 {
        count50 += 1;
    }
    assert_eq!(count50, 0, "Should see no edges at ts 50");

    // At ts 150 (after first commit, before second) - should see only first batch
    let iter150 = arc_storage.iter_edges_at_ts(&txn150).unwrap();
    let mut count150 = 0;
    for _ in iter150 {
        count150 += 1;
    }
    assert_eq!(
        count150, 3,
        "Should see 3 edges from first transaction at ts 150"
    );

    // At ts 250 (after both commits) - should see all edges
    let iter250 = arc_storage.iter_edges_at_ts(&txn250).unwrap();
    let mut count250 = 0;
    for _ in iter250 {
        count250 += 1;
    }
    assert_eq!(
        count250, 6,
        "Should see 6 edges from both transactions at ts 250"
    );
}

#[test]
fn test_uncommitted_data_isolation() {
    let storage2 = make_storage();
    let arc_storage2 = Arc::new(storage2);

    let txn_v_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 500);
    let txn500 = MemTransaction::new(
        arc_storage2.clone(),
        txn_v_id,
        txn_v_id,
        IsolationLevel::Snapshot,
    );

    // Create vertex
    let _ = arc_storage2.create_vertex(
        &txn500,
        OlapVertex {
            vid: 1,
            properties: PropertyRecord::default(),
            block_offset: 0,
        },
    );

    // Start transaction A (uncommitted)
    let txn_a_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 1000);
    let txn_a = MemTransaction::new(
        arc_storage2.clone(),
        txn_a_id,
        txn_a_id,
        IsolationLevel::Snapshot,
    );

    // Insert edge in transaction A (uncommitted)
    let eid = arc_storage2
        .create_edge_in_txn(
            &txn_a,
            OlapEdge {
                label_id: NonZeroU32::new(500),
                src_id: 1,
                dst_id: 100,
                properties: OlapPropertyStore::default(),
            },
        )
        .unwrap();

    // Transaction A should see its own uncommitted edge
    let edge_result = arc_storage2.get_edge_at_ts(&txn_a, eid);
    assert!(edge_result.is_ok(), "Transaction A should see its own edge");
    assert!(
        edge_result.unwrap().is_some(),
        "Transaction A should see its own edge"
    );

    // Another transaction B should NOT see transaction A's uncommitted edge
    let txn_b_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 2000);
    let txn_b = MemTransaction::new(
        arc_storage2.clone(),
        txn_b_id,
        txn_b_id,
        IsolationLevel::Snapshot,
    );
    let edge_result_b = arc_storage2.get_edge_at_ts(&txn_b, eid);
    if let Ok(item) = edge_result_b {
        assert!(
            item.is_none(),
            "Transaction B should not see transaction A's edge"
        );
    }
    // Test iter_edges_at_ts with uncommitted data
    let iter_a = arc_storage2.iter_edges_at_ts(&txn_a).unwrap();
    let mut found_in_a = false;
    for edge in iter_a {
        if let Ok(e) = edge
            && e.label_id == NonZeroU32::new(500)
        {
            found_in_a = true;
            break;
        }
    }
    assert!(
        found_in_a,
        "Transaction A should find its own edge in iterator"
    );

    let iter_b = arc_storage2.iter_edges_at_ts(&txn_b).unwrap();
    let mut found_in_b = false;
    for edge in iter_b {
        if let Ok(e) = edge
            && e.label_id == NonZeroU32::new(500)
        {
            found_in_b = true;
            break;
        }
    }
    assert!(
        !found_in_b,
        "Transaction B should not find transaction A's edge in iterator"
    );
}

#[test]
fn test_set_edge_property_in_txn_basic() {
    let storage = make_storage();
    let arc_storage = Arc::new(storage);
    let txn_id_1 = Timestamp::with_ts(Timestamp::TXN_ID_START + 1);
    let txn_1 = MemTransaction::new(
        arc_storage.clone(),
        txn_id_1,
        txn_id_1,
        IsolationLevel::Snapshot,
    );

    // Create vertex and edge
    let _ = arc_storage.create_vertex(
        &txn_1,
        OlapVertex {
            vid: 1,
            properties: PropertyRecord::default(),
            block_offset: 0,
        },
    );

    let eid = arc_storage
        .create_edge_in_txn(
            &txn_1,
            OlapEdge {
                label_id: NonZeroU32::new(100),
                src_id: 1,
                dst_id: 10,
                properties: OlapPropertyStore::new(vec![
                    Some(ScalarValue::Int32(Some(42))),
                    Some(ScalarValue::String(Some("hello".to_string()))),
                ]),
            },
        )
        .unwrap();
    txn_1.commit_at(None).expect("Commit should succeed");

    // Test setting a single property
    let txn_id_2 = Timestamp::with_ts(Timestamp::TXN_ID_START + 2);
    let txn_2 = MemTransaction::new(
        arc_storage.clone(),
        txn_id_2,
        txn_id_2,
        IsolationLevel::Snapshot,
    );
    let result = arc_storage.set_edge_property_in_txn(
        &txn_2,
        eid,
        vec![0],
        vec![ScalarValue::Int32(Some(10086))],
    );
    assert!(result.is_ok(), "Setting edge property should succeed");
    txn_2.commit_at(None).expect("Commit should succeed");

    // Verify the property was updated
    let txn_id_3 = Timestamp::with_ts(Timestamp::TXN_ID_START + 3);
    let get_txn = MemTransaction::new(
        arc_storage.clone(),
        txn_id_3,
        txn_id_3,
        IsolationLevel::Snapshot,
    );
    get_txn.commit_at(None).expect("Commit should succeed");
    let edge = arc_storage.get_edge_at_ts(&get_txn, eid).unwrap();
    assert_eq!(
        edge.as_ref().unwrap().properties.get(0),
        Some(ScalarValue::Int32(Some(10086))),
        "Property should be updated"
    );
    assert_eq!(
        edge.as_ref().unwrap().properties.get(1),
        Some(ScalarValue::String(Some("hello".to_string()))),
        "Other properties should remain unchanged"
    );
}

#[test]
fn test_set_edge_property_in_txn_multiple_properties() {
    let storage = make_storage();
    let arc_storage = Arc::new(storage);
    let txn_id_1 = Timestamp::with_ts(Timestamp::TXN_ID_START + 1);
    let txn_1 = MemTransaction::new(
        arc_storage.clone(),
        txn_id_1,
        txn_id_1,
        IsolationLevel::Snapshot,
    );

    // Create vertex and edge with multiple properties
    let _ = arc_storage.create_vertex(
        &txn_1,
        OlapVertex {
            vid: 1,
            properties: PropertyRecord::default(),
            block_offset: 0,
        },
    );

    let eid = arc_storage
        .create_edge_in_txn(
            &txn_1,
            OlapEdge {
                label_id: NonZeroU32::new(100),
                src_id: 1,
                dst_id: 10,
                properties: OlapPropertyStore::new(vec![
                    Some(ScalarValue::Int32(Some(42))),
                    Some(ScalarValue::String(Some("hello".to_string()))),
                    Some(ScalarValue::Boolean(Some(true))),
                ]),
            },
        )
        .unwrap();
    txn_1.commit_at(None).expect("Commit should succeed");

    // Test setting multiple properties
    let txn_id_2 = Timestamp::with_ts(Timestamp::TXN_ID_START + 2);
    let txn_2 = MemTransaction::new(
        arc_storage.clone(),
        txn_id_2,
        txn_id_2,
        IsolationLevel::Snapshot,
    );
    let result = arc_storage.set_edge_property_in_txn(
        &txn_2,
        eid,
        vec![0, 2],
        vec![
            ScalarValue::Int32(Some(10086)),
            ScalarValue::Boolean(Some(false)),
        ],
    );
    txn_2.commit_at(None).expect("Commit should succeed");
    assert!(
        result.is_ok(),
        "Setting multiple edge properties should succeed"
    );

    // Verify all properties were updated
    let get_txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 3);
    let get_txn = MemTransaction::new(
        arc_storage.clone(),
        get_txn_id,
        get_txn_id,
        IsolationLevel::Snapshot,
    );
    get_txn.commit_at(None).expect("Commit should succeed");
    let edge = arc_storage.get_edge_at_ts(&get_txn, eid).unwrap();
    assert_eq!(
        edge.as_ref().unwrap().properties.get(0),
        Some(ScalarValue::Int32(Some(10086))),
        "First property should be updated"
    );
    assert_eq!(
        edge.as_ref().unwrap().properties.get(1),
        Some(ScalarValue::String(Some("hello".to_string()))),
        "Second property should remain unchanged"
    );
    assert_eq!(
        edge.as_ref().unwrap().properties.get(2),
        Some(ScalarValue::Boolean(Some(false))),
        "Third property should be updated"
    );
}

#[test]
fn test_set_edge_property_in_txn_nonexistent_edge() {
    let storage = make_storage();
    let arc_storage = Arc::new(storage);
    let txn = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(1),
        Timestamp::with_ts(1),
        IsolationLevel::Snapshot,
    );

    // Try to set property on non-existent edge
    let result = arc_storage.set_edge_property_in_txn(
        &txn,
        999u64,
        vec![0],
        vec![ScalarValue::Int32(Some(10086))],
    );
    assert!(result.is_err(), "Should return error for non-existent edge");
    assert!(
        matches!(result.unwrap_err(), StorageError::EdgeNotFound(_)),
        "Should return EdgeNotFound error"
    );
}

#[test]
fn test_set_edge_property_in_txn_transaction_rollback() {
    let storage = make_storage();
    let arc_storage = Arc::new(storage);
    let txn_id_1 = Timestamp::with_ts(Timestamp::TXN_ID_START + 1);
    let txn_1 = MemTransaction::new(
        arc_storage.clone(),
        txn_id_1,
        txn_id_1,
        IsolationLevel::Snapshot,
    );

    // Create vertex and edge
    let _ = arc_storage.create_vertex(
        &txn_1,
        OlapVertex {
            vid: 1,
            properties: PropertyRecord::default(),
            block_offset: 0,
        },
    );

    let eid = arc_storage
        .create_edge_in_txn(
            &txn_1,
            OlapEdge {
                label_id: NonZeroU32::new(100),
                src_id: 1,
                dst_id: 10,
                properties: OlapPropertyStore::new(vec![Some(ScalarValue::Int32(Some(42)))]),
            },
        )
        .unwrap();
    txn_1.commit_at(None).expect("Commit should succeed");

    // Start a transaction to set property
    let txn_id_2 = Timestamp::with_ts(Timestamp::TXN_ID_START + 2);
    let txn_2 = MemTransaction::new(
        arc_storage.clone(),
        txn_id_2,
        txn_id_2,
        IsolationLevel::Snapshot,
    );
    let _ = arc_storage.set_edge_property_in_txn(
        &txn_2,
        eid,
        vec![0],
        vec![ScalarValue::Int32(Some(10086))],
    );

    // Rollback the transaction
    txn_2.abort().expect("Rollback should succeed");

    // Verify the property was not changed
    let get_txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 3);
    let get_txn = MemTransaction::new(
        arc_storage.clone(),
        get_txn_id,
        get_txn_id,
        IsolationLevel::Snapshot,
    );
    get_txn.commit_at(None).expect("Commit should succeed");
    let edge = arc_storage.get_edge_at_ts(&get_txn, eid).unwrap();
    assert_eq!(
        edge.as_ref().unwrap().properties.get(0),
        Some(ScalarValue::Int32(Some(42))),
        "Property should remain unchanged after rollback"
    );
}

#[test]
fn test_delete_edge_in_txn_basic() {
    let storage = make_storage();
    let arc_storage = Arc::new(storage);
    let txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 1);
    let txn = MemTransaction::new(
        arc_storage.clone(),
        txn_id,
        txn_id,
        IsolationLevel::Snapshot,
    );

    // Create vertex and edge
    let _ = arc_storage.create_vertex(
        &txn,
        OlapVertex {
            vid: 1,
            properties: PropertyRecord::default(),
            block_offset: 0,
        },
    );

    let eid = arc_storage
        .create_edge_in_txn(
            &txn,
            OlapEdge {
                label_id: NonZeroU32::new(100),
                src_id: 1,
                dst_id: 10,
                properties: OlapPropertyStore::new(vec![Some(ScalarValue::Int32(Some(42)))]),
            },
        )
        .unwrap();
    txn.commit_at(None).expect("Commit should succeed");

    // Test deleting the edge
    let delete_txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 2);
    let delete_txn = MemTransaction::new(
        arc_storage.clone(),
        delete_txn_id,
        delete_txn_id,
        IsolationLevel::Snapshot,
    );
    delete_txn.commit_at(None).expect("Commit should succeed");
    let result = arc_storage.delete_edge_in_txn(&delete_txn, eid);
    assert!(result.is_ok(), "Deleting edge should succeed");

    // Verify the edge is gone
    let get_txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 3);
    let get_txn = MemTransaction::new(
        arc_storage.clone(),
        get_txn_id,
        get_txn_id,
        IsolationLevel::Snapshot,
    );
    get_txn.commit_at(None).expect("Commit should succeed");
    let edge = arc_storage.get_edge_at_ts(&get_txn, eid);
    assert!(
        matches!(edge, Ok(None)),
        "Edge should not be found after deletion, got: {:?}",
        edge
    );
}

#[test]
fn test_iter_adjacency_at_ts_filters() {
    // Test 1: Basic visibility filtering with multiple timestamps
    let storage = make_storage();
    let arc_storage = Arc::new(storage);

    let target_ts_50 = Timestamp::with_ts(Timestamp::TXN_ID_START + 50);
    let txn50 = MemTransaction::new(
        arc_storage.clone(),
        target_ts_50,
        target_ts_50,
        IsolationLevel::Snapshot,
    );

    // Create vertex
    let _ = arc_storage.create_vertex(
        &txn50,
        OlapVertex {
            vid: 1,
            properties: PropertyRecord::default(),
            block_offset: 0,
        },
    );
    txn50.commit_at(None).expect("Vertex commit should succeed");

    // Create edges at 100 timestamps
    let txn_id1 = Timestamp::with_ts(Timestamp::TXN_ID_START + 100);
    create_test_edges(&arc_storage, txn_id1, 0);

    let target_ts_150 = Timestamp::with_ts(Timestamp::TXN_ID_START + 150);
    let txn150 = MemTransaction::new(
        arc_storage.clone(),
        target_ts_150,
        target_ts_150,
        IsolationLevel::Snapshot,
    );
    txn150
        .commit_at(None)
        .expect("txn150 commit should succeed");

    // Create edges at 200 timestamps
    let txn_id2 = Timestamp::with_ts(Timestamp::TXN_ID_START + 200);
    create_test_edges(&arc_storage, txn_id2, 100);

    let target_ts_250 = Timestamp::with_ts(Timestamp::TXN_ID_START + 250);
    let txn250 = MemTransaction::new(
        arc_storage.clone(),
        target_ts_250,
        target_ts_250,
        IsolationLevel::Snapshot,
    );
    txn250
        .commit_at(None)
        .expect("txn250 commit should succeed");

    // Test visibility at different target timestamps
    // At ts 50 (before any commits) - should see nothing
    let iter50 = arc_storage.iter_adjacency_at_ts(&txn50, 1).unwrap();
    let mut count50 = 0;
    for _result in iter50 {
        count50 += 1;
    }
    assert_eq!(count50, 0, "Should see no edges at ts 50");

    // At ts 150 (after first commit, before second) - should see only first batch
    let iter150 = arc_storage.iter_adjacency_at_ts(&txn150, 1).unwrap();
    let mut count150 = 0;
    for _ in iter150 {
        count150 += 1;
    }
    assert_eq!(
        count150, 3,
        "Should see 3 edges from first transaction at ts 150"
    );

    // At ts 250 (after both commits) - should see all edges
    let iter250 = arc_storage.iter_adjacency_at_ts(&txn250, 1).unwrap();
    let mut count250 = 0;
    for _ in iter250 {
        count250 += 1;
    }
    assert_eq!(
        count250, 6,
        "Should see 6 edges from both transactions at ts 250"
    );
}

#[test]
fn test_delete_edge_in_txn_transaction_rollback() {
    let storage = make_storage();
    let arc_storage = Arc::new(storage);
    let txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 1);
    let txn = MemTransaction::new(
        arc_storage.clone(),
        txn_id,
        txn_id,
        IsolationLevel::Snapshot,
    );

    // Create vertex and edge
    let _ = arc_storage.create_vertex(
        &txn,
        OlapVertex {
            vid: 1,
            properties: PropertyRecord::default(),
            block_offset: 0,
        },
    );

    let eid = arc_storage
        .create_edge_in_txn(
            &txn,
            OlapEdge {
                label_id: NonZeroU32::new(100),
                src_id: 1,
                dst_id: 10,
                properties: OlapPropertyStore::new(vec![Some(ScalarValue::Int32(Some(42)))]),
            },
        )
        .unwrap();
    txn.commit_at(None).expect("Commit should succeed");

    // Start a transaction to delete edge
    let delete_txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 2);
    let delete_txn = MemTransaction::new(
        arc_storage.clone(),
        delete_txn_id,
        delete_txn_id,
        IsolationLevel::Snapshot,
    );
    let _ = arc_storage.delete_edge_in_txn(&delete_txn, eid);

    // Rollback the transaction
    delete_txn.abort().expect("Rollback should succeed");

    // Verify the edge is still there
    let get_txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 3);
    let get_txn = MemTransaction::new(
        arc_storage.clone(),
        get_txn_id,
        get_txn_id,
        IsolationLevel::Snapshot,
    );
    get_txn.commit_at(None).expect("Commit should succeed");
    let edge = arc_storage.get_edge_at_ts(&get_txn, eid).unwrap();
    assert_eq!(
        edge.as_ref().unwrap().properties.get(0),
        Some(ScalarValue::Int32(Some(42))),
        "Edge should still exist after rollback"
    );
}

#[test]
fn test_delete_edge_in_txn_nonexistent_edge() {
    let storage = make_storage();
    let arc_storage = Arc::new(storage);
    let txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 1);
    let txn = MemTransaction::new(
        arc_storage.clone(),
        txn_id,
        txn_id,
        IsolationLevel::Snapshot,
    );

    // Try to delete non-existent edge
    let result = arc_storage.delete_edge_in_txn(&txn, 999u64);
    assert!(result.is_err(), "Should return error for non-existent edge");
    assert!(
        matches!(result.unwrap_err(), StorageError::EdgeNotFound(_)),
        "Should return EdgeNotFound error"
    );
}

#[test]
fn test_delete_edge_in_txn_with_properties() {
    let storage = make_storage();
    let arc_storage = Arc::new(storage);
    let txn_id_1 = Timestamp::with_ts(Timestamp::TXN_ID_START + 1);
    let txn_1 = MemTransaction::new(
        arc_storage.clone(),
        txn_id_1,
        txn_id_1,
        IsolationLevel::Snapshot,
    );

    // Create vertex and edge with multiple properties
    let _ = arc_storage.create_vertex(
        &txn_1,
        OlapVertex {
            vid: 1,
            properties: PropertyRecord::default(),
            block_offset: 0,
        },
    );

    let eid = arc_storage
        .create_edge_in_txn(
            &txn_1,
            OlapEdge {
                label_id: NonZeroU32::new(100),
                src_id: 1,
                dst_id: 10,
                properties: OlapPropertyStore::new(vec![
                    Some(ScalarValue::Int32(Some(42))),
                    Some(ScalarValue::String(Some("hello".to_string()))),
                ]),
            },
        )
        .unwrap();
    txn_1.commit_at(None).expect("Commit should succeed");

    // Test deleting the edge
    let delete_txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 2);
    let delete_txn = MemTransaction::new(
        arc_storage.clone(),
        delete_txn_id,
        delete_txn_id,
        IsolationLevel::Snapshot,
    );
    delete_txn.commit_at(None).expect("Commit should succeed");
    let result = arc_storage.delete_edge_in_txn(&delete_txn, eid);
    assert!(
        result.is_ok(),
        "Deleting edge with properties should succeed"
    );

    // Verify the edge is gone
    let get_txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 3);
    let get_txn = MemTransaction::new(
        arc_storage.clone(),
        get_txn_id,
        get_txn_id,
        IsolationLevel::Snapshot,
    );
    get_txn.commit_at(None).expect("Commit should succeed");
    let edge = arc_storage.get_edge_at_ts(&get_txn, eid);
    assert!(
        matches!(edge, Ok(None)),
        "Edge should not be found after deletion, got: {:?}",
        edge
    );
}
