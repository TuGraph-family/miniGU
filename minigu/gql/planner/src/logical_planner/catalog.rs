use std::sync::Arc;

use crate::bound::{
    BoundCatalogModifyingStatement, BoundCreateIndexStatement, BoundDropIndexStatement,
};
use crate::error::PlanResult;
use crate::logical_planner::LogicalPlanner;
use crate::plan::PlanNode;
use crate::plan::create_index::CreateIndex;
use crate::plan::drop_index::DropIndex;

impl LogicalPlanner {
    pub fn plan_catalog_modifying_statement(
        &self,
        statement: BoundCatalogModifyingStatement,
    ) -> PlanResult<PlanNode> {
        match statement {
            BoundCatalogModifyingStatement::Call(call) => self.plan_call_procedure_statement(call),
            BoundCatalogModifyingStatement::CreateIndex(statement) => {
                let BoundCreateIndexStatement {
                    name,
                    if_not_exists,
                    index_key,
                    metric,
                    dimension,
                    label,
                    property,
                    no_op,
                } = statement;
                let plan = CreateIndex::new(
                    name,
                    if_not_exists,
                    index_key,
                    metric,
                    dimension,
                    label,
                    property,
                    no_op,
                );
                Ok(PlanNode::LogicalCreateIndex(Arc::new(plan)))
            }
            BoundCatalogModifyingStatement::DropIndex(statement) => {
                let BoundDropIndexStatement {
                    name,
                    if_exists,
                    index_key,
                    metadata,
                    no_op,
                } = statement;
                let plan = DropIndex::new(name, if_exists, index_key, metadata, no_op);
                Ok(PlanNode::LogicalDropIndex(Arc::new(plan)))
            }
            _ => todo!(),
        }
    }
}
