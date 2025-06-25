mod create_test_graph;
mod echo;
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
    ]
}
