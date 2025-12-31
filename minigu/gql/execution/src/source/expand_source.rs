use std::sync::Arc;

use arrow::array::{ArrayRef, UInt64Array};
use minigu_common::types::{EdgeId, LabelId, VertexId, VertexIdArray};
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_storage::common::model::edge::Neighbor;
use minigu_storage::iterators::AdjacencyIteratorTrait;
use minigu_storage::tp::transaction::IsolationLevel;
use minigu_transaction::GraphTxnManager;

use super::ExpandSource;
use crate::error::ExecutionResult;

/// An iterator that yields batches of neighbor arrays for a vertex.
pub struct GraphExpandIter {
    neighbors: Vec<Neighbor>,
    offset: usize,
    batch_size: usize,
    _graph_storage: GraphStorage,
    _txn: Arc<minigu_storage::tp::transaction::MemTransaction>,
}

impl Iterator for GraphExpandIter {
    type Item = ExecutionResult<Vec<ArrayRef>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.neighbors.len() {
            return None;
        }

        let end = (self.offset + self.batch_size).min(self.neighbors.len());
        let batch = &self.neighbors[self.offset..end];

        // Collect edge IDs and neighbor IDs from the batch
        let edge_ids: Vec<EdgeId> = batch.iter().map(|n| n.eid()).collect();
        let neighbor_ids: Vec<VertexId> = batch.iter().map(|n| n.neighbor_id()).collect();

        // Create arrays for edge IDs and neighbor IDs
        let edge_id_array = UInt64Array::from_iter_values(edge_ids.iter().copied());
        let neighbor_array = VertexIdArray::from_iter_values(neighbor_ids.iter().copied());

        self.offset = end;

        // Return two columns: edge IDs and target vertex IDs
        Some(Ok(vec![Arc::new(edge_id_array), Arc::new(neighbor_array)]))
    }
}

impl ExpandSource for GraphContainer {
    type ExpandIter = GraphExpandIter;

