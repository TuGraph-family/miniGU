use std::sync::Arc;

use minigu_common::data_type::DataSchema;
use serde::Serialize;

use crate::bound::{BoundExpr, BoundGraphPattern};
use crate::plan::{PlanBase, PlanData, PlanNode};

/// Represents an OPTIONAL MATCH operation in the logical plan.
/// Semantically equivalent to LEFT OUTER JOIN in relational databases.
///
/// The child plan provides the "left" side of the LEFT JOIN.
/// If the optional pattern matches, the output includes combined rows.
/// If the optional pattern doesn't match, the output includes the left row
/// with NULL values for columns introduced by the optional pattern.
#[derive(Debug, Clone, Serialize)]
pub struct LogicalOptionalMatch {
    pub base: PlanBase,
    /// The child plan providing the "left" side of the LEFT JOIN.
    /// For a standalone OPTIONAL MATCH, this is typically a OneRow node.
    /// For chained OPTIONAL MATCH, this is the previous plan node.
    pub child: PlanNode,
    /// The graph pattern to optionally match.
    pub pattern: BoundGraphPattern,
    /// Expressions to yield from the optional pattern.
    pub yield_clause: Vec<BoundExpr>,
    /// Schema of the output (includes NULL-able columns from optional pattern).
    pub output_schema: DataSchema,
}

impl LogicalOptionalMatch {
    pub fn new(
        child: PlanNode,
        pattern: BoundGraphPattern,
        yield_clause: Vec<BoundExpr>,
        output_schema: DataSchema,
    ) -> Self {
        let schema_ref = Some(Arc::new(output_schema.clone()));
        let base = PlanBase::new(schema_ref, vec![child.clone()]);
        Self {
            base,
            child,
            pattern,
            yield_clause,
            output_schema,
        }
    }
}

impl PlanData for LogicalOptionalMatch {
    fn base(&self) -> &PlanBase {
        &self.base
    }

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let mut output = format!("{}LogicalOptionalMatch\n", indent_str);

        for child in self.children() {
            output.push_str(child.explain(indent + 1)?.as_str());
        }

        Some(output)
    }
}

/// Physical plan node for OPTIONAL MATCH execution.
/// This is the physical counterpart of LogicalOptionalMatch.
#[derive(Debug, Clone, Serialize)]
pub struct PhysicalOptionalMatch {
    pub base: PlanBase,
    /// The left child (preserved rows).
    pub left: PlanNode,
    /// The right child (optional pattern).
    pub right: PlanNode,
    /// Schema for the right side (used to generate NULL values when no match).
    pub right_schema: DataSchema,
}

impl PhysicalOptionalMatch {
    pub fn new(left: PlanNode, right: PlanNode, right_schema: DataSchema) -> Self {
        // Output schema is left schema + right schema (with nullable columns)
        let left_schema = left.schema().expect("left child must have schema");
        let mut output_fields: Vec<minigu_common::data_type::DataField> = left_schema.fields().to_vec();
        for field in right_schema.fields() {
            use minigu_common::data_type::DataField;
            // Make right columns nullable
            let nullable_field = DataField::new(
                field.name().to_string(),
                field.ty().clone(),
                true,
            );
            output_fields.push(nullable_field);
        }
        let output_schema = Arc::new(DataSchema::new(output_fields));
        let base = PlanBase::new(Some(output_schema), vec![left.clone(), right.clone()]);
        Self {
            base,
            left,
            right,
            right_schema,
        }
    }
}

impl PlanData for PhysicalOptionalMatch {
    fn base(&self) -> &PlanBase {
        &self.base
    }

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let mut output = format!("{}PhysicalOptionalMatch\n", indent_str);

        for child in self.children() {
            output.push_str(child.explain(indent + 1)?.as_str());
        }

        Some(output)
    }
}