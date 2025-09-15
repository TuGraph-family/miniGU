use std::str::FromStr;
use std::sync::Arc;

use arrow::array::{AsArray, Int32Array};
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::{DataSchema, LogicalType};
use minigu_common::types::VectorMetric;
use minigu_common::value::ScalarValue;
use minigu_context::session::SessionContext;
use minigu_planner::bound::{BoundExpr, BoundExprKind};
use minigu_planner::plan::{PlanData, PlanNode};

use crate::evaluator::BoxedEvaluator;
use crate::evaluator::column_ref::ColumnRef;
use crate::evaluator::constant::Constant;
use crate::evaluator::vector_distance::VectorDistanceEvaluator;
use crate::executor::procedure_call::ProcedureCallBuilder;
use crate::executor::sort::SortSpec;
use crate::executor::vector_search::VectorSearchBuilder;
use crate::executor::{BoxedExecutor, Executor, IntoExecutor};

const DEFAULT_CHUNK_SIZE: usize = 2048;

pub struct ExecutorBuilder {
    session: SessionContext,
}

impl ExecutorBuilder {
    pub fn new(session: SessionContext) -> Self {
        Self { session }
    }

    pub fn build(self, physical_plan: &PlanNode) -> BoxedExecutor {
        self.build_executor(physical_plan)
    }

    fn build_executor(&self, physical_plan: &PlanNode) -> BoxedExecutor {
        let children = physical_plan.children();
        match physical_plan {
            PlanNode::PhysicalFilter(filter) => {
                assert_eq!(children.len(), 1);
                let schema = children[0].schema().expect("child should have a schema");
                let predicate = self.build_evaluator(&filter.predicate, schema);
                Box::new(self.build_executor(&children[0]).filter(move |c| {
                    predicate
                        .evaluate(c)
                        .map(|a| a.into_array().as_boolean().clone())
                }))
            }
            PlanNode::PhysicalProject(project) => {
                assert_eq!(children.len(), 1);
                let schema = children[0].schema().expect("child should have a schema");
                let evaluators = project
                    .exprs
                    .iter()
                    .map(|e| self.build_evaluator(e, schema))
                    .collect();
                Box::new(self.build_executor(&children[0]).project(evaluators))
            }
            PlanNode::PhysicalCall(call) => {
                assert!(children.is_empty());
                let procedure = call.procedure.object().clone();
                let session = self.session.clone();
                let args = call.args.clone();
                Box::new(ProcedureCallBuilder::new(procedure, session, args).into_executor())
            }
            // We don't need an independent executor for PhysicalOneRow. Returning a chunk with a
            // single row is enough.
            PlanNode::PhysicalOneRow(one_row) => {
                assert!(children.is_empty());
                let schema = &one_row.schema().expect("one_row should have a data schema");
                assert_eq!(schema.fields().len(), 1);
                let field = &schema.fields()[0];
                assert_eq!(field.ty(), &LogicalType::Int32);
                assert!(!field.is_nullable());
                let columns = vec![Arc::new(Int32Array::from_iter_values([0])) as _];
                let chunk = DataChunk::new(columns);
                Box::new([Ok(chunk)].into_executor())
            }
            PlanNode::PhysicalSort(sort) => {
                assert_eq!(children.len(), 1);
                let schema = children[0].schema().expect("child should have a schema");
                let specs = sort
                    .specs
                    .iter()
                    .map(|s| {
                        let key = self.build_evaluator(&s.key, schema);
                        SortSpec::new(key, s.ordering, s.null_ordering)
                    })
                    .collect();
                Box::new(
                    self.build_executor(&children[0])
                        .sort(specs, DEFAULT_CHUNK_SIZE),
                )
            }
            PlanNode::PhysicalLimit(limit) => {
                assert_eq!(children.len(), 1);
                Box::new(self.build_executor(&children[0]).limit(limit.limit))
            }
            PlanNode::PhysicalVectorSearch(vector_search) => {
                assert!(children.is_empty());
                let schema_ref = vector_search
                    .schema()
                    .expect("vector_search should have a schema");
                let query_evaluator =
                    self.build_evaluator(&vector_search.query_vector_expr, schema_ref);
                Box::new(
                    VectorSearchBuilder::new(
                        self.session.clone(),
                        vector_search.index_key,
                        query_evaluator,
                        vector_search.k,
                        vector_search.metric,
                        schema_ref.clone(),
                    )
                    .into_executor(),
                )
            }
            _ => unreachable!(),
        }
    }

    fn build_evaluator(&self, expr: &BoundExpr, schema: &DataSchema) -> BoxedEvaluator {
        match &expr.kind {
            BoundExprKind::Value(value) => Box::new(Constant::new(value.clone())),
            BoundExprKind::Variable(variable) => {
                let index = schema
                    .get_field_index_by_name(variable)
                    .expect("variable should be present in the schema");
                Box::new(ColumnRef::new(index))
            }
            BoundExprKind::FunctionCall { func_name, args } => {
                self.build_function_evaluator(func_name, args, schema)
            }
        }
    }

    fn build_function_evaluator(
        &self,
        func_name: &str,
        args: &[BoundExpr],
        schema: &DataSchema,
    ) -> BoxedEvaluator {
        match func_name.to_uppercase().as_str() {
            "VECTOR_DISTANCE" => {
                assert_eq!(
                    args.len(),
                    3,
                    "VECTOR_DISTANCE should have 3 args (verified by binder)"
                );

                let query_vector = self.build_evaluator(&args[0], schema);
                let target_vector = self.build_evaluator(&args[1], schema);

                // Extract metric from the third argument (validated by binder)
                let scalar_value = args[2]
                    .clone()
                    .evaluate_scalar()
                    .expect("metric should be compile-time constant");

                let metric_str = if let ScalarValue::String(Some(s)) = scalar_value {
                    s
                } else {
                    panic!("metric should be string")
                };

                let metric = VectorMetric::from_str(&metric_str).expect("metric should be valid");

                Box::new(VectorDistanceEvaluator::new(
                    query_vector,
                    target_vector,
                    metric,
                ))
            }
            _ => panic!(
                "Unsupported function '{}' (should be caught by binder)",
                func_name
            ),
        }
    }
}
