use serde::Serialize;
use crate::plan::{PlanBase, PlanData};

#[derive(Debug, Clone, Serialize)]
pub struct PhysicalNodeScan {
    pub base: PlanBase,
    pub var: String,
    pub labels: Vec<String>,
    pub graph_id: i64,
    // Schema Ref.
}


// impl PhysicalNodeScan {
//     pub fn new()
// }
impl PlanData for PhysicalNodeScan {
    fn base(&self) -> &PlanBase { &self.base}
}