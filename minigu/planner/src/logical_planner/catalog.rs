use minigu_ir::bound::BoundCatalogModifyingStatement;
use minigu_ir::plan::PlanNode;

use crate::error::PlanResult;
use crate::logical_planner::LogicalPlanner;

impl LogicalPlanner {
    pub fn plan_catalog_modifying_statement(
        &self,
        statement: BoundCatalogModifyingStatement,
    ) -> PlanResult<PlanNode> {
        match statement {
            BoundCatalogModifyingStatement::Call(call) => self.plan_call_procedure_statement(call),
            _ => todo!(),
        }
    }
}
