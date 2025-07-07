use serde::Serialize;

use crate::bound::BoundSortSpec;
use crate::plan::{PlanBase, PlanData, PlanNode};

#[derive(Debug, Clone, Serialize)]
pub struct Sort {
    pub base: PlanBase,
    pub specs: Vec<BoundSortSpec>,
}

impl Sort {
    pub fn new(child: PlanNode, specs: Vec<BoundSortSpec>) -> Self {
        assert!(!specs.is_empty());
        let base = PlanBase {
            schema: child.schema().cloned(),
            children: vec![child],
        };
        Self { base, specs }
    }
}

impl PlanData for Sort {
    fn base(&self) -> &PlanBase {
        &self.base
    }
}
