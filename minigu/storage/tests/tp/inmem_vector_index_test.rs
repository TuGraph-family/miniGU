//! Optimized vector index integration tests for miniGU storage layer
//!
//! This module tests the integration between miniGU's MemoryGraph and DiskANN-rs,
//! focusing on core functionality: index building, vector search, and ID mapping.

use diskann::model::IndexConfiguration;
use diskann::model::configuration::index_write_parameters::IndexWriteParametersBuilder;
use diskann::utils::round_up;
use minigu_common::types::{PropertyId, VertexId};
use minigu_common::value::{F32, ScalarValue};
use minigu_storage::error::StorageResult;
use minigu_storage::model::properties::PropertyRecord;
use minigu_storage::model::vertex::Vertex;
use minigu_storage::tp::IsolationLevel;
use vector::Metric;

use crate::common::*;

// Test constants
const _NAME_PROPERTY_ID: PropertyId = 0;
const EMBEDDING_PROPERTY_ID: PropertyId = 1;
const TEST_DIMENSION: usize = 104; // DiskANN dimension support 104, 128, 256

// ===== TEST UTILITIES =====

/// Creates a test vertex with vector embedding
fn create_vertex_with_vector(id: VertexId, name: &str, embedding: Vec<f32>) -> Vertex {
    let vector_value = ScalarValue::Vector(Some(embedding.into_iter().map(F32::from).collect()));

    Vertex::new(
        id,
        PERSON_LABEL_ID,
        PropertyRecord::new(vec![
            ScalarValue::String(Some(name.to_string())), // Property 0: name
            vector_value,                                // Property 1: embedding
        ]),
    )
}

/// Creates optimized test index configuration for small datasets
fn create_test_index_config(dimension: usize) -> IndexConfiguration {
    let write_params = IndexWriteParametersBuilder::new(20, 16) // Reduced for small datasets
        .with_alpha(1.2) // Keep user's preferred alpha
        .build();

    IndexConfiguration {
        index_write_parameter: write_params,
        dist_metric: Metric::L2,
        dim: dimension,
        aligned_dim: round_up(dimension, 8),
        max_points: 1000,
        num_frozen_pts: 0,
        use_pq_dist: false,
        num_pq_chunks: 0,
        use_opq: false,
        growth_potential: 1.2,
    }
}

/// Creates spatially close vectors to ensure graph connectivity for DiskANN
fn create_connected_test_vectors() -> Vec<(VertexId, String, Vec<f32>)> {
    vec![
        // Cluster 1: Around origin with small perturbations (ensures connectivity)
        {
            let mut vec = vec![0.0f32; TEST_DIMENSION];
            vec[0] = 0.1f32;
            vec[1] = 0.1f32;
            (1u64, "cluster1_a".to_string(), vec)
        },
        {
            let mut vec = vec![0.0f32; TEST_DIMENSION];
            vec[0] = 0.2f32;
            vec[1] = 0.1f32;
            (2u64, "cluster1_b".to_string(), vec)
        },
        {
            let mut vec = vec![0.0f32; TEST_DIMENSION];
            vec[0] = 0.1f32;
            vec[1] = 0.2f32;
            (3u64, "cluster1_c".to_string(), vec)
        },
        {
            let mut vec = vec![0.0f32; TEST_DIMENSION];
            vec[0] = 0.15f32;
            vec[1] = 0.15f32;
            (4u64, "cluster1_center".to_string(), vec)
        },
        // Cluster 2: Nearby cluster with overlap region (ensures inter-cluster connectivity)
        {
            let mut vec = vec![0.0f32; TEST_DIMENSION];
            vec[0] = 0.3f32;
            vec[1] = 0.2f32;
            (5u64, "cluster2_a".to_string(), vec)
        },
        {
            let mut vec = vec![0.0f32; TEST_DIMENSION];
            vec[0] = 0.4f32;
            vec[1] = 0.2f32;
            (6u64, "cluster2_b".to_string(), vec)
        },
        {
            let mut vec = vec![0.0f32; TEST_DIMENSION];
            vec[0] = 0.3f32;
            vec[1] = 0.3f32;
            (7u64, "cluster2_c".to_string(), vec)
        },
        {
            let mut vec = vec![0.0f32; TEST_DIMENSION];
            vec[0] = 0.35f32;
            vec[1] = 0.25f32;
            (8u64, "cluster2_center".to_string(), vec)
        },
        // Bridge points to ensure overall connectivity
        {
            let mut vec = vec![0.0f32; TEST_DIMENSION];
            vec[0] = 0.25f32;
            vec[1] = 0.15f32; // Between clusters
            (9u64, "bridge".to_string(), vec)
        },
        {
            let mut vec = vec![0.0f32; TEST_DIMENSION];
            vec[0] = 0.05f32;
            vec[1] = 0.05f32; // Near origin
            (10u64, "origin_near".to_string(), vec)
        },
    ]
}

