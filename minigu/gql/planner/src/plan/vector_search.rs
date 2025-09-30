use minigu_common::data_type::{DataSchemaRef, LogicalType};
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
    ) -> Result<Self, String> {
        // Validate dimension compatibility between index and query vector
        match &query_vector_expr.logical_type {
            LogicalType::Vector(query_dim) => {
                if *query_dim != index_key.dimension {
                    return Err(format!(
                        "Vector dimension mismatch: index expects {}, query vector has {}",
                        index_key.dimension, query_dim
                    ));
                }
            }
            _ => {
                return Err("Query expression must be a vector type".to_string());
            }
        }

        let base = PlanBase::new(Some(schema), vec![]);
        Ok(Self {
            base,
            index_key,
            query_vector_expr,
            k,
            metric,
        })
    }
}

impl PlanData for VectorSearch {
    fn base(&self) -> &PlanBase {
        &self.base
    }
}
