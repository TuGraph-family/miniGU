use std::sync::Arc;

use minigu_common::error::not_implemented;
use minigu_ir::bound::{
    BoundCompositeQueryStatement, BoundLinearQueryStatement, BoundResultStatement,
    BoundReturnStatement, BoundSimpleQueryStatement,
};
use minigu_ir::plan::{PlanNode, Project};

use crate::error::PlanResult;
use crate::logical_planner::LogicalPlanner;

impl LogicalPlanner {
    pub fn plan_composite_query_statement(
        &self,
        statement: BoundCompositeQueryStatement,
    ) -> PlanResult<PlanNode> {
        match statement {
            BoundCompositeQueryStatement::Conjunction { .. } => {
                not_implemented("query conjunction", None)
            }
            BoundCompositeQueryStatement::Primary(statement) => {
                self.plan_linear_query_statement(statement)
            }
        }
    }

    pub fn plan_linear_query_statement(
        &self,
        statement: BoundLinearQueryStatement,
    ) -> PlanResult<PlanNode> {
        match statement {
            BoundLinearQueryStatement::Query {
                mut statements,
                result,
            } => {
                if statements.len() > 1 {
                    return not_implemented("multiple statements", None);
                }
                if statements.is_empty() {
                    return not_implemented("result-only query", None);
                }
                let statement = statements
                    .pop()
                    .expect("at least one statement should exist");
                let plan = self.plan_simple_query_statement(statement)?;
                self.plan_result_statement(result, plan)
            }
            BoundLinearQueryStatement::Nested(_) => not_implemented("nested query", None),
            BoundLinearQueryStatement::Select => not_implemented("select statement", None),
        }
    }

    pub fn plan_simple_query_statement(
        &self,
        statement: BoundSimpleQueryStatement,
    ) -> PlanResult<PlanNode> {
        match statement {
            BoundSimpleQueryStatement::Call(statement) => {
                self.plan_call_procedure_statement(statement)
            }
        }
    }

    pub fn plan_result_statement(
        &self,
        statement: BoundResultStatement,
        plan: PlanNode,
    ) -> PlanResult<PlanNode> {
        match statement {
            BoundResultStatement::Return {
                statement,
                order_by,
            } => {
                if order_by.is_some() {
                    return not_implemented("order by", None);
                }
                self.plan_return_statement(statement, plan)
            }
            BoundResultStatement::Finish => not_implemented("finish statement", None),
        }
    }

    pub fn plan_return_statement(
        &self,
        statement: BoundReturnStatement,
        plan: PlanNode,
    ) -> PlanResult<PlanNode> {
        if statement.quantifier.is_some() {
            return not_implemented("set quantifier in return statement", None);
        }
        let project = Project::new(plan, statement.items);
        Ok(PlanNode::LogicalProject(Arc::new(project)))
    }
}
