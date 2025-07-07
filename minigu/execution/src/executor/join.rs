use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use arrow::array::{ArrayRef, UInt32Array};
use itertools::Itertools;
use minigu_common::data_chunk::DataChunk;
use minigu_common::value::{ScalarValue, ScalarValueAccessor};

use super::{Executor, IntoExecutor};
use crate::evaluator::datum::DatumRef;
use crate::evaluator::BoxedEvaluator;
use crate::executor::utils::gen_try;
#[derive(Debug)]
pub struct JoinBuilder<L, R> {
    left: L,
    right: R,
    conds: Vec<JoinCond>,
}

#[derive(Debug)]
#[allow(unused)]
pub struct JoinCond {
    left_key: BoxedEvaluator,
    right_key: BoxedEvaluator,
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct JoinKey(Vec<ScalarValue>);

fn make_join_key(arrs: &[ArrayRef], row: usize) -> JoinKey {
    let mut keys = Vec::with_capacity(arrs.len());
    for arr in arrs {
        keys.push(arr.as_ref().index(row));
    }
    JoinKey(keys)
}

impl JoinCond {
    pub fn new(left_key: BoxedEvaluator, right_key: BoxedEvaluator) -> Self {
        Self {
            left_key,
            right_key,
        }
    }
}

impl<L, R> JoinBuilder<L, R> {
    pub fn new(left: L, right: R, conds: Vec<JoinCond>) -> Self {
        Self { left, right, conds }
    }
}

impl<L, R> IntoExecutor for JoinBuilder<L, R>
where
    L: Executor,
    R: Executor,
{
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let JoinBuilder { left, right, conds } = self;
            let (left_eval, right_eval): (Vec<_>, Vec<_>) =
                conds.into_iter().map(|c| (c.left_key, c.right_key)).unzip();

            // build
            let mut hash_table: HashMap<JoinKey, Vec<(u32, u32)>> = HashMap::new();
            let mut data_chunk_vec = vec![];

            for chunk in left.into_iter() {
                let chunk = Arc::new(gen_try!(chunk));
                let key_cols: Vec<_> = gen_try!(
                    left_eval
                        .iter()
                        .map(|e| e.evaluate(&chunk).map(DatumRef::into_array))
                        .try_collect()
                );
                for row in 0..chunk.len() {
                    let key = make_join_key(&key_cols, row);
                    hash_table
                        .entry(key)
                        .or_default()
                        .push((data_chunk_vec.len() as u32, row as u32));
                }
                data_chunk_vec.push(chunk.clone());
            }
            // probe
            for chunk in right.into_iter() {
                let chunk: DataChunk = gen_try!(chunk);
                let key_cols: Vec<_> = gen_try!(
                    right_eval
                        .iter()
                        .map(|e| e.evaluate(&chunk).map(DatumRef::into_array))
                        .try_collect()
                );
                let mut triples = vec![]; // (chunk_id, left_row, right_row)
                for row in 0..chunk.len() {
                    let key = make_join_key(&key_cols, row);
                    if let Some(match_rows) = hash_table.get(&key) {
                        for (left_chunk, left_index) in match_rows {
                            triples.push((*left_chunk, *left_index, row as u32));
                        }
                    }
                }
                // yield
                if !triples.is_empty() {
                    let mut grouped: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
                    for (chunk_id, left_row, right_row) in triples {
                        grouped
                            .entry(chunk_id)
                            .or_default()
                            .push((left_row, right_row));
                    }

                    let mut joined_chunks: Vec<DataChunk> = Vec::new();
                    for (chunk_id, pairs) in grouped {
                        let (left_rows, right_rows): (Vec<u32>, Vec<u32>) =
                            pairs.into_iter().unzip();

                        let mut left_chunk =
                            data_chunk_vec[chunk_id as usize].take(&UInt32Array::from(left_rows));
                        let mut right_chunk = chunk.take(&UInt32Array::from(right_rows));
                        left_chunk.append_columns(right_chunk.columns().iter().cloned());
                        joined_chunks.push(left_chunk);
                    }
                    let joined_chunk = DataChunk::concat(joined_chunks);
                    yield Ok(joined_chunk);
                }
            }
        }
        .into_executor()
    }
}

#[cfg(test)]
mod tests {
    use minigu_common::data_chunk;

