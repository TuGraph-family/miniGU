// use std::collections::HashSet;

// use minigu_common::types::{LabelId, VertexId, PropertyId};
// use minigu_common::value::{F32, ScalarValue};
// use minigu_storage::error::StorageResult;
// use minigu_storage::model::properties::PropertyRecord;
// use minigu_storage::model::vertex::Vertex;
// use minigu_storage::tp::IsolationLevel;

// use crate::common::*;

// // Test constants
// const MOVIE_LABEL_ID: LabelId = LabelId::new(10).unwrap();
// const USER_LABEL_ID: LabelId = LabelId::new(11).unwrap();

// // PropertyId constants for testing
// const NAME_PROPERTY_ID: PropertyId = 0;      // First property: name
// const EMBEDDING_PROPERTY_ID: PropertyId = 1; // Second property: embedding 
// const PLOT_VECTOR_PROPERTY_ID: PropertyId = 2; // Third property: plot_vector

// /// Create a test vertex with vector properties
// fn create_test_vertex_with_vector(
//     id: VertexId, 
//     name: &str, 
//     embedding: Vec<f32>,
//     plot_vector: Option<Vec<f32>>
// ) -> Vertex {
//     let mut properties = vec![
//         ScalarValue::String(Some(name.to_string())),
//         ScalarValue::Vector(Some(embedding.into_iter().map(F32::from).collect())),
//     ];
    
//     if let Some(plot_vec) = plot_vector {
//         properties.push(ScalarValue::Vector(Some(plot_vec.into_iter().map(F32::from).collect())));
//     }
    
//     Vertex::new(id, MOVIE_LABEL_ID, PropertyRecord::new(properties))
// }

// /// Create test data with mixed vector dimensions
// fn create_movies_with_embeddings() -> Vec<(String, Vec<f32>, Option<Vec<f32>>)> {
//     vec![
//         // Movie name, embedding (3D), plot_vector (2D)
//         ("Inception".to_string(), vec![0.1, 0.2, 0.3], Some(vec![0.8, 0.9])),
//         ("Matrix".to_string(), vec![0.4, 0.5, 0.6], Some(vec![0.7, 0.8])),
//         ("Interstellar".to_string(), vec![0.2, 0.1, 0.4], Some(vec![0.9, 0.6])),
//         ("Memento".to_string(), vec![0.3, 0.4, 0.1], Some(vec![0.5, 0.7])),
//         ("Prestige".to_string(), vec![0.6, 0.3, 0.2], None), // No plot vector
//     ]
// }

// #[test]
// fn test_build_vector_index_for_single_property() -> StorageResult<()> {
//     // Arrange
//     let (graph, _cleaner) = create_empty_graph();
//     let txn = graph.begin_transaction(IsolationLevel::Serializable);
    
//     // Create test vertices with embedding vectors
//     let movies = create_movies_with_embeddings();
//     for (i, (name, embedding, _)) in movies.iter().enumerate() {
//         let vertex = create_test_vertex_with_vector((i + 1) as u64, name, embedding.clone(), None);
//         graph.create_vertex(&txn, vertex)?;
//     }
    
//     txn.commit()?;
    
//     // Act - Build vector index for embedding property (PropertyId 1)
//     let index_txn = graph.begin_transaction(IsolationLevel::Snapshot);
//     let result = graph.build_vector_index(
//         &index_txn,
//         EMBEDDING_PROPERTY_ID,
//         3, // 3D vectors
//         BuildParams::default()
//     );
    
//     // Assert
//     assert!(result.is_ok(), "Failed to build vector index: {:?}", result.err());
    
//     // Verify index exists
//     let vector_index = graph.get_vector_index(EMBEDDING_PROPERTY_ID);
//     assert!(vector_index.is_some(), "Vector index should exist after building");
    
//     let index = vector_index.unwrap();
//     assert_eq!(index.get_dimension(), 3, "Index dimension should match");
//     assert_eq!(index.size(), 4, "Index should contain 4 vectors"); // Prestige has no embedding
    
//     Ok(())
// }

// #[test]
// fn test_build_multiple_vector_indices() -> StorageResult<()> {
//     // Arrange
//     let (graph, _cleaner) = create_empty_graph();
//     let txn = graph.begin_transaction(IsolationLevel::Serializable);
    
//     // Create test vertices with both embedding and plot vectors
//     let movies = create_movies_with_embeddings();
//     for (i, (name, embedding, plot_vector)) in movies.iter().enumerate() {
//         let vertex = create_test_vertex_with_vector(
//             (i + 1) as u64, 
//             name, 
//             embedding.clone(), 
//             plot_vector.clone()
//         );
//         graph.create_vertex(&txn, vertex)?;
//     }
    
//     txn.commit()?;
    
//     // Act - Build indices for both properties
//     let index_txn = graph.begin_transaction(IsolationLevel::Snapshot);
    
//     // Build embedding index (3D)
//     graph.build_vector_index(&index_txn, EMBEDDING_PROPERTY_ID, 3, BuildParams::default())?;
    
//     // Build plot_vector index (2D)  
//     graph.build_vector_index(&index_txn, PLOT_VECTOR_PROPERTY_ID, 2, BuildParams::default())?;
    
