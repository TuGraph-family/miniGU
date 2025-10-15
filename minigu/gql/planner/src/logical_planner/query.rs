use std::sync::Arc;
use minigu_common::data_type::DataSchemaRef;
use minigu_common::error::{not_implemented, NotImplemented};

use crate::bound::{BoundCompositeQueryStatement, BoundGraphPatternBindingTable, BoundLinearQueryStatement, BoundMatchStatement, BoundOrderByAndPageStatement, BoundResultStatement, BoundReturnStatement, BoundSimpleQueryStatement};
use crate::error::{PlanError, PlanResult};
use crate::logical_planner::LogicalPlanner;
use crate::plan::PlanNode;
use crate::plan::limit::Limit;
use crate::plan::logical_match::{LogicalMatch, MatchKind};
use crate::plan::one_row::OneRow;
use crate::plan::project::Project;
use crate::plan::sort::Sort;

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
            BoundSimpleQueryStatement::Match(statement) => {
                self.plan_match_statement(statement)
            }
        }
    }
    
    pub fn plan_match_statement(
        &self,
        statement: BoundMatchStatement,
    ) -> PlanResult<PlanNode> {
        match statement {
            BoundMatchStatement::Simple(binding) => {
                let node = LogicalMatch::new(
                    MatchKind::Simple, binding.pattern,binding.yield_clause, binding.output_schema
                );
                Ok(PlanNode::LogicalMatch(Arc::new(node)))
            },
            BoundMatchStatement::Optional => {
                not_implemented("match statement optional", None)
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
        if let Some(limit) = statement.limit {
            let limit = Limit::new(plan, limit);
            plan = PlanNode::LogicalLimit(Arc::new(limit));
        }
        Ok(plan)
    }
}
