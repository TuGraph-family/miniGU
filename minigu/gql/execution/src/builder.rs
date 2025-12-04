use std::sync::Arc;

use arrow::array::{AsArray, Int32Array};
use minigu_catalog::label_set::LabelSet;
use minigu_catalog::provider::GraphTypeProvider;
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_common::types::VertexIdArray;
use minigu_context::graph::GraphContainer;
use minigu_context::session::SessionContext;
use minigu_planner::bound::{BoundExpr, BoundExprKind};
use minigu_planner::plan::{PlanData, PlanNode};

use crate::evaluator::BoxedEvaluator;
use crate::evaluator::column_ref::ColumnRef;
use crate::evaluator::constant::Constant;
use crate::evaluator::vector_distance::VectorDistanceEvaluator;
use crate::evaluator::vertex_constructor::VertexConstructor;
use crate::executor::procedure_call::ProcedureCallBuilder;
use crate::executor::sort::SortSpec;
use crate::executor::vector_index_scan::VectorIndexScanBuilder;
use crate::executor::{BoxedExecutor, Executor, IntoExecutor};
use crate::source::VertexSource;

const DEFAULT_CHUNK_SIZE: usize = 2048;

pub struct ExecutorBuilder {
    session: SessionContext,
}

impl ExecutorBuilder {
    pub fn new(session: SessionContext) -> Self {
        Self { session }
    }

