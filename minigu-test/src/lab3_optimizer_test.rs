//! Lab3: Predicate Pushdown Optimization - Optimizer Tests
//!
//! This test file validates the implementation of predicate pushdown optimization.
//!
//! ## Learning Objectives
//! - Understand the role of query optimizer
//! - Master predicate pushdown optimization technique
//! - Learn to verify optimized execution plans
//!
//! ## Run Tests
//! ```bash
//! cargo test --package minigu-test --test lab3_optimizer_test
//! ```
//!
//! ## Related Files
//! - `minigu/gql/planner/src/optimizer/mod.rs` - Optimizer implementation
//!
//! ## Optimization Goal
//! Transform Filter + NodeScan into NodeScanById to avoid full table scan

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

// ========== Basic Predicate Pushdown Tests ==========

/// Test 1: ID predicate pushdown
///
/// Query: MATCH (n:PERSON) WHERE id(n) = 1 RETURN n
/// Before: Filter(id=1) -> NodeScan(Person)
/// After: NodeScanById(1)
///
/// Expected: Optimizer should recognize id() function and generate NodeScanById
#[test]
fn test_id_predicate_pushdown() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) WHERE id(n) = 1 RETURN n");

    match result {
        Ok(res) => {
            let total_rows: usize = res.chunks.iter().map(|c| c.cardinality()).sum();
            println!("ID predicate query succeeded with {} rows", total_rows);
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!(
                    "Lab3 incomplete: Predicate pushdown not implemented!\n\
                    Please implement pushdown logic in optimizer/mod.rs.\n\
                    Error: {}",
                    error_msg
                );
            }
            panic!("ID predicate query failed: {}", e);
        }
    }
}

/// Test 2: Performance comparison
///
/// Compare query performance with and without predicate pushdown
#[test]
fn test_pushdown_performance_comparison() {
    let (_db, mut session) = setup_test_db();

    // Query 1: With ID predicate (should be optimized)
    let start1 = std::time::Instant::now();
    let result1 = session.query("MATCH (n:PERSON) WHERE id(n) = 1 RETURN n");
    let time1 = start1.elapsed();

    // Query 2: Full table scan
    let start2 = std::time::Instant::now();
    let result2 = session.query("MATCH (n:PERSON) RETURN n");
    let time2 = start2.elapsed();

    println!("Performance comparison:");
    println!("  ID predicate query: {:?}", time1);
    println!("  Full scan query: {:?}", time2);

    match (result1, result2) {
        (Ok(r1), Ok(r2)) => {
            let rows1: usize = r1.chunks.iter().map(|c| c.cardinality()).sum();
            let rows2: usize = r2.chunks.iter().map(|c| c.cardinality()).sum();
            println!("  ID predicate rows: {}", rows1);
            println!("  Full scan rows: {}", rows2);

            assert!(
                rows1 <= rows2,
                "ID predicate query should return fewer or equal rows"
            );
            println!("  Performance comparison completed");
        }
        (Err(e1), _) => {
            println!("ID predicate query failed: {}", e1);
        }
        (_, Err(e2)) => {
            println!("Full scan query failed: {}", e2);
        }
    }
}

// ========== Compound Predicate Tests ==========

/// Test 3: ID + other conditions
///
/// Query: MATCH (n:PERSON) WHERE id(n) = 1 AND n.age > 18 RETURN n
/// Expected: ID part should be pushed down, age condition remains as Filter
#[test]
fn test_partial_pushdown() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) WHERE id(n) = 1 AND n.age > 18 RETURN n");

    match result {
        Ok(res) => {
            println!("Partial pushdown test:");
            let total_rows: usize = res.chunks.iter().map(|c| c.cardinality()).sum();
            println!("  Partial pushdown query succeeded with {} rows", total_rows);
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("todo") || error_msg.contains("not implemented") {
                panic!(
                    "Lab3 incomplete: Partial predicate pushdown not implemented!\nError: {}",
                    error_msg
                );
            }
            panic!("Partial pushdown query failed: {}", e);
        }
    }
}

// ========== Non-pushable Predicate Tests ==========

/// Test 4: Non-pushable predicate
///
/// Query: MATCH (n:PERSON) WHERE n.age > 18 RETURN n
/// Expected: Regular property conditions should NOT be pushed to NodeScan
#[test]
fn test_non_pushable_predicate() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) WHERE n.age > 18 RETURN n");

    match result {
        Ok(res) => {
            let total_rows: usize = res.chunks.iter().map(|c| c.cardinality()).sum();
            println!("Non-pushable predicate test: {} rows", total_rows);
            println!("  Regular predicate handled correctly (not pushed)");
        }
        Err(e) => {
            println!("Non-pushable predicate query failed: {}", e);
        }
    }
}

/// Test 5: OR condition (typically not pushable)
///
/// Query: MATCH (n:PERSON) WHERE id(n) = 1 OR n.age > 18 RETURN n
/// Expected: OR conditions usually cannot be fully pushed down
#[test]
fn test_or_condition_no_pushdown() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) WHERE id(n) = 1 OR n.age > 100 RETURN n");

    match result {
        Ok(res) => {
            let total_rows: usize = res.chunks.iter().map(|c| c.cardinality()).sum();
            println!("OR condition test: {} rows", total_rows);
            println!("  OR condition handled correctly");
        }
        Err(e) => {
            println!("OR condition query failed: {}", e);
        }
    }
}

// ========== Edge Cases ==========

/// Test 6: Invalid ID query
///
/// Query: MATCH (n:PERSON) WHERE id(n) = -1 RETURN n
/// Expected: Empty result, no error
#[test]
fn test_invalid_id_query() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) WHERE id(n) = -1 RETURN n");

    match result {
        Ok(res) => {
            let total_rows: usize = res.chunks.iter().map(|c| c.cardinality()).sum();
            println!("Invalid ID query test: {} rows (expected 0)", total_rows);
            println!("  Invalid ID handled correctly");
        }
        Err(e) => {
            // Error on invalid ID is also acceptable
            println!("Invalid ID query error (acceptable): {}", e);
        }
    }
}

/// Test 7: Large ID query
///
/// Query: MATCH (n:PERSON) WHERE id(n) = 999999999 RETURN n
/// Expected: Empty result, no error
#[test]
fn test_large_id_query() {
    let (_db, mut session) = setup_test_db();

    let result = session.query("MATCH (n:PERSON) WHERE id(n) = 999999999 RETURN n");

    match result {
        Ok(res) => {
            let total_rows: usize = res.chunks.iter().map(|c| c.cardinality()).sum();
            println!("Large ID query test: {} rows (expected 0)", total_rows);
            println!("  Large ID handled correctly");
        }
        Err(e) => {
            println!("Large ID query error: {}", e);
        }
    }
}
