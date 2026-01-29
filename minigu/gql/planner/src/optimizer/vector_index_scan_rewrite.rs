use std::collections::BTreeMap;
use std::sync::Arc;

use minigu_common::types::{VectorIndexKey, VectorMetric};

use crate::bound::{BoundExpr, BoundExprKind};
use crate::error::PlanResult;
use crate::plan::explain::Explain;
use crate::plan::filter::Filter;
use crate::plan::hash_join::{HashJoin, JoinCondition};
use crate::plan::limit::Limit;
use crate::plan::offset::Offset;
use crate::plan::project::Project;
use crate::plan::property_fetch::{PropertyOutput, VertexPropertyFetch};
use crate::plan::sort::Sort;
use crate::plan::{PlanData, PlanNode};

/// Placeholder rewrite pass for vector index scans.
///
/// The optimizer currently calls into this hook before physical planning.
/// Currently it detects LIMIT APPROXIMATE + ORDER BY VECTOR_DISTANCE patterns
/// and, when matched, rewrites Sort+Limit into LogicalVectorIndexScan. A follow-up pass will
/// insert property fetch based on projection needs.
pub fn rewrite(plan: PlanNode) -> PlanResult<PlanNode> {
    rewrite_node(plan)
}

fn rewrite_node(plan: PlanNode) -> PlanResult<PlanNode> {
    use crate::plan::PlanNode::*;

    match plan {
        LogicalExplain(explain) => {
            let child = explain
                .children()
                .first()
                .cloned()
                .expect("explain should have exactly one child");
            let new_child = rewrite_node(child)?;
            Ok(LogicalExplain(Arc::new(Explain::new(new_child))))
        }
        LogicalProject(project) => {
            let child = project
                .children()
                .first()
                .cloned()
                .expect("project should have exactly one child");
            let new_child = rewrite_node(child)?;
            let new_child = ensure_project_property_fetch(new_child, &project.exprs);
            let rewritten = Project::new(
                new_child,
                project.exprs.clone(),
                project.schema().expect("schema required").clone(),
            );
            Ok(LogicalProject(Arc::new(rewritten)))
        }
        LogicalSort(sort) => {
            let child = sort
                .children()
                .first()
                .cloned()
                .expect("sort should have exactly one child");
            let new_child = rewrite_node(child)?;
            let rewritten = Sort::new(new_child, sort.specs.clone());
            Ok(LogicalSort(Arc::new(rewritten)))
        }
        LogicalLimit(limit) => {
            if limit.approximate {
                let original_child = limit
                    .children()
                    .first()
                    .cloned()
                    .expect("limit should have exactly one child");
                if let Some(plan) = try_rewrite_vector_limit(limit.as_ref(), original_child)? {
                    return Ok(plan);
                }
            }

            let child = limit
                .children()
                .first()
                .cloned()
                .expect("limit should have exactly one child");
            let new_child = rewrite_node(child)?;
            Ok(LogicalLimit(Arc::new(Limit::new(
                new_child,
                limit.limit,
                limit.approximate,
            ))))
        }
        LogicalOffset(offset) => {
            let child = offset
                .children()
                .first()
                .cloned()
                .expect("offset should have exactly one child");
            let new_child = rewrite_node(child)?;
            Ok(LogicalOffset(Arc::new(Offset::new(
                new_child,
                offset.offset,
            ))))
        }
        LogicalVertexPropertyFetch(fetch) => {
            let child = fetch
                .children()
                .first()
                .cloned()
                .expect("property fetch should have exactly one child");
            let new_child = rewrite_node(child)?;
            Ok(LogicalVertexPropertyFetch(Arc::new(
                fetch.clone_with_child(new_child),
            )))
        }
        LogicalVectorIndexScan(scan) => {
            let child = scan
                .children()
                .first()
                .cloned()
                .expect("vector scan should have exactly one child");
            let new_child = rewrite_node(child)?;
            Ok(LogicalVectorIndexScan(Arc::new(
                scan.clone_with_child(new_child),
            )))
        }
        LogicalFilter(filter) => {
            let child = filter
                .children()
                .first()
                .cloned()
                .expect("filter should have exactly one child");
            let new_child = rewrite_node(child)?;
            Ok(LogicalFilter(Arc::new(Filter::new(
                new_child,
                filter.predicate.clone(),
            ))))
        }
        LogicalHashJoin(join) => {
            let children = join.children();
            assert_eq!(children.len(), 2);
            let left = rewrite_node(children[0].clone())?;
            let right = rewrite_node(children[1].clone())?;
            Ok(LogicalHashJoin(Arc::new(
                join.clone_with_children(left, right),
            )))
        }
        other => Ok(other),
    }
}

