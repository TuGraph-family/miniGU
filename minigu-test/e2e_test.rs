//! This file defines end-to-end tests for miniGU.
//!
//! Test cases can be found in `../../resources/gql`, and expected outputs can be found in
//! `snapshots`.
use insta::internals::SettingsBindDropGuard;
use insta::{Settings, assert_yaml_snapshot};
use pastey::paste;
use arrow::array::Array;
use minigu::database::{Database, DatabaseConfig};
use minigu::result::QueryResult;

fn setup(snapshot_path: &str) -> SettingsBindDropGuard {
    let mut settings = Settings::clone_current();
    settings.set_snapshot_path(snapshot_path);
    settings.set_snapshot_suffix("e2e");
    settings.set_omit_expression(true);
    settings.set_prepend_module_to_snapshot(false);
    settings.bind_to_scope()
}

fn query_executor(input: &str) -> String {
    let config = DatabaseConfig::default();
    let database = Database::open_in_memory(&config).unwrap();
    let mut session = database.session().unwrap();
    let result = session.query(input).unwrap();
    result_to_string(&result)
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
        for _row_idx in 0..rows_count {
            let row_values: Vec<String> = (0..column.len())
                .map(|col_idx| {
                    let array = &column[col_idx];
                    format!("{:?}", array.to_data())
                })
                .collect();
            output.push_str(&row_values.join("\t"));
            output.push('\n');
        }
    }
    output.trim_end().to_string()
}

macro_rules! add_e2e_tests {
    ($dataset:expr, [ $($query:expr),* ]) => {
        paste! {
            $(
                #[test]
                fn [<e2e_ $dataset _ $query>]() {
                    let _guard = setup(concat!("gql/", $dataset, "/", $query, "/"));
                    let query_str = include_str!(concat!("gql/", $dataset, "/", $query, "/", $query, ".gql"));
                    assert_yaml_snapshot!($query, query_executor(query_str));
                }
            )*
        }
    }
}

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
add_e2e_tests!("gql_on_one_page", ["gql_on_one_page"]);
add_e2e_tests!("misc", [
    "ddl_drop",
    "ddl_truncate",
    "dml_dql",
    "vector_index"
]);