//     // Assert
//     let embedding_index = graph.get_vector_index(EMBEDDING_PROPERTY_ID);
//     let plot_index = graph.get_vector_index(PLOT_VECTOR_PROPERTY_ID);
    
//     assert!(embedding_index.is_some(), "Embedding index should exist");
//     assert!(plot_index.is_some(), "Plot vector index should exist");
    
//     assert_eq!(embedding_index.as_ref().unwrap().get_dimension(), 3);
//     assert_eq!(plot_index.as_ref().unwrap().get_dimension(), 2);
//     assert_eq!(embedding_index.as_ref().unwrap().size(), 4);
//     assert_eq!(plot_index.as_ref().unwrap().size(), 4);

//     Ok(())
// }

// #[test]
// fn test_vector_search_basic() -> StorageResult<()> {
//     // Arrange
//     let (graph, _cleaner) = create_empty_graph();
//     let txn = graph.begin_transaction(IsolationLevel::Serializable);
    
//     let movies = create_movies_with_embeddings();
//     for (i, (name, embedding, _)) in movies.iter().enumerate() {
//         let vertex = create_test_vertex_with_vector((i + 1) as u64, name, embedding.clone(), None);
//         graph.create_vertex(&txn, vertex)?;
//     }
    
//     txn.commit()?;
    
//     // Build index
//     let index_txn = graph.begin_transaction(IsolationLevel::Snapshot);
//     graph.build_vector_index(&index_txn, EMBEDDING_PROPERTY_ID, 3, BuildParams::default())?;
    
//     // Act - Search for similar vectors
//     let query_vector = vec![0.1, 0.2, 0.3]; // Similar to "Inception"
//     let search_results = graph.vector_search(
//         EMBEDDING_PROPERTY_ID,
//         &query_vector,
//         2, // Top 2 results
//         SearchParams::default(),
//         None // No filter
//     )?;
    
//     // Assert
//     assert!(!search_results.is_empty(), "Search should return results");
//     assert!(search_results.len() <= 2, "Should return at most 2 results");
    
//     // First result should be the most similar (likely Inception with id=1)
//     assert!(search_results.contains(&1), "Should find vertex 1 (Inception)");
    
//     Ok(())
// }

// #[test]
// fn test_vector_search_with_different_k_values() -> StorageResult<()> {
//     // Arrange
//     let (graph, _cleaner) = create_empty_graph();
//     let txn = graph.begin_transaction(IsolationLevel::Serializable);
    
//     let movies = create_movies_with_embeddings();
//     for (i, (name, embedding, _)) in movies.iter().enumerate() {
//         let vertex = create_test_vertex_with_vector((i + 1) as u64, name, embedding.clone(), None);
//         graph.create_vertex(&txn, vertex)?;
//     }
    
//     txn.commit()?;
    
//     // Build index
//     let index_txn = graph.begin_transaction(IsolationLevel::Snapshot);
//     graph.build_vector_index(&index_txn, EMBEDDING_PROPERTY_ID, 3, BuildParams::default())?;
    
//     let query_vector = vec![0.1, 0.2, 0.3];
    
//     // Test different k values
//     let k1_results = graph.vector_search(EMBEDDING_PROPERTY_ID, &query_vector, 1, SearchParams::default(), None)?;
//     let k3_results = graph.vector_search(EMBEDDING_PROPERTY_ID, &query_vector, 3, SearchParams::default(), None)?;
//     let k10_results = graph.vector_search(EMBEDDING_PROPERTY_ID, &query_vector, 10, SearchParams::default(), None)?;
    
//     // Assert
//     assert_eq!(k1_results.len(), 1);
//     assert_eq!(k3_results.len(), 3);
//     assert_eq!(k10_results.len(), 4); // Only 4 vectors available
    
//     Ok(())
// }

// #[test]
// fn test_vector_search_empty_property() -> StorageResult<()> {
//     // Arrange
//     let (graph, _cleaner) = create_empty_graph();
//     let txn = graph.begin_transaction(IsolationLevel::Serializable);
    
//     // Create vertex without vector property
//     let vertex = create_test_vertex(1, "NoVector", 25);
//     graph.create_vertex(&txn, vertex)?;
    
//     txn.commit()?;
    
//     // Act - Try to build index for non-existent property (PropertyId 99 doesn't exist)
//     let index_txn = graph.begin_transaction(IsolationLevel::Snapshot);
//     let result = graph.build_vector_index(&index_txn, 99, 3, BuildParams::default());
    
//     // Assert - Should handle empty vector set gracefully
//     // This could either succeed with an empty index or return a specific error
//     match result {
//         Ok(()) => {
//             let index = graph.get_vector_index(99);
//             if let Some(idx) = index {
//                 assert_eq!(idx.size(), 0, "Index should be empty");
//             }
//         }
//         Err(_) => {
//             // Also acceptable - building index on empty vector set might fail
//         }
//     }
    
//     Ok(())
// }

