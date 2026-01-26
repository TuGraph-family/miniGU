//! This file defines end-to-end tests for miniGU.
//!
//! Test cases can be found in `../../resources/gql`, and expected outputs can be found in
//! `snapshots`.
//!
//! # How to add an E2E test
//!
//! 1. Add a test case file:
//! - `minigu-test/gql/<dataset>/<query>.gql`
//! - You can write multiple statements; statements are split by a trailing `;` on a line.
//! - Full-line comments starting with `--` or `//` are skipped.
//!
//! 2. Register the test at the bottom of this file using `add_e2e_tests!`:
//! - No preloaded data: `add_e2e_tests!("<dataset>", ["<query>"]);`
//! - With preloaded graph data (import manifest + set current graph): `add_e2e_tests!("<dataset>",
//!   ["<query>"], ("<graph_name>", "<manifest_dir>"));`
//!
//! 3. Run the test (and optionally update snapshots):
//! - Single test: `cargo test -p minigu-test e2e_<dataset>_<query> -- --nocapture`
//!
//! # Parameters and data layout
//!
//! `add_e2e_tests!("<dataset>", ["<query>"])`:
//! - `<dataset>` maps to directory `minigu-test/gql/<dataset>/`
//! - `<query>` maps to file `minigu-test/gql/<dataset>/<query>.gql`
//!
//! `add_e2e_tests!(..., ("<graph_name>", "<manifest_dir>"))` additionally:
//! - `<graph_name>` is the name to register the imported graph under; the test also sets it as the
//!   current graph for convenience.
//! - `<manifest_dir>` is a directory (relative to this crate root, resolved via
//!   `CARGO_MANIFEST_DIR`) that must contain `manifest.json`.
//! - Paths inside `manifest.json` (e.g. `person.csv`) are interpreted relative to `<manifest_dir>`.
//!
//! NOTE: The harness always creates a per-test temp directory and routes checkpoint/WAL writes into
//! it (see `setup_database()`), to avoid polluting the repo and to prevent cross-test interference.

use std::env;
use std::fmt::Write;
use std::path::Path;

use gql_parser::parse_gql;
use insta::internals::SettingsBindDropGuard;
use insta::{Settings, assert_snapshot, assert_yaml_snapshot};
use minigu::common::data_chunk::display::{TableBuilder, TableOptions};
use minigu::database::{Database, DatabaseConfig};
use minigu::result::QueryResult;
use minigu::session::Session;
use pastey::paste;
use tempfile::{TempDir, tempdir};

const GQL_COMMENT_PREFIX: &str = "--";
const FILE_COMMENT_PREFIX: &str = "//";
const QUERY_END_SUFFIX: &str = ";";

struct SessionDropGuard {
    session: Session,
    // Hold the temp dir for the whole test so checkpoint/WAL paths remain valid and are cleaned
    // up.
    _temp_dir: TempDir,
}

