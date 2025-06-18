use minigu_common::data_type::DataSchemaRef;
use minigu_common::value::ScalarValue;
use serde::Serialize;

use crate::named_ref::NamedProcedureRef;
use crate::plan::{PlanBase, PlanNode};

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

    pub fn schema(&self) -> Option<&DataSchemaRef> {
        self.base.schema()
    }

    pub fn children(&self) -> &[PlanNode] {
        &self.base.children
    }
}
