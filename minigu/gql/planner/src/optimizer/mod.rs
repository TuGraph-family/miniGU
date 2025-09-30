use std::sync::Arc;

use itertools::Itertools;
use minigu_common::types::{VectorIndexKey, VectorMetric};

use crate::bound::BoundExpr;
use crate::error::PlanResult;
use crate::plan::filter::Filter;
use crate::plan::limit::Limit;
use crate::plan::project::Project;
use crate::plan::sort::Sort;
use crate::plan::vector_search::VectorSearch;
use crate::plan::{PlanData, PlanNode};

#[derive(Debug, Default)]
pub struct Optimizer {}

impl Optimizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_physical_plan(self, logical_plan: &PlanNode) -> PlanResult<PlanNode> {
        create_physical_plan_impl(logical_plan)
    }
}

fn create_physical_plan_impl(logical_plan: &PlanNode) -> PlanResult<PlanNode> {
    let children: Vec<_> = logical_plan
        .children()
        .iter()
        .map(create_physical_plan_impl)
        .try_collect()?;
    match logical_plan {
        PlanNode::LogicalMatch(_) => todo!(),
        PlanNode::LogicalFilter(filter) => {
            let [child] = children
                .try_into()
                .expect("filter should have exactly one child");
            let predicate = filter.predicate.clone();
            let filter = Filter::new(child, predicate);
            Ok(PlanNode::PhysicalFilter(Arc::new(filter)))
        }
        PlanNode::LogicalProject(project) => {
            let [child] = children
                .try_into()
                .expect("project should have exactly one child");
            let exprs = project.exprs.clone();
            let schema = project.schema().expect("project should have a schema");
            let project = Project::new(child, exprs, schema.clone());
            Ok(PlanNode::PhysicalProject(Arc::new(project)))
        }
        PlanNode::LogicalCall(call) => {
            assert!(children.is_empty());
            Ok(PlanNode::PhysicalCall(call.clone()))
        }
        PlanNode::LogicalOneRow(one_row) => Ok(PlanNode::PhysicalOneRow(one_row.clone())),
        PlanNode::LogicalSort(sort) => {
            let [child] = children
                .try_into()
                .expect("sort should have exactly one child");
            let specs = sort.specs.clone();
            let sort = Sort::new(child, specs);
            Ok(PlanNode::PhysicalSort(Arc::new(sort)))
        }
        PlanNode::LogicalLimit(limit) => {
            let [child] = children
                .try_into()
                .expect("limit should have exactly one child");
            if limit.approximate {
                if let Some(vector_search) = try_optimize_vector_search(&child, limit) {
                    return Ok(PlanNode::PhysicalVectorSearch(Arc::new(vector_search)));
                }
            }
            let limit = Limit::new(child, limit.limit, limit.approximate);
            Ok(PlanNode::PhysicalLimit(Arc::new(limit)))
        }
        _ => unreachable!(),
    }
}

/// Attempts to optimize a vector search query pattern:
/// LogicalSort(VECTOR_DISTANCE) + LogicalLimit(approximate=true)
fn try_optimize_vector_search(child: &PlanNode, limit: &Limit) -> Option<VectorSearch> {
    if let PlanNode::PhysicalSort(sort) = child {
        for sort_spec in &sort.specs {
            if let Some((query_vector_expr, target_vector, metric)) =
                extract_vector_distance_args(&sort_spec.key)
            {
                if let Some(index_key) = extract_index_key_from_target(&target_vector) {
                    if let Some(schema) = child.schema().cloned() {
                        return VectorSearch::new(
                            schema,
                            index_key,
                            query_vector_expr,
                            limit.limit, /* return k results directly. TODO: Iterative
                                          * post-filtering */
                            metric,
                        )
                        .ok();
                    }
                }
            }
        }
    }
    None
}

/// Extracts VectorIndexKey from target vector expression
/// TODO: Implement proper index key extraction based on expression analysis
fn extract_index_key_from_target(_target_expr: &BoundExpr) -> Option<VectorIndexKey> {
    todo!()
}

/// Extracts arguments from VECTOR_DISTANCE function call
/// Returns: (query_vector_expr, target_vector_expr, metric)
fn extract_vector_distance_args(_expr: &BoundExpr) -> Option<(BoundExpr, BoundExpr, VectorMetric)> {
    todo!()
}
