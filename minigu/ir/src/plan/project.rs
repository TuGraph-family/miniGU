use minigu_common::data_type::DataSchemaRef;
use serde::Serialize;

use crate::bound::BoundExpr;
use crate::plan::{PlanBase, PlanData, PlanNode};

#[derive(Debug, Clone, Serialize)]
pub struct Project {
    pub base: PlanBase,
    pub exprs: Vec<BoundExpr>,
}

impl Project {
    pub fn new(child: PlanNode, exprs: Vec<BoundExpr>, schema: DataSchemaRef) -> Self {
        assert_eq!(exprs.len(), schema.fields().len());
        assert!(
            exprs
                .iter()
                .zip(schema.fields())
                .all(|(e, f)| &e.logical_type == f.ty())
        );
        let base = PlanBase {
            schema: Some(schema),
            children: vec![child],
        };
        Self { base, exprs }
    }
}

impl PlanData for Project {
    fn base(&self) -> &PlanBase {
        &self.base
    }
}
