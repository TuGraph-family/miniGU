use std::sync::Arc;

use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_common::types::LabelId;
use serde::Serialize;

use crate::plan::{PlanBase, PlanData};

#[derive(Debug, Clone, Serialize)]
pub struct PhysicalNodeScan {
    pub base: PlanBase,
    pub var: String,
    // DNF: outer OR, inner AND
    // labels = [ [] ] => Any
    // labels = [ [A,B] ] LabelA and LabelB
    // labels = [ [A], [B] ] LabelA or LabelB
    pub labels: Vec<Vec<LabelId>>,
    pub graph_id: i64,
}

impl PhysicalNodeScan {
    pub fn new(var: &str, labels: Vec<Vec<LabelId>>, graph_id: i64) -> Self {
        // For Single Node Scan, We just assume the id is only needed.
        let field = DataField::new(var.to_string(), LogicalType::Int64, false);
        let schema = DataSchema::new(vec![field]);
        let base = PlanBase {
            schema: Some(Arc::new(schema)),
            children: vec![],
        };
        Self {
            base,
            var: var.to_string(),
            labels,
            graph_id,
        }
    }
}

impl PlanData for PhysicalNodeScan {
    fn base(&self) -> &PlanBase {
        &self.base
    }
}
