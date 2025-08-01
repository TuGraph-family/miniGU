use serde::Serialize;

use crate::bound::BoundExpr;
use crate::plan::{PlanBase, PlanData, PlanNode};

#[derive(Debug, Clone, Serialize)]
pub struct Filter {
    pub base: PlanBase,
    pub predicate: BoundExpr,
}

impl Filter {
    pub fn new(child: PlanNode, predicate: BoundExpr) -> Self {
        assert!(child.schema().is_some());
        let schema = child.schema().cloned();
        let base = PlanBase {
            schema,
            children: vec![child],
        };
        Self { base, predicate }
    }
}

impl PlanData for Filter {
    fn base(&self) -> &PlanBase {
        &self.base
    }
}
