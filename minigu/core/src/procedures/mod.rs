mod create_test_graph;
mod echo;
mod export_import;
mod show_procedures;
mod create_test_graph_data;


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
        (
            "import".to_string(),
            export_import::import::build_procedure(),
        ),
        (
            "export".to_string(),
            export_import::export::build_procedure(),
        ),
    ]
}