    fn expand_from_vertex(
        &self,
        vertex: VertexId,
        edge_labels: Option<Vec<Vec<LabelId>>>,
        target_vertex_labels: Option<Vec<Vec<LabelId>>>,
    ) -> Option<Self::ExpandIter> {
        let mem = match self.graph_storage() {
            GraphStorage::Memory(m) => Arc::clone(m),
        };

        let txn = match mem
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
        {
            Ok(txn) => txn,
            Err(_) => return None,
        };

        // Check if vertex exists
        if mem.get_vertex(&txn, vertex).is_err() {
            return None;
        }

        // Use the transaction's adjacency iterator to get all visible outgoing neighbors
        let mut neighbors = Vec::new();
        let mut adj_iter = txn.iter_adjacency_outgoing(vertex);

        // Filter by edge labels
        if let Some(labels) = &edge_labels {
            use std::collections::HashSet;
            let allowed_labels: HashSet<LabelId> = labels.iter().flatten().copied().collect();
            adj_iter = AdjacencyIteratorTrait::filter(adj_iter, move |neighbor| {
                allowed_labels.contains(&neighbor.label_id())
            });
        }

        // Filter by target vertex labels
        for neighbor_result in adj_iter {
            match neighbor_result {
                Ok(neighbor) => {
                    // If target vertex labels are specified, check if the neighbor matches
                    if let Some(target_labels) = &target_vertex_labels {
                        match mem.get_vertex(&txn, neighbor.neighbor_id()) {
                            Ok(neighbor_vertex) => {
                                // Check if neighbor vertex label matches target labels
                                let neighbor_label = neighbor_vertex.label_id;
                                let mut matches = false;
                                for and_labels in target_labels {
                                    if and_labels.is_empty() {
                                        matches = true;
                                        break;
                                    }
                                    if and_labels.contains(&neighbor_label) {
                                        matches = true;
                                        break;
                                    }
                                }
                                if !matches {
                                    continue; // Skip neighbors that don't match target labels
                                }
                            }
                            Err(_) => {
                                // If we can't get the vertex, skip it
                                continue;
                            }
                        }
                    }
                    neighbors.push(neighbor);
                }
                Err(_) => {
                    // If there's an error iterating neighbors, we can skip it
                    continue;
                }
            }
        }

        // TODO:Should add GlobalConfig to determine the batch_size;
        Some(GraphExpandIter {
            neighbors,
            offset: 0,
            batch_size: 64,
            _graph_storage: match self.graph_storage() {
                GraphStorage::Memory(m) => GraphStorage::Memory(Arc::clone(m)),
            },
            _txn: txn,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use arrow::array::AsArray;
    use minigu_catalog::label_set::LabelSet;
    use minigu_catalog::memory::graph_type::{
        MemoryEdgeTypeCatalog, MemoryGraphTypeCatalog, MemoryVertexTypeCatalog,
    };
    use minigu_catalog::property::Property;
    use minigu_common::data_type::LogicalType;
    use minigu_common::types::LabelId;
    use minigu_common::value::ScalarValue;
    use minigu_context::graph::{GraphContainer, GraphStorage};
    use minigu_storage::common::{Edge, PropertyRecord, Vertex};
    use minigu_storage::tp::MemoryGraph;
    use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

    use super::*;

    fn create_test_graph() -> GraphContainer {
        let graph = MemoryGraph::in_memory();
        let mut graph_type = MemoryGraphTypeCatalog::new();

        // Add labels
        let person_label_id = graph_type.add_label("PERSON".to_string()).unwrap();
        let friend_label_id = graph_type.add_label("FRIEND".to_string()).unwrap();

        // Create vertex type
        let person_label_set: LabelSet = vec![person_label_id].into_iter().collect();
        let person = Arc::new(MemoryVertexTypeCatalog::new(
            person_label_set.clone(),
            vec![
                Property::new("name".to_string(), LogicalType::String, false),
                Property::new("age".to_string(), LogicalType::Int8, false),
            ],
        ));

        // Create edge type
        let friend_label_set: LabelSet = vec![friend_label_id].into_iter().collect();
        let friend = Arc::new(MemoryEdgeTypeCatalog::new(
            friend_label_set.clone(),
            person.clone(),
            person.clone(),
            vec![Property::new(
                "distance".to_string(),
                LogicalType::Int32,
                false,
            )],
        ));

        graph_type.add_vertex_type(person_label_set, person);
        graph_type.add_edge_type(friend_label_set, friend);

        GraphContainer::new(Arc::new(graph_type), GraphStorage::Memory(graph))
    }

    fn setup_test_data(container: &GraphContainer) {
        let mem = match container.graph_storage() {
            GraphStorage::Memory(m) => Arc::clone(m),
        };

        let txn = mem
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        // Create vertices: 1, 2, 3, 4
        let person_label_id = LabelId::new(1).unwrap();

        let v1 = Vertex::new(
            1,
            person_label_id,
            PropertyRecord::new(vec![
                ScalarValue::String(Some("Alice".to_string())),
                ScalarValue::Int8(Some(25)),
            ]),
        );

        let v2 = Vertex::new(
            2,
            person_label_id,
            PropertyRecord::new(vec![
                ScalarValue::String(Some("Bob".to_string())),
                ScalarValue::Int8(Some(30)),
            ]),
        );

        let v3 = Vertex::new(
            3,
            person_label_id,
            PropertyRecord::new(vec![
                ScalarValue::String(Some("Carol".to_string())),
                ScalarValue::Int8(Some(28)),
            ]),
        );

        let v4 = Vertex::new(
            4,
            person_label_id,
            PropertyRecord::new(vec![
                ScalarValue::String(Some("David".to_string())),
                ScalarValue::Int8(Some(32)),
            ]),
        );

        mem.create_vertex(&txn, v1).unwrap();
        mem.create_vertex(&txn, v2).unwrap();
        mem.create_vertex(&txn, v3).unwrap();
        mem.create_vertex(&txn, v4).unwrap();

        // Create edges:
        // 1 -> 2 (FRIEND)
        // 1 -> 3 (FRIEND)
        // 2 -> 3(FRIEND)
        // (vertex 4 has no outgoing edges)
        let friend_label_id = LabelId::new(1).unwrap();

        let e1 = Edge::new(
            1,
            1,
            2,
            friend_label_id,
            PropertyRecord::new(vec![ScalarValue::Int32(Some(10))]),
        );

        let e2 = Edge::new(
            2,
            1,
            3,
            friend_label_id,
            PropertyRecord::new(vec![ScalarValue::Int32(Some(20))]),
        );

        let e3 = Edge::new(
            3,
            2,
            3,
            friend_label_id,
            PropertyRecord::new(vec![ScalarValue::Int32(Some(15))]),
        );

        mem.create_edge(&txn, e1).unwrap();
        mem.create_edge(&txn, e2).unwrap();
        mem.create_edge(&txn, e3).unwrap();

        txn.commit().unwrap();
    }

    #[test]
    fn test_expand_from_nonexistent_vertex() {
        let container = create_test_graph();
        setup_test_data(&container);

        // Try to expand from a non-existent vertex
        let result = container.expand_from_vertex(999, None, None);
        assert!(
            result.is_none(),
            "Should return None for non-existent vertex"
        );
    }

    #[test]
    fn test_expand_from_vertex_with_no_neighbors() {
        let container = create_test_graph();
        setup_test_data(&container);

        // Vertex 4 has no outgoing edges
        let result = container.expand_from_vertex(4, None, None);
        assert!(result.is_some(), "Should return Some for existing vertex");

        let mut iter = result.unwrap();
        // Should return an empty iterator
        assert!(iter.next().is_none(), "Should have no neighbors");
    }

    #[test]
    fn test_expand_from_vertex_with_neighbors() {
        let container = create_test_graph();
        setup_test_data(&container);

        // Vertex 1 has neighbors: 2, 3
        let result = container.expand_from_vertex(1, None, None);
        assert!(result.is_some(), "Should return Some for existing vertex");

        let mut iter = result.unwrap();
        let first_batch = iter.next();
        assert!(first_batch.is_some(), "Should have at least one batch");

        let batch = first_batch.unwrap().unwrap();
        assert_eq!(
            batch.len(),
            2,
            "Should have two columns (edge IDs and neighbor IDs)"
        );

        let edge_id_array: &UInt64Array = batch[0].as_primitive();
        let neighbor_array: &VertexIdArray = batch[1].as_primitive();
        assert_eq!(edge_id_array.len(), 2, "Should have 2 edge IDs");
        assert_eq!(neighbor_array.len(), 2, "Should have 2 neighbors");

        let neighbors: Vec<u64> = neighbor_array.values().iter().copied().collect();
        assert!(neighbors.contains(&2), "Should contain neighbor 2");
        assert!(neighbors.contains(&3), "Should contain neighbor 3");

        // Should be done after one batch
        assert!(iter.next().is_none(), "Should have no more batches");
    }

    #[test]
    fn test_expand_from_vertex_with_single_neighbor() {
        let container = create_test_graph();
        setup_test_data(&container);

        // Vertex 2 has one neighbor: 3
        let result = container.expand_from_vertex(2, None, None);
        assert!(result.is_some(), "Should return Some for existing vertex");

        let mut iter = result.unwrap();
        let first_batch = iter.next();
        assert!(first_batch.is_some(), "Should have at least one batch");

        let batch = first_batch.unwrap().unwrap();
        assert_eq!(
            batch.len(),
            2,
            "Should have two columns (edge IDs and neighbor IDs)"
        );
        let edge_id_array: &UInt64Array = batch[0].as_primitive();
        let neighbor_array: &VertexIdArray = batch[1].as_primitive();
        assert_eq!(edge_id_array.len(), 1, "Should have 1 edge ID");
        assert_eq!(neighbor_array.len(), 1, "Should have 1 neighbor");
        assert_eq!(neighbor_array.value(0), 3, "Should be neighbor 3");

        // Should be done after one batch
        assert!(iter.next().is_none(), "Should have no more batches");
    }

    #[test]
    fn test_expand_batching() {
        let container = create_test_graph();
        setup_test_data(&container);

        // Create a vertex with many neighbors to test batching
        let mem = match container.graph_storage() {
            GraphStorage::Memory(m) => Arc::clone(m),
        };

        let txn = mem
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        // Create vertex 5
        let person_label_id = LabelId::new(1).unwrap();
        let v5 = Vertex::new(
            5,
            person_label_id,
            PropertyRecord::new(vec![
                ScalarValue::String(Some("Eve".to_string())),
                ScalarValue::Int8(Some(27)),
            ]),
        );
        mem.create_vertex(&txn, v5).unwrap();

        // Create 100 edges from vertex 5 to vertices 1-100
        // (We'll create vertices 6-100 first)
        let friend_label_id = LabelId::new(1).unwrap();
        for i in 6..=100 {
            let v = Vertex::new(
                i,
                person_label_id,
                PropertyRecord::new(vec![
                    ScalarValue::String(Some(format!("Person{}", i))),
                    ScalarValue::Int8(Some(20)),
                ]),
            );
            mem.create_vertex(&txn, v).unwrap();

            let edge = Edge::new(
                i,
                5,
                i,
                friend_label_id,
                PropertyRecord::new(vec![ScalarValue::Int32(Some(1))]),
            );
            mem.create_edge(&txn, edge).unwrap();
        }

        txn.commit().unwrap();

        // Now expand from vertex 5
        let result = container.expand_from_vertex(5, None, None);
        assert!(result.is_some(), "Should return Some for existing vertex");

        let iter = result.unwrap();
        let mut total_neighbors = 0;
        let mut batch_count = 0;

        for batch_result in iter {
            batch_count += 1;
            let batch = batch_result.unwrap();
            let neighbor_array: &VertexIdArray = batch[1].as_primitive(); // Second column is neighbor IDs
            total_neighbors += neighbor_array.len();
        }

        assert_eq!(total_neighbors, 95, "Should have 95 neighbors (6-100)");
        // With batch_size=64, we should have 2 batches: 64 + 31
        assert!(batch_count >= 1, "Should have at least one batch");
        assert!(batch_count <= 2, "Should have at most 2 batches");
    }
}
