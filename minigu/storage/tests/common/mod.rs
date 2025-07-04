use std::fs;
use std::sync::Arc;

use minigu_common::types::{EdgeId, LabelId, VertexId};
use minigu_common::value::ScalarValue;
use minigu_storage::model::edge::Edge;
use minigu_storage::model::properties::PropertyRecord;
use minigu_storage::model::vertex::Vertex;
use minigu_storage::tp::checkpoint::CheckpointManagerConfig;
use minigu_storage::wal::graph_wal::WalManagerConfig;
use minigu_storage::{IsolationLevel, MemoryGraph};

pub const PERSON_LABEL_ID: LabelId = LabelId::new(1).unwrap();
pub const FRIEND_LABEL_ID: LabelId = LabelId::new(1).unwrap();
pub const FOLLOW_LABEL_ID: LabelId = LabelId::new(2).unwrap();

pub struct TestCleaner {
    wal_path: std::path::PathBuf,
    checkpoint_dir: std::path::PathBuf,
}

impl TestCleaner {
    pub fn new(checkpoint_config: &CheckpointManagerConfig, wal_config: &WalManagerConfig) -> Self {
        Self {
            wal_path: wal_config.wal_path.clone(),
            checkpoint_dir: checkpoint_config.checkpoint_dir.clone(),
        }
    }
}

impl Drop for TestCleaner {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.wal_path);
        let _ = fs::remove_dir_all(&self.checkpoint_dir);
    }
}

pub fn create_test_checkpoint_config() -> CheckpointManagerConfig {
    let dir = std::env::temp_dir().join(format!(
        "test_isolation_checkpoint_{}_{}",
        chrono::Utc::now().timestamp(),
        rand::random::<u32>()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    CheckpointManagerConfig {
        checkpoint_dir: dir,
        max_checkpoints: 3,
        auto_checkpoint_interval_secs: 0,
        checkpoint_prefix: "test_isolation".to_string(),
        transaction_timeout_secs: 10,
    }
}

pub fn create_test_wal_config() -> WalManagerConfig {
    let file_name = format!(
        "test_isolation_wal_{}_{}.log",
        chrono::Utc::now().timestamp(),
        rand::random::<u32>()
    );
    let path = std::env::temp_dir().join(file_name);
    WalManagerConfig { wal_path: path }
}

pub fn create_empty_graph() -> (Arc<MemoryGraph>, TestCleaner) {
    let checkpoint_config = create_test_checkpoint_config();
    let wal_config = create_test_wal_config();
    let cleaner = TestCleaner::new(&checkpoint_config, &wal_config);
    let graph = MemoryGraph::with_config_fresh(checkpoint_config, wal_config);
    (graph, cleaner)
}

#[allow(dead_code)]
pub fn create_test_graph() -> (Arc<MemoryGraph>, TestCleaner) {
    let (graph, cleaner) = create_empty_graph();

    // Initialize some test data
    let txn = graph.begin_transaction(IsolationLevel::Serializable);

    let alice = Vertex::new(
        1,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            ScalarValue::String(Some("Alice".to_string())),
            ScalarValue::Int32(Some(25)),
        ]),
    );

    let bob = Vertex::new(
        2,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            ScalarValue::String(Some("Bob".to_string())),
            ScalarValue::Int32(Some(30)),
        ]),
    );

    graph.create_vertex(&txn, alice).unwrap();
    graph.create_vertex(&txn, bob).unwrap();

    let friend_edge = Edge::new(
        1,
        1,
        2,
        FRIEND_LABEL_ID,
        PropertyRecord::new(vec![ScalarValue::String(Some("2024-01-01".to_string()))]),
    );

    graph.create_edge(&txn, friend_edge).unwrap();
    txn.commit().unwrap();

    (graph, cleaner)
}

#[allow(dead_code)]
pub fn create_test_vertex(id: VertexId, name: &str, age: i32) -> Vertex {
    Vertex::new(
        id,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            ScalarValue::String(Some(name.to_string())),
            ScalarValue::Int32(Some(age)),
        ]),
    )
}

#[allow(dead_code)]
pub fn create_test_edge(id: EdgeId, from: VertexId, to: VertexId, relation: LabelId) -> Edge {
    Edge::new(
        id,
        from,
        to,
        relation,
        PropertyRecord::new(vec![ScalarValue::String(Some("2024-01-01".to_string()))]),
    )
}
