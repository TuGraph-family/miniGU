use gql_parser::ast::Procedure;
use minigu_catalog::txn::error::CatalogTxnError;
use minigu_context::session::SessionContext;

use crate::binder::Binder;
use crate::error::{PlanError, PlanResult};
use crate::logical_planner::LogicalPlanner;
use crate::optimizer::Optimizer;
use crate::plan::PlanNode;
pub mod binder;
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

    pub fn plan_query(&mut self, query: &Procedure) -> PlanResult<PlanNode> {
        let snapshot = self.context.clone();
        let catalog = snapshot.database().catalog();
        let current_schema = snapshot.current_schema.clone().map(|s| s as _);
        let home_schema = snapshot.home_schema.clone().map(|s| s as _);
        let current_graph = snapshot.current_graph.clone();
        let home_graph = snapshot.home_graph.clone();

        let mut plan_out: Option<PlanNode> = None;
        let res = self.context.with_statement_txn(|txn| {
            let binder = Binder::new(
                catalog,
                current_schema.clone(),
                home_schema.clone(),
                current_graph.clone(),
                home_graph.clone(),
                txn,
            );
            let bound = binder
                .bind(query)
                .map_err(|e| CatalogTxnError::External(Box::new(e)))?;
            let logical_plan = LogicalPlanner::new()
                .create_logical_plan(bound)
                .map_err(|e| CatalogTxnError::External(Box::new(e)))?;
            let plan = Optimizer::new()
                .create_physical_plan(&logical_plan)
                .map_err(|e| CatalogTxnError::External(Box::new(e)))?;
            plan_out = Some(plan);
            Ok(())
        });
        match res {
            Ok(()) => Ok(plan_out.expect("planner should produce a plan")),
            Err(CatalogTxnError::External(e)) => match e.downcast::<PlanError>() {
                Ok(pe) => Err(*pe),
                Err(e) => Err(PlanError::Transaction(CatalogTxnError::External(e))),
            },
            Err(e) => Err(PlanError::Transaction(e)),
        }
    }
}
