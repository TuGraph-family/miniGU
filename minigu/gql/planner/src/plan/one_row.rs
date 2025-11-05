use std::sync::Arc;

use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use serde::Serialize;

use crate::plan::{PlanBase, PlanData};

#[derive(Debug, Clone, Serialize)]
pub struct OneRow {
    pub base: PlanBase,
}

impl Default for OneRow {
    fn default() -> Self {
        Self::new()
    }
}

impl OneRow {
    pub fn new() -> Self {
        let schema = DataSchema::new(vec![DataField::new(
            "one_row".into(),
            LogicalType::Int32,
            false,
        )]);
        let base = PlanBase {
            schema: Some(Arc::new(schema)),
            children: vec![],
        };
        Self { base }
    }
}

impl PlanData for OneRow {
    fn base(&self) -> &PlanBase {
        &self.base
    }

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let mut output = String::new();
        output.push_str(&format!("{}OneRow\n", indent_str));

        for child in self.children() {
            output.push_str(child.explain(indent + 1)?.as_str());
        }

        Some(output)
    }
}
