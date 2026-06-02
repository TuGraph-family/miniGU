use std::sync::Arc;

use crate::bound::BoundDataModifyingStatement;
use crate::error::PlanResult;
use crate::logical_planner::LogicalPlanner;
use crate::plan::PlanNode;
use crate::plan::data_modify::Insert;

impl LogicalPlanner {
    pub fn plan_data_modifying_statement(
        &self,
        statement: BoundDataModifyingStatement,
    ) -> PlanResult<PlanNode> {
        match statement {
            BoundDataModifyingStatement::Insert(insert) => {
                Ok(PlanNode::PhysicalInsert(Arc::new(Insert::new(insert))))
            }
        }
    }
}
