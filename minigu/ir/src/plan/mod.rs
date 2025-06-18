mod call;
mod filter;
mod logical_match;
mod project;

use std::sync::Arc;

pub use call::Call;
pub use filter::Filter;
pub use logical_match::LogicalMatch;
use minigu_common::data_type::DataSchemaRef;
pub use project::Project;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PlanBase {
    schema: Option<DataSchemaRef>,
    children: Vec<PlanNode>,
}

impl PlanBase {
    pub fn new(schema: Option<DataSchemaRef>, children: Vec<PlanNode>) -> Self {
        Self { schema, children }
    }

    pub fn schema(&self) -> Option<&DataSchemaRef> {
        self.schema.as_ref()
    }

    pub fn children(&self) -> &[PlanNode] {
        &self.children
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum PlanNode {
    LogicalMatch(Arc<LogicalMatch>),
    LogicalFilter(Arc<Filter>),
    LogicalProject(Arc<Project>),
    LogicalCall(Arc<Call>),

    PhysicalFilter(Arc<Filter>),
    PhysicalProject(Arc<Project>),
    PhysicalCall(Arc<Call>),
}

impl PlanNode {
    // TODO: Rewrite this with macro
    pub fn schema(&self) -> Option<&DataSchemaRef> {
        match self {
            PlanNode::LogicalMatch(node) => node.schema(),
            PlanNode::LogicalFilter(node) => node.schema(),
            PlanNode::LogicalProject(node) => node.schema(),
            PlanNode::LogicalCall(node) => node.schema(),
            PlanNode::PhysicalFilter(node) => node.schema(),
            PlanNode::PhysicalProject(node) => node.schema(),
            PlanNode::PhysicalCall(node) => node.schema(),
        }
    }

    // TODO: Rewrite this with macro
    pub fn children(&self) -> &[PlanNode] {
        match self {
            PlanNode::LogicalMatch(node) => node.children(),
            PlanNode::LogicalFilter(node) => node.children(),
            PlanNode::LogicalProject(node) => node.children(),
            PlanNode::LogicalCall(node) => node.children(),
            PlanNode::PhysicalFilter(node) => node.children(),
            PlanNode::PhysicalProject(node) => node.children(),
            PlanNode::PhysicalCall(node) => node.children(),
        }
    }
}
