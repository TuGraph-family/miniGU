use minigu_common::types::LabelId;
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LabelSet(SmallVec<[LabelId; 4]>);

impl LabelSet {
    pub fn new(labels: impl IntoIterator<Item = LabelId>) -> Self {
        let mut set = SmallVec::from_iter(labels);
        set.sort_unstable();
        Self(set)
    }
}
