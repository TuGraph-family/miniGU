use minigu_context::procedure::ProcedureContainer;

mod echo;

pub fn build_predefined_procedures() -> Vec<(String, ProcedureContainer)> {
    vec![("echo".into(), echo::echo())]
}
