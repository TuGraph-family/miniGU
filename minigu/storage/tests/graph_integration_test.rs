use std::sync::Arc;
use std::{env, fs};

use minigu_common::datatype::types::{EdgeId, LabelId, VertexId};
use minigu_common::datatype::value::PropertyValue;
use minigu_storage::memory::checkpoint::CheckpointManagerConfig;
use minigu_storage::model::edge::Edge;
use minigu_storage::model::properties::PropertyRecord;
use minigu_storage::model::vertex::Vertex;
use minigu_storage::wal::graph_wal::WalManagerConfig;
use minigu_storage::{
    Graph, IsolationLevel, MemoryGraph, MutGraph, StorageResult, StorageTransaction,
};

const PERSON_LABEL_ID: LabelId = 0;
const FRIEND_LABEL_ID: LabelId = 1;
const FOLLOW_LABEL_ID: LabelId = 2;

fn create_test_vertex(id: VertexId, name: &str, age: i32) -> Vertex {
    Vertex::new(
        id,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            PropertyValue::String(name.into()),
            PropertyValue::Int(age),
        ]),
    )
}

fn create_test_edge(id: EdgeId, from: VertexId, to: VertexId, relation: LabelId) -> Edge {
    Edge::new(
        id,
        from,
        to,
        relation,
        PropertyRecord::new(vec![PropertyValue::String("2024-01-01".into())]),
    )
}

fn mock_checkpoint_config() -> CheckpointManagerConfig {
    let dir = env::temp_dir().join(format!(
        "test_checkpoint_{}_{}",
        chrono::Utc::now(),
        rand::random::<u32>()
    ));
    fs::create_dir_all(&dir).unwrap();
    CheckpointManagerConfig {
        checkpoint_dir: dir,
        max_checkpoints: 3,
        auto_checkpoint_interval_secs: 0, // Disable auto checkpoints for testing
        checkpoint_prefix: "test_checkpoint".to_string(),
        transaction_timeout_secs: 10,
    }
}

fn mock_wal_config() -> WalManagerConfig {
    let file_name = format!(
        "test_wal_{}_{}.log",
        chrono::Utc::now(),
        rand::random::<u32>()
    );
    let path = env::temp_dir().join(file_name);
    WalManagerConfig { wal_path: path }
}

pub struct Cleaner {
    wal_path: std::path::PathBuf,
    checkpoint_dir: std::path::PathBuf,
}

impl Cleaner {
    pub fn new(checkpoint_config: &CheckpointManagerConfig, wal_config: &WalManagerConfig) -> Self {
        Self {
            wal_path: wal_config.wal_path.clone(),
            checkpoint_dir: checkpoint_config.checkpoint_dir.clone(),
        }
    }
}

impl Drop for Cleaner {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.wal_path);
        let _ = fs::remove_dir_all(&self.checkpoint_dir);
    }
}

fn mock_empty_graph() -> (Arc<MemoryGraph>, Cleaner) {
    let checkpoint_config = mock_checkpoint_config();
    let wal_config = mock_wal_config();
    let cleaner = Cleaner::new(&checkpoint_config, &wal_config);
    let graph = MemoryGraph::with_config_fresh(checkpoint_config, wal_config);
    (graph, cleaner)
}

#[test]
fn test_graph_basic_operations() -> StorageResult<()> {
    // 1. Create MemGraph
    let (graph, _cleaner) = mock_empty_graph();

    // 2. Open transaction
    let txn = graph.begin_transaction(IsolationLevel::Serializable);

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
            let name = v.properties()[0].as_string().unwrap();
            name == "Alice" || name == "Bob" || name == "Carol" || name == "Dave"
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
    let verify_txn = graph.begin_transaction(IsolationLevel::Serializable);

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
    let delete_txn = graph.begin_transaction(IsolationLevel::Serializable);
    graph.delete_vertex(&delete_txn, alice_id)?;
    graph.delete_edge(&delete_txn, another_follow_edge_id)?;
    delete_txn.commit()?;

    // 12. Open new transaction and verify data
    let verify_txn = graph.begin_transaction(IsolationLevel::Serializable);

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
            .filter(|v| {
                let name = v.properties()[0].as_string().unwrap();
                name == "Alice" || name == "Bob" || name == "Carol" || name == "Dave"
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
        let txn = graph.begin_transaction(IsolationLevel::Serializable);
        txn.commit()?;
    }

    Ok(())
}
