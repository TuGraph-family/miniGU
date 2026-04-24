use std::num::NonZeroU32;
use std::sync::{Arc, Barrier, mpsc};
use std::thread;

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
fn create_test_edges(
    storage: &Arc<OlapStorage>,
    txn_id: Timestamp,
    start_ts: Timestamp,
    edge_offset: u32,
) {
    // Create vertex first
    let txn = MemTransaction::new(storage.clone(), txn_id, start_ts, IsolationLevel::Snapshot);

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

    txn.commit_at(Some(start_ts))
        .expect("Commit should succeed");
}

#[test]
fn test_ap_commit_replaces_txn_id() {
    let storage = make_storage();
    let arc_storage = Arc::new(storage);

    let base_txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START);

    let vertex_txn = MemTransaction::new(
        arc_storage.clone(),
        base_txn_id,
        Timestamp::with_ts(0),
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
        Timestamp::with_ts(1),
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
    let edge1_commit_ts = edge1_txn
        .commit_at(None)
        .expect("Edge1 commit should succeed");

    let edge2_txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 2);
    let edge2_txn = MemTransaction::new(
        arc_storage.clone(),
        edge2_txn_id,
        Timestamp::with_ts(2),
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

    let edge2_commit_ts = edge2_txn.commit_at(None).expect("commit should succeed");

    let edges = arc_storage.edges.read().unwrap();
    let block = edges.first().unwrap();

    assert_eq!(block.edges[0].commit_ts, edge2_commit_ts);
    assert_eq!(block.min_ts, edge1_commit_ts);
    assert_eq!(block.max_ts, edge2_commit_ts);
}

#[test]
fn test_iter_edges_at_ts_filters() {
    // Test 1: Basic visibility filtering with multiple timestamps
    let storage = make_storage();
    let arc_storage = Arc::new(storage);

    let target_ts_50 = Timestamp::with_ts(50);
    let txn50 = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 50),
        target_ts_50,
        IsolationLevel::Snapshot,
    );

    // Create edges at 100 timestamps
    let txn_id1 = Timestamp::with_ts(Timestamp::TXN_ID_START + 100);
    create_test_edges(&arc_storage, txn_id1, Timestamp::with_ts(100), 0);

    let target_ts_150 = Timestamp::with_ts(150);
    let txn150 = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 150),
        target_ts_150,
        IsolationLevel::Snapshot,
    );

    // Create edges at 200 timestamps
    let txn_id2 = Timestamp::with_ts(Timestamp::TXN_ID_START + 200);
    create_test_edges(&arc_storage, txn_id2, Timestamp::with_ts(200), 100);

    let target_ts_250 = Timestamp::with_ts(250);
    let txn250 = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 250),
        target_ts_250,
        IsolationLevel::Snapshot,
    );

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
        Timestamp::with_ts(500),
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
        Timestamp::with_ts(1000),
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
        Timestamp::with_ts(2000),
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
        Timestamp::with_ts(1),
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
        Timestamp::with_ts(2),
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
        Timestamp::max_commit_ts(),
        IsolationLevel::Snapshot,
    );
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
        Timestamp::with_ts(1),
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
        Timestamp::with_ts(2),
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
        Timestamp::max_commit_ts(),
        IsolationLevel::Snapshot,
    );
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
        Timestamp::with_ts(Timestamp::TXN_ID_START + 1),
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
        Timestamp::with_ts(1),
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
        Timestamp::with_ts(2),
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
        Timestamp::max_commit_ts(),
        IsolationLevel::Snapshot,
    );
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
        Timestamp::with_ts(1),
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
        Timestamp::with_ts(2),
        IsolationLevel::Snapshot,
    );
    let result = arc_storage.delete_edge_in_txn(&delete_txn, eid);
    assert!(result.is_ok(), "Deleting edge should succeed");
    delete_txn.commit_at(None).expect("Commit should succeed");

    // Verify the edge is gone
    let get_txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 3);
    let get_txn = MemTransaction::new(
        arc_storage.clone(),
        get_txn_id,
        Timestamp::max_commit_ts(),
        IsolationLevel::Snapshot,
    );
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

    let target_ts_50 = Timestamp::with_ts(50);
    let txn50 = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 50),
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
    create_test_edges(&arc_storage, txn_id1, Timestamp::with_ts(100), 0);

    let target_ts_150 = Timestamp::with_ts(150);
    let txn150 = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 150),
        target_ts_150,
        IsolationLevel::Snapshot,
    );

    // Create edges at 200 timestamps
    let txn_id2 = Timestamp::with_ts(Timestamp::TXN_ID_START + 200);
    create_test_edges(&arc_storage, txn_id2, Timestamp::with_ts(200), 100);

    let target_ts_250 = Timestamp::with_ts(250);
    let txn250 = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 250),
        target_ts_250,
        IsolationLevel::Snapshot,
    );

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
        Timestamp::with_ts(1),
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
        Timestamp::with_ts(2),
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
        Timestamp::max_commit_ts(),
        IsolationLevel::Snapshot,
    );
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
        Timestamp::with_ts(1),
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
        Timestamp::with_ts(1),
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
        Timestamp::with_ts(2),
        IsolationLevel::Snapshot,
    );
    let result = arc_storage.delete_edge_in_txn(&delete_txn, eid);
    assert!(
        result.is_ok(),
        "Deleting edge with properties should succeed"
    );
    delete_txn.commit_at(None).expect("Commit should succeed");

    // Verify the edge is gone
    let get_txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 3);
    let get_txn = MemTransaction::new(
        arc_storage.clone(),
        get_txn_id,
        Timestamp::max_commit_ts(),
        IsolationLevel::Snapshot,
    );
    let edge = arc_storage.get_edge_at_ts(&get_txn, eid);
    assert!(
        matches!(edge, Ok(None)),
        "Edge should not be found after deletion, got: {:?}",
        edge
    );
}

