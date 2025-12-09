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

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let mut output = String::new();
        let proc_name = self.procedure.name().to_string();
        let args_str = self
            .args
            .iter()
            .map(|arg| format!("{:?}", arg))
            .collect::<Vec<_>>()
            .join(", ");
        output.push_str(&format!(
            "{}Call: {}({})\n",
            indent_str, proc_name, args_str
        ));

        for child in self.children() {
            output.push_str(child.explain(indent + 1)?.as_str());
        }

        Some(output)
    }
}
