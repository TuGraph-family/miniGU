use serde::Serialize;

use crate::bound::BoundExpr;
use crate::plan::{PlanBase, PlanData, PlanNode};

#[derive(Debug, Clone, Serialize)]
pub struct Filter {
    pub base: PlanBase,
    pub predicate: BoundExpr,
}

impl Filter {
    pub fn new(child: PlanNode, predicate: BoundExpr) -> Self {
        assert!(child.schema().is_some());
        let schema = child.schema().cloned();
        let base = PlanBase {
            schema,
            children: vec![child],
        };
        Self { base, predicate }
    }
}

impl PlanData for Filter {
    fn base(&self) -> &PlanBase {
        &self.base
    }

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let mut output = String::new();
        output.push_str(&format!("{}Filter: {:?}\n", indent_str, self.predicate));

        for child in self.children() {
            output.push_str(child.explain(indent + 1)?.as_str());
        }

        Some(output)
    }
}
