//! This file defines end-to-end tests for miniGU.
//!
//! Test cases can be found in `../../resources/gql`, and expected outputs can be found in
//! `snapshots`.
use gql_parser::parse_gql;
use insta::internals::SettingsBindDropGuard;
use insta::{Settings, assert_snapshot, assert_yaml_snapshot};
use minigu::database::{Database, DatabaseConfig};
use minigu::result::QueryResult;
use pastey::paste;

const GQL_COMMENT_PREFIX: &str = "--";
const FILE_COMMENT_PREFIX: &str = "//";
const QUERY_END_SUFFIX: &str = ";";

fn setup(suffix: &str, snapshot_path: &str) -> SettingsBindDropGuard {
    let mut settings = Settings::clone_current();
    settings.set_snapshot_path(snapshot_path);
    settings.set_snapshot_suffix(suffix);
    settings.set_omit_expression(true);
    settings.set_prepend_module_to_snapshot(false);
    settings.bind_to_scope()
}

fn preprocess_statements(input: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current_statement = String::new();

    for line in input.lines() {
        let trimmed_line = line.trim();

        // Skip comment lines starting with -- or //
        if trimmed_line.starts_with(GQL_COMMENT_PREFIX)
            || trimmed_line.starts_with(FILE_COMMENT_PREFIX)
        {
            continue;
        }

        // Add line to current statement
        if !current_statement.is_empty() {
            current_statement.push('\n');
        }
        current_statement.push_str(line);

        // Check if statement is complete (ends with semicolon)
        if trimmed_line.ends_with(QUERY_END_SUFFIX) {
            // Remove trailing semicolon and trim
            let statement = current_statement
                .trim_end_matches(QUERY_END_SUFFIX)
                .trim()
                .to_string();
            if !statement.is_empty() {
                statements.push(statement);
            }
            current_statement.clear();
        }
    }

    // Handle any remaining statement without semicolon
    let remaining = current_statement.trim();
    if !remaining.is_empty() {
        statements.push(remaining.to_string());
    }

    statements
}

fn query_e2e_test(statements: Vec<String>) -> String {
    let config = DatabaseConfig::default();
    let database = Database::open_in_memory(&config).unwrap();
    let mut session = database.session().unwrap();

    let mut output = String::new();
    for (idx, statement) in statements.iter().enumerate() {
        if idx > 0 {
            output.push_str("\n---\n");
        }

        match session.query(statement) {
            Ok(result) => {
                let result_str = result_to_string(&result);
                output.push_str(&result_str);
            }
            Err(e) => {
                let debug_str = format!("{:#?}", e);
                let display_str = debug_str.replace("\\n", "\n");
                output.push_str(&format!("Error: {}", display_str));
            }
        }
    }

    output
}

fn result_to_string(result: &QueryResult) -> String {
    if result.iter().count() == 0 {
        return "Statement OK. No results".to_string();
    }
    let mut output = String::new();
    if let Some(schema) = result.schema() {
        let headers: Vec<String> = schema
            .fields()
            .iter()
            .map(|f| f.name().to_string())
            .collect();
        output.push_str(&headers.join("\t"));
        output.push('\n');
    }
    for chunk in result.iter() {
        let rows_count = chunk.cardinality();
        let column = chunk.columns();
        for row_idx in 0..rows_count {
            let row_values: Vec<String> = (0..column.len())
                .map(|col_idx| {
                    let array = &column[col_idx];
                    extract_string_value(array, row_idx)
                })
                .collect();
            output.push_str(&row_values.join("\t"));
            output.push('\n');
        }
    }
    output.trim_end().to_string()
}

fn extract_string_value(array: &arrow::array::ArrayRef, row_idx: usize) -> String {
    use arrow::array::*;

    if let Some(string_array) = array.as_any().downcast_ref::<StringArray>() {
        string_array.value(row_idx).to_string()
    } else if let Some(string_array) = array.as_any().downcast_ref::<LargeStringArray>() {
        string_array.value(row_idx).to_string()
    } else if let Some(int_array) = array.as_any().downcast_ref::<Int64Array>() {
        int_array.value(row_idx).to_string()
    } else if let Some(double_array) = array.as_any().downcast_ref::<Float64Array>() {
        double_array.value(row_idx).to_string()
    } else if let Some(bool_array) = array.as_any().downcast_ref::<BooleanArray>() {
        bool_array.value(row_idx).to_string()
    } else {
        format!("{:?}", array.to_data())
    }
}

// #[allow(unused_macros)]
macro_rules! add_parser_tests {
    ($dataset:expr, [ $($query:expr),* ]) => {
        paste! {
            $(
                #[test]
                fn [<parse_ $dataset _ $query>]() {
                    let _guard = setup("parser", concat!("../gql/", $dataset, "/"));
                    let test_cases = include_str!(concat!("../gql/", $dataset, "/", $query, ".gql"));
                    let statements = preprocess_statements(test_cases);
                    let results: Vec<_> = statements.iter().map(|statement| parse_gql(statement)).collect();
                    assert_yaml_snapshot!($query, results);
                }
            )*
        }
    }
}

macro_rules! add_e2e_tests {
    ($dataset:expr, [ $($query:expr),* ]) => {
        paste! {
            $(
                #[test]
                fn [<e2e_ $dataset _ $query>]() {
                    let _guard = setup("e2e", concat!("../gql/", $dataset, "/"));
                    let test_cases = include_str!(concat!("../gql/", $dataset, "/", $query, ".gql"));
                    let statements = preprocess_statements(test_cases);
                    let result = query_e2e_test(statements);
                    assert_snapshot!($query, &result);
                }
            )*
        }
    }
}

add_e2e_tests!("basic", ["multi_statement_test"]);
add_e2e_tests!("finbench", ["tsr1", "tsr2", "tsr3", "tsr4", "tsr5", "tsr6"]);
add_e2e_tests!("snb", ["is1", "is2", "is3", "is4", "is5", "is6", "is7"]);
add_e2e_tests!("opengql", [
    "create_graph",
    "create_schema",
    "insert",
    "match_and_insert",
    "match",
    "session_set"
]);
add_e2e_tests!("misc", [
    "text2graph",
    "ddl_drop",
    "ddl_truncate",
    "dml_dql",
    "vector_index"
]);
add_e2e_tests!("utility", [
    "explain_call",
    "explain_filter",
    "explain_limit",
    "explain_logical_match",
    "explain_one_row",
    "explain_sort",
    "explain_vector_index_scan"
]);

add_parser_tests!("basic", ["multi_statement_test"]);
add_parser_tests!("finbench", ["tsr1", "tsr2", "tsr3", "tsr4", "tsr5", "tsr6"]);
add_parser_tests!("snb", ["is1", "is2", "is3", "is4", "is5", "is6", "is7"]);
add_parser_tests!("opengql", [
    "create_graph",
    "create_schema",
    "insert",
    "match_and_insert",
    "match",
    "session_set"
]);
add_parser_tests!("misc", [
    "text2graph",
    "ddl_drop",
    "ddl_truncate",
    "dml_dql",
    "vector_index"
]);
add_parser_tests!("utility", [
    "explain_call",
    "explain_filter",
    "explain_limit",
    "explain_logical_match",
    "explain_one_row",
    "explain_sort",
    "explain_vector_index_scan"
]);
