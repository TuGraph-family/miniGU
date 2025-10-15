use std::sync::Arc;

use itertools::Itertools;
use minigu_common::error::not_implemented;
use crate::bound::{BoundElementPattern, BoundGraphPattern, BoundPathPatternExpr};
use crate::error::{PlanError, PlanResult};
use crate::plan::filter::Filter;
use crate::plan::limit::Limit;
use crate::plan::project::Project;
use crate::plan::sort::Sort;
use crate::plan::{PlanData, PlanNode};
use crate::plan::scan::PhysicalNodeScan;

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

fn extract_single_node(p: &BoundGraphPattern) -> Option<(String, Vec<String>, i64)> {
    if p.paths.len() != 1 { return None; }
    if p.predicate.is_some() { return None; }

    let path = &p.paths[0];

    match path.expr {
        BoundPathPatternExpr::Pattern(pattern) => {
            match pattern {
                BoundElementPattern::Vertex(vertex) => {
                    
                }
            }
        }
        _ => None
    }
    if let Some(node) = path.expr {
        let var = node.var.clone();                   // 例如 "n"
        let labels = node.labels.clone();             // 例如 ["Person"] 或 []
        let graph_id = 1;                             // 若你支持多图，在 pattern/match_mode 里取
        return Some((var, labels, graph_id));
    }


    None
}

fn create_physical_plan_impl(logical_plan: &PlanNode) -> PlanResult<PlanNode> {
    let children: Vec<_> = logical_plan
        .children()
        .iter()
        .map(create_physical_plan_impl)
        .try_collect()?;
    match logical_plan {
        PlanNode::LogicalMatch(m) => {
            assert!(children.is_empty());
            let (var, labels, graph_id) = extract_single_node(&m.pattern)
                .ok_or_else(|| PlanError::InvalidOperation("extract_error"))?;
            let col_name = choose_node_id_col_name(&m.output_schema, &var);
            let field = DataField::new(col_name, LogicalType::Int64, /*nullable=*/false);
            let scan_schema = DataSchema::new(vec![field]);
            let node = PhysicalNodeScan {
                base: PlanBase { schema: Some(Arc::new(scan_schema)), children: vec![] },
                var,
                labels,
                graph_id,
            };
            Ok(PlanNode::PhysicalNodeScan(Arc::new(node)))
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
            let limit = Limit::new(child, limit.limit);
            Ok(PlanNode::PhysicalLimit(Arc::new(limit)))
        }
        _ => unreachable!(),
    }
}
