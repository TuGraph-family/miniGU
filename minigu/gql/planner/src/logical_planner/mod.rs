mod catalog;
mod procedure_call;
mod procedure_spec;
mod query;

use crate::bound::BoundProcedure;
use crate::error::PlanResult;
use crate::plan::PlanNode;

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
