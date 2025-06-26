use serde::Serialize;

use crate::plan::{PlanBase, PlanData};

#[derive(Debug, Clone, Serialize)]
pub struct LogicalMatch {
    pub base: PlanBase,
}

impl PlanData for LogicalMatch {
    fn base(&self) -> &PlanBase {
        &self.base
    }
}
