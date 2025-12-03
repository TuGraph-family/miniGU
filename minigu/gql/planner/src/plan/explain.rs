use std::sync::Arc;

use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use serde::Serialize;

use crate::plan::{PlanBase, PlanData, PlanNode};

#[derive(Debug, Clone, Serialize)]
pub struct Explain {
    pub base: PlanBase,
}

impl Explain {
    pub fn new(child: PlanNode) -> Self {
        let schema = Some(Arc::new(DataSchema::new(vec![DataField::new(
            "EXPLAIN".to_string(),
            LogicalType::String,
            false,
        )])));
        let base = PlanBase {
            schema,
            children: vec![child],
        };
        Self { base }
    }
}

impl PlanData for Explain {
    fn base(&self) -> &PlanBase {
        &self.base
    }

    fn explain(&self, indent: usize) -> Option<String> {
        let mut output = String::new();

        for child in self.children() {
            output.push_str(child.explain(indent)?.as_str());
        }

        Some(output)
    }
}