#[test]
fn test_concurrent_set_and_delete_serializes_writes() {
    let storage = make_storage();
    let arc_storage = Arc::new(storage);

    let setup_txn = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 10),
        Timestamp::with_ts(10),
        IsolationLevel::Snapshot,
    );
    let _ = arc_storage.create_vertex(
        &setup_txn,
        OlapVertex {
            vid: 1,
            properties: PropertyRecord::default(),
            block_offset: 0,
        },
    );
    let eid = arc_storage
        .create_edge_in_txn(
            &setup_txn,
            OlapEdge {
                label_id: NonZeroU32::new(100),
                src_id: 1,
                dst_id: 10,
                properties: OlapPropertyStore::new(vec![Some(ScalarValue::Int32(Some(1)))]),
            },
        )
        .unwrap();
    setup_txn
        .commit_at(None)
        .expect("Setup commit should succeed");

    let barrier = Arc::new(Barrier::new(2));

    let storage_a = arc_storage.clone();
    let barrier_a = barrier.clone();
    let handle_a = thread::spawn(move || {
        let txn = MemTransaction::new(
            storage_a.clone(),
            Timestamp::with_ts(Timestamp::TXN_ID_START + 11),
            Timestamp::with_ts(11),
            IsolationLevel::Snapshot,
        );
        barrier_a.wait();
        let _ = storage_a.set_edge_property_in_txn(
            &txn,
            eid,
            vec![0],
            vec![ScalarValue::Int32(Some(111))],
        );
        txn.commit_at(None).expect("Set commit should succeed");
    });

    let storage_b = arc_storage.clone();
    let barrier_b = barrier.clone();
    let handle_b = thread::spawn(move || {
        let txn = MemTransaction::new(
            storage_b.clone(),
            Timestamp::with_ts(Timestamp::TXN_ID_START + 12),
            Timestamp::with_ts(12),
            IsolationLevel::Snapshot,
        );
        barrier_b.wait();
        let _ = storage_b.delete_edge_in_txn(&txn, eid);
        txn.commit_at(None).expect("Delete commit should succeed");
    });

    handle_a.join().expect("Thread A should finish");
    handle_b.join().expect("Thread B should finish");

    let read_txn = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 13),
        Timestamp::max_commit_ts(),
        IsolationLevel::Snapshot,
    );
    let edge = arc_storage.get_edge_at_ts(&read_txn, eid).unwrap();
    assert!(
        edge.is_none(),
        "Edge should be deleted after concurrent writes"
    );
}

