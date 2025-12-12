//! Lab2: Executor Implementation - Project and Expand Operator Tests
//!
//! This test file validates the implementation of Project and Expand executors.
//!
//! ## Learning Objectives
//! - Understand the Volcano Model execution pattern
//! - Master Project operator for column projection
//! - Master Expand operator for graph traversal
//!
//! ## Run Tests
//! ```bash
//! cargo test --package minigu-test --test lab2_executor_test
//! ```
//!
//! ## Related Files
//! - `minigu/gql/execution/src/executor/project.rs` - Project operator
//! - `minigu/gql/execution/src/executor/expand.rs` - Expand operator

use minigu::database::Database;

/// Create test database environment
fn setup_test_db() -> (Database, minigu::session::Session) {
    let db = Database::open_in_memory(&Default::default()).expect("Failed to create database");
    let mut session = db.session().expect("Failed to create session");

    // Create test graph with sample data (10 vertices)
    session
        .query("CALL create_test_graph_data('test_graph', 10)")
        .expect("Failed to create test graph");

    session
        .query("SESSION SET GRAPH test_graph")
        .expect("Failed to set graph");

    (db, session)
}

// ========== Project Operator Tests ==========

/// Test 1: Single column projection
///
/// Query: MATCH (n:PERSON) RETURN n.name
/// Expected: Only name column returned
#[test]
fn test_project_single_column() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) RETURN n.name");

    match result {
        Ok(res) => {
            println!("Single column projection test:");
            println!("  Schema: {:?}", res.schema());

            if let Some(schema) = res.schema() {
                assert_eq!(
                    schema.fields().len(),
                    1,
                    "Single column projection should return 1 column, got {}",
                    schema.fields().len()
                );
                println!("  Column count correct: 1");
            }

            for (i, chunk) in res.chunks.iter().enumerate() {
                println!(
                    "  Chunk {}: {} rows, {} columns",
                    i,
                    chunk.cardinality(),
                    chunk.columns().len()
                );
            }
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!(
                    "Lab2 incomplete: Project operator not implemented!\n\
                    Please implement into_executor method in project.rs.\n\
                    Error: {}",
                    error_msg
                );
            }
            panic!("Query execution failed: {}", e);
        }
    }
}

/// Test 2: Multiple column projection
///
/// Query: MATCH (n:PERSON) RETURN n.name, n.age
/// Expected: Both name and age columns returned
#[test]
fn test_project_multiple_columns() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) RETURN n.name, n.age");

    match result {
        Ok(res) => {
            println!("Multiple column projection test:");
            println!("  Schema: {:?}", res.schema());

            if let Some(schema) = res.schema() {
                assert_eq!(
                    schema.fields().len(),
                    2,
                    "Two column projection should return 2 columns, got {}",
                    schema.fields().len()
                );
                println!("  Column count correct: 2");
            }

            for (i, chunk) in res.chunks.iter().enumerate() {
                println!(
                    "  Chunk {}: {} rows, {} columns",
                    i,
                    chunk.cardinality(),
                    chunk.columns().len()
                );
            }
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!("Lab2 incomplete: Project operator not implemented!\nError: {}", error_msg);
            }
            panic!("Query execution failed: {}", e);
        }
    }
}

/// Test 3: Expression projection
///
/// Query: MATCH (n:PERSON) RETURN n.age + 1 AS next_age
/// Expected: Computed expression result
#[test]
fn test_project_expression() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) RETURN n.age + 1 AS next_age");

    match result {
        Ok(res) => {
            println!("Expression projection test:");
            println!("  Schema: {:?}", res.schema());
            println!("  Expression projection successful");

            for (i, chunk) in res.chunks.iter().enumerate() {
                println!("  Chunk {}: {} rows", i, chunk.cardinality());
            }
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!(
                    "Lab2 incomplete: Project expression evaluation not implemented!\nError: {}",
                    error_msg
                );
            }
            panic!("Query execution failed: {}", e);
        }
    }
}

/// Test 4: Return node itself
///
/// Query: MATCH (n:PERSON) RETURN n
/// Expected: Complete node returned
#[test]
fn test_project_node() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) RETURN n");

    match result {
        Ok(res) => {
            println!("Node projection test:");
            println!("  Schema: {:?}", res.schema());
            println!("  Node projection successful");

            for (i, chunk) in res.chunks.iter().enumerate() {
                println!("  Chunk {}: {} rows", i, chunk.cardinality());
            }
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!("Lab2 incomplete: Project operator not implemented!\nError: {}", error_msg);
            }
            panic!("Query execution failed: {}", e);
        }
    }
}

// ========== Expand Operator Tests ==========

