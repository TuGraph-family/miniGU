//! This file defines end-to-end tests for miniGU.
//!
//! Test cases can be found in `../../resources/gql`, and expected outputs can be found in
//! `snapshots`.
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

fn setup(suffix: &str, snapshot_path: &str) -> SettingsBindDropGuard {
    let mut settings = Settings::clone_current();
    settings.set_snapshot_path(snapshot_path);
    settings.set_snapshot_suffix(suffix);
    settings.set_omit_expression(true);
    settings.set_prepend_module_to_snapshot(false);
    settings.bind_to_scope()
}

fn query_executor(input: &str) -> String {
    let config = DatabaseConfig::default();
    let database = Database::open_in_memory(&config).unwrap();
    let mut session = database.session().unwrap();
    match session.query(input) {
        Ok(result) => result_to_string(&result),
        Err(e) => {
            let debug_str = format!("{:#?}", e);
            let display_str = debug_str.replace("\\n", "\n");
            format!("Error: {}", display_str)
        }
    }
}

fn result_to_string(result: &QueryResult) -> String {
    if result.iter().count() == 0 {
        return "No results".to_string();
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

macro_rules! add_e2e_tests {
    ($dataset:expr, [ $($query:expr),* ]) => {
        paste! {
            $(
                #[test]
                fn [<e2e_ $dataset _ $query>]() {
                    let _guard = setup("e2e", concat!("../gql/", $dataset, "/"));
                    let query_str = include_str!(concat!("../gql/", $dataset, "/", $query, ".gql"));
                    let result = query_executor(query_str);
                    assert_snapshot!($query, &result);
                }
            )*
        }
    }
}

macro_rules! add_tests {
    ($dataset:expr, [ $($query:expr),* ]) => {
        paste! {
            $(
                #[test]
                fn [<parse_ $dataset _ $query>]() {
                    let _guard = setup("parser", concat!("../gql/", $dataset, "/"));
                    let query_str = include_str!(concat!("../gql/", $dataset, "/", $query, ".gql"));
                    assert_yaml_snapshot!($query, parse_gql(query_str));
                }

                #[test]
                fn [<e2e_ $dataset _ $query>]() {
                    let _guard = setup("e2e", concat!("../gql/", $dataset, "/"));
                    let query_str = include_str!(concat!("../gql/", $dataset, "/", $query, ".gql"));
                    let result = query_executor(query_str);
                    assert_snapshot!($query, &result);
                }
            )*
        }
    }
}

add_tests!("finbench", ["tsr1", "tsr2", "tsr3", "tsr4", "tsr5", "tsr6"]);
add_tests!("snb", ["is1", "is2", "is3", "is4", "is5", "is6", "is7"]);
add_tests!("opengql", [
    "create_graph",
    "create_schema",
    "insert",
    "match_and_insert",
    "match",
    "session_set"
]);
add_tests!("gql_on_one_page", ["gql_on_one_page"]);
add_e2e_tests!("misc", [
    "ddl_drop",
    "ddl_truncate",
    "dml_dql",
    "vector_index"
]);
add_tests!("utility", [
    "explain_call",
    "explain_filter",
    "explain_limit",
    "explain_logical_match",
    "explain_one_row",
    "explain_sort",
    "explain_vector_index_scan"
]);