#[test]
fn test_concurrent_read_hides_uncommitted_edge() {
    let storage = make_storage();
    let arc_storage = Arc::new(storage);

    let setup_txn = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 20),
        Timestamp::with_ts(20),
        IsolationLevel::Snapshot,
    );
    let _ = arc_storage.create_vertex(
        &setup_txn,
        OlapVertex {
            vid: 1,
            properties: PropertyRecord::default(),
            block_offset: 0,
        },
    );
    setup_txn
        .commit_at(None)
        .expect("Setup commit should succeed");

    let (ready_tx, ready_rx) = mpsc::channel::<u64>();
    let (done_tx, done_rx) = mpsc::channel::<()>();

    let storage_writer = arc_storage.clone();
    let handle_writer = thread::spawn(move || {
        let txn = MemTransaction::new(
            storage_writer.clone(),
            Timestamp::with_ts(Timestamp::TXN_ID_START + 21),
            Timestamp::with_ts(21),
            IsolationLevel::Snapshot,
        );
        let eid = storage_writer
            .create_edge_in_txn(
                &txn,
                OlapEdge {
                    label_id: NonZeroU32::new(200),
                    src_id: 1,
                    dst_id: 100,
                    properties: OlapPropertyStore::default(),
                },
            )
            .unwrap();
        ready_tx.send(eid).expect("Ready send should succeed");
        done_rx.recv().expect("Done recv should succeed");
        txn.commit_at(None).expect("Writer commit should succeed");
        eid
    });

    let storage_reader = arc_storage.clone();
    let handle_reader = thread::spawn(move || {
        let eid = ready_rx.recv().expect("Ready recv should succeed");
        let txn = MemTransaction::new(
            storage_reader.clone(),
            Timestamp::with_ts(Timestamp::TXN_ID_START + 22),
            Timestamp::with_ts(22),
            IsolationLevel::Snapshot,
        );
        let edge = storage_reader.get_edge_at_ts(&txn, eid).unwrap();
        assert!(edge.is_none(), "Reader should not see uncommitted edge");

        let iter = storage_reader.iter_edges_at_ts(&txn).unwrap();
        let mut found = false;
        for next in iter {
            if let Ok(e) = next
                && e.label_id == NonZeroU32::new(200)
            {
                found = true;
                break;
            }
        }
        assert!(!found, "Iterator should not see uncommitted edge");
        done_tx.send(()).expect("Done send should succeed");
    });

    handle_reader.join().expect("Reader thread should finish");
    let eid = handle_writer.join().expect("Writer thread should finish");

    let read_txn = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 23),
        Timestamp::max_commit_ts(),
        IsolationLevel::Snapshot,
    );
    let edge = arc_storage.get_edge_at_ts(&read_txn, eid).unwrap();
    assert!(edge.is_some(), "Edge should be visible after commit");
}