    pub fn build(self, plan: &PlanNode) -> BoxedExecutor {
        self.build_executor(plan)
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
            PlanNode::PhysicalNodeScan(_node_scan) => {
                // NodeScan provide graph id and label, Handle in next pr.
                assert_eq!(children.len(), 0);
                let container: Arc<GraphContainer> = self
                    .session
                    .current_graph
                    .clone()
                    .expect("current graph should be set")
                    .object()
                    .clone()
                    .downcast_arc::<GraphContainer>()
                    .expect("failed to downcast to GraphContainer");

                let batches = container
                    .vertex_source(&Some(_node_scan.labels.clone()), 1024)
                    .expect("failed to create vertex source");
                let source = batches.map(|arr: Arc<VertexIdArray>| Ok(arr));
                Box::new(source.scan_vertex())
            }
            PlanNode::PhysicalExpand(expand) => {
                assert_eq!(children.len(), 1);
                let child = self.build_executor(&children[0]);
                let container: Arc<GraphContainer> = self
                    .session
                    .current_graph
                    .clone()
                    .expect("current graph should be set")
                    .object()
                    .clone()
                    .downcast_arc::<GraphContainer>()
                    .expect("failed to downcast to GraphContainer");

                // Get the number of columns before expand
                let child_schema = children[0].schema().expect("child should have a schema");
                let num_child_columns = child_schema.fields().len();

                // Expand adds new columns (as ListArray) that need to be flattened.
                // ExpandSource returns 2 columns: edge IDs and target vertex IDs
                // We need to flatten both columns at indices num_child_columns and
                // num_child_columns + 1
                let expand_executor = child.expand(
                    expand.input_column_index,
                    Some(expand.edge_labels.clone()),
                    expand.target_vertex_labels.clone(),
                    container,
                );
                let column_indices_to_flatten: Vec<usize> =
                    (num_child_columns..num_child_columns + 2).collect();
                Box::new(expand_executor.flatten(column_indices_to_flatten))
            }
            PlanNode::PhysicalProject(project) => {
                assert_eq!(children.len(), 1);
                let child_schema = children[0].schema().expect("child should have a schema");
                let mut child_executor = self.build_executor(&children[0]);
                let output_schema = physical_plan.schema().expect("there should be a schema");

                let mut updated_schema = child_schema.clone();

                // Check if any expression is a Vertex type that needs properties
                // If output type is Vertex, we need to scan properties
                for expr in &project.exprs {
                    if let LogicalType::Vertex(_) = &expr.logical_type {
                        if let BoundExprKind::Variable(var_name) = &expr.kind {
                            // Check child schema to see if this variable only has id (Int64)
                            let child_field = child_schema
                                .get_field_by_name(var_name)
                                .expect("variable should be present in child schema");

                            // If child schema only has id (Int64), need to add VertexPropertyScan
                            if matches!(child_field.ty(), LogicalType::Int64) {
                                let vid_index = child_schema
                                    .get_field_index_by_name(var_name)
                                    .expect("variable should be present in child schema");

                                let container: Arc<GraphContainer> = self
                                    .session
                                    .current_graph
                                    .clone()
                                    .expect("current graph should be set")
                                    .object()
                                    .clone()
                                    .downcast_arc::<GraphContainer>()
                                    .expect("failed to downcast to GraphContainer");

                                let property_list = if let Some(label_specs) =
                                    output_schema.get_var_label(var_name.as_str())
                                {
                                    let graph_type = container.graph_type();
                                    let mut property_ids = Vec::new();
                                    if let Some(first_label_set) = label_specs.first() {
                                        if let Ok(Some(vertex_type)) = graph_type.get_vertex_type(
                                            &LabelSet::from_iter(first_label_set.clone()),
                                        ) {
                                            for (idx, _) in
                                                vertex_type.properties().iter().enumerate()
                                            {
                                                property_ids.push(idx as u32);
                                            }
                                        }
                                    }
                                    property_ids
                                } else {
                                    Vec::new()
                                };

                                child_executor = Box::new(child_executor.scan_vertex_property(
                                    vid_index,
                                    property_list.clone(),
                                    container,
                                ));

                                // Update schema: add property columns with
                                // __{var_name}_prop_{index} naming
                                let mut new_fields = updated_schema.fields().to_vec();
                                for (idx, _) in property_list.iter().enumerate() {
                                    new_fields.push(DataField::new(
                                        format!("__{}_prop_{}", var_name, idx),
                                        LogicalType::String, /* Type will be determined from
                                                              * actual data */
                                        true, // nullable
                                    ));
                                }
                                updated_schema = Arc::new(DataSchema::new(new_fields));
                            }
                        }
                    }
                }

                // Build evaluators with updated schema
                let evaluators = project
                    .exprs
                    .iter()
                    .map(|e| self.build_evaluator(e, &updated_schema))
                    .collect();
                Box::new(child_executor.project(evaluators))
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
            PlanNode::PhysicalOffset(offset) => {
                assert_eq!(children.len(), 1);
                Box::new(self.build_executor(&children[0]).offset(offset.offset))
            }
            PlanNode::PhysicalVectorIndexScan(vector_scan) => {
                assert!(children.is_empty());
                VectorIndexScanBuilder::new(self.session.clone(), vector_scan.clone())
                    .into_executor()
            }
            PlanNode::PhysicalExplain(explain) => {
                let explain_str = explain.explain(0).unwrap_or_default();
                let lines: Vec<&str> = explain_str.lines().collect();
                let string_array = arrow::array::StringArray::from_iter_values(lines);
                let columns = vec![Arc::new(string_array) as _];
                let chunk = DataChunk::new(columns);
                Box::new([Ok(chunk)].into_executor())
            }
            _ => unreachable!(),
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn build_evaluator(&self, expr: &BoundExpr, schema: &DataSchema) -> BoxedEvaluator {
        match &expr.kind {
            BoundExprKind::Value(value) => Box::new(Constant::new(value.clone())),
            BoundExprKind::Variable(variable) => {
                // Check if this is a Vertex type that needs to be constructed
                if let LogicalType::Vertex(_vertex_fields) = &expr.logical_type {
                    // Find the vertex ID column
                    let vid_index = schema
                        .get_field_index_by_name(variable)
                        .expect("variable should be present in the schema");

                    let label_specs = schema.get_var_label(variable);

                    let mut property_column_indices = Vec::new();
                    for i in 0.. {
                        let prop_name = format!("__{}_prop_{}", variable, i);
                        if let Some(prop_idx) = schema.get_field_index_by_name(&prop_name) {
                            property_column_indices.push(prop_idx);
                        } else {
                            break;
                        }
                    }

                    return Box::new(VertexConstructor::new(
                        vid_index,
                        property_column_indices,
                        label_specs,
                    ));
                }

                // Default: just return the column reference
                let index = schema
                    .get_field_index_by_name(variable)
                    .expect("variable should be present in the schema");
                Box::new(ColumnRef::new(index))
            }
            BoundExprKind::VectorDistance {
                lhs,
                rhs,
                metric,
                dimension,
            } => {
                let lhs = self.build_evaluator(lhs.as_ref(), schema);
                let rhs = self.build_evaluator(rhs.as_ref(), schema);
                Box::new(VectorDistanceEvaluator::new(lhs, rhs, *metric, *dimension))
            }
        }
    }
}
