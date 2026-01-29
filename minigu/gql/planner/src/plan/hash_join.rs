use std::sync::Arc;

use minigu_common::data_type::DataSchema;
use serde::Serialize;

use crate::bound::BoundExpr;
use crate::plan::{PlanBase, PlanData, PlanNode};

#[derive(Debug, Clone, Serialize)]
pub struct HashJoin {
    pub base: PlanBase,
    pub conds: Vec<JoinCondition>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JoinCondition {
    pub left_key: BoundExpr,
    pub right_key: BoundExpr,
}

impl JoinCondition {
    pub fn new(left_key: BoundExpr, right_key: BoundExpr) -> Self {
        Self {
            left_key,
            right_key,
        }
    }
}

impl HashJoin {
    pub fn new(left: PlanNode, right: PlanNode, conds: Vec<JoinCondition>) -> Self {
        let schema = merge_schema(&left, &right);
        let base = PlanBase::new(schema, vec![left, right]);
        Self { base, conds }
    }

    pub fn clone_with_children(&self, left: PlanNode, right: PlanNode) -> Self {
        HashJoin::new(left, right, self.conds.clone())
    }
}

impl PlanData for HashJoin {
    fn base(&self) -> &PlanBase {
        &self.base
    }

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let mut output = format!("{}HashJoin: {} conds\n", indent_str, self.conds.len());
        for child in self.children() {
            output.push_str(child.explain(indent + 1)?.as_str());
        }
        Some(output)
    }
}

fn merge_schema(left: &PlanNode, right: &PlanNode) -> Option<Arc<DataSchema>> {
    let left_schema = left.schema()?.as_ref().clone();
    let right_schema = right.schema()?.as_ref().clone();
    let mut merged = left_schema;
    merged.append(&right_schema);
    Some(Arc::new(merged))
}
