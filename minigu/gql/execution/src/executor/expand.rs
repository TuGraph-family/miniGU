use std::sync::Arc;

use arrow::array::{Array, AsArray, ListArray};
use arrow::buffer::OffsetBuffer;
use arrow::datatypes::Field;
use itertools::Itertools;
use minigu_common::types::VertexIdArray;

use super::utils::gen_try;
use super::{Executor, IntoExecutor};
use crate::source::ExpandSource;

#[derive(Debug)]
pub struct ExpandBuilder<E, S> {
    child: E,
    input_column_index: usize,
    source: S,
}

impl<E, S> ExpandBuilder<E, S> {
    pub fn new(child: E, input_column_index: usize, source: S) -> Self {
        Self {
            child,
            input_column_index,
            source,
        }
    }
}

impl<E, S> IntoExecutor for ExpandBuilder<E, S>
where
    E: Executor,
    S: ExpandSource,
{
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let ExpandBuilder {
                child,
                input_column_index,
                source,
            } = self;
            for chunk in child.into_iter() {
                let mut chunk = gen_try!(chunk);
                // Compact the chunk to avoid expanding from vertices filtered out.
                chunk.compact();
                if chunk.is_empty() {
                    continue;
                }
                let input_column = chunk
                    .columns()
                    .get(input_column_index)
                    .expect("column with `input_column_index` should exist");
                let input_column: VertexIdArray = input_column.as_primitive().clone();
                // Only non-nullable columns can be expanded.
                assert!(
                    !input_column.is_nullable(),
                    "input column should not be nullable"
                );
                // TODO: Allow multiple vertices to be expanded at the same time.
                // NOTE: Due to the limitation of gen blocks, we cannot use the following code:
                // for (i, vertex) in input_column.values().into_iter().copied().enumerate() { ... }
                for i in 0..input_column.len() {
                    let vertex = input_column.value(i);
                    // Slice the chunk to the current row.
                    let chunk = chunk.slice(i, 1);
                    let expand_iter = if let Some(expand_iter) = source.expand_from_vertex(vertex) {
                        expand_iter
                    } else {
                        continue;
                    };
                    for neighbor_columns in expand_iter {
                        let mut chunk = chunk.clone();
                        let neighbor_columns = gen_try!(neighbor_columns);
                        let lists: Vec<_> = gen_try!(
                            neighbor_columns
                                .iter()
                                .map(|c| {
                                    let field = Field::new_list_field(c.data_type().clone(), false);
                                    let offsets = OffsetBuffer::from_lengths([c.len()]);
                                    ListArray::try_new(Arc::new(field), offsets, c.clone(), None)
                                        .map(|a| Arc::new(a) as _)
                                })
                                .try_collect()
                        );
                        chunk.append_columns(lists);
                        yield Ok(chunk);
                    }
                }
            }
        }
        .into_executor()
    }
}

#[cfg(test)]
mod tests {
    use arrow::array::{ArrayRef, ListBuilder, StringBuilder, UInt64Builder, create_array};
    use arrow::datatypes::DataType;
    use minigu_common::data_chunk;
    use minigu_common::data_chunk::DataChunk;

    use super::*;
    use crate::source::mock::{MockExpandSource, MockExpandSourceBuilder};

    fn build_test_source() -> MockExpandSource {
        MockExpandSourceBuilder::new(2)
            .add_vertex(1.try_into().unwrap())
            .add_vertex(3.try_into().unwrap())
            .add_vertex(5.try_into().unwrap())
            .add_edge(1.try_into().unwrap(), 2.try_into().unwrap(), "e1".into())
            .add_edge(1.try_into().unwrap(), 3.try_into().unwrap(), "e2".into())
            .add_edge(1.try_into().unwrap(), 4.try_into().unwrap(), "e3".into())
            .add_edge(3.try_into().unwrap(), 123.try_into().unwrap(), "e4".into())
            .build()
    }

    #[test]
    fn test_expand() {
        let chunk = data_chunk!(
            { true, false, true, true, true},
            (UInt64, [1, 2, 3, 4, 5]),
            (Utf8, ["abc", "def", "ghi", "jkl", "mno"])
        );
        let chunk: DataChunk = [Ok(chunk)]
            .into_executor()
            .expand(0, build_test_source())
            .into_iter()
            .try_collect()
            .unwrap();
        let neighbors_field = Field::new_list_field(DataType::UInt64, false);
        let mut neighbors_builder =
            ListBuilder::new(UInt64Builder::new()).with_field(Arc::new(neighbors_field));
        neighbors_builder.append_value([Some(2), Some(3)]);
        neighbors_builder.append_value([Some(4)]);
        neighbors_builder.append_value([Some(123)]);
        let neighbors = Arc::new(neighbors_builder.finish()) as ArrayRef;

        let properties_field = Field::new_list_field(DataType::Utf8, false);
        let mut properties_builder =
            ListBuilder::new(StringBuilder::new()).with_field(Arc::new(properties_field));
        properties_builder.append_value([Some("e1"), Some("e2")]);
        properties_builder.append_value([Some("e3")]);
        properties_builder.append_value([Some("e4")]);
        let properties = Arc::new(properties_builder.finish()) as ArrayRef;

        let expected = DataChunk::new(vec![
            create_array!(UInt64, [1, 1, 3]) as _,
            create_array!(Utf8, ["abc", "abc", "ghi"]) as _,
            neighbors as _,
            properties as _,
        ]);
        assert_eq!(chunk, expected);
    }
}
