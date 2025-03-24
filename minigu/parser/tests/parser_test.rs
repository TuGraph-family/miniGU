use gql_parser::parse_gql;
use insta::internals::SettingsBindDropGuard;
use insta::{Settings, assert_yaml_snapshot};
use paste::paste;

fn setup(snapshot_path: &str) -> SettingsBindDropGuard {
    let mut settings = Settings::clone_current();
    settings.set_snapshot_path(snapshot_path);
    settings.set_omit_expression(true);
    settings.set_prepend_module_to_snapshot(false);
    settings.bind_to_scope()
}

macro_rules! add_parser_tests {
    ($dataset:expr, [ $($query:expr),* ]) => {
        paste! {
            $(
                #[test]
                fn [<parse_ $dataset _ $query>]() {
                    let _guard = setup(concat!("snapshots/", $dataset));
                    let query_str = include_str!(concat!("../../../resources/gql/", $dataset, "/", $query, ".gql"));
                    assert_yaml_snapshot!($query, parse_gql(query_str));
                }
            )*
        }
    }
}

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
add_parser_tests!("gql_on_one_page", ["gql_on_one_page"]);
