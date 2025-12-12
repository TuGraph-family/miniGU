use std::sync::Arc;

use minigu_common::data_type::{DataField, DataSchema};
use serde::Serialize;

use crate::bound::BoundCatalogModifyingStatement;
use crate::plan::{PlanBase, PlanData};

#[derive(Debug, Clone, Serialize)]
pub struct CatalogDdl {
    #[serde(skip)]
    base: PlanBase,
    pub statement: BoundCatalogModifyingStatement,
}

impl CatalogDdl {
    pub fn new(statement: BoundCatalogModifyingStatement) -> Self {
        // DDL returns a single empty row (no columns)
        let base = PlanBase::new(
            Some(Arc::new(DataSchema::new(Vec::<DataField>::new()))),
            vec![],
        );
        Self { base, statement }
    }
}

impl PlanData for CatalogDdl {
    fn base(&self) -> &PlanBase {
        &self.base
    }
}
