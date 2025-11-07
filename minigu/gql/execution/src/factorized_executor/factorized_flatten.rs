use arrow::array::ArrayRef;
use minigu_common::data_chunk::DataChunk;
use minigu_common::result_set::DataChunkPos;
#[cfg(test)]
use minigu_common::result_set::ResultSet;

use super::{FactorizedExecutor, IntoFactorizedExecutor};
use crate::executor::utils::gen_try;

#[derive(Debug)]
pub struct FactorizedFlattenBuilder<E> {
    child: E,
    target_chunk_pos: DataChunkPos,
}

impl<E> FactorizedFlattenBuilder<E> {
    pub fn new(child: E, target_chunk_pos: DataChunkPos) -> Self {
        Self {
            child,
            target_chunk_pos,
        }
    }
}

impl<E> IntoFactorizedExecutor for FactorizedFlattenBuilder<E>
where
    E: FactorizedExecutor,
{
    type IntoFactorizedExecutor = impl FactorizedExecutor;

    fn into_factorized_executor(self) -> Self::IntoFactorizedExecutor {
        gen move {
            let FactorizedFlattenBuilder {
                mut child,
                target_chunk_pos,
            } = self;

            while let Some(result) = child.next_resultset() {
                let mut input_rs = gen_try!(result);

                let target_chunk = input_rs
                    .get_data_chunk(target_chunk_pos)
                    .unwrap()
                    .as_ref()
                    .clone();
                assert!(target_chunk.is_unflat());
                let target_num_rows = target_chunk.len();

                // Get the flat chunk (should be exactly one)
                let flat_chunk_pos = input_rs.get_flat_chunks()[0];
                let flat_chunk = input_rs
                    .get_data_chunk(flat_chunk_pos)
                    .unwrap()
                    .as_ref()
                    .clone();
                let flat_cur_idx = flat_chunk.cur_idx().unwrap();

                let mut merged_columns: Vec<ArrayRef> = Vec::new();
                // Replicate the row at flat_cur_idx from flat chunk to match target_num_rows
                let replicate_indices: arrow::array::UInt64Array =
                    (0..target_num_rows).map(|_| Some(0u64)).collect();
                for col in flat_chunk.columns() {
                    let single_value = col.slice(flat_cur_idx, 1);
                    let replicated =
                        arrow::compute::take(single_value.as_ref(), &replicate_indices, None)
                            .unwrap();
                    merged_columns.push(replicated);
                }

                // Add target chunk columns
                merged_columns.extend(target_chunk.columns().iter().cloned());
                let merged_flat_chunk = DataChunk::new(merged_columns);

                // Remove both chunks at once (automatically handles ordering)
                input_rs.remove_multiple_chunks(&[flat_chunk_pos, target_chunk_pos]);

                // For each row in the target chunk, output a ResultSet
                for row_idx in 0..target_num_rows {
                    let mut output_rs = input_rs.clone();
                    let mut new_flat_chunk = merged_flat_chunk.clone();
                    new_flat_chunk.set_cur_idx(Some(row_idx));
                    output_rs.push(new_flat_chunk);

                    yield Ok(output_rs);
                }
            }
        }
        .into_factorized_executor()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use minigu_common::{data_chunk, result_set};

    use super::*;

    #[test]
    fn test_factorized_flatten() {
        // chunk0 (flat, cur_idx=1): [1, 2]
        // chunk1 (unflat): [100, 200]
        // chunk2 (unflat): ["a", "b", "c"]
        //
        // after flatten chunk1:
        // resultset1
        //  chunk0 (flat, cur_idx=0): [2, 2], [100, 200]
        //  chunk1 (unflat): ["a", "b", "c"]
        // resultset2
        //  chunk0 (flat, cur_idx=1): [2, 2], [100, 200]
        //  chunk1 (unflat): ["a", "b", "c"]
        let mut flat_chunk = data_chunk!((Int32, [1, 2]));
        flat_chunk.set_cur_idx(Some(1));
        let mut unflat1 = data_chunk!((Int32, [100, 200]));
        unflat1.set_unflat();
        let mut unflat2 = data_chunk!((Utf8, ["a", "b", "c"]));
        unflat2.set_unflat();
        let input_rs = result_set!(flat_chunk, unflat1, unflat2);

        let results: Vec<ResultSet> = [Ok(input_rs)]
            .into_factorized_executor()
            .factorized_flatten(DataChunkPos(1))
            .into_iter()
            .try_collect()
            .unwrap();

        let mut expected_unflat = data_chunk!((Utf8, ["a", "b", "c"]));
        expected_unflat.set_unflat();
        // resultset1
        let flat1 = results[0]
            .get_data_chunk(results[0].get_flat_chunks()[0])
            .unwrap();
        let mut expected_flat1 = data_chunk!((Int32, [2, 2]), (Int32, [100, 200]));
        expected_flat1.set_cur_idx(Some(0));
        assert_eq!(flat1.as_ref(), &expected_flat1);
        let unflat1 = results[0]
            .get_data_chunk(results[0].get_unflat_chunks()[0])
            .unwrap();
        assert_eq!(unflat1.as_ref(), &expected_unflat);

        // resultset2
        let flat2 = results[1]
            .get_data_chunk(results[1].get_flat_chunks()[0])
            .unwrap();
        let mut expected_flat2 = data_chunk!((Int32, [2, 2]), (Int32, [100, 200]));
        expected_flat2.set_cur_idx(Some(1));
        assert_eq!(flat2.as_ref(), &expected_flat2);
        let unflat2 = results[1]
            .get_data_chunk(results[1].get_unflat_chunks()[0])
            .unwrap();
        assert_eq!(unflat2.as_ref(), &expected_unflat);
    }
}
