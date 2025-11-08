use std::sync::Arc;

use itertools::Itertools;
use minigu_common::error::not_implemented;
use minigu_common::types::{GraphId, LabelId};

use crate::bound::{
    BoundEdgePatternKind, BoundElementPattern, BoundGraphPattern, BoundLabelExpr,
    BoundPathPatternExpr, PathPatternInfo,
};
use crate::error::PlanResult;
use crate::plan::expand::{Expand, ExpandDirection};
use crate::plan::filter::Filter;
use crate::plan::limit::Limit;
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

fn extract_path_pattern_from_graph_pattern(
    g: &BoundGraphPattern,
    graph_id: GraphId,
) -> PlanResult<PathPatternInfo> {
    if g.predicate.is_some() {
        return not_implemented("MATCH with predicate (WHERE) is not supported yet", Some(1));
    }
    if g.paths.len() != 1 {
        return not_implemented("multiple paths in MATCH are not supported yet", Some(1));
    }

    extract_path_pattern(&g.paths[0].expr, graph_id)
}

fn lower_label_expr_to_specs(expr: &BoundLabelExpr) -> PlanResult<Vec<Vec<LabelId>>> {
    use BoundLabelExpr::*;
    match expr {
        Any => Ok(vec![vec![]]),
        Label(id) => Ok(vec![vec![*id]]),

        // Disjunction => concatenate routes
        Disjunction(lhs, rhs) => {
            let mut a = lower_label_expr_to_specs(lhs)?;
            let mut b = lower_label_expr_to_specs(rhs)?;
            a.append(&mut b);
            Ok(a)
        }

        // Conjunction => distributive product of routes, merging inner AND sets
        Conjunction(lhs, rhs) => {
            let left = lower_label_expr_to_specs(lhs)?;
            let right = lower_label_expr_to_specs(rhs)?;
            let mut out: Vec<Vec<LabelId>> = Vec::with_capacity(left.len() * right.len());
            for l in &left {
                for r in &right {
                    let mut merged = Vec::with_capacity(l.len() + r.len());
                    merged.extend_from_slice(l);
                    merged.extend_from_slice(r);
                    merged.sort_unstable();
                    merged.dedup();
                    out.push(merged);
                }
            }
            Ok(out)
        }
        Negation(_) => not_implemented("label negation is not supported yet", None),
    }
}

fn extract_path_pattern(
    expr: &BoundPathPatternExpr,
    graph_id: GraphId,
) -> PlanResult<PathPatternInfo> {
    use BoundPathPatternExpr::*;
    match expr {
        Pattern(BoundElementPattern::Vertex(v)) => {
            let var = v.var.clone();
            let label_specs: Vec<Vec<LabelId>> = match &v.label {
                None => vec![vec![]],
                Some(lbl) => lower_label_expr_to_specs(lbl)?,
            };
            Ok(PathPatternInfo::SingleVertex {
                var,
                label_specs,
                graph_id,
            })
        }
        Concat(parts) => {
            if parts.is_empty() {
                return not_implemented("empty concat in path pattern", None);
            }
            let mut vertices = Vec::new();
            let mut edges = Vec::new();
            for (i, part) in parts.iter().enumerate() {
                match part {
                    Pattern(BoundElementPattern::Vertex(v)) => {
                        let var = v.var.clone();
                        let label_specs: Vec<Vec<LabelId>> = match &v.label {
                            None => vec![vec![]],
                            Some(lbl) => lower_label_expr_to_specs(lbl)?,
                        };
                        vertices.push((var, label_specs));
                    }
                    Pattern(BoundElementPattern::Edge(e)) => {
                        let edge_labels: Vec<Vec<LabelId>> = match &e.label {
                            None => vec![vec![]],
                            Some(lbl) => lower_label_expr_to_specs(lbl)?,
                        };
                        let direction = match e.kind {
                            BoundEdgePatternKind::Right => ExpandDirection::Outgoing,
                            BoundEdgePatternKind::Left => ExpandDirection::Incoming,
                            _ => unreachable!(),
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
            Ok(PathPatternInfo::Path {
                vertices,
                edges,
                graph_id,
            })
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
        PlanNode::LogicalMatch(m) => {
            let graph_id: GraphId = 1;
            match extract_path_pattern_from_graph_pattern(&m.pattern, graph_id)? {
                PathPatternInfo::SingleVertex {
                    var,
                    label_specs,
                    graph_id,
                } => {
                    let node = NodeIdScan::new(var.as_str(), label_specs, graph_id);
                    Ok(PlanNode::PhysicalNodeScan(Arc::new(node)))
                }
                PathPatternInfo::Path {
                    vertices,
                    edges,
                    graph_id,
                } => {
                    if vertices.is_empty() {
                        return not_implemented("empty path patterns", None);
                    }
                    let (first_var, first_lables) = vertices[0].clone();
                    let mut current_plan = PlanNode::PhysicalNodeScan(Arc::new(NodeIdScan::new(
                        first_var.as_str(),
                        first_lables,
                        graph_id,
                    )));
                    for (i, (edge_info, next_vertex)) in
                        edges.iter().zip(vertices.iter().skip(1)).enumerate()
                    {
                        let (edge_var, edge_labels, direction) = edge_info;
                        let (next_var, next_labels) = next_vertex;
                        let expand = Expand::new(
                            current_plan.clone(),
                            0,
                            edge_labels.clone(),
                            edge_var.clone(),
                            direction.clone(),
                            graph_id,
                        );
                        current_plan = PlanNode::PhysicalExpand(Arc::new(expand));
                    }
                    Ok(current_plan)
                }
            }
        }
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
        PlanNode::LogicalVectorIndexScan(vector_scan) => {
            assert!(children.is_empty());
            Ok(PlanNode::PhysicalVectorIndexScan(vector_scan.clone()))
        }
        _ => unreachable!(),
    }
}
