use serde::Serialize;

use crate::plan::{PlanBase, PlanData, PlanNode};

#[derive(Debug, Clone, Serialize)]
pub struct Limit {
    pub base: PlanBase,
    pub limit: usize,
    pub approximate: bool, // if true, enable ANN search
}

impl Limit {
    pub fn new(child: PlanNode, limit: usize, approximate: bool) -> Self {
        let base = PlanBase {
            schema: child.schema().cloned(),
            children: vec![child],
        };
        Self {
            base,
            limit,
            approximate,
        }
    }
}

impl PlanData for Limit {
    fn base(&self) -> &PlanBase {
        &self.base
    }

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let mut output = String::new();
        output.push_str(&format!("{}Limit: {:?}\n", indent_str, self.limit));

        for child in self.children() {
            output.push_str(child.explain(indent + 1)?.as_str());
        }

        Some(output)
    }
}