/// Creates connected dataset for scalability testing
fn create_connected_dataset(size: usize) -> Vec<(VertexId, String, Vec<f32>)> {
    let clusters = 3;
    let points_per_cluster = (size + clusters - 1) / clusters; // Ceiling division

    (0..size)
        .map(|i| {
            let cluster_id = i / points_per_cluster;
            let point_in_cluster = i % points_per_cluster;

            let mut vector = vec![0.0f32; TEST_DIMENSION];

            // Create overlapping clusters for connectivity
            let cluster_center_x = (cluster_id as f32) * 0.03 + 0.1;
            let cluster_center_y = (cluster_id as f32) * 0.02 + 0.1;

            // Add small random-like but deterministic offsets within cluster
            let offset_x = ((point_in_cluster as f32) * 0.1).sin() * 0.1;
            let offset_y = ((point_in_cluster as f32) * 0.1).cos() * 0.1;

            vector[0] = cluster_center_x + offset_x;
            vector[1] = cluster_center_y + offset_y;

            // Add some variation in other dimensions for uniqueness
            vector[2] = (i as f32) * 0.01;

            ((i + 1) as VertexId, format!("connected_{}", i), vector)
        })
        .collect()
}

/// Creates boundary test vectors with connectivity for u32::MAX testing
fn create_boundary_test_vectors() -> Vec<(VertexId, String, Vec<f32>)> {
    let mut vectors = create_connected_test_vectors();

    // Add boundary ID vertex with connected coordinates
    let max_id = u32::MAX as u64;
    let mut boundary_vec = vec![0.0f32; TEST_DIMENSION];
    boundary_vec[0] = 0.2f32; // Place it near existing cluster
    boundary_vec[1] = 0.2f32;

    vectors.push((max_id, "boundary_max".to_string(), boundary_vec));
    vectors
}

// ===== CORE FUNCTIONALITY TESTS =====

#[test]
fn test_vector_index_build_and_verify() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph.begin_transaction(IsolationLevel::Serializable);

    // Create test vertices with connected vectors
    let test_vectors = create_connected_test_vectors();
    for (id, name, embedding) in &test_vectors {
        let vertex = create_vertex_with_vector(*id, name, embedding.clone());
        graph.create_vertex(&txn, vertex)?;
    }

    // Build vector index
    let config = create_test_index_config(TEST_DIMENSION);
    graph.build_vector_index(&txn, EMBEDDING_PROPERTY_ID, config)?;

    // Verify index creation and properties
    let index = graph
        .get_vector_index(EMBEDDING_PROPERTY_ID)
        .expect("Index should exist after build");
    assert_eq!(index.size(), test_vectors.len());
    assert_eq!(index.get_dimension(), TEST_DIMENSION);

    txn.commit()?;
    Ok(())
}

#[test]
fn test_vector_search_accuracy() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph.begin_transaction(IsolationLevel::Serializable);

    // Create test dataset with connected vectors
    let test_vectors = create_connected_test_vectors();
    for (id, name, embedding) in &test_vectors {
        let vertex = create_vertex_with_vector(*id, name, embedding.clone());
        graph.create_vertex(&txn, vertex)?;
    }

    // Build index
    let config = create_test_index_config(TEST_DIMENSION);
    graph.build_vector_index(&txn, EMBEDDING_PROPERTY_ID, config)?;

    // Test 1: Search in cluster 1 area
    let mut cluster1_query = vec![0.0f32; TEST_DIMENSION];
    cluster1_query[0] = 0.15f32;
    cluster1_query[1] = 0.15f32;
    let results = graph.vector_search(EMBEDDING_PROPERTY_ID, &cluster1_query, 3, 20, None)?;
    assert!(!results.is_empty(), "Should find vectors in cluster 1");
    // Should find cluster1_center (ID 4) as closest
    assert!(results.contains(&4u64), "Should find cluster1_center");

    // Test 2: Search in cluster 2 area
    let mut cluster2_query = vec![0.0f32; TEST_DIMENSION];
    cluster2_query[0] = 0.35f32;
    cluster2_query[1] = 0.25f32;
    let results = graph.vector_search(EMBEDDING_PROPERTY_ID, &cluster2_query, 3, 20, None)?;
    assert!(!results.is_empty(), "Should find vectors in cluster 2");
    // Should find cluster2_center (ID 8) as closest
    assert!(results.contains(&8u64), "Should find cluster2_center");

    txn.commit()?;
    Ok(())
}

