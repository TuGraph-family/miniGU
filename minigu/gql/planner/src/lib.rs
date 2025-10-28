use gql_parser::ast::Procedure;
use minigu_context::session::SessionContext;
use minigu_transaction::IsolationLevel;
use minigu_transaction::manager::GraphTxnManager;

use crate::binder::Binder;
use crate::error::{PlanError, PlanResult};
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
        let txn = if let Some(txn) = &self.context.current_txn {
            txn.clone()
        } else {
            self.context
                .catalog_txn_mgr
                .begin_transaction(IsolationLevel::Snapshot)
                .map_err(PlanError::Transaction)?
        };
        let binder = Binder::new(
            self.context.database().catalog(),
            self.context.current_schema.clone().map(|s| s as _),
            self.context.home_schema.clone().map(|s| s as _),
            self.context.current_graph.clone(),
            self.context.home_graph.clone(),
            txn.as_ref(),
        );
        let bound = binder.bind(query)?;
        let logical_plan = LogicalPlanner::new().create_logical_plan(bound)?;
        Optimizer::new().create_physical_plan(&logical_plan)
    }
}
