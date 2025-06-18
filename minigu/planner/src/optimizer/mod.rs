use std::sync::Arc;

use itertools::Itertools;
use minigu_ir::plan::{Call, PlanNode};

use crate::error::PlanResult;

#[derive(Debug, Default)]
pub struct Optimizer {}

impl Optimizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_physical_plan(self, logical_plan: PlanNode) -> PlanResult<PlanNode> {
        match logical_plan {
            PlanNode::LogicalMatch(_) => todo!(),
            PlanNode::LogicalFilter(filter) => Ok(PlanNode::PhysicalFilter(filter)),
            PlanNode::LogicalProject(project) => Ok(PlanNode::PhysicalProject(project)),
            PlanNode::LogicalCall(call) => Ok(PlanNode::PhysicalCall(call)),
            _ => unreachable!(),
        }
    }
}
