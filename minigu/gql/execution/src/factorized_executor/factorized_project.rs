use std::collections::HashSet;

use minigu_common::result_set::ResultSet;

use super::{FactorizedExecutor, IntoFactorizedExecutor};
use crate::evaluator::factorized_evaluator::BoxedFactorizedEvaluator;
use crate::executor::utils::gen_try;

#[derive(Debug)]
pub struct FactorizedProjectBuilder<E> {
    child: E,
    evaluators: Vec<BoxedFactorizedEvaluator>,
}

impl<E> FactorizedProjectBuilder<E> {
    pub fn new(child: E, evaluators: Vec<BoxedFactorizedEvaluator>) -> Self {
        Self { child, evaluators }
    }
}

impl<E> IntoFactorizedExecutor for FactorizedProjectBuilder<E>
where
    E: FactorizedExecutor,
{
    type IntoFactorizedExecutor = impl FactorizedExecutor;

    fn into_factorized_executor(self) -> Self::IntoFactorizedExecutor {
        gen move {
            let FactorizedProjectBuilder {
                mut child,
                evaluators,
            } = self;

            while let Some(result) = child.next_resultset() {
                let input = gen_try!(result);

                let input_all_chunks: HashSet<_> = (0..input.num_data_chunks())
                    .map(minigu_common::result_set::DataChunkPos)
                    .collect();
                let input_total_num_tuples = input.get_num_tuples(&input_all_chunks);

                let mut result_set = ResultSet::new();
                for evaluator in &evaluators {
                    let chunk = gen_try!(evaluator.evaluate(&input));
                    result_set.push(chunk);
                }

                let output_all_chunks: HashSet<_> = (0..result_set.num_data_chunks())
                    .map(minigu_common::result_set::DataChunkPos)
                    .collect();
                let output_num_tuples_without_factor =
                    result_set.get_num_tuples_without_factor(&output_all_chunks);

                // new factor = input_total_num_tuples / output_num_tuples_without_factor
                result_set.factor = input_total_num_tuples / output_num_tuples_without_factor;

                yield Ok(result_set);
            }
        }
        .into_factorized_executor()
    }
}

#[cfg(test)]
mod tests {
    use arrow::array::create_array;
    use itertools::Itertools;
    use minigu_common::data_chunk::DataChunk;
    use minigu_common::result_set::{DataChunkPos, DataPos, ResultSet};
    use minigu_common::{data_chunk, result_set};

    use crate::data_ref;
    use crate::evaluator::factorized_evaluator::{FactorizedConstant, FactorizedEvaluator};
    use crate::factorized_executor::{FactorizedExecutor, IntoFactorizedExecutor};

    #[test]
    fn test_flat_flat() {
        // chunk0 (flat): [1, 2, 3],
        // chunk1 (flat): [10, 20, 30]
        let chunk0 = data_chunk!((Int32, [1, 2, 3]));
        let chunk1 = data_chunk!((Int32, [10, 20, 30]));
        let input_rs = result_set!(chunk0, chunk1);

        // flat + flat: cur_idx=0, 1 + 10 = 11
        let e1 = data_ref!(0, 0).add(data_ref!(1, 0));

        let results: Vec<ResultSet> = [Ok(input_rs)]
            .into_factorized_executor()
            .factorized_project(vec![Box::new(e1)])
            .into_iter()
            .try_collect()
            .unwrap();
        let result = &results[0];

        assert_eq!(result.num_data_chunks(), 1);
        assert_eq!(result.factor, 1);

        let chunk = result.get_data_chunk(DataChunkPos(0)).unwrap();
        assert!(chunk.cur_idx().is_some());
        let expected = create_array!(Int32, [11]);
        assert_eq!(chunk.columns()[0].as_ref(), expected.as_ref());
    }

    #[test]
    fn test_flat_unflat() {
        // chunk0 (flat): [1, 2, 3],
        // chunk1 (unflat): [10, 20, 30],
        // chunk2 (unflat): [40, 50, 60]
        // input_total_num_tuples = 3 * 3 * factor = 9 * 1 = 9
        let chunk0 = data_chunk!((Int32, [1, 2, 3]));
        let mut chunk1 = data_chunk!((Int32, [10, 20, 30]));
        chunk1.set_unflat();
        let mut chunk2 = data_chunk!((Int32, [40, 50, 60]));
        chunk2.set_unflat();
        let input_rs = result_set!(chunk0, chunk1, chunk2);

        // flat + unflat: 1 + [10, 20, 30] = [11, 21, 31]
        let e1 = data_ref!(0, 0).add(data_ref!(1, 0));

        let results: Vec<ResultSet> = [Ok(input_rs)]
            .into_factorized_executor()
            .factorized_project(vec![Box::new(e1)])
            .into_iter()
            .try_collect()
            .unwrap();
        let result = &results[0];

        assert_eq!(result.num_data_chunks(), 1);
        // new factor = 9 / 3 = 3
        assert_eq!(result.factor, 3);

        let chunk = result.get_data_chunk(DataChunkPos(0)).unwrap();
        assert!(chunk.is_unflat());
        let expected = create_array!(Int32, [11, 21, 31]);
        assert_eq!(chunk.columns()[0].as_ref(), expected.as_ref());
    }

