use std::sync::Arc;

use minigu_common::data_type::{DataField, DataSchema, DataSchemaRef, LogicalType};
use minigu_common::types::{VectorIndexKey, VectorMetric};
use serde::Serialize;

use crate::bound::BoundExpr;
use crate::plan::{PlanBase, PlanData};

/// Plan node representing a vector index scan.
///
/// Downstream phases are expected to append this node after MATCH/WHERE filtering so the scan
/// can leverage the candidate bitmap produced by graph pattern evaluation.
///
/// The node produces two columns:
/// - The binding column (vertex identifier) named after `binding`.
/// - The precomputed distance column identified by `distance_alias`.
#[derive(Debug, Clone, Serialize)]
pub struct VectorIndexScan {
    pub base: PlanBase,
    /// Logical binding name (e.g. the MATCH variable) the scan populates.
    pub binding: String,
    /// Column name that downstream expressions can reference for distances.
    pub distance_alias: String,
    /// Descriptor identifying the vector index to use.
    pub index_key: VectorIndexKey,
    /// Expression yielding the query vector (typically a literal or parameter).
    pub query: BoundExpr,
    /// Similarity metric used by the index.
    pub metric: VectorMetric,
    /// Vector dimension enforced at bind time.
    pub dimension: usize,
    /// Number of neighbors to return.
    pub limit: usize,
    /// Whether the scan should perform approximate search (ANN) or exact.
    pub approximate: bool,
}

impl VectorIndexScan {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        binding: String,
        distance_alias: String,
        index_key: VectorIndexKey,
        query: BoundExpr,
        metric: VectorMetric,
        dimension: usize,
        limit: usize,
        approximate: bool,
    ) -> Self {
        let schema = DataSchema::new(vec![
            DataField::new(binding.clone(), LogicalType::UInt64, false),
            DataField::new(distance_alias.clone(), LogicalType::Float32, false),
        ]);
        let base = PlanBase::new(Some(Arc::new(schema)), vec![]);
        Self {
            base,
            binding,
            distance_alias,
            index_key,
            query,
            metric,
            dimension,
            limit,
            approximate,
        }
    }

    pub fn schema(&self) -> Option<&DataSchemaRef> {
        self.base.schema()
    }
}

impl PlanData for VectorIndexScan {
    fn base(&self) -> &PlanBase {
        &self.base
    }
}
