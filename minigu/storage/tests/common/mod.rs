use std::sync::Arc;

use minigu_common::types::{EdgeId, LabelId, VertexId};
use minigu_common::value::ScalarValue;
use minigu_storage::model::edge::Edge;
use minigu_storage::model::properties::PropertyRecord;
use minigu_storage::model::vertex::Vertex;
use minigu_storage::tp::MemoryGraph;
// use minigu_storage::tp::checkpoint::CheckpointManagerConfig;
// use minigu_storage::wal::graph_wal::WalManagerConfig;
use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

pub const PERSON_LABEL_ID: LabelId = LabelId::new(1).unwrap();
pub const FRIEND_LABEL_ID: LabelId = LabelId::new(1).unwrap();
pub const FOLLOW_LABEL_ID: LabelId = LabelId::new(2).unwrap();

pub fn create_empty_graph() -> Arc<MemoryGraph> {
    MemoryGraph::in_memory()
}

#[allow(dead_code)]
pub fn create_test_graph() -> Arc<MemoryGraph> {
    let graph = create_empty_graph();

    // Initialize some test data
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)
        .unwrap();

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

    graph
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
