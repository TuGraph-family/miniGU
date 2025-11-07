use std::sync::Arc;
use std::time::Instant;

use gql_parser::ast::{
    GraphExpr, Procedure, ProgramActivity, SessionActivity, SessionResetArgs, SessionSet,
    TransactionActivity,
};
use gql_parser::parse_gql;
use itertools::Itertools;
use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_common::error::not_implemented;
use minigu_context::database::DatabaseContext;
use minigu_context::session::SessionContext;
use minigu_execution::builder::ExecutorBuilder;
use minigu_execution::executor::Executor;
use minigu_planner::Planner;
use minigu_planner::plan::PlanData;

use crate::error::{Error, Result};
use crate::metrics::QueryMetrics;
use crate::result::QueryResult;

pub struct Session {
    context: SessionContext,
    closed: bool,
}

impl Session {
    pub(crate) fn new(
        database: Arc<DatabaseContext>,
        default_schema: Arc<MemorySchemaCatalog>,
    ) -> Result<Self> {
        let mut context = SessionContext::new(database);
        context.home_schema = Some(default_schema.clone());
        context.current_schema = Some(default_schema);
        Ok(Self {
            context,
            closed: false,
        })
    }

    pub fn query(&mut self, query: &str) -> Result<QueryResult> {
        if self.closed {
            return Err(Error::SessionClosed);
        }
        let start = Instant::now();
        let program = parse_gql(query)?;
        let parsing_time = start.elapsed();
        let mut result = program
            .value()
            .activity
            .as_ref()
            .map(|activity| match activity.value() {
                ProgramActivity::Session(activity) => self.handle_session_activity(activity),
                ProgramActivity::Transaction(activity) => {
                    self.handle_transaction_activity(activity)
                }
            })
            .transpose()?
            .unwrap_or_default();
        result.metrics.parsing_time = parsing_time;
        if program.value().session_close {
            self.closed = true;
        }
        Ok(result)
    }

    fn handle_session_activity(&mut self, activity: &SessionActivity) -> Result<QueryResult> {
        for s in &activity.set {
            let set = s.value();
            match &set {
                SessionSet::Schema(sp_ref) => {
                    self.context.set_current_schema(sp_ref.value().clone())?;
                }
                SessionSet::Graph(sp_ref) => match sp_ref.value() {
                    GraphExpr::Name(graph_name) => {
                        self.context.set_current_graph(graph_name.to_string());
                    }
                    _ => {
                        return not_implemented("not allowed there", None);
                    }
                },
                _ => {
                    return not_implemented("not implemented ", None);
                }
            }
        }
        for reset in &activity.reset {
            let reset = reset.value();
            if let Some(args) = &reset.0 {
                let arg = args.value();
                match arg {
                    SessionResetArgs::Schema => {
                        self.context.reset_current_schema();
                    }
                    SessionResetArgs::Graph => {
                        self.context.reset_current_graph();
                    }
                    _ => {
                        return not_implemented("not allowed there", None);
                    }
                }
            }
        }
        Ok(QueryResult::default())
    }

    fn handle_transaction_activity(&self, activity: &TransactionActivity) -> Result<QueryResult> {
        if activity.start.is_some() {
            return not_implemented("start transaction", None);
        }
        if activity.end.is_some() {
            return not_implemented("end transaction", None);
        }
        let result = activity
            .procedure
            .as_ref()
            .map(|procedure| self.handle_procedure(procedure.value()))
            .transpose()?
            .unwrap_or_default();
        Ok(result)
    }

    fn handle_procedure(&self, procedure: &Procedure) -> Result<QueryResult> {
        let mut metrics = QueryMetrics::default();

        let start = Instant::now();
        let planner = Planner::new(self.context.clone());
        let physical_plan = planner.plan_query(procedure)?;
        metrics.planning_time = start.elapsed();

        let schema = physical_plan.schema().cloned();
        let start = Instant::now();
        let chunks: Vec<_> = self.context.database().runtime().scope(|_| {
            let mut executor = ExecutorBuilder::new(self.context.clone()).build(&physical_plan);
            executor.into_iter().try_collect()
        })?;
        metrics.execution_time = start.elapsed();

        Ok(QueryResult {
            schema,
            metrics,
            chunks,
        })
    }
}