fn setup_insta_settings(suffix: &str, snapshot_path: &str) -> SettingsBindDropGuard {
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

// Setup a database and create a tmp directory to store checkpoints and wal.
fn setup_database() -> SessionDropGuard {
    let temp_dir = tempdir().unwrap();
    let temp_dir_path = temp_dir.path();

    let config = DatabaseConfig {
        // Ensure checkpoint/WAL writes are test-scoped (no repo pollution, no cross-test clashes).
        db_path: Some(temp_dir_path.join(".minigu")),
        ..Default::default()
    };
    let database = Database::open_in_memory(config).unwrap();
    SessionDropGuard {
        session: database.session().unwrap(),
        _temp_dir: temp_dir,
    }
}

// For simplicity, `path` is the manifest directory, i.e. the manifest file is
// `Path::new(path).join("manifest.json")`.
fn setup_db_with_data(graph_name: &str, path: &str) -> SessionDropGuard {
    let mut guard = setup_database();
    // Resolve test data relative to this crate so tests are independent of process CWD.
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(path)
        .join("manifest.json");
    guard
        .session
        .import_graph(graph_name, manifest_path)
        .unwrap();

    guard
}

fn query_e2e_test(mut session: Session, statements: Vec<String>) -> String {
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
    let options = TableOptions::style_for_test();
    if let Some(schema) = result.schema() {
        let mut builder = TableBuilder::new(Some(schema.clone()), options);
        let mut num_rows = 0;
        for chunk in result.iter() {
            num_rows += chunk.cardinality();
            builder = builder.append_chunk(chunk);
        }
        let table = builder.build();

        writeln!(&mut output, "{table}").unwrap();
        writeln!(&mut output, "{num_rows} rows").unwrap();
    } else {
        // Handle result sets without a schema (e.g. EXPLAIN output).
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
    }
    output
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

macro_rules! add_parser_tests {
    ($dataset:expr, [ $($query:expr),* ]) => {
        paste! {
            $(
                #[test]
                fn [<parse_ $dataset _ $query>]() {
                    let _guard = setup_insta_settings("parser", concat!("../gql/", $dataset, "/"));
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
                    let _guard = setup_insta_settings("e2e", concat!("../gql/", $dataset, "/"));
                    let test_cases = include_str!(concat!("../gql/", $dataset, "/", $query, ".gql"));
                    let statements = preprocess_statements(test_cases);
                    let session_guard = setup_database();
                    let result = query_e2e_test(session_guard.session, statements);
                    assert_snapshot!($query, &result);
                }
            )*
        }
    };
    ($dataset:expr, [ $($query:expr),* ], ($graph_name:expr, $manifest_dir:expr)) => {
        paste! {
            $(
                #[test]
                fn [<e2e_ $dataset _ $query>]() {
                    let _guard = setup_insta_settings("e2e", concat!("../gql/", $dataset, "/"));
                    let test_cases = include_str!(concat!("../gql/", $dataset, "/", $query, ".gql"));
                    let statements = preprocess_statements(test_cases);
                    let session_guard = setup_db_with_data($graph_name, $manifest_dir);
                    let result = query_e2e_test(session_guard.session, statements);
                    assert_snapshot!($query, &result);
                }
            )*
        }
    }
}

add_e2e_tests!("basic", ["multi_statement_test"]);
add_e2e_tests!("basic", ["data_setup_example"], ("test", "data/basic"));
add_e2e_tests!("finbench", ["tsr1", "tsr2", "tsr3", "tsr4", "tsr5", "tsr6"]);
add_e2e_tests!("snb", ["is1", "is2", "is3", "is4", "is5", "is6", "is7"]);
add_e2e_tests!(
    "ddl",
    [
        "create_graph",
        "create_schema",
        "create_drop_vector_index",
        "ddl_drop",
        "ddl_truncate"
    ]
);
add_e2e_tests!("dql", ["dql"]);
add_e2e_tests!("dcl", ["session_set"]);
add_e2e_tests!("dml", ["insert", "match_and_insert", "match", "dml_dql"]);
add_e2e_tests!("misc", ["text2graph", "vector_index"]);
add_e2e_tests!(
    "utility",
    [
        "explain_call",
        "explain_create_vector_index",
        "explain_drop_vector_index",
        "explain_filter",
        "explain_limit",
        "explain_logical_match",
        "explain_one_row",
        "explain_sort",
        "explain_vector_index_scan"
    ]
);

add_parser_tests!("basic", ["multi_statement_test"]);
add_parser_tests!("finbench", ["tsr1", "tsr2", "tsr3", "tsr4", "tsr5", "tsr6"]);
add_parser_tests!("snb", ["is1", "is2", "is3", "is4", "is5", "is6", "is7"]);
add_parser_tests!(
    "ddl",
    [
        "create_graph",
        "create_schema",
        "create_drop_vector_index",
        "ddl_drop",
        "ddl_truncate"
    ]
);
add_parser_tests!("dql", ["dql"]);
add_parser_tests!("dcl", ["session_set"]);
add_parser_tests!("dml", ["insert", "match_and_insert", "match", "dml_dql"]);
add_parser_tests!("misc", ["text2graph", "vector_index"]);
add_parser_tests!(
    "utility",
    [
        "explain_call",
        "explain_create_vector_index",
        "explain_drop_vector_index",
        "explain_filter",
        "explain_limit",
        "explain_offset",
        "explain_logical_match",
        "explain_one_row",
        "explain_sort",
        "explain_vector_index_scan"
    ]
);