/// Test 5: Outgoing expand
///
/// Query: MATCH (n:PERSON)-[e]->(m) RETURN n.name, m.name
/// Expected: Expand from n to m along outgoing edges
#[test]
fn test_expand_outgoing() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON)-[e]->(m) RETURN n.name, m.name");

    match result {
        Ok(res) => {
            println!("Outgoing expand test:");
            println!("  Schema: {:?}", res.schema());

            let total_rows: usize = res.chunks.iter().map(|c| c.cardinality()).sum();
            println!("  Expand successful, {} total rows", total_rows);

            for (i, chunk) in res.chunks.iter().enumerate() {
                println!("  Chunk {}: {} rows", i, chunk.cardinality());
            }
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!(
                    "Lab2 incomplete: Expand operator not implemented!\n\
                    Please implement into_executor method in expand.rs.\n\
                    Error: {}",
                    error_msg
                );
            }
            panic!("Query execution failed: {}", e);
        }
    }
}

/// Test 6: Incoming expand
///
/// Query: MATCH (n:PERSON)<-[e]-(m) RETURN n.name, m.name
/// Expected: Expand from n along incoming edges
#[test]
fn test_expand_incoming() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON)<-[e]-(m) RETURN n.name, m.name");

    match result {
        Ok(res) => {
            println!("Incoming expand test:");
            println!("  Schema: {:?}", res.schema());

            let total_rows: usize = res.chunks.iter().map(|c| c.cardinality()).sum();
            println!("  Reverse expand successful, {} total rows", total_rows);
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!(
                    "Lab2 incomplete: Expand incoming not implemented!\nError: {}",
                    error_msg
                );
            }
            panic!("Query execution failed: {}", e);
        }
    }
}

/// Test 7: Both direction expand
///
/// Query: MATCH (n:PERSON)-[e]-(m) RETURN n.name, m.name
/// Expected: Expand in both directions
#[test]
fn test_expand_both() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON)-[e]-(m) RETURN n.name, m.name");

    match result {
        Ok(res) => {
            println!("Both direction expand test:");
            println!("  Schema: {:?}", res.schema());

            let total_rows: usize = res.chunks.iter().map(|c| c.cardinality()).sum();
            println!("  Bidirectional expand successful, {} total rows", total_rows);
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!(
                    "Lab2 incomplete: Expand bidirectional not implemented!\nError: {}",
                    error_msg
                );
            }
            panic!("Query execution failed: {}", e);
        }
    }
}

/// Test 8: Expand with edge type filter
///
/// Query: MATCH (n:PERSON)-[e:FRIEND]->(m:PERSON) RETURN n.name, m.name
/// Expected: Only expand along 'FRIEND' edges
#[test]
fn test_expand_with_edge_type() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON)-[e:FRIEND]->(m:PERSON) RETURN n.name, m.name");

    match result {
        Ok(res) => {
            println!("Expand with edge type test:");
            println!("  Schema: {:?}", res.schema());

            let total_rows: usize = res.chunks.iter().map(|c| c.cardinality()).sum();
            println!("  Edge type filtered expand successful, {} total rows", total_rows);
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!(
                    "Lab2 incomplete: Expand edge type filter not implemented!\nError: {}",
                    error_msg
                );
            }
            panic!("Query execution failed: {}", e);
        }
    }
}

/// Test 9: Multi-hop expand
///
/// Query: MATCH (n:PERSON)-[e1]->(m)-[e2]->(k) RETURN n.name, k.name
/// Expected: Two-hop traversal
#[test]
fn test_expand_multi_hop() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON)-[e1]->(m)-[e2]->(k) RETURN n.name, k.name");

    match result {
        Ok(res) => {
            println!("Multi-hop expand test:");
            println!("  Schema: {:?}", res.schema());

            let total_rows: usize = res.chunks.iter().map(|c| c.cardinality()).sum();
            println!("  Multi-hop expand successful, {} total rows", total_rows);
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!(
                    "Lab2 incomplete: Multi-hop expand not implemented!\nError: {}",
                    error_msg
                );
            }
            panic!("Query execution failed: {}", e);
        }
    }
}

// ========== Combined Tests ==========

/// Test 10: Expand then Project with alias
///
/// Query: MATCH (n:PERSON)-[e]->(m) RETURN n.name AS source, m.name AS target
/// Expected: Expand followed by column projection and renaming
#[test]
fn test_expand_then_project() {
    let (_db, mut session) = setup_test_db();

    let result =
        session.query("MATCH (n:PERSON)-[e]->(m) RETURN n.name AS source, m.name AS target");

    match result {
        Ok(res) => {
            println!("Expand then project test:");
            println!("  Schema: {:?}", res.schema());

            if let Some(schema) = res.schema() {
                assert_eq!(schema.fields().len(), 2, "Should have 2 columns");
                println!("  Column count correct: 2");
            }

            let total_rows: usize = res.chunks.iter().map(|c| c.cardinality()).sum();
            println!("  Expand + Project successful, {} total rows", total_rows);
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!(
                    "Lab2 incomplete: Expand or Project not implemented!\nError: {}",
                    error_msg
                );
            }
            panic!("Query execution failed: {}", e);
        }
    }
}
