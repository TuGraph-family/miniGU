mod common;
mod create_test_graph;
mod create_test_graph_data;
mod create_test_vector_graph_data;
mod echo;
#[cfg(not(target_family = "wasm"))]
mod export_graph;
#[cfg(not(target_family = "wasm"))]
mod import_graph;
mod show_graph;
mod show_procedures;

#[cfg(not(target_family = "wasm"))]
pub(crate) use import_graph::import;
use minigu_context::procedure::Procedure;

pub fn build_predefined_procedures() -> Vec<(String, Procedure)> {
    let mut procedures = vec![
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
            "create_test_vector_graph_data".to_string(),
            create_test_vector_graph_data::build_procedure(),
        ),
        // Show graph in current schema.
        ("show_graph".to_string(), show_graph::build_procedure()),
    ];

    #[cfg(not(target_family = "wasm"))]
    {
        procedures.push(("import_graph".to_string(), import_graph::build_procedure()));
        procedures.push(("export_graph".to_string(), export_graph::build_procedure()));
    }

    procedures
}
