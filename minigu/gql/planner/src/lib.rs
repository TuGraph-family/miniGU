use gql_parser::ast::Procedure;
use minigu_context::session::SessionContext;

use crate::binder::Binder;
use crate::error::PlanResult;
use crate::logical_planner::LogicalPlanner;
use crate::optimizer::Optimizer;
use crate::plan::PlanNode;

mod binder;
pub mod bound;
pub mod error;
mod logical_planner;
mod optimizer;
pub mod plan;

pub struct Planner {
    context: SessionContext,
}

impl Planner {
    pub fn new(context: SessionContext) -> Self {
        Self { context }
    }

    pub fn plan_query(&self, query: &Procedure) -> PlanResult<PlanNode> {
        let binder = Binder::new(
            self.context.database().catalog(),
            self.context.current_schema.clone().map(|s| s as _),
            self.context.home_schema.clone().map(|s| s as _),
            self.context.current_graph.clone(),
            self.context.home_graph.clone(),
        );
        let bound = binder.bind(query)?;
        let logical_plan = LogicalPlanner::new().create_logical_plan(bound)?;
        Optimizer::new().create_physical_plan(&logical_plan)
    }
}
