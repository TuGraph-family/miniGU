use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

use gql_parser::ast::{
    GraphExpr, Procedure, ProgramActivity, SessionActivity, SessionResetArgs, SessionSet,
    TransactionActivity,
};
use gql_parser::parse_gql;
use itertools::Itertools;
use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
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

#[cfg(not(target_arch = "wasm32"))]
#[inline]
fn instant_now() -> Instant {
    Instant::now()
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn instant_now() -> () {
    ()
}

#[cfg(not(target_arch = "wasm32"))]
#[inline]
fn instant_elapsed(start: Instant) -> Duration {
    start.elapsed()
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn instant_elapsed(_: ()) -> Duration {
    Duration::ZERO
}

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
        let start = instant_now();
        let program = parse_gql(query)?;
        let parsing_time = instant_elapsed(start);
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
                        self.context.set_current_graph(graph_name.to_string())?;
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

        let start = instant_now();
        let planner = Planner::new(self.context.clone());
        let plan = planner.plan_query(procedure)?;
        metrics.planning_time = instant_elapsed(start);

        let schema = plan.schema().cloned();
        let start = instant_now();
        let chunks: Vec<_> = self.context.database().runtime().install(|| {
            let mut executor = ExecutorBuilder::new(self.context.clone()).build(&plan);
            executor.into_iter().try_collect()
        })?;
        metrics.execution_time = instant_elapsed(start);

        Ok(QueryResult {
            schema,
            metrics,
            chunks,
        })
    }

    // Test-harness helper: import a graph from an export manifest, then set it as current graph.
    //
    // This is intended for integration/system tests (e.g. `minigu-test`)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn import_graph<P: AsRef<Path>>(
        &mut self,
        graph_name: &str,
        manifest_path: P,
    ) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        use crate::procedures::import;

        import(self.context.clone(), graph_name, manifest_path)?;
        // For simplicity, set the current graph to `graph_name`.
        self.context.set_current_graph(graph_name.to_string())?;
        Ok(())
    }
}