// #[test]
// fn test_vector_index_with_dimension_mismatch() -> StorageResult<()> {
//     // Arrange
//     let (graph, _cleaner) = create_empty_graph();
//     let txn = graph.begin_transaction(IsolationLevel::Serializable);
    
//     // Create vertices with different dimension vectors
//     let vertex1 = create_test_vertex_with_vector(1, "Movie1", vec![0.1, 0.2, 0.3], None); // 3D
//     let vertex2 = create_test_vertex_with_vector(2, "Movie2", vec![0.4, 0.5], None);       // 2D - Wrong!
    
//     graph.create_vertex(&txn, vertex1)?;
//     graph.create_vertex(&txn, vertex2)?;
    
//     txn.commit()?;
    
//     // Act - Try to build index expecting 3D vectors
//     let index_txn = graph.begin_transaction(IsolationLevel::Snapshot);
//     let result = graph.build_vector_index(&index_txn, EMBEDDING_PROPERTY_ID, 3, BuildParams::default());
    
//     // Assert - Should fail due to dimension mismatch
//     assert!(result.is_err(), "Building index with mismatched dimensions should fail");
    
//     Ok(())
// }

// #[test] 
// fn test_vector_index_id_mapping_consistency() -> StorageResult<()> {
//     // Arrange
//     let (graph, _cleaner) = create_empty_graph();
//     let txn = graph.begin_transaction(IsolationLevel::Serializable);
    
//     let movies = create_movies_with_embeddings();
//     let mut expected_node_ids = HashSet::new();
    
//     for (i, (name, embedding, _)) in movies.iter().enumerate() {
//         let node_id = (i + 1) as u64;
//         let vertex = create_test_vertex_with_vector(node_id, name, embedding.clone(), None);
//         graph.create_vertex(&txn, vertex)?;
//         expected_node_ids.insert(node_id as u32); // Vector search returns u32 node IDs
//     }
    
//     txn.commit()?;
    
//     // Build index
//     let index_txn = graph.begin_transaction(IsolationLevel::Snapshot);
//     graph.build_vector_index(&index_txn, EMBEDDING_PROPERTY_ID, 3, BuildParams::default())?;
    
//     // Act - Search and verify node IDs are correctly mapped
//     let query_vector = vec![0.0, 0.0, 0.0];
//     let search_results = graph.vector_search(
//         EMBEDDING_PROPERTY_ID, 
//         &query_vector, 
//         10, 
//         SearchParams::default(), 
//         None
//     )?;
    
//     // Assert - All returned IDs should be valid node IDs
//     for &node_id in &search_results {
//         assert!(expected_node_ids.contains(&node_id), 
//             "Returned node ID {} should exist in graph", node_id);
//     }
    
//     assert_eq!(search_results.len(), 4, "Should return all 4 vectors");
    
//     Ok(())
// }

// #[test]
// fn test_concurrent_vector_index_access() -> StorageResult<()> {
//     use std::sync::Arc;
//     use std::thread;
    
//     // Arrange
//     let (graph, _cleaner) = create_empty_graph();
//     let txn = graph.begin_transaction(IsolationLevel::Serializable);
    
//     // Create test data
//     let movies = create_movies_with_embeddings();
//     for (i, (name, embedding, _)) in movies.iter().enumerate() {
//         let vertex = create_test_vertex_with_vector((i + 1) as u64, name, embedding.clone(), None);
//         graph.create_vertex(&txn, vertex)?;
//     }
    
//     txn.commit()?;
    
//     // Build index
//     let index_txn = graph.begin_transaction(IsolationLevel::Snapshot);
//     graph.build_vector_index(&index_txn, EMBEDDING_PROPERTY_ID, 3, BuildParams::default())?;
    
//     // Act - Concurrent search operations
//     let graph_arc = Arc::clone(&graph);
//     let handles: Vec<_> = (0..4).map(|i| {
//         let graph_clone = Arc::clone(&graph_arc);
//         thread::spawn(move || {
//             let query = vec![i as f32 * 0.1, i as f32 * 0.2, i as f32 * 0.3];
//             graph_clone.vector_search(EMBEDDING_PROPERTY_ID, &query, 2, SearchParams::default(), None)
//         })
//     }).collect();
    
//     // Assert - All searches should complete successfully
//     for handle in handles {
//         let result = handle.join().unwrap();
//         assert!(result.is_ok(), "Concurrent search should succeed");
//         let search_results = result.unwrap();
//         assert!(!search_results.is_empty(), "Should return some results");
//     }
    
//     Ok(())
// }

// // TODO: Tests for filtering functionality (to be implemented)
// #[test]
// fn test_vector_search_with_pre_filtering() -> StorageResult<()> {
//     // This test will be implemented when filtering functionality is added
//     // For now, just ensure the basic structure compiles
//     Ok(())
// }

// #[test] 
// fn test_vector_search_with_post_filtering() -> StorageResult<()> {
//     // This test will be implemented when filtering functionality is added
//     // For now, just ensure the basic structure compiles
//     Ok(())
// }

// #[test]
// fn test_hybrid_graph_vector_query() -> StorageResult<()> {
//     // This test will simulate a hybrid query that combines graph traversal 
//     // with vector similarity search - to be implemented
//     Ok(())
// }