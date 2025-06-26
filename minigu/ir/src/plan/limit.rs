use serde::Serialize;

use crate::plan::{PlanBase, PlanData, PlanNode};

#[derive(Debug, Clone, Serialize)]
pub struct Limit {
    pub base: PlanBase,
    pub limit: usize,
}

impl Limit {
    pub fn new(child: PlanNode, limit: usize) -> Self {
        let base = PlanBase {
            schema: child.schema().cloned(),
            children: vec![child],
        };
        Self { base, limit }
    }
}

impl PlanData for Limit {
    fn base(&self) -> &PlanBase {
        &self.base
    }
}
