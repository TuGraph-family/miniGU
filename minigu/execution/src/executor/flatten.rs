use arrow::array::{Array, AsArray, ListArray, UInt64Builder};
use arrow::compute;
use itertools::Itertools;
use minigu_common::data_chunk::DataChunk;

use super::utils::gen_try;
use super::{Executor, IntoExecutor};

pub struct FlattenBuilder<E> {
    child: E,
    column_indices: Vec<usize>,
}

impl<E> FlattenBuilder<E> {
    pub fn new(child: E, column_indices: Vec<usize>) -> Self {
        assert!(
            !column_indices.is_empty(),
            "at least one column index should be provided"
        );
        assert!(
            column_indices.iter().all_unique(),
            "column indices should be unique"
        );
        Self {
            child,
            column_indices,
        }
    }
}

impl<E> IntoExecutor for FlattenBuilder<E>
where
    E: Executor,
{
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let FlattenBuilder {
                child,
                column_indices,
            } = self;
            for chunk in child.into_iter() {
                let mut chunk = gen_try!(chunk);
                chunk.compact();
                let columns_to_flatten: Vec<&ListArray> = column_indices
                    .iter()
                    .map(|&i| {
                        let column = chunk
                            .columns()
                            .get(i)
                            .expect("column with `i` should exist");
                        column.as_list()
                    })
                    .collect();
                assert!(
                    columns_to_flatten.iter().all(|c| !c.is_nullable()),
                    "only non-nullable lists can be flatten"
                );
                let mut total_flat_value_count = 0;
                let mut flat_column_segments =
                    vec![Vec::with_capacity(chunk.len()); columns_to_flatten.len()];
                for i in 0..chunk.len() {
                    let mut flat_value_count = None;
                    for (column, segments) in
                        columns_to_flatten.iter().zip(&mut flat_column_segments)
                    {
                        let segment = column.value(i);
                        if let Some(flat_value_count) = flat_value_count {
                            assert_eq!(
                                segment.len(),
                                flat_value_count,
                                "all flattened column segments should have the same length"
                            );
                        } else {
                            flat_value_count = Some(segment.len());
                        }
                        segments.push(segment);
                    }
                    total_flat_value_count += flat_value_count.unwrap();
                }
                let mut builder = UInt64Builder::with_capacity(total_flat_value_count);
                for i in 0..chunk.len() {
                    let flat_value_count = flat_column_segments[0][i].len();
                    builder.append_value_n(i as _, flat_value_count);
                }
                let indices = builder.finish();
                let flat_columns: Vec<_> = gen_try!(
                    flat_column_segments
                        .into_iter()
                        .map(|segments| {
                            compute::concat(&segments.iter().map(AsRef::as_ref).collect_vec())
                        })
                        .try_collect()
                );
                let mut new_columns = Vec::with_capacity(chunk.columns().len());
                for (i, column) in chunk.columns().iter().enumerate() {
                    if let Some((j, _)) = column_indices.iter().find_position(|j| **j == i) {
                        new_columns.push(flat_columns[j].clone());
                    } else {
                        new_columns.push(gen_try!(compute::take(column, &indices, None)));
                    }
                }
                yield Ok(DataChunk::new(new_columns));
            }
        }
        .into_executor()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use arrow::array::{ListBuilder, StringBuilder, create_array};
    use arrow::datatypes::{DataType, Field};
    use minigu_common::data_chunk;

    use super::*;

    #[test]
    fn test_flatten() {
        // c1, c2, c3, c4
        // 1, 4, [1], [a]
        // 2, 5, [2, 3, 4], [b, c, d]
        // 3, 6, [], []
        // after flatten:
        // c1, c2, c3, c4
        // 1, 4, 1, a
        // 2, 5, 2, b
        // 2, 5, 3, c
        // 2, 5, 4, d
        let c1 = create_array!(Int32, [1, 2, 3]);
        let c2 = create_array!(Int32, [4, 5, 6]);
        let c3 = {
            let field = Field::new_list_field(DataType::UInt64, false);
            let mut builder = ListBuilder::new(UInt64Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(1)]);
            builder.append_value([Some(2), Some(3), Some(4)]);
            builder.append_value([]);
            Arc::new(builder.finish())
        };
        let c4 = {
            let field = Field::new_list_field(DataType::Utf8, false);
            let mut builder = ListBuilder::new(StringBuilder::new()).with_field(Arc::new(field));
            builder.append_value([Some("a")]);
            builder.append_value([Some("b"), Some("c"), Some("d")]);
            builder.append_value([] as [Option<&str>; 0]);
            Arc::new(builder.finish())
        };
        let chunk = DataChunk::new(vec![c1, c2, c3, c4]);
        let chunk: DataChunk = [Ok(chunk)]
            .into_executor()
            .flatten(vec![2, 3])
            .into_iter()
            .try_collect()
            .unwrap();
        let expected = data_chunk!(
            (Int32, [1, 2, 2, 2]),
            (Int32, [4, 5, 5, 5]),
            (UInt64, [1, 2, 3, 4]),
            (Utf8, ["a", "b", "c", "d"])
        );
        assert_eq!(chunk, expected);
    }
}
