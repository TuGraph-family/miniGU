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

/// Internal identifier associated with an edge.
pub type EdgeId = u64;

/// Internal identifier associated with a transaction.
pub type TxnId = u64;

/// Internal identifier associated with a property.
pub type PropertyId = u32;

/// Internal identifier associated with a graph.
pub type GraphId = NonZeroU32;
