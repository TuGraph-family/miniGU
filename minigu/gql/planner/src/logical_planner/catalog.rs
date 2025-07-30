use crate::bound::BoundCatalogModifyingStatement;
use crate::error::PlanResult;
use crate::logical_planner::LogicalPlanner;
use crate::plan::PlanNode;

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
