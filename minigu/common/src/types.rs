use std::num::NonZeroU32;

/// Identifier associated with a label.
///
/// # Examples
/// [`NonZeroU32`] is used to enable some memory layout optimizations.
/// For example, `Option<LabelId>` is guaranteed to have the same size as `LabelId`, which is 4
/// bytes:
/// ```
/// use std::mem::size_of;
///
/// use minigu_common::types::LabelId;
///
/// assert_eq!(size_of::<Option<LabelId>>(), size_of::<LabelId>());
/// assert_eq!(size_of::<Option<LabelId>>(), 4);
/// ```
pub type LabelId = NonZeroU32;