// ===== ERROR HANDLING TESTS =====

#[test]
fn test_error_index_not_found() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph.begin_transaction(IsolationLevel::Serializable);

    // Try to search without building index
    let query = vec![1.0f32; TEST_DIMENSION];
    let result = graph.vector_search(EMBEDDING_PROPERTY_ID, &query, 1, 20, None);

    // Should fail with IndexNotFound error
    assert!(result.is_err());

    txn.commit()?;
    Ok(())
}

#[test]
fn test_error_empty_dataset() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph.begin_transaction(IsolationLevel::Serializable);

    // Try to build index on empty dataset
    let config = create_test_index_config(TEST_DIMENSION);
    let result = graph.build_vector_index(&txn, EMBEDDING_PROPERTY_ID, config);

    // Should fail with appropriate error
    assert!(result.is_err());

    txn.commit()?;
    Ok(())
}

#[test]
fn test_error_dimension_mismatch() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph.begin_transaction(IsolationLevel::Serializable);

    // Create index with valid vectors
    let test_vectors = create_connected_test_vectors();
    for (id, name, embedding) in &test_vectors {
        let vertex = create_vertex_with_vector(*id, name, embedding.clone());
        graph.create_vertex(&txn, vertex)?;
    }

    let config = create_test_index_config(TEST_DIMENSION);
    graph.build_vector_index(&txn, EMBEDDING_PROPERTY_ID, config)?;

    // Try to search with wrong dimension query
    let wrong_dim_query = vec![0.0f32; 50]; // Wrong dimension
    let result = graph.vector_search(EMBEDDING_PROPERTY_ID, &wrong_dim_query, 1, 20, None);

    // Should fail due to dimension mismatch
    assert!(result.is_err());

    txn.commit()?;
    Ok(())
}

// ===== VERTEX ID MAPPING TESTS =====

#[test]
fn test_vertex_id_mapping_correctness() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph.begin_transaction(IsolationLevel::Serializable);

    // Create connected vertices with specific IDs to test mapping
    let test_vectors = create_connected_test_vectors();
    // Replace some IDs with specific values for testing
    let mut modified_vectors = test_vectors;
    modified_vectors[0].0 = 10u64;
    modified_vectors[1].0 = 42u64;
    modified_vectors[2].0 = 100u64;
    modified_vectors[3].0 = 999u64;

    for (id, name, embedding) in &modified_vectors {
        let vertex = create_vertex_with_vector(*id, name, embedding.clone());
        graph.create_vertex(&txn, vertex)?;
    }

    let config = create_test_index_config(TEST_DIMENSION);
    graph.build_vector_index(&txn, EMBEDDING_PROPERTY_ID, config)?;

    // Search should return correct vertex IDs for first few modified vectors
    for i in 0..4 {
        let (expected_id, _, embedding) = &modified_vectors[i];
        let results = graph.vector_search(EMBEDDING_PROPERTY_ID, embedding, 1, 20, None)?;
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0], *expected_id,
            "ID mapping failed for vertex {}",
            expected_id
        );
    }

    txn.commit()?;
    Ok(())
}

#[test]
fn test_vertex_id_boundary_values() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph.begin_transaction(IsolationLevel::Serializable);

    // Create connected test vectors with one boundary ID vertex
    let test_vectors = create_boundary_test_vectors();
    for (id, name, embedding) in &test_vectors {
        let vertex = create_vertex_with_vector(*id, name, embedding.clone());
        graph.create_vertex(&txn, vertex)?;
    }

    // Building index should succeed with connected data
    let config = create_test_index_config(TEST_DIMENSION);
    let result = graph.build_vector_index(&txn, EMBEDDING_PROPERTY_ID, config);
    assert!(
        result.is_ok(),
        "Index build should succeed with connected vectors"
    );

    // Search should return results including the boundary ID
    let max_valid_id = u32::MAX as u64;
    let mut query = vec![0.0f32; TEST_DIMENSION];
    query[0] = 0.2f32; // Query near the boundary vertex
    query[1] = 0.2f32;
    let results = graph.vector_search(EMBEDDING_PROPERTY_ID, &query, 3, 20, None)?;

    assert!(!results.is_empty(), "Should find nearby vectors");
    assert!(
        results.contains(&max_valid_id),
        "Should find the boundary ID vertex"
    );

    txn.commit()?;
    Ok(())
}

