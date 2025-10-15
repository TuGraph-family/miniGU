use std::sync::Arc;
use serde::Serialize;
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_common::types::LabelId;
use crate::plan::{PlanBase, PlanData};

#[derive(Debug, Clone, Serialize)]
pub struct PhysicalNodeScan {
    pub base: PlanBase,
    pub var: String,
    pub labels: Vec<LabelId>,
    pub graph_id: i64,
}

impl PhysicalNodeScan {
    pub fn new(var: &str, labels: Vec<LabelId>, graph_id: i64) -> Self {
        // For Single Node Scan, We just assume the id is only need.
        let field = DataField::new("id".to_string(), LogicalType::Int64,false);
        let schema = DataSchema::new(vec![field]);
        let base = PlanBase {
            schema: Some(Arc::new(schema)),
            children: vec![]
        };
        Self {
            base:base,
            var:var.to_string(),
            labels: labels.clone(),
            graph_id,
        }
    }
}
impl PlanData for PhysicalNodeScan {
    fn base(&self) -> &PlanBase { &self.base}
}