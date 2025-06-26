mod catalog;
mod procedure_call;
mod procedure_spec;
mod query;

use minigu_ir::bound::BoundProcedure;
use minigu_ir::plan::PlanNode;

use crate::error::PlanResult;

#[derive(Debug, Default)]
pub struct LogicalPlanner {}

impl LogicalPlanner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_logical_plan(self, procedure: BoundProcedure) -> PlanResult<PlanNode> {
        self.plan_procedure(procedure)
    }
}
