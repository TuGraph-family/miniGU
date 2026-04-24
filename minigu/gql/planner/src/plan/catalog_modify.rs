use serde::Serialize;
use smol_str::SmolStr;

use crate::bound::{BoundGraphType, CreateKind};
use crate::plan::{PlanBase, PlanData};

/// Physical plan node for CREATE GRAPH statement
#[derive(Debug, Clone, Serialize)]
pub struct CreateGraph {
    pub base: PlanBase,
    pub name: SmolStr,
    pub kind: CreateKind,
    pub graph_type: BoundGraphType,
}

impl CreateGraph {
    pub fn new(name: SmolStr, kind: CreateKind, graph_type: BoundGraphType) -> Self {
        let base = PlanBase::new(None, vec![]);
        Self {
            base,
            name,
            kind,
            graph_type,
        }
    }
}

impl PlanData for CreateGraph {
    fn base(&self) -> &PlanBase {
        &self.base
    }

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let kind_str = match self.kind {
            CreateKind::Create => "CREATE",
            CreateKind::CreateIfNotExists => "CREATE IF NOT EXISTS",
            CreateKind::CreateOrReplace => "CREATE OR REPLACE",
        };
        Some(format!(
            "{}CreateGraph: {} GRAPH {}\n",
            indent_str, kind_str, self.name
        ))
    }
}

/// Physical plan node for DROP GRAPH statement
#[derive(Debug, Clone, Serialize)]
pub struct DropGraph {
    pub base: PlanBase,
    pub name: SmolStr,
    pub if_exists: bool,
}

impl DropGraph {
    pub fn new(name: SmolStr, if_exists: bool) -> Self {
        let base = PlanBase::new(None, vec![]);
        Self {
            base,
            name,
            if_exists,
        }
    }
}

impl PlanData for DropGraph {
    fn base(&self) -> &PlanBase {
        &self.base
    }

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let if_exists_str = if self.if_exists { " IF EXISTS" } else { "" };
        Some(format!(
            "{}DropGraph: DROP GRAPH{} {}\n",
            indent_str, if_exists_str, self.name
        ))
    }
}
