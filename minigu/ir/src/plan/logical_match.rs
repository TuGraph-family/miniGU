use minigu_common::data_type::DataSchemaRef;
use serde::Serialize;

use crate::plan::{PlanBase, PlanNode};

#[derive(Debug, Clone, Serialize)]
pub struct LogicalMatch {
    pub base: PlanBase,
}

impl LogicalMatch {
    pub fn schema(&self) -> Option<&DataSchemaRef> {
        self.base.schema()
    }

    pub fn children(&self) -> &[PlanNode] {
        &self.base.children
    }
}
