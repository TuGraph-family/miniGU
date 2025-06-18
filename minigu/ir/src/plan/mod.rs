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
        todo!()
    }

    // TODO: Rewrite this with macro
    pub fn children(&self) -> &[PlanNode] {
        todo!()
    }
}