    use super::*;
    use crate::evaluator::column_ref::ColumnRef;
    #[test]
    fn test_hash_join_basic() {
        let left_chunk = data_chunk!((Int32, [1, 2, 3]));
        let right_chunk = data_chunk!((Int32, [2, 3, 4]));
        let conds = vec![JoinCond::new(
            Box::new(ColumnRef::new(0)),
            Box::new(ColumnRef::new(0)),
        )];
        let left_executor = [Ok(left_chunk)].into_executor();
        let right_executor = [Ok(right_chunk)].into_executor();
        let join_executor = left_executor.join_with(right_executor, conds);

        let results: Vec<DataChunk> = join_executor.into_iter().try_collect().unwrap();
        let expected = data_chunk!((Int32, [2, 3]), (Int32, [2, 3]));
        assert_eq!(results, vec![expected]);
    }

    #[test]
    fn test_hash_join_duplicate_matches() {
        let left_chunk = data_chunk!((Int32, [1, 1, 2]));
        let right_chunk = data_chunk!((Int32, [1]));

        let conds = vec![JoinCond::new(
            Box::new(ColumnRef::new(0)),
            Box::new(ColumnRef::new(0)),
        )];

        let left_executor = [Ok(left_chunk)].into_executor();
        let right_executor = [Ok(right_chunk)].into_executor();

        let join_executor = left_executor.join_with(right_executor, conds);
        let results: Vec<DataChunk> = join_executor.into_iter().try_collect().unwrap();

        let expected = data_chunk!((Int32, [1, 1]), (Int32, [1, 1]));
        assert_eq!(results, vec![expected]);
    }

    #[test]
    fn test_hash_join_no_match() {
        let left_chunk = data_chunk!((Int32, [10, 20]));
        let right_chunk = data_chunk!((Int32, [1, 2]));

        let conds = vec![JoinCond::new(
            Box::new(ColumnRef::new(0)),
            Box::new(ColumnRef::new(0)),
        )];

        let left_executor = [Ok(left_chunk)].into_executor();
        let right_executor = [Ok(right_chunk)].into_executor();

        let join_executor = left_executor.join_with(right_executor, conds);
        let results: Vec<DataChunk> = join_executor.into_iter().try_collect().unwrap();

        assert!(results.is_empty());
    }

    #[test]
    fn test_hash_join_empty_right() {
        let left_chunk = data_chunk!((Int32, [1, 2, 3]));
        let right_chunk = data_chunk!((Int32, [None, None, None]));

        let conds = vec![JoinCond::new(
            Box::new(ColumnRef::new(0)),
            Box::new(ColumnRef::new(0)),
        )];

        let left_executor = [Ok(left_chunk)].into_executor();
        let right_executor = [Ok(right_chunk)].into_executor();

        let join_executor = left_executor.join_with(right_executor, conds);
        let results: Vec<DataChunk> = join_executor.into_iter().try_collect().unwrap();

        assert!(results.is_empty());
    }

    #[test]
    fn test_hash_join_multi_column_key_match() {
        let left_chunk = data_chunk!((Int32, [1, 1, 2]), (Utf8, ["a", "b", "c"]));
        let right_chunk = data_chunk!((Int32, [1, 1]), (Utf8, ["b", "c"]));

        let conds = vec![JoinCond::new(
            Box::new(ColumnRef::new(0)),
            Box::new(ColumnRef::new(0)),
        )];

        let left_executor = [Ok(left_chunk)].into_executor();
        let right_executor = [Ok(right_chunk)].into_executor();

        let join_executor = left_executor.join_with(right_executor, conds);
        let results: Vec<DataChunk> = join_executor.into_iter().try_collect().unwrap();

        let expected = data_chunk!(
            (Int32, [1, 1, 1, 1]),
            (Utf8, ["a", "b", "a", "b"]),
            (Int32, [1, 1, 1, 1]),
            (Utf8, ["b", "b", "c", "c"])
        );
        assert_eq!(results, vec![expected]);
    }

    #[test]
    fn test_hash_join_multi_key_match() {
        let left_chunk = data_chunk!((Int32, [1, 1, 2]), (Utf8, ["a", "b", "c"]));
        let right_chunk = data_chunk!((Int32, [1, 1]), (Utf8, ["a", "x"]));

        let conds = vec![
            JoinCond::new(Box::new(ColumnRef::new(0)), Box::new(ColumnRef::new(0))),
            JoinCond::new(Box::new(ColumnRef::new(1)), Box::new(ColumnRef::new(1))),
        ];

        let left_executor = [Ok(left_chunk)].into_executor();
        let right_executor = [Ok(right_chunk)].into_executor();

        let join_executor = left_executor.join_with(right_executor, conds);
        let results: Vec<DataChunk> = join_executor.into_iter().try_collect().unwrap();

        let expected = data_chunk!((Int32, [1]), (Utf8, ["a"]), (Int32, [1]), (Utf8, ["a"]));
        assert_eq!(results, vec![expected]);
    }
}
