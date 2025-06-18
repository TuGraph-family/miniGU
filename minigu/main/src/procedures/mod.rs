mod echo;

use minigu_context::procedure::Procedure;

pub fn build_predefined_procedures() -> Vec<(String, Procedure)> {
    vec![("echo".to_string(), echo::build_procedure())]
}
