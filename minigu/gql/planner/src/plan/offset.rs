use serde::Serialize;

use crate::plan::{PlanBase, PlanData, PlanNode};

#[derive(Debug, Clone, Serialize)]
pub struct Offset {
    pub base: PlanBase,
    pub offset: usize,
}

impl Offset {
    pub fn new(child: PlanNode, offset: usize) -> Self {
        let base = PlanBase {
            schema: child.schema().cloned(),
            children: vec![child],
        };
        Self { base, offset }
    }
}

impl PlanData for Offset {
    fn base(&self) -> &PlanBase {
        &self.base
    }

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let mut output = String::new();
        output.push_str(&format!("{}Offset: {:?}\n", indent_str, self.offset));

        for child in self.children() {
            output.push_str(child.explain(indent + 1)?.as_str());
        }

        Some(output)
    }
}
