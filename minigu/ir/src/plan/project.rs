use std::sync::Arc;

use minigu_common::data_type::{DataField, DataSchema, DataSchemaRef};
use serde::Serialize;

use crate::bound::BoundExpr;
use crate::plan::{PlanBase, PlanNode};

#[derive(Debug, Clone, Serialize)]
pub struct Project {
    pub base: PlanBase,
    pub exprs: Vec<BoundExpr>,
}

impl Project {
    pub fn new(child: PlanNode, exprs: Vec<BoundExpr>) -> Self {
        let schema = Arc::new(DataSchema::new(
            exprs
                .iter()
                .map(|e| DataField::new(e.name.clone(), e.logical_type.clone(), false))
                .collect(),
        ));
        let base = PlanBase {
            schema: Some(schema),
            children: vec![child],
        };
        Self { base, exprs }
    }

    pub fn schema(&self) -> Option<&DataSchemaRef> {
        self.base.schema()
    }

    pub fn children(&self) -> &[PlanNode] {
        &self.base.children
    }
}
