use minigu_common::data_type::DataSchemaRef;
use minigu_common::types::{VectorIndexKey, VectorMetric};
use serde::Serialize;

use crate::bound::BoundExpr;
use crate::plan::{PlanBase, PlanData};

#[derive(Debug, Clone, Serialize)]
pub struct VectorSearch {
    pub base: PlanBase,
    pub index_key: VectorIndexKey,
    pub query_vector_expr: BoundExpr,
    pub k: usize,
    pub metric: VectorMetric,
}

impl VectorSearch {
    pub fn new(
        schema: DataSchemaRef,
        index_key: VectorIndexKey,
        query_vector_expr: BoundExpr,
        k: usize,
        metric: VectorMetric,
    ) -> Self {
        let base = PlanBase::new(Some(schema), vec![]);
        Self {
            base,
            index_key,
            query_vector_expr,
            k,
            metric,
        }
    }
}

impl PlanData for VectorSearch {
    fn base(&self) -> &PlanBase {
        &self.base
    }
}
