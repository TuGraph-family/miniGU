use minigu_common::types::LabelId;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LabelSet(SmallVec<[LabelId; 4]>);

impl LabelSet {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    
    #[inline]
    pub fn first(&self) -> Option<LabelId> {
        self.0.first().cloned()
    }

    #[inline]
    pub fn contains(&self, label: LabelId) -> bool {
        self.0.binary_search(&label).is_ok()
    }
}

impl FromIterator<LabelId> for LabelSet {
    fn from_iter<T: IntoIterator<Item = LabelId>>(iter: T) -> Self {
        let mut set = SmallVec::from_iter(iter);
        set.sort_unstable();
        Self(set)
    }
}
