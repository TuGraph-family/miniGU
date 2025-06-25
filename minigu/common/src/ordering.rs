use arrow::compute::SortOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SortOrdering {
    #[default]
    Ascending,
    Descending,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NullOrdering {
    First,
    #[default]
    Last,
}

#[inline]
pub fn build_sort_options(ordering: SortOrdering, null_ordering: NullOrdering) -> SortOptions {
    let descending = matches!(ordering, SortOrdering::Descending);
    let nulls_first = matches!(null_ordering, NullOrdering::First);
    SortOptions::new(descending, nulls_first)
}
