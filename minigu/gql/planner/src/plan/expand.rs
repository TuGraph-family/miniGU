use minigu_common::types::LabelId;
use serde::Serialize;

use crate::plan::{PlanBase, PlanData, PlanNode};

#[derive(Debug, Clone, Serialize)]
pub enum ExpandDirection {
    Outgoing,
    Incoming,
    Both,
}

#[derive(Debug, Clone, Serialize)]
pub struct Expand {
    pub base: PlanBase,
    pub input_column_index: usize,
    pub edge_labels: Vec<Vec<LabelId>>,
    pub target_vertex_labels: Option<Vec<Vec<LabelId>>>,
    pub output_var: Option<String>, // Edge variable name (e.g., "f")
    pub target_vertex_var: Option<String>, // Target vertex variable name (e.g., "n")
    pub direction: ExpandDirection,
}

impl Expand {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        child: PlanNode,
        input_column_index: usize,
        edge_labels: Vec<Vec<LabelId>>,
        target_vertex_labels: Option<Vec<Vec<LabelId>>>,
        output_var: Option<String>,
        target_vertex_var: Option<String>,
        direction: ExpandDirection,
    ) -> Self {
        use std::sync::Arc;

        use minigu_common::data_type::{DataField, DataSchema, LogicalType};

        // Start with child's schema
        let mut new_fields = if let Some(child_schema) = child.schema() {
            child_schema.fields().to_vec()
        } else {
            Vec::new()
        };

        // Add edge variable if specified
        if let Some(edge_var) = &output_var {
            // Check if edge variable already exists in schema
            if !new_fields.iter().any(|f| f.name() == edge_var) {
                let edge_field = DataField::new(
                    edge_var.clone(),
                    LogicalType::Edge(vec![DataField::new("id".into(), LogicalType::Int64, false)]),
                    false,
                );
                new_fields.push(edge_field);
            }
        }

        // Add target vertex variable if specified
        if let Some(vertex_var) = &target_vertex_var {
            // Check if vertex variable already exists in schema
            if !new_fields.iter().any(|f| f.name() == vertex_var) {
                let vertex_field = DataField::new(
                    vertex_var.clone(),
                    LogicalType::Vertex(vec![DataField::new(
                        "id".into(),
                        LogicalType::Int64,
                        false,
                    )]),
                    false,
                );
                new_fields.push(vertex_field);
            }
        }

        let schema = Some(Arc::new(DataSchema::new(new_fields)));
        let base = PlanBase {
            schema,
            children: vec![child],
        };
        Self {
            base,
            input_column_index,
            edge_labels,
            target_vertex_labels,
            output_var,
            target_vertex_var,
            direction,
        }
    }
}

impl PlanData for Expand {
    fn base(&self) -> &PlanBase {
        &self.base
    }

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let mut output = String::new();
        output.push_str(&format!(
            "{}Expand: direction={:?}, edge_labels={:?}, target_vertex_labels={:?}, output_var={:?}, target_vertex_var={:?}\n",
            indent_str,
            self.direction,
            self.edge_labels,
            self.target_vertex_labels,
            self.output_var,
            self.target_vertex_var
        ));

        for child in self.children() {
            output.push_str(child.explain(indent + 1)?.as_str());
        }

        Some(output)
    }
}