#[test]
fn test_concurrent_insert_and_set_preserves_properties() {
    let storage = make_storage();
    let arc_storage = Arc::new(storage);

    let setup_txn = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 30),
        Timestamp::with_ts(30),
        IsolationLevel::Snapshot,
    );
    let _ = arc_storage.create_vertex(
        &setup_txn,
        OlapVertex {
            vid: 1,
            properties: PropertyRecord::default(),
            block_offset: 0,
        },
    );
    let eid = arc_storage
        .create_edge_in_txn(
            &setup_txn,
            OlapEdge {
                label_id: NonZeroU32::new(300),
                src_id: 1,
                dst_id: 20,
                properties: OlapPropertyStore::new(vec![Some(ScalarValue::Int32(Some(1)))]),
            },
        )
        .unwrap();
    setup_txn
        .commit_at(None)
        .expect("Setup commit should succeed");

    let barrier = Arc::new(Barrier::new(2));

    let storage_set = arc_storage.clone();
    let barrier_set = barrier.clone();
    let handle_set = thread::spawn(move || {
        let txn = MemTransaction::new(
            storage_set.clone(),
            Timestamp::with_ts(Timestamp::TXN_ID_START + 31),
            Timestamp::with_ts(31),
            IsolationLevel::Snapshot,
        );
        barrier_set.wait();
        let _ = storage_set.set_edge_property_in_txn(
            &txn,
            eid,
            vec![0],
            vec![ScalarValue::Int32(Some(999))],
        );
        txn.commit_at(None).expect("Set commit should succeed");
    });

    let storage_insert = arc_storage.clone();
    let barrier_insert = barrier.clone();
    let handle_insert = thread::spawn(move || {
        let txn = MemTransaction::new(
            storage_insert.clone(),
            Timestamp::with_ts(Timestamp::TXN_ID_START + 32),
            Timestamp::with_ts(32),
            IsolationLevel::Snapshot,
        );
        barrier_insert.wait();
        let _ = storage_insert
            .create_edge_in_txn(
                &txn,
                OlapEdge {
                    label_id: NonZeroU32::new(301),
                    src_id: 1,
                    dst_id: 10,
                    properties: OlapPropertyStore::new(vec![Some(ScalarValue::Int32(Some(2)))]),
                },
            )
            .unwrap();
        txn.commit_at(None).expect("Insert commit should succeed");
    });

    handle_set.join().expect("Set thread should finish");
    handle_insert.join().expect("Insert thread should finish");

    let read_txn = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 33),
        Timestamp::max_commit_ts(),
        IsolationLevel::Snapshot,
    );
    let edge = arc_storage.get_edge_at_ts(&read_txn, eid).unwrap();
    let props = edge.unwrap().properties;
    assert_eq!(
        props.get(0),
        Some(ScalarValue::Int32(Some(999))),
        "Property update should remain on the original edge"
    );
}

#[test]
fn test_concurrent_commit_and_abort_preserve_committed_value() {
    let storage = make_storage();
    let arc_storage = Arc::new(storage);

    let setup_txn = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 40),
        Timestamp::with_ts(40),
        IsolationLevel::Snapshot,
    );
    let _ = arc_storage.create_vertex(
        &setup_txn,
        OlapVertex {
            vid: 1,
            properties: PropertyRecord::default(),
            block_offset: 0,
        },
    );
    let eid = arc_storage
        .create_edge_in_txn(
            &setup_txn,
            OlapEdge {
                label_id: NonZeroU32::new(400),
                src_id: 1,
                dst_id: 10,
                properties: OlapPropertyStore::new(vec![Some(ScalarValue::Int32(Some(10)))]),
            },
        )
        .unwrap();
    setup_txn
        .commit_at(None)
        .expect("Setup commit should succeed");

    let barrier = Arc::new(Barrier::new(2));

    let storage_commit = arc_storage.clone();
    let barrier_commit = barrier.clone();
    let handle_commit = thread::spawn(move || {
        let txn = MemTransaction::new(
            storage_commit.clone(),
            Timestamp::with_ts(Timestamp::TXN_ID_START + 41),
            Timestamp::with_ts(41),
            IsolationLevel::Snapshot,
        );
        barrier_commit.wait();
        let _ = storage_commit.set_edge_property_in_txn(
            &txn,
            eid,
            vec![0],
            vec![ScalarValue::Int32(Some(1111))],
        );
        txn.commit_at(None).expect("Commit txn should succeed");
    });

    let storage_abort = arc_storage.clone();
    let barrier_abort = barrier.clone();
    let handle_abort = thread::spawn(move || {
        let txn = MemTransaction::new(
            storage_abort.clone(),
            Timestamp::with_ts(Timestamp::TXN_ID_START + 42),
            Timestamp::with_ts(42),
            IsolationLevel::Snapshot,
        );
        barrier_abort.wait();
        let _ = storage_abort.set_edge_property_in_txn(
            &txn,
            eid,
            vec![0],
            vec![ScalarValue::Int32(Some(2222))],
        );
        txn.abort().expect("Abort txn should succeed");
    });

    handle_commit.join().expect("Commit thread should finish");
    handle_abort.join().expect("Abort thread should finish");

    let read_txn = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 43),
        Timestamp::max_commit_ts(),
        IsolationLevel::Snapshot,
    );
    let edge = arc_storage.get_edge_at_ts(&read_txn, eid).unwrap();
    let props = edge.unwrap().properties;
    assert_eq!(
        props.get(0),
        Some(ScalarValue::Int32(Some(1111))),
        "Committed value should win over aborted update"
    );
}

