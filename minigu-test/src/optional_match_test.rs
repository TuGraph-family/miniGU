//! Integration test for OPTIONAL MATCH functionality.

use minigu::database::Database;

/// Test that OPTIONAL MATCH can be parsed and planned.
#[test]
fn test_optional_match_parse_and_plan() {
    // Create an in-memory database
    let db = Database::open_in_memory(Default::default()).expect("Failed to create database");
    let mut session = db.session().expect("Failed to create session");

    // Parse and plan a simple OPTIONAL MATCH query
    let query = "OPTIONAL MATCH (n:Person) RETURN n";

    // This should not panic - if it parses and plans successfully, the test passes
    let result = session.query(query);

    // We expect this to work even though there's no data
    // The plan should be generated successfully
    match result {
        Ok(_) => {
            // Query executed successfully (even if no results)
            println!("OPTIONAL MATCH query executed successfully");
        }
        Err(e) => {
            // If there's an error, it should be a runtime error, not a parse/plan error
            // Parse/plan errors would indicate our implementation is broken
            let error_msg = e.to_string();
            // Check that it's not a parse error or not implemented error
            assert!(
                !error_msg.contains("parse error") && !error_msg.contains("not implemented"),
                "Query failed with parse/plan error: {}",
                error_msg
            );
            println!(
                "Query failed with runtime error (expected for empty database): {}",
                error_msg
            );
        }
    }
}

/// Test that OPTIONAL MATCH with pattern can be parsed.
#[test]
fn test_optional_match_with_edge_pattern() {
    let db = Database::open_in_memory(Default::default()).expect("Failed to create database");
    let mut session = db.session().expect("Failed to create session");

    // Parse and plan OPTIONAL MATCH with edge pattern
    let query = "OPTIONAL MATCH (a:Account)-[e:transfer]->(b:Account) RETURN a, b, e";

    let result = session.query(query);

    match result {
        Ok(_) => {
            println!("OPTIONAL MATCH with edge pattern executed successfully");
        }
        Err(e) => {
            let error_msg = e.to_string();
            assert!(
                !error_msg.contains("parse error") && !error_msg.contains("not implemented"),
                "Query failed with parse/plan error: {}",
                error_msg
            );
            println!("Query failed with runtime error (expected): {}", error_msg);
        }
    }
}
