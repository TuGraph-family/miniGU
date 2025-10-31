use std::sync::Arc;
use std::time::{Duration, Instant};

use arrow::array::create_array;
use gql_parser::ast::{Procedure, Program, ProgramActivity, SessionActivity, TransactionActivity};
use gql_parser::parse_gql;
use itertools::Itertools;
use minigu_catalog::memory::MemoryCatalog;
use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_catalog::provider::SchemaRef;
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

    fn handle_session_activity(&self, activity: &SessionActivity) -> Result<QueryResult> {
        not_implemented("session activity", None)
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
        let plan = planner.plan_query(procedure)?;
        let is_logical_plan = matches!(
            plan,
            minigu_planner::plan::PlanNode::LogicalMatch(_)
                | minigu_planner::plan::PlanNode::LogicalFilter(_)
                | minigu_planner::plan::PlanNode::LogicalProject(_)
                | minigu_planner::plan::PlanNode::LogicalSort(_)
                | minigu_planner::plan::PlanNode::LogicalLimit(_)
                | minigu_planner::plan::PlanNode::LogicalCall(_)
                | minigu_planner::plan::PlanNode::LogicalOneRow(_)
        );
        if is_logical_plan {
            return self.format_explain(plan); // return explain output string for logical plan
        }
        metrics.planning_time = start.elapsed();

        let schema = plan.schema().cloned();
        let start = Instant::now();
        let chunks: Vec<_> = self.context.database().runtime().scope(|_| {
            let mut executor = ExecutorBuilder::new(self.context.clone()).build(&plan);
            executor.into_iter().try_collect()
        })?;
        metrics.execution_time = start.elapsed();

        Ok(QueryResult {
            schema,
            metrics,
            chunks,
        })
    }

    fn format_explain(&self, plan: impl PlanData) -> Result<QueryResult> {
        let mut output = String::new();
        self.format_explain_output(&plan, &mut output, 0);

        let schema = Some(Arc::new(DataSchema::new(vec![DataField::new(
            "EXPLAIN".to_string(),
            LogicalType::String,
            false,
        )])));

        let chunks = vec![DataChunk::new(vec![Arc::new(
            arrow::array::StringArray::from(vec![output]),
        )])];
        Ok(QueryResult {
            schema,
            metrics: QueryMetrics::default(),
            chunks,
        })
    }

    #[allow(clippy::only_used_in_recursion)]
    fn format_explain_output(&self, plan: &impl PlanData, output: &mut String, indent: usize) {
        let indent_str = " ".repeat(indent * 2);
        let explain_str = plan.explain().unwrap_or_else(|| "ERROR".to_string());
        output.push_str(&format!("{}{}\n", indent_str, explain_str));
        for child in plan.children() {
            self.format_explain_output(child, output, indent + 1);
        }
    }
}
