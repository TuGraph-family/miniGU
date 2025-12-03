use serde::Serialize;

use crate::plan::{PlanBase, PlanData, PlanNode};

#[derive(Debug, Clone, Serialize)]
pub struct Explain {
    pub base: PlanBase,
}

impl Explain {
    pub fn new(child: PlanNode) -> Self {
        let base = PlanBase {
            schema: None,
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
        let indent_str = " ".repeat(indent * 2);
        let mut output = String::new();
        output.push_str(&format!("{}Explain:\n", indent_str));

        for child in self.children() {
            output.push_str(child.explain(indent + 1)?.as_str());
        }

        Some(output)
    }
}
