use arrow::compute::SortOptions;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SortOrdering {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NullOrdering {
    First,
    Last,
}

#[inline]
pub fn build_sort_options(ordering: SortOrdering, null_ordering: NullOrdering) -> SortOptions {
    let descending = matches!(ordering, SortOrdering::Descending);
    let nulls_first = matches!(null_ordering, NullOrdering::First);
    SortOptions::new(descending, nulls_first)
}