// ===== PERFORMANCE AND SCALABILITY TESTS =====

#[test]
fn test_scalability_dataset() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph.begin_transaction(IsolationLevel::Serializable);

    // Create connected dataset to test scalability
    let dataset_size = 50;
    let test_vectors = create_connected_dataset(dataset_size);

    for (id, name, embedding) in &test_vectors {
        let vertex = create_vertex_with_vector(*id, name, embedding.clone());
        graph.create_vertex(&txn, vertex)?;
    }

    // Build index
    let config = create_test_index_config(TEST_DIMENSION);
    graph.build_vector_index(&txn, EMBEDDING_PROPERTY_ID, config)?;

    // Verify index properties
    let index = graph.get_vector_index(EMBEDDING_PROPERTY_ID).unwrap();
    assert_eq!(index.size(), dataset_size);

    // Test search with various k values
    let mut query = vec![0.0f32; TEST_DIMENSION];
    query[0] = 0.2f32;
    query[1] = 0.15f32;
    let results = graph.vector_search(EMBEDDING_PROPERTY_ID, &query, 10, 20, None)?;
    assert!(!results.is_empty());
    assert!(results.len() <= 10);

    txn.commit()?;
    Ok(())
}

#[test]
fn test_transaction_isolation() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();

    // Transaction 1: Build index
    let txn1 = graph.begin_transaction(IsolationLevel::Serializable);
    let test_vectors = create_connected_test_vectors();
    for (id, name, embedding) in &test_vectors {
        let vertex = create_vertex_with_vector(*id, name, embedding.clone());
        graph.create_vertex(&txn1, vertex)?;
    }

    let config = create_test_index_config(TEST_DIMENSION);
    graph.build_vector_index(&txn1, EMBEDDING_PROPERTY_ID, config)?;
    txn1.commit()?;

    // Transaction 2: Use index with different isolation levels
    for &isolation in &[IsolationLevel::Snapshot, IsolationLevel::Serializable] {
        let txn2 = graph.begin_transaction(isolation);
        let mut query = vec![0.0f32; TEST_DIMENSION];
        query[0] = 0.15f32;
        query[1] = 0.15f32;
        let results = graph.vector_search(EMBEDDING_PROPERTY_ID, &query, 3, 20, None)?;
        assert!(!results.is_empty());
        txn2.commit()?;
    }

    Ok(())
}

// ===== INTEGRATION TESTS =====

#[test]
fn test_multiple_indices_per_graph() -> StorageResult<()> {
    let (graph, _cleaner) = create_empty_graph();
    let txn = graph.begin_transaction(IsolationLevel::Serializable);

    // Create vertices with vectors on different properties
    let test_vectors = create_connected_test_vectors();
    for (id, name, embedding) in &test_vectors {
        // Create property with different embeddings for property 1 and 2
        let embedding_1 = embedding.clone();
        let mut embedding_2 = embedding.clone();
        embedding_2[0] += 0.1; // Slight variation

        let vertex = Vertex::new(
            *id,
            PERSON_LABEL_ID,
            PropertyRecord::new(vec![
                ScalarValue::String(Some(name.clone())),
                ScalarValue::Vector(Some(embedding_1.into_iter().map(F32::from).collect())),
                ScalarValue::Vector(Some(embedding_2.into_iter().map(F32::from).collect())),
            ]),
        );
        graph.create_vertex(&txn, vertex)?;
    }

    // Build indices on different properties
    let config = create_test_index_config(TEST_DIMENSION);
    graph.build_vector_index(&txn, 1, config.clone())?; // Property 1
    graph.build_vector_index(&txn, 2, config)?; // Property 2

    // Verify both indices work independently
    let mut query = vec![0.0f32; TEST_DIMENSION];
    query[0] = 0.15f32;
    query[1] = 0.15f32;
    let results_1 = graph.vector_search(1, &query, 2, 20, None)?;
    let results_2 = graph.vector_search(2, &query, 2, 20, None)?;

    assert!(!results_1.is_empty());
    assert!(!results_2.is_empty());

    txn.commit()?;
    Ok(())
}
