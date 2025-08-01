use minigu_common::data_chunk::DataChunk;

use super::utils::gen_try;
use super::{Executor, IntoExecutor};
use crate::source::VertexSource;

#[derive(Debug)]
pub struct VertexScanBuilder<S>(S);

impl<S> VertexScanBuilder<S> {
    pub fn new(source: S) -> Self {
        Self(source)
    }
}

impl<S> IntoExecutor for VertexScanBuilder<S>
where
    S: VertexSource,
{
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            for vertices in self.0 {
                let vertices = gen_try!(vertices);
                if vertices.is_empty() {
                    continue;
                }
                yield Ok(DataChunk::new(vec![vertices]))
            }
        }
        .into_executor()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use minigu_common::data_chunk;
    use minigu_common::types::VertexIdArray;

    use super::*;

    #[test]
    fn test_vertex_scan() {
        let vertices = Arc::new(VertexIdArray::from_iter(0..5));
        let chunk = [Ok(vertices)]
            .into_iter()
            .scan_vertex()
            .next_chunk()
            .unwrap()
            .unwrap();
        let expected = data_chunk!((UInt64, [0, 1, 2, 3, 4]));
        assert_eq!(chunk, expected);
    }
}
