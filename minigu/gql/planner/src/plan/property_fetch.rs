use std::sync::Arc;

use minigu_common::data_type::{DataField, DataSchemaRef, LogicalType};
use minigu_common::types::PropertyId;
use serde::Serialize;

use crate::plan::{PlanBase, PlanData, PlanNode};

#[derive(Debug, Clone, Serialize)]
pub struct VertexPropertyFetch {
    pub base: PlanBase,
    pub binding: String,
    pub property_ids: Vec<PropertyId>,
    pub outputs: Vec<PropertyOutput>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PropertyOutput {
    pub column_alias: String,
    pub ty: LogicalType,
    pub nullable: bool,
}

impl VertexPropertyFetch {
    pub fn new(
        child: PlanNode,
        binding: String,
        property_ids: Vec<PropertyId>,
        outputs: Vec<PropertyOutput>,
    ) -> Self {
        let mut schema = child
            .schema()
            .expect("child should have schema")
            .as_ref()
            .clone();
        for output in &outputs {
            schema.push_back(&DataField::new(
                output.column_alias.clone(),
                output.ty.clone(),
                output.nullable,
            ));
        }
        let base = PlanBase::new(Some(Arc::new(schema)), vec![child]);
        Self {
            base,
            binding,
            property_ids,
            outputs,
        }
    }

    pub fn clone_with_child(&self, child: PlanNode) -> Self {
        VertexPropertyFetch::new(
            child,
            self.binding.clone(),
            self.property_ids.clone(),
            self.outputs.clone(),
        )
    }

    pub fn schema(&self) -> Option<&DataSchemaRef> {
        self.base.schema()
    }
}

impl PlanData for VertexPropertyFetch {
    fn base(&self) -> &PlanBase {
        &self.base
    }

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let mut output = format!(
            "{}VertexPropertyFetch: binding={}, properties={} ids={:?}\n",
            indent_str,
            self.binding,
            self.outputs
                .iter()
                .map(|o| o.column_alias.clone())
                .collect::<Vec<_>>()
                .join(","),
            self.property_ids
        );
        for child in self.children() {
            output.push_str(child.explain(indent + 1)?.as_str());
        }
        Some(output)
    }
}
