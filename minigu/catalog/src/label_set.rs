use minigu_common::types::LabelId;
use smallvec::SmallVec;

/// Set of label identifiers used to identify a node type or a relationship type.
///
/// Label identifiers in the set are guaranteed to be sorted.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LabelSet(SmallVec<[LabelId; 4]>);
