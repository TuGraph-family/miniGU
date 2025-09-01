use std::num::NonZeroU32;

use arrow::array::UInt64Array;

/// Internal identifier associated with a label.
///
/// # Examples
/// [`NonZeroU32`] is used to enable some memory layout optimizations.
/// For example, `Option<LabelId>` is guaranteed to have the same size as `LabelId`, which is 4
/// bytes:
/// ```
/// # use std::mem::size_of;
/// # use minigu_common::types::LabelId;
/// assert_eq!(size_of::<Option<LabelId>>(), size_of::<LabelId>());
/// assert_eq!(size_of::<Option<LabelId>>(), 4);
/// ```
pub type LabelId = NonZeroU32;

/// Internal identifier associated with a vertex.
pub type VertexId = u64;

/// An array of vertex IDs.
pub type VertexIdArray = UInt64Array;

/// Internal identifier associated with an edge (graph-wide unique).
pub type EdgeId = u64;

/// Internal identifier associated with a transaction (database-wide unique).
pub type TxnId = u64;

/// Internal identifier associated with a graph (database-wide unique).
pub type GraphId = u32;

/// Internal identifier associated with a property (vertex/edge-type-wide unique).
pub type PropertyId = u32;

/// Internal identifier associated with a procedure (database-wide unique).
pub type ProcedureId = u32;

/// Uses (LabelId, PropertyId) to uniquely identify vector indices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VectorIndexKey {
    pub label_id: LabelId,
    pub property_id: PropertyId,
}

impl VectorIndexKey {
    #[inline]
    pub fn new(label_id: LabelId, property_id: PropertyId) -> Self {
        Self {
            label_id,
            property_id,
        }
    }
}
