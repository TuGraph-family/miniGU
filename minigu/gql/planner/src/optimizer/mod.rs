use std::sync::Arc;

use itertools::Itertools;
use minigu_common::error::not_implemented;
use minigu_common::types::LabelId;

use crate::bound::{
    BoundEdgePatternKind, BoundElementPattern, BoundGraphPattern, BoundPathPatternExpr,
    PathPatternInfo,
};
use crate::error::PlanResult;
use crate::plan::expand::{Expand, ExpandDirection};
use crate::plan::filter::Filter;
use crate::plan::limit::Limit;
use crate::plan::offset::Offset;
use crate::plan::project::Project;
use crate::plan::scan::NodeIdScan;
use crate::plan::sort::Sort;
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

fn extract_path_pattern_from_graph_pattern(g: &BoundGraphPattern) -> PlanResult<PathPatternInfo> {
    if g.predicate.is_some() {
        return not_implemented("MATCH with predicate (WHERE) is not supported yet", Some(1));
    }
    if g.paths.len() != 1 {
        return not_implemented("multiple paths in MATCH are not supported yet", Some(1));
    }

    extract_path_pattern(&g.paths[0].expr)
}

fn extract_path_pattern(expr: &BoundPathPatternExpr) -> PlanResult<PathPatternInfo> {
    use BoundPathPatternExpr::*;
    match expr {
        Pattern(BoundElementPattern::Vertex(v)) => {
            let var = v.var.clone();
            let label_specs: Vec<Vec<LabelId>> = v.label.clone();
            Ok(PathPatternInfo::SingleVertex { var, label_specs })
        }
        Concat(parts) => {
            if parts.is_empty() {
                return not_implemented("empty concat in path pattern", None);
            }
            let mut vertices = Vec::new();
            let mut edges = Vec::new();
            for part in parts.iter() {
                match part {
                    Pattern(BoundElementPattern::Vertex(v)) => {
                        let var = v.var.clone();
                        let label_specs: Vec<Vec<LabelId>> = v.label.clone();
                        vertices.push((var, label_specs));
                    }
                    Pattern(BoundElementPattern::Edge(e)) => {
                        let edge_labels: Vec<Vec<LabelId>> = e.label.clone();
                        let direction = match e.kind {
                            BoundEdgePatternKind::Right => ExpandDirection::Outgoing,
                            BoundEdgePatternKind::Left => ExpandDirection::Incoming,
                            _ => {
                                return not_implemented(
                                    format!(
                                        "edge direction {:?} is not supported in path pattern",
                                        e.kind
                                    ),
                                    None,
                                );
                            }
                        };
                        edges.push((e.var.clone(), edge_labels, direction));
                    }
                    _ => {
                        return not_implemented(
                            "complex nest patterns in concat is not supported yet",
                            None,
                        );
                    }
                }
            }
            Ok(PathPatternInfo::Path { vertices, edges })
        }

        Subpath(_) => not_implemented("sub path is not supported", None),
        Alternation(_) => not_implemented(
            "alternation (A|B) in path pattern is not supported yet",
            None,
        ),
        Union(_) => not_implemented("union of path patterns is not supported yet", None),
        Quantified { .. } => {
            not_implemented("quantified path (*, +, {m,n}) is not supported yet", None)
        }
        Optional(_) => not_implemented("optional path (?) is not supported yet", None),
        Pattern(BoundElementPattern::Edge(_)) => not_implemented(
            "top-level single edge without anchors is not supported yet",
            None,
        ),
    }
}

fn create_physical_plan_impl(logical_plan: &PlanNode) -> PlanResult<PlanNode> {
    let children: Vec<_> = logical_plan
        .children()
        .iter()
        .map(create_physical_plan_impl)
        .try_collect()?;
    match logical_plan {
        PlanNode::LogicalMatch(m) => match extract_path_pattern_from_graph_pattern(&m.pattern)? {
            PathPatternInfo::SingleVertex { var, label_specs } => {
                let node = NodeIdScan::new(var.as_str(), label_specs);
                Ok(PlanNode::PhysicalNodeScan(Arc::new(node)))
            }
            PathPatternInfo::Path { vertices, edges } => {
                if vertices.is_empty() {
                    return not_implemented("empty path patterns", None);
                }
                let (first_var, first_labels) = vertices[0].clone();
                let mut current_plan = PlanNode::PhysicalNodeScan(Arc::new(NodeIdScan::new(
                    first_var.as_str(),
                    first_labels,
                )));
                for (edge_info, next_vertex) in edges.iter().zip(vertices.iter().skip(1)) {
                    let (edge_var, edge_labels, direction) = edge_info;
                    let (next_var, next_labels) = next_vertex;
                    let expand = Expand::new(
                        current_plan.clone(),
                        0,
                        edge_labels.clone(),
                        Some(next_labels.clone()),
                        edge_var.clone(),
                        Some(next_var.clone()),
                        direction.clone(),
                    );
                    current_plan = PlanNode::PhysicalExpand(Arc::new(expand));
                }
                Ok(current_plan)
            }
        },
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
            let limit = Limit::new(child, limit.limit, limit.approximate);
            Ok(PlanNode::PhysicalLimit(Arc::new(limit)))
        }
        PlanNode::LogicalOffset(offset) => {
            let [child] = children
                .try_into()
                .expect("offset should have exactly one child");
            let offset = Offset::new(child, offset.offset);
            Ok(PlanNode::PhysicalOffset(Arc::new(offset)))
        }
        PlanNode::LogicalVectorIndexScan(vector_scan) => {
            assert!(children.is_empty());
            Ok(PlanNode::PhysicalVectorIndexScan(vector_scan.clone()))
        }
        PlanNode::LogicalExplain(explain) => Ok(PlanNode::PhysicalExplain(explain.clone())),
        PlanNode::LogicalCreateVectorIndex(create_index) => {
            assert!(children.is_empty());
            Ok(PlanNode::PhysicalCreateVectorIndex(create_index.clone()))
        }
        PlanNode::LogicalDropVectorIndex(drop_index) => {
            assert!(children.is_empty());
            Ok(PlanNode::PhysicalDropVectorIndex(drop_index.clone()))
        }
        _ => unreachable!(),
    }
}
