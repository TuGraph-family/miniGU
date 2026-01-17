use minigu_catalog::provider::VectorIndexCatalogEntry;
use minigu_common::types::VectorIndexKey;
use serde::Serialize;
use smol_str::SmolStr;

use crate::plan::{PlanBase, PlanData};

#[derive(Debug, Clone, Serialize)]
pub struct DropVectorIndex {
    pub base: PlanBase,
    pub name: SmolStr,
    pub if_exists: bool,
    pub index_key: Option<VectorIndexKey>,
    pub metadata: Option<VectorIndexCatalogEntry>,
    pub no_op: bool,
}

impl DropVectorIndex {
    pub fn new(
        name: SmolStr,
        if_exists: bool,
        index_key: Option<VectorIndexKey>,
        metadata: Option<VectorIndexCatalogEntry>,
        no_op: bool,
    ) -> Self {
        let base = PlanBase::new(None, vec![]);
        Self {
            base,
            name,
            if_exists,
            index_key,
            metadata,
            no_op,
        }
    }
}

impl PlanData for DropVectorIndex {
    fn base(&self) -> &PlanBase {
        &self.base
    }

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let mut output = format!(
            "{}DropVectorIndex: name={} if_exists={} no_op={} key_present={}\n",
            indent_str,
            self.name,
            self.if_exists,
            self.no_op,
            self.index_key.is_some(),
        );
        for child in self.children() {
            output.push_str(child.explain(indent + 1)?.as_str());
        }
        Some(output)
    }
}
