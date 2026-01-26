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

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let mut output = String::new();
        let specs_str = self
            .specs
            .iter()
            .map(|s| format!("{:?}", s))
            .collect::<Vec<_>>()
            .join(", ");
        output.push_str(&format!("{}Sort: {}\n", indent_str, specs_str));

        for child in self.children() {
            output.push_str(child.explain(indent + 1)?.as_str());
        }

        Some(output)
    }
}
