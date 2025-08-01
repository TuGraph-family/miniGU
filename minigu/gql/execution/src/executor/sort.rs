use arrow::array::UInt64Array;
use arrow::row::{RowConverter, SortField};
use itertools::Itertools;
use minigu_common::data_chunk::DataChunk;
use minigu_common::ordering::{NullOrdering, SortOrdering, build_sort_options};

use super::utils::gen_try;
use super::{Executor, IntoExecutor};
use crate::evaluator::BoxedEvaluator;
use crate::evaluator::datum::DatumRef;

#[derive(Debug)]
pub struct SortSpec {
    key: BoxedEvaluator,
    sort_ordering: SortOrdering,
    null_ordering: NullOrdering,
}

impl SortSpec {
    #[inline]
    pub fn new(
        key: BoxedEvaluator,
        sort_ordering: SortOrdering,
        null_ordering: NullOrdering,
    ) -> Self {
        Self {
            key,
            sort_ordering,
            null_ordering,
        }
    }
}

#[derive(Debug)]
pub struct SortBuilder<E> {
    child: E,
    specs: Vec<SortSpec>,
    max_chunk_size: usize,
}

impl<E> SortBuilder<E> {
    pub fn new(child: E, specs: Vec<SortSpec>, max_chunk_size: usize) -> Self {
        assert!(!specs.is_empty(), "at least one sort spec is required");
        assert_ne!(max_chunk_size, 0, "max chunk size must be positive");
        Self {
            child,
            specs,
            max_chunk_size,
        }
    }
}

impl<E> IntoExecutor for SortBuilder<E>
where
    E: Executor,
{
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let SortBuilder {
                child,
                specs,
                max_chunk_size,
            } = self;
            let chunk: DataChunk = gen_try!(child.into_iter().try_collect());
            // `chunk` is guaranteed to be compacted here.
            if chunk.is_empty() {
                return;
            }
            let key_columns: Vec<_> = gen_try!(
                specs
                    .iter()
                    .map(|s| s.key.evaluate(&chunk).map(DatumRef::into_array))
                    .try_collect()
            );
            let fields = key_columns
                .iter()
                .zip(specs)
                .map(|(c, spec)| {
                    SortField::new_with_options(
                        c.data_type().clone(),
                        build_sort_options(spec.sort_ordering, spec.null_ordering),
                    )
                })
                .collect();
            let converter = gen_try!(RowConverter::new(fields));
            let rows = gen_try!(converter.convert_columns(&key_columns));
            let indices = rows
                .into_iter()
                .enumerate()
                .sorted_unstable_by_key(|(_, r)| *r)
                .map(|(i, _)| i as u64);
            let indices = UInt64Array::from_iter_values(indices);
            let chunk = chunk.take(&indices);
            let len = chunk.len();
            for offset in (0..len).step_by(max_chunk_size) {
                let length = max_chunk_size.min(len - offset);
                yield Ok(chunk.slice(offset, length));
            }
        }
        .into_executor()
    }
}

#[cfg(test)]
mod tests {
    use minigu_common::data_chunk;

    use super::*;
    use crate::evaluator::Evaluator;
    use crate::evaluator::column_ref::ColumnRef;

    #[test]
    fn test_sort_1() {
        let chunk1 = data_chunk!(
            (Int32, [Some(1), Some(1), Some(2)]),
            (Utf8, [Some("a"), Some("c"), Some("b")]),
            (Int32, [4, 5, 6])
        );
        let chunk2 = data_chunk!(
            { false, true, true },
            (Int32, [None, None, None]),
            (Utf8, [Some("c"), Some("d"), None]),
            (Int32, [Some(1), None, Some(3)])
        );
        // ORDER BY c1 ASC NULLS LAST, c2 DESC NULLS FIRST
        let key1 = Box::new(ColumnRef::new(0));
        let key2 = Box::new(ColumnRef::new(1));
        let chunks: Vec<_> = [Ok(chunk1), Ok(chunk2)]
            .into_executor()
            .sort(
                vec![
                    SortSpec::new(key1, SortOrdering::Ascending, NullOrdering::Last),
                    SortSpec::new(key2, SortOrdering::Descending, NullOrdering::First),
                ],
                3,
            )
            .into_iter()
            .try_collect()
            .unwrap();
        let expected = vec![
            data_chunk!(
                (Int32, [Some(1), Some(1), Some(2)]),
                (Utf8, [Some("c"), Some("a"), Some("b")]),
                (Int32, [Some(5), Some(4), Some(6)])
            ),
            data_chunk!(
                (Int32, [None, None]),
                (Utf8, [None, Some("d")]),
                (Int32, [Some(3), None])
            ),
        ];
        assert_eq!(chunks, expected);
    }

    #[test]
    fn test_sort_2() {
        let chunk1 = data_chunk!(
            (Int32, [Some(1), Some(1), Some(2)]),
            (Utf8, [Some("a"), Some("c"), Some("b")]),
            (Int32, [4, 5, 6])
        );
        let chunk2 = data_chunk!(
            { false, true, true },
            (Int32, [None, None, None]),
            (Utf8, [Some("c"), Some("d"), None]),
            (Int32, [Some(1), None, Some(3)])
        );
        // ORDER BY -c1 ASC NULLS LAST, c2 DESC NULLS FIRST
        let key1 = Box::new(ColumnRef::new(0).neg());
        let key2 = Box::new(ColumnRef::new(1));
        let chunks: Vec<_> = [Ok(chunk1), Ok(chunk2)]
            .into_executor()
            .sort(
                vec![
                    SortSpec::new(key1, SortOrdering::Ascending, NullOrdering::Last),
                    SortSpec::new(key2, SortOrdering::Descending, NullOrdering::First),
                ],
                3,
            )
            .into_iter()
            .try_collect()
            .unwrap();
        let expected = vec![
            data_chunk!(
                (Int32, [Some(2), Some(1), Some(1)]),
                (Utf8, [Some("b"), Some("c"), Some("a")]),
                (Int32, [Some(6), Some(5), Some(4)])
            ),
            data_chunk!(
                (Int32, [None, None]),
                (Utf8, [None, Some("d")]),
                (Int32, [Some(3), None])
            ),
        ];
        assert_eq!(chunks, expected);
    }
}
