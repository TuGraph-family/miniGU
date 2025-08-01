use minigu_catalog::named_ref::NamedProcedureRef;
use minigu_common::data_type::DataSchemaRef;
use minigu_common::value::ScalarValue;
use serde::Serialize;

use crate::plan::{PlanBase, PlanData};

#[derive(Debug, Clone, Serialize)]
pub struct Call {
    pub base: PlanBase,
    pub procedure: NamedProcedureRef,
    pub args: Vec<ScalarValue>,
}

impl Call {
    pub fn new(
        procedure: NamedProcedureRef,
        args: Vec<ScalarValue>,
        schema: Option<DataSchemaRef>,
    ) -> Self {
        let base = PlanBase {
            schema,
            children: vec![],
        };
        Self {
            base,
            procedure,
            args,
        }
    }
}

impl PlanData for Call {
    fn base(&self) -> &PlanBase {
        &self.base
    }
}
