use serde::Serialize;

use crate::bound::BoundInsertStatement;
use crate::plan::{PlanBase, PlanData};

/// Physical plan node for an `INSERT` statement.
#[derive(Debug, Clone, Serialize)]
pub struct Insert {
    pub base: PlanBase,
    pub statement: BoundInsertStatement,
}

impl Insert {
    pub fn new(statement: BoundInsertStatement) -> Self {
        Self {
            base: PlanBase::new(None, vec![]),
            statement,
        }
    }
}

impl PlanData for Insert {
    fn base(&self) -> &PlanBase {
        &self.base
    }

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        Some(format!(
            "{}Insert: {} vertices, {} edges\n",
            indent_str,
            self.statement.vertices.len(),
            self.statement.edges.len(),
        ))
    }
}
