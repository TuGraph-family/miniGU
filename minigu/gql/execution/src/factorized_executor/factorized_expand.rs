use arrow::array::{Array, ArrayRef, AsArray};
use arrow::compute;
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_pos;
use minigu_common::result_set::{DataChunkPos, DataPos};
use minigu_common::types::VertexIdArray;

use super::{FactorizedExecutor, IntoFactorizedExecutor};
use crate::executor::utils::gen_try;
use crate::source::ExpandSource;

#[derive(Debug)]
pub struct FactorizedExpandBuilder<E, S> {
    child: E,
    source: S,
    expand_chunk_pos: DataChunkPos,
    expand_col_idx: usize,
}

impl<E, S> FactorizedExpandBuilder<E, S> {
    pub fn new(child: E, source: S, expand_chunk_pos: DataChunkPos, expand_col_idx: usize) -> Self {
        Self {
            child,
            source,
            expand_chunk_pos,
            expand_col_idx,
        }
    }
}

impl<E, S> IntoFactorizedExecutor for FactorizedExpandBuilder<E, S>
where
    E: FactorizedExecutor,
    S: ExpandSource,
{
    type IntoFactorizedExecutor = impl FactorizedExecutor;

    fn into_factorized_executor(self) -> Self::IntoFactorizedExecutor {
        gen move {
            let FactorizedExpandBuilder {
                child,
                source,
                expand_chunk_pos,
                expand_col_idx,
            } = self;

            for result_set in child.into_iter() {
                let mut result_set = gen_try!(result_set);

                let col_to_expand =
                    result_set.get_column(&data_pos!(expand_chunk_pos, expand_col_idx));
                let input_column: VertexIdArray = col_to_expand.as_primitive().clone();
                // Only non-nullable columns can be expanded.
                assert!(
                    !input_column.is_nullable(),
                    "input column should not be nullable"
                );

                let chunk_to_expand = result_set
                    .get_data_chunk(expand_chunk_pos)
                    .expect("Chunk must exist");
                let cur_idx = chunk_to_expand
                    .cur_idx()
                    .expect("Expand: flat chunk has no cursor index");

                if cur_idx >= input_column.len() || input_column.is_null(cur_idx) {
                    yield Ok(result_set);
                    continue;
                }

                let vertex_id = input_column.value(cur_idx);
                let mut all_neighbor_columns: Vec<Vec<ArrayRef>> = vec![];

                if let Some(neighbor_iter) = source.expand_from_vertex(vertex_id) {
                    for neighbor_columns_res in neighbor_iter {
                        let neighbor_columns = gen_try!(neighbor_columns_res);
                        all_neighbor_columns.push(neighbor_columns);
                    }
                }

                if all_neighbor_columns.is_empty() {
                    yield Ok(result_set);
                    continue;
                }

                let num_columns = all_neighbor_columns[0].len();
                let mut merged_columns: Vec<ArrayRef> = Vec::with_capacity(num_columns);
                let mut arrays_to_concat: Vec<&dyn Array> =
                    Vec::with_capacity(all_neighbor_columns.len());

                for col_idx in 0..num_columns {
                    arrays_to_concat.clear();
                    for batch in &all_neighbor_columns {
                        arrays_to_concat.push(batch[col_idx].as_ref());
                    }

                    let concatenated = gen_try!(compute::concat(&arrays_to_concat));
                    merged_columns.push(concatenated);
                }

                let mut new_chunk = DataChunk::new(merged_columns);
                new_chunk.set_unflat();
                result_set.push(new_chunk);

                yield Ok(result_set);
            }
        }
        .into_factorized_executor()
    }
}

#[cfg(test)]
mod tests {
    use minigu_common::result_set::ResultSet;
    use minigu_common::{data_chunk, result_set};

    use super::*;
    use crate::error::ExecutionResult;
    use crate::source::mock::{MockExpandSource, MockExpandSourceBuilder};

    struct MockInput(ResultSet);
    impl FactorizedExecutor for MockInput {
        fn next_resultset(&mut self) -> Option<ExecutionResult<ResultSet>> {
            if self.0.num_data_chunks() > 0 {
                Some(Ok(std::mem::take(&mut self.0)))
            } else {
                None
            }
        }
    }

    // Edges:
    //     1 -> 2 (e1)
    //     1 -> 3 (e2)
    //     1 -> 4 (e3)
    //     3 -> 123 (e4)
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
    fn test_factorized_expand() {
        // ResultSet:
        //     Chunk (flat, idx = 0)
        //         [1, 3]
        // Test expand operation from vertex 1
        //
        // Output ResultSet:
        //     Chunk (flat, idx = 0)
        //         [1, 3]
        //     Chunk (unflat)
        //         [2, 3, 4]  // Neighbors of vertex 1 added as new unflat chunk
        use itertools::Itertools;

        let mut chunk = data_chunk!((UInt64, [1u64, 3u64]));
        chunk.set_cur_idx(Some(0));
        let input_result = result_set!(chunk.clone());

        let expand_executor = FactorizedExpandBuilder::new(
            MockInput(input_result),
            build_test_source(),
            DataChunkPos(0),
            0,
        )
        .into_factorized_executor();

        let results: Vec<ResultSet> = expand_executor.into_iter().try_collect().unwrap();

        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.num_data_chunks(), 2);
        let unflat_chunk = result
            .get_data_chunk(
                *result
                    .get_unflat_chunks()
                    .first()
                    .expect("Should have one unflat chunk"),
            )
            .unwrap();
        let neighbor_ids: &arrow::array::UInt64Array =
            unflat_chunk.columns()[0].as_any().downcast_ref().unwrap();
        let neighbor_values: Vec<u64> = neighbor_ids.values().iter().copied().collect();
        assert_eq!(neighbor_values, vec![2, 3, 4]);
    }
}
