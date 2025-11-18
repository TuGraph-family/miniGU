mod GCard;
mod build_ldbc_mainfest;
mod create_test_graph;
mod create_test_graph_data;
mod echo;
mod export_import;
mod show_graph;
mod show_procedures;

use minigu_context::procedure::Procedure;

pub fn build_predefined_procedures() -> Vec<(String, Procedure)> {
    vec![
        ("echo".to_string(), echo::build_procedure()),
        (
            "show_procedures".to_string(),
            show_procedures::build_procedure(),
        ),
        (
            "create_test_graph".to_string(),
            create_test_graph::build_procedure(),
        ),
        (
            "create_test_graph_data".to_string(),
            create_test_graph_data::build_procedure(),
        ),
        // Show graph in current schema.
        ("show_graph".to_string(), show_graph::build_procedure()),
        (
            "import".to_string(),
            export_import::import::build_procedure(),
        ),
        (
            "export".to_string(),
            export_import::export::build_procedure(),
        ),
        ("GCard_build".to_string(), GCard::build_procedure()),
        (
            "build_ldbc_manifest".to_string(),
            build_ldbc_mainfest::build_procedure(),
        ),
    ]
}