    #[test]
    fn test_unflat_in_same_chunk() {
        // chunk (unflat): [1, 2, 3], [10, 20, 30]
        // input_total_num_tuples = 3 * factor = 3 * 1 = 3
        let c1 = create_array!(Int32, [1, 2, 3]);
        let c2 = create_array!(Int32, [10, 20, 30]);
        let mut chunk = DataChunk::new(vec![c1, c2]);
        chunk.set_unflat();
        let input_rs = result_set!(chunk);

        // unflat * unflat: [1*10, 2*20, 3*30] = [10, 40, 90]
        let e1 = data_ref!(0, 0).mul(data_ref!(0, 1));

        let results: Vec<ResultSet> = [Ok(input_rs)]
            .into_factorized_executor()
            .factorized_project(vec![Box::new(e1)])
            .into_iter()
            .try_collect()
            .unwrap();
        let result = &results[0];

        assert_eq!(result.num_data_chunks(), 1);
        // new factor = 3 / 3 = 1
        assert_eq!(result.factor, 1);

        let chunk = result.get_data_chunk(DataChunkPos(0)).unwrap();
        assert!(chunk.is_unflat());
        let expected = create_array!(Int32, [10, 40, 90]);
        assert_eq!(chunk.columns()[0].as_ref(), expected.as_ref());
    }

    #[test]
    fn test_unflat_in_different_chunks() {
        // chunk0 (unflat): [1, 2, 3]
        // chunk1 (unflat): [10, 20]
        // chunk2 (unflat): [100, 200, 300]
        // input_total_num_tuples = 3 * 2 * 3 * factor = 3 * 2 * 3 * 1 = 18
        let mut chunk0 = data_chunk!((Int32, [1, 2, 3]));
        chunk0.set_unflat();
        let mut chunk1 = data_chunk!((Int32, [10, 20]));
        chunk1.set_unflat();
        let mut chunk2 = data_chunk!((Int32, [100, 200, 300]));
        chunk2.set_unflat();
        let input_rs = result_set!(chunk0, chunk1, chunk2);

        // [1, 2, 3] * [10, 20] = [10, 20, 20, 40, 30, 60]
        let e1 = data_ref!(0, 0).mul(data_ref!(1, 0));

        let results: Vec<ResultSet> = [Ok(input_rs)]
            .into_factorized_executor()
            .factorized_project(vec![Box::new(e1)])
            .into_iter()
            .try_collect()
            .unwrap();
        let result = &results[0];

        assert_eq!(result.num_data_chunks(), 1);
        // new factor = 18 / 6 = 3
        assert_eq!(result.factor, 3);

        let chunk = result.get_data_chunk(DataChunkPos(0)).unwrap();
        assert!(chunk.is_unflat());
        let expected = create_array!(Int32, [10, 20, 20, 40, 30, 60]);
        assert_eq!(chunk.columns()[0].as_ref(), expected.as_ref());
    }

    #[test]
    fn test_constant_mul_unflat() {
        // chunk0 (unflat): [5, 10, 15]
        // input_total_num_tuples = 3 * factor = 3 * 1 = 3
        let mut chunk0 = data_chunk!((Int32, [5, 10, 15]));
        chunk0.set_unflat();
        let input_rs = result_set!(chunk0);

        // 2 * [5, 10, 15] = [10, 20, 30]
        let e1 = FactorizedConstant::new(2i32.into()).mul(data_ref!(0, 0));

        let results: Vec<ResultSet> = [Ok(input_rs)]
            .into_factorized_executor()
            .factorized_project(vec![Box::new(e1)])
            .into_iter()
            .try_collect()
            .unwrap();
        let result = &results[0];

        assert_eq!(result.num_data_chunks(), 1);
        // new factor = 3 / 3 = 1
        assert_eq!(result.factor, 1);

        let chunk = result.get_data_chunk(DataChunkPos(0)).unwrap();
        assert!(chunk.is_unflat());
        let expected = create_array!(Int32, [10, 20, 30]);
        assert_eq!(chunk.columns()[0].as_ref(), expected.as_ref());
    }
}
