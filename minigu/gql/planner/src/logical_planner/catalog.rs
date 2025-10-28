use std::sync::Arc;

use crate::bound::BoundCatalogModifyingStatement;
use crate::error::PlanResult;
use crate::logical_planner::LogicalPlanner;
use crate::plan::PlanNode;
use crate::plan::ddl::CatalogDdl;

impl LogicalPlanner {
    pub fn plan_catalog_modifying_statement(
        &self,
        statement: BoundCatalogModifyingStatement,
    ) -> PlanResult<PlanNode> {
        match statement {
            BoundCatalogModifyingStatement::Call(call) => self.plan_call_procedure_statement(call),
            other => {
                let ddl = CatalogDdl::new(other);
                Ok(PlanNode::LogicalCatalogDdl(Arc::new(ddl)))
            }
        }
    }
}
