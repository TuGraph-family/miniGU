use minigu_common::error::not_implemented;
use minigu_ir::bound::{BoundProcedure, BoundStatement};
use minigu_ir::plan::PlanNode;

use crate::error::PlanResult;
use crate::logical_planner::LogicalPlanner;

impl LogicalPlanner {
    pub fn plan_procedure(&self, procedure: BoundProcedure) -> PlanResult<PlanNode> {
        if !procedure.next_statements.is_empty() {
            return not_implemented("next statements", None);
        }
        let plan = self.plan_statement(procedure.statement)?;
        Ok(plan)
    }

    pub fn plan_statement(&self, statement: BoundStatement) -> PlanResult<PlanNode> {
        match statement {
            BoundStatement::Catalog(mut statements) => {
                assert!(!statements.is_empty());
                if statements.len() > 1 {
                    return not_implemented("multiple catalog modifying statements", None);
                }
                let statement = statements
                    .pop()
                    .expect("at least one statement should be present");
                self.plan_catalog_modifying_statement(statement)
            }
            BoundStatement::Query(statement) => self.plan_composite_query_statement(statement),
        }
    }
}
