pub mod call;
pub mod explain;
pub mod filter;
pub mod limit;
pub mod logical_match;
pub mod one_row;
pub mod project;
pub mod scan;
pub mod sort;
pub mod vector_index_scan;

use std::sync::Arc;

use minigu_common::data_type::DataSchemaRef;
use serde::Serialize;

use crate::plan::call::Call;
use crate::plan::explain::Explain;
use crate::plan::filter::Filter;
use crate::plan::limit::Limit;
use crate::plan::logical_match::LogicalMatch;
use crate::plan::one_row::OneRow;
use crate::plan::project::Project;
use crate::plan::scan::PhysicalNodeScan;
use crate::plan::sort::Sort;
use crate::plan::vector_index_scan::VectorIndexScan;

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

pub trait PlanData {
    fn base(&self) -> &PlanBase;

    fn schema(&self) -> Option<&DataSchemaRef> {
        self.base().schema()
    }

    fn children(&self) -> &[PlanNode] {
        self.base().children()
    }
    // each node needs
    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let mut output = format!("{}ERROR: explain() not implemented\n", indent_str);
        for child in self.children() {
            output.push_str(child.explain(indent + 1)?.as_str());
        }
        Some(output)
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum PlanNode {
    LogicalMatch(Arc<LogicalMatch>),
    LogicalFilter(Arc<Filter>),
    LogicalProject(Arc<Project>),
    LogicalCall(Arc<Call>),
    LogicalOneRow(Arc<OneRow>),
    // TODO: Remove logical sort in the future.
    // Ordering is a physical property of a plan node, and it should be enforced by the optimizer
    // (by inserting PhysicalSort).
    LogicalSort(Arc<Sort>),
    LogicalLimit(Arc<Limit>),
    LogicalVectorIndexScan(Arc<VectorIndexScan>),
    LogicalExplain(Arc<Explain>),

    PhysicalFilter(Arc<Filter>),
    PhysicalProject(Arc<Project>),
    PhysicalCall(Arc<Call>),
    PhysicalOneRow(Arc<OneRow>),
    PhysicalSort(Arc<Sort>),
    PhysicalLimit(Arc<Limit>),
    PhysicalVectorIndexScan(Arc<VectorIndexScan>),
    //  PhysicalNodeScan retrieves node ids based on labels during the scan phase,
    //  without immediately materializing full node attributes.
    //  During subsequent matching and computation, these ids are lazily expanded
    //  into complete attribute representations (ArrayRefs) only when required,
    //  to improve performance and reduce unnecessary data loading.
    PhysicalNodeScan(Arc<PhysicalNodeScan>),
    // PhysicalCatalogModify(Arc<PhysicalCatalogModify>)
    PhysicalExplain(Arc<Explain>),
}

impl PlanData for PlanNode {
    fn base(&self) -> &PlanBase {
        match self {
            PlanNode::LogicalMatch(node) => node.base(),
            PlanNode::LogicalFilter(node) => node.base(),
            PlanNode::LogicalProject(node) => node.base(),
            PlanNode::LogicalCall(node) => node.base(),
            PlanNode::LogicalOneRow(node) => node.base(),
            PlanNode::LogicalSort(node) => node.base(),
            PlanNode::LogicalLimit(node) => node.base(),
            PlanNode::LogicalExplain(node) => node.base(),

            PlanNode::PhysicalFilter(node) => node.base(),
            PlanNode::PhysicalProject(node) => node.base(),
            PlanNode::PhysicalCall(node) => node.base(),
            PlanNode::PhysicalOneRow(node) => node.base(),
            PlanNode::PhysicalSort(node) => node.base(),
            PlanNode::PhysicalLimit(node) => node.base(),
            PlanNode::PhysicalNodeScan(node) => node.base(),
            PlanNode::LogicalVectorIndexScan(node) => node.base(),
            PlanNode::PhysicalVectorIndexScan(node) => node.base(),
            PlanNode::PhysicalExplain(node) => node.base(),
        }
    }

    fn explain(&self, indent: usize) -> Option<String> {
        match self {
            PlanNode::LogicalMatch(node) => node.explain(indent),
            PlanNode::LogicalFilter(node) => node.explain(indent),
            PlanNode::LogicalProject(node) => node.explain(indent),
            PlanNode::LogicalCall(node) => node.explain(indent),
            PlanNode::LogicalOneRow(node) => node.explain(indent),
            PlanNode::LogicalSort(node) => node.explain(indent),
            PlanNode::LogicalLimit(node) => node.explain(indent),
            PlanNode::LogicalVectorIndexScan(node) => node.explain(indent),
            PlanNode::LogicalExplain(node) => node.explain(indent),

            PlanNode::PhysicalFilter(node) => node.explain(indent),
            PlanNode::PhysicalProject(node) => node.explain(indent),
            PlanNode::PhysicalCall(node) => node.explain(indent),
            PlanNode::PhysicalOneRow(node) => node.explain(indent),
            PlanNode::PhysicalSort(node) => node.explain(indent),
            PlanNode::PhysicalLimit(node) => node.explain(indent),
            PlanNode::PhysicalVectorIndexScan(node) => node.explain(indent),
            PlanNode::PhysicalNodeScan(node) => node.explain(indent),
            PlanNode::PhysicalExplain(node) => node.explain(indent),
        }
    }
}
