use minigu_common::types::{GraphId, LabelId};
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
    pub output_var: Option<String>,
    pub direction: ExpandDirection,
    pub graph_id: GraphId,
}

impl Expand {
    pub fn new(
        child: PlanNode,
        input_column_index: usize,
        edge_labels: Vec<Vec<LabelId>>,
        target_vertex_labels: Option<Vec<Vec<LabelId>>>,
        output_var: Option<String>,
        direction: ExpandDirection,
        graph_id: GraphId,
    ) -> Self {
        let schema = child.schema().cloned();
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
            direction,
            graph_id
        }
    }
}

impl PlanData for Expand {
    fn base(&self) -> &PlanBase { &self.base }
}
