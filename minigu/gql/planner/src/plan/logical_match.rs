use std::sync::Arc;

use minigu_common::data_type::DataSchema;
use serde::Serialize;

use crate::bound::{BoundExpr, BoundGraphPattern};
use crate::plan::{PlanBase, PlanData};

#[derive(Debug, Clone, Serialize)]
pub enum MatchKind {
    Simple,
    Optional,
}
#[derive(Debug, Clone, Serialize)]
pub struct LogicalMatch {
    pub base: PlanBase,
    pub kind: MatchKind,
    pub pattern: BoundGraphPattern,
    pub yield_clause: Vec<BoundExpr>,
    pub output_schema: DataSchema,
}

impl LogicalMatch {
    pub fn new(
        kind: MatchKind,
        pattern: BoundGraphPattern,
        yield_clause: Vec<BoundExpr>,
        output_schema: DataSchema,
    ) -> Self {
        let schema_ref = Some(Arc::new(output_schema.clone()));
        let base = PlanBase {
            schema: schema_ref,
            children: vec![],
        };
        Self {
            base,
            kind,
            pattern,
            yield_clause,
            output_schema,
        }
    }
}

impl PlanData for LogicalMatch {
    fn base(&self) -> &PlanBase {
        &self.base
    }
}
