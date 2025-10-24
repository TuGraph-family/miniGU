use std::sync::Arc;

use minigu_common::error::not_implemented;

use crate::bound::{
    BoundCompositeQueryStatement, BoundLinearQueryStatement, BoundOrderByAndPageStatement,
    BoundResultStatement, BoundReturnStatement, BoundSimpleQueryStatement, BoundVectorIndexScan,
};
use crate::error::PlanResult;
use crate::logical_planner::LogicalPlanner;
use crate::plan::PlanNode;
use crate::plan::limit::Limit;
use crate::plan::one_row::OneRow;
use crate::plan::project::Project;
use crate::plan::sort::Sort;
use crate::plan::vector_index_scan::VectorIndexScan;

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
                let plan = if statements.is_empty() {
                    PlanNode::LogicalOneRow(Arc::new(OneRow::new()))
                } else {
                    let statement = statements
                        .pop()
                        .expect("at least one statement should be present");
                    self.plan_simple_query_statement(statement)?
                };
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
            BoundSimpleQueryStatement::VectorIndexScan(statement) => {
                self.plan_vector_index_scan_statement(statement)
            }
        }
    }

    fn plan_vector_index_scan_statement(
        &self,
        statement: BoundVectorIndexScan,
    ) -> PlanResult<PlanNode> {
        let scan = VectorIndexScan::new(
            statement.binding,
            statement.distance_alias,
            statement.index_key,
            statement.query,
            statement.metric,
            statement.dimension,
            statement.limit,
            statement.approximate,
        );
        Ok(PlanNode::LogicalVectorIndexScan(Arc::new(scan)))
    }

    pub fn plan_result_statement(
        &self,
        statement: BoundResultStatement,
        plan: PlanNode,
    ) -> PlanResult<PlanNode> {
        match statement {
            BoundResultStatement::Return {
                statement,
                order_by_and_page,
            } => {
                let mut plan = self.plan_return_statement(statement, plan)?;
                if let Some(order_by_and_page) = order_by_and_page {
                    plan = self.plan_order_by_and_page_statement(order_by_and_page, plan)?;
                }
                Ok(plan)
            }
            BoundResultStatement::Finish => not_implemented("finish statement", None),
        }
    }

    pub fn plan_return_statement(
        &self,
        statement: BoundReturnStatement,
        mut plan: PlanNode,
    ) -> PlanResult<PlanNode> {
        if statement.quantifier.is_some() {
            return not_implemented("set quantifier in return statement", None);
        }
        if let Some(items) = statement.items {
            let project = Project::new(plan, items, statement.schema);
            plan = PlanNode::LogicalProject(Arc::new(project));
        }
        Ok(plan)
    }

    pub fn plan_order_by_and_page_statement(
        &self,
        statement: BoundOrderByAndPageStatement,
        mut plan: PlanNode,
    ) -> PlanResult<PlanNode> {
        let specs = statement.order_by;
        if !specs.is_empty() {
            let sort = Sort::new(plan, specs);
            plan = PlanNode::LogicalSort(Arc::new(sort));
        }
        if statement.offset.is_some() {
            return not_implemented("offset clause", None);
        }
        if let Some(limit_clause) = statement.limit {
            let limit = Limit::new(plan, limit_clause.count, limit_clause.approximate);
            plan = PlanNode::LogicalLimit(Arc::new(limit));
        }
        Ok(plan)
    }
}
