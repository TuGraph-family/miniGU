use gql_parser::ast::Procedure;
use minigu_catalog::txn::catalog_txn::CatalogTxn;
use minigu_context::session::SessionContext;

use crate::binder::Binder;
use crate::error::PlanResult;
use crate::logical_planner::LogicalPlanner;
use crate::optimizer::Optimizer;
use crate::plan::PlanNode;
pub mod binder;
pub mod bound;
pub mod error;
mod logical_planner;
mod optimizer;
pub mod plan;

pub struct Planner<'a> {
    context: SessionContext,
    active_txn: Option<&'a CatalogTxn>,
}

impl<'a> Planner<'a> {
    pub fn new(context: SessionContext) -> Self {
        Self {
            context,
            active_txn: None,
        }
    }

    pub fn with_txn(mut self, txn: &'a CatalogTxn) -> Self {
        self.active_txn = Some(txn);
        self
    }

    pub fn plan_query(&mut self, query: &Procedure) -> PlanResult<PlanNode> {
        let database = self.context.database_arc();
        let catalog = database.catalog();
        let current_schema = self.context.current_schema.clone().map(|s| s as _);
        let home_schema = self.context.home_schema.clone().map(|s| s as _);
        let current_graph = self.context.current_graph.clone();
        let home_graph = self.context.home_graph.clone();

        let bind_and_optimize = |txn: &CatalogTxn| -> PlanResult<PlanNode> {
            let binder = Binder::new(
                catalog,
                current_schema.clone(),
                home_schema.clone(),
                current_graph.clone(),
                home_graph.clone(),
                txn,
            );
            let bound = binder.bind(query)?;
            let logical_plan = LogicalPlanner::new().create_logical_plan(bound.clone())?;
            Optimizer::new().create_physical_plan(&logical_plan)
        };

        if let Some(active_txn) = self.active_txn {
            return bind_and_optimize(active_txn);
        }

        if let Some(active_txn) = self.context.current_txn() {
            return bind_and_optimize(active_txn.as_ref());
        }

        self.context
            .with_statement_result(|txn| bind_and_optimize(txn))
    }
}
