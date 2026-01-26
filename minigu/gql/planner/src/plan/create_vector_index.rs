use minigu_common::types::{VectorIndexKey, VectorMetric};
use serde::Serialize;
use smol_str::SmolStr;

use crate::plan::{PlanBase, PlanData};

#[derive(Debug, Clone, Serialize)]
pub struct CreateVectorIndex {
    pub base: PlanBase,
    pub name: SmolStr,
    pub if_not_exists: bool,
    pub index_key: VectorIndexKey,
    pub metric: VectorMetric,
    pub dimension: usize,
    pub label: SmolStr,
    pub property: SmolStr,
    pub no_op: bool,
}

impl CreateVectorIndex {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: SmolStr,
        if_not_exists: bool,
        index_key: VectorIndexKey,
        metric: VectorMetric,
        dimension: usize,
        label: SmolStr,
        property: SmolStr,
        no_op: bool,
    ) -> Self {
        let base = PlanBase::new(None, vec![]);
        Self {
            base,
            name,
            if_not_exists,
            index_key,
            metric,
            dimension,
            label,
            property,
            no_op,
        }
    }
}

impl PlanData for CreateVectorIndex {
    fn base(&self) -> &PlanBase {
        &self.base
    }

    fn explain(&self, indent: usize) -> Option<String> {
        let indent_str = " ".repeat(indent * 2);
        let mut output = format!(
            "{}CreateVectorIndex: name={}, binding=({}:{}) metric={:?} dim={} if_not_exists={} no_op={}\n",
            indent_str,
            self.name,
            self.label,
            self.property,
            self.metric,
            self.dimension,
            self.if_not_exists,
            self.no_op,
        );
        for child in self.children() {
            output.push_str(child.explain(indent + 1)?.as_str());
        }
        Some(output)
    }
}
