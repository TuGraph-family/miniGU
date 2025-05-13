use arrow::array::AsArray;

use super::utils::gen_try;
use super::{Executor, IntoExecutor};
use crate::source::VertexPropertySource;

#[derive(Debug)]
pub struct VertexPropertyScanBuilder<E, S> {
    child: E,
    input_column_index: usize,
    source: S,
}

impl<E, S> VertexPropertyScanBuilder<E, S> {
    pub fn new(child: E, input_column_index: usize, source: S) -> Self {
        Self {
            child,
            input_column_index,
            source,
        }
    }
}

impl<E, S> IntoExecutor for VertexPropertyScanBuilder<E, S>
where
    E: Executor,
    S: VertexPropertySource,
{
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let VertexPropertyScanBuilder {
                child,
                input_column_index,
                source,
            } = self;
            for chunk in child.into_iter() {
                let mut chunk = gen_try!(chunk);
                // Compact the chunk to avoid scanning the properties of vertices filtered out.
                chunk.compact();
                if chunk.is_empty() {
                    continue;
                }
                let input_column = chunk
                    .columns()
                    .get(input_column_index)
                    .expect("column with `input_column_index` should exist");
                let input_column = input_column.as_primitive();
                let properties = gen_try!(source.scan_vertex_properties(input_column));
                chunk.append_columns(properties);
                yield Ok(chunk);
            }
        }
        .into_executor()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use minigu_common::data_chunk;
    use minigu_common::data_chunk::DataChunk;

    use super::*;
    use crate::source::mock::MockVertexPropertySource;

    fn build_test_source() -> MockVertexPropertySource {
        let mut source = MockVertexPropertySource::new();
        source.add_vertex_property(1, "v1".to_string());
        source.add_vertex_property(2, "v2".to_string());
        source.add_vertex_property(3, "v3".to_string());
        source
    }

    #[test]
    fn test_vertex_property_scan() {
        let chunk = data_chunk!(
            { true, false, true, true, true},
            (UInt64, [1, 2, 3, 4, 5]),
            (Utf8, ["abc", "def", "ghi", "jkl", "mno"])
        );
        let chunk: DataChunk = [Ok(chunk)]
            .into_executor()
            .scan_vertex_property(0, build_test_source())
            .into_iter()
            .try_collect()
            .unwrap();
        let expected = data_chunk!(
            (UInt64, [1, 3, 4, 5]),
            (Utf8, ["abc", "ghi", "jkl", "mno"]),
            (Utf8, [Some("v1"), Some("v3"), None, None])
        );
        assert_eq!(chunk, expected);
    }
}
