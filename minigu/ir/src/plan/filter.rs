use minigu_common::data_type::DataSchemaRef;
use serde::Serialize;

use crate::bound::BoundExpr;
use crate::plan::{PlanBase, PlanNode};

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

    pub fn schema(&self) -> Option<&DataSchemaRef> {
        self.base.schema()
    }

    pub fn children(&self) -> &[PlanNode] {
        &self.base.children
    }
}