fn try_rewrite_vector_limit(limit: &Limit, child: PlanNode) -> PlanResult<Option<PlanNode>> {
    use crate::plan::PlanNode::*;

    let sorted = match child {
        LogicalSort(sort) => sort,
        _ => return Ok(None),
    };

    if sorted.specs.len() != 1 {
        return Ok(None);
    }

    let sort_child = sorted
        .children()
        .first()
        .cloned()
        .expect("sort should have child");

    let project_node = match sort_child {
        LogicalProject(project) => project,
        _ => return Ok(None),
    };

    // Resolve the actual sort expression (it might be a projected alias).
    let spec_expr = resolve_sort_key(&sorted.specs[0].key, &project_node);
    let Some(vector) = extract_vector_distance(&spec_expr) else {
        return Ok(None);
    };

    let project_child = project_node
        .children()
        .first()
        .cloned()
        .expect("project should have exactly one child");
    let base_child = rewrite_node(project_child)?;

    let VectorPattern {
        binding,
        distance_alias,
        index_key,
        query,
        metric,
        dimension,
    } = vector;

    let scan = crate::plan::vector_index_scan::VectorIndexScan::new(
        base_child.clone(),
        binding.clone(),
        distance_alias,
        index_key,
        query,
        metric,
        dimension,
        limit.limit,
        true,
    );
    let left_node = PlanNode::LogicalVectorIndexScan(Arc::new(scan));
    let right_node = build_property_fetch_plan(base_child, &binding, &project_node.exprs);
    let join = HashJoin::new(
        left_node,
        right_node,
        vec![JoinCondition::new(
            make_binding_expr(&binding),
            make_binding_expr(&binding),
        )],
    );
    let project = Project::new(
        PlanNode::LogicalHashJoin(Arc::new(join)),
        project_node.exprs.clone(),
        project_node
            .schema()
            .expect("project schema should exist")
            .clone(),
    );
    Ok(Some(PlanNode::LogicalProject(Arc::new(project))))
}

fn build_property_fetch_plan(child: PlanNode, binding: &str, exprs: &[BoundExpr]) -> PlanNode {
    let mut all_specs: BTreeMap<String, BTreeMap<u32, PropertyOutput>> = BTreeMap::new();
    for expr in exprs {
        collect_property_refs(expr, &mut all_specs);
    }
    let binding_specs = match all_specs.get(binding) {
        Some(specs) if !specs.is_empty() => specs.clone(),
        _ => return child,
    };

    let (property_ids, outputs): (Vec<_>, Vec<_>) = binding_specs.into_iter().unzip();
    let fetch = VertexPropertyFetch::new(child, binding.to_string(), property_ids, outputs);
    PlanNode::LogicalVertexPropertyFetch(std::sync::Arc::new(fetch))
}

fn make_binding_expr(binding: &str) -> BoundExpr {
    BoundExpr::variable(
        binding.to_string(),
        minigu_common::data_type::LogicalType::UInt64,
        false,
    )
}

fn resolve_sort_key(sort_key: &BoundExpr, project: &Project) -> BoundExpr {
    match &sort_key.kind {
        BoundExprKind::Variable(alias) => {
            let schema = project.schema().expect("project schema is required");
            if let Some(idx) = schema.get_field_index_by_name(alias) {
                project
                    .exprs
                    .get(idx)
                    .cloned()
                    .unwrap_or_else(|| sort_key.clone())
            } else {
                sort_key.clone()
            }
        }
        _ => sort_key.clone(),
    }
}

fn extract_vector_distance(expr: &BoundExpr) -> Option<VectorPattern> {
    if let BoundExprKind::VectorDistance {
        lhs,
        rhs,
        metric,
        dimension,
    } = &expr.kind
    {
        // We expect rhs to be a bound property carrying label/property ids.
        let (binding, label_id, property_id, _property_name) = match &rhs.kind {
            BoundExprKind::Property {
                binding,
                label_id: Some(lid),
                property_id: Some(pid),
                property,
            } => (binding.clone(), *lid, *pid, property.clone()),
            _ => return None, // cannot build index key without metadata
        };

        Some(VectorPattern {
            binding,
            distance_alias: "distance".to_string(),
            index_key: VectorIndexKey::new(label_id, property_id),
            query: (*lhs.clone()),
            metric: *metric,
            dimension: *dimension,
        })
    } else {
        None
    }
}

struct VectorPattern {
    binding: String,
    distance_alias: String,
    index_key: VectorIndexKey,
    query: BoundExpr,
    metric: VectorMetric,
    dimension: usize,
}

fn collect_property_refs(
    expr: &BoundExpr,
    specs: &mut BTreeMap<String, BTreeMap<u32, PropertyOutput>>,
) {
    match &expr.kind {
        BoundExprKind::Property {
            binding,
            property_id: Some(pid),
            property,
            ..
        } => {
            let column_alias = format!("{}_{}", binding, property);
            specs
                .entry(binding.clone())
                .or_default()
                .entry(*pid)
                .or_insert_with(|| PropertyOutput {
                    column_alias,
                    ty: expr.logical_type.clone(),
                    nullable: expr.nullable,
                });
        }
        BoundExprKind::VectorDistance { lhs, rhs, .. } => {
            collect_property_refs(lhs, specs);
            collect_property_refs(rhs, specs);
        }
        _ => {}
    }
}

fn ensure_project_property_fetch(child: PlanNode, exprs: &[BoundExpr]) -> PlanNode {
    let mut specs: BTreeMap<String, BTreeMap<u32, PropertyOutput>> = BTreeMap::new();
    for expr in exprs {
        collect_property_refs(expr, &mut specs);
    }

    if specs.is_empty() {
        return child;
    }

    let mut current = child;
    for (binding, outputs_map) in specs {
        let mut filtered: Vec<(u32, PropertyOutput)> = outputs_map.into_iter().collect();
        filtered.retain(|(_, output)| {
            current
                .schema()
                .and_then(|schema| schema.get_field_by_name(&output.column_alias))
                .is_none()
        });

        if filtered.is_empty() {
            continue;
        }

        let (property_ids, outputs): (Vec<_>, Vec<_>) = filtered.into_iter().unzip();
        let fetch = VertexPropertyFetch::new(current, binding.clone(), property_ids, outputs);
        current = PlanNode::LogicalVertexPropertyFetch(Arc::new(fetch));
    }

    current
}