#[test]
fn test_abort_create_edge_keeps_property_alignment() {
    let storage = make_storage();
    let arc_storage = Arc::new(storage);

    let setup_txn = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 50),
        Timestamp::with_ts(50),
        IsolationLevel::Snapshot,
    );
    let _ = arc_storage.create_vertex(
        &setup_txn,
        OlapVertex {
            vid: 1,
            properties: PropertyRecord::default(),
            block_offset: 0,
        },
    );
    setup_txn
        .commit_at(None)
        .expect("Setup commit should succeed");

    let base_txn = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 51),
        Timestamp::with_ts(51),
        IsolationLevel::Snapshot,
    );

    let eid10 = arc_storage
        .create_edge_in_txn(
            &base_txn,
            OlapEdge {
                label_id: NonZeroU32::new(600),
                src_id: 1,
                dst_id: 10,
                properties: OlapPropertyStore::new(vec![Some(ScalarValue::Int32(Some(10)))]),
            },
        )
        .unwrap();
    let eid20 = arc_storage
        .create_edge_in_txn(
            &base_txn,
            OlapEdge {
                label_id: NonZeroU32::new(601),
                src_id: 1,
                dst_id: 20,
                properties: OlapPropertyStore::new(vec![Some(ScalarValue::Int32(Some(20)))]),
            },
        )
        .unwrap();
    let eid30 = arc_storage
        .create_edge_in_txn(
            &base_txn,
            OlapEdge {
                label_id: NonZeroU32::new(602),
                src_id: 1,
                dst_id: 30,
                properties: OlapPropertyStore::new(vec![Some(ScalarValue::Int32(Some(30)))]),
            },
        )
        .unwrap();
    base_txn
        .commit_at(None)
        .expect("Base commit should succeed");

    let abort_txn = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 52),
        Timestamp::with_ts(52),
        IsolationLevel::Snapshot,
    );
    let aborted_eid = arc_storage
        .create_edge_in_txn(
            &abort_txn,
            OlapEdge {
                label_id: NonZeroU32::new(603),
                src_id: 1,
                dst_id: 15,
                properties: OlapPropertyStore::new(vec![Some(ScalarValue::Int32(Some(999)))]),
            },
        )
        .unwrap();
    abort_txn.abort().expect("Abort should succeed");

    assert!(
        arc_storage.edge_id_map.get(&aborted_eid).is_none(),
        "Aborted edge should not remain in edge_id_map"
    );

    let read_txn = MemTransaction::new(
        arc_storage.clone(),
        Timestamp::with_ts(Timestamp::TXN_ID_START + 53),
        Timestamp::max_commit_ts(),
        IsolationLevel::Snapshot,
    );

    let edge10 = arc_storage
        .get_edge_at_ts(&read_txn, eid10)
        .unwrap()
        .unwrap();
    let edge20 = arc_storage
        .get_edge_at_ts(&read_txn, eid20)
        .unwrap()
        .unwrap();
    let edge30 = arc_storage
        .get_edge_at_ts(&read_txn, eid30)
        .unwrap()
        .unwrap();

    assert_eq!(edge10.properties.get(0), Some(ScalarValue::Int32(Some(10))));
    assert_eq!(edge20.properties.get(0), Some(ScalarValue::Int32(Some(20))));
    assert_eq!(edge30.properties.get(0), Some(ScalarValue::Int32(Some(30))));

    let locations = [
        (eid10, ScalarValue::Int32(Some(10))),
        (eid20, ScalarValue::Int32(Some(20))),
        (eid30, ScalarValue::Int32(Some(30))),
    ]
    .into_iter()
    .map(|(eid, expected)| {
        let (block_idx, offset) = *arc_storage.edge_id_map.get(&eid).unwrap().value();
        (block_idx, offset, expected)
    })
    .collect::<Vec<_>>();

    let property_columns = arc_storage.property_columns.read().unwrap();
    let prop_col = &property_columns[0];
    for (block_idx, offset, expected) in locations {
        let block = prop_col.blocks.get(block_idx).unwrap();
        let last = block.values[offset].last().unwrap();
        assert_eq!(last.value, Some(expected));
    }
}
