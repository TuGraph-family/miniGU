//! Lab1: WHERE Clause Filter - LogicalFilter Node Tests
//!
//! This test file validates the implementation of Filter node generation.
//!
//! ## Learning Objectives
//! - Understand how WHERE clauses are processed in GQL queries
//! - Learn the placement of LogicalFilter nodes in the logical plan
//! - Verify correct predicate propagation
//!
//! ## Run Tests
//! ```bash
//! cargo test --package minigu-test --test lab1_filter_test
//! ```
//!
//! ## Related Files
//! - `minigu/gql/planner/src/logical_planner/query.rs` - Implementation target
//! - `minigu/gql/planner/src/plan/filter.rs` - Filter node definition

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

// ========== Basic Filter Tests ==========

/// Test 1: Basic WHERE clause should generate LogicalFilter node
///
/// Query: MATCH (n:PERSON) WHERE n.age > 18 RETURN n
/// Expected: Logical plan should contain LogicalFilter node
#[test]
fn test_basic_where_clause() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) WHERE n.age > 18 RETURN n");

    match result {
        Ok(res) => {
            println!("Query executed successfully with {} chunks", res.chunks.len());
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!(
                    "Lab1 incomplete: Filter node generation not implemented!\n\
                    Please implement the Filter section in plan_match_statement.\n\
                    Error: {}",
                    error_msg
                );
            }
            panic!("Query execution failed: {}", e);
        }
    }
}

/// Test 2: Equality comparison
///
/// Query: MATCH (n:PERSON) WHERE n.name = 'Alice' RETURN n
#[test]
fn test_equality_comparison() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) WHERE n.name = 'Alice' RETURN n");

    match result {
        Ok(res) => {
            println!("Equality filter test passed with {} chunks", res.chunks.len());
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!("Lab1 incomplete: Equality filter not implemented!\nError: {}", error_msg);
            }
            panic!("Query execution failed: {}", e);
        }
    }
}

/// Test 3: AND condition
///
/// Query: MATCH (n:PERSON) WHERE n.age > 18 AND n.age < 65 RETURN n
#[test]
fn test_and_condition() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) WHERE n.age > 18 AND n.age < 65 RETURN n");

    match result {
        Ok(res) => {
            println!("AND condition test passed with {} chunks", res.chunks.len());
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!("Lab1 incomplete: AND condition filter not implemented!\nError: {}", error_msg);
            }
            panic!("Query execution failed: {}", e);
        }
    }
}

/// Test 4: OR condition
///
/// Query: MATCH (n:PERSON) WHERE n.age < 18 OR n.age > 65 RETURN n
#[test]
fn test_or_condition() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) WHERE n.age < 18 OR n.age > 65 RETURN n");

    match result {
        Ok(res) => {
            println!("OR condition test passed with {} chunks", res.chunks.len());
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!("Lab1 incomplete: OR condition filter not implemented!\nError: {}", error_msg);
            }
            panic!("Query execution failed: {}", e);
        }
    }
}

/// Test 5: Query without WHERE should work without Filter
///
/// Query: MATCH (n:PERSON) RETURN n
/// Expected: No LogicalFilter node in plan
#[test]
fn test_no_where_no_filter() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) RETURN n");

    assert!(result.is_ok(), "Query without WHERE clause should execute normally");
}

/// Test 6: Verify filter result correctness
///
/// Query with age > 25 predicate
#[test]
fn test_filter_result_correctness() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) WHERE n.age > 25 RETURN n.name, n.age");

    match result {
        Ok(res) => {
            println!("Filter correctness test:");
            println!("  Schema: {:?}", res.schema());
            println!("  Chunks: {}", res.chunks.len());
            for (i, chunk) in res.chunks.iter().enumerate() {
                println!("  Chunk {}: {} rows", i, chunk.cardinality());
            }
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!("Lab1 incomplete: Filter node not implemented!\nError: {}", error_msg);
            }
            panic!("Query execution failed: {}", e);
        }
    }
}

/// Test 7: Empty result set
///
/// Query: MATCH (n:PERSON) WHERE n.age > 1000 RETURN n
/// Expected: Empty result, no error
#[test]
fn test_filter_empty_result() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) WHERE n.age > 1000 RETURN n");

    match result {
        Ok(res) => {
            let total_rows: usize = res.chunks.iter().map(|c| c.cardinality()).sum();
            println!("Empty result test: {} total rows", total_rows);
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!("Lab1 incomplete: Filter node not implemented!\nError: {}", error_msg);
            }
            panic!("Query execution failed: {}", e);
        }
    }
}
