use std::collections::HashMap;
use std::iter::zip;
use std::sync::Arc;

use arrow::array::{Array, ArrayRef, UInt32Array};
use arrow::compute::concat;
use itertools::Itertools;
use minigu_common::data_chunk::DataChunk;
use minigu_common::value::{ScalarValue, ScalarValueAccessor};

use super::{Executor, IntoExecutor};
use crate::evaluator::BoxedEvaluator;
use crate::evaluator::datum::DatumRef;
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
            let mut hash_table: HashMap<JoinKey, Vec<(Arc<DataChunk>, usize)>> = HashMap::new();

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
                        .push((chunk.clone(), row));
                }
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
                let mut left_chunks = vec![];
                let mut left_indices = vec![];
                let mut right_indices = vec![];
                for row in 0..chunk.len() {
                    let key = make_join_key(&key_cols, row);
                    if let Some(match_rows) = hash_table.get(&key) {
                        for (left_chunk, left_index) in match_rows {
                            left_chunks.push(left_chunk.clone());
                            left_indices.push(left_index);
                            right_indices.push(row as u32);
                        }
                    }
                }
                if !left_chunks.is_empty() {
                    let mut grouped : HashMap<*const DataChunk, (Arc<DataChunk>, Vec<u32>)> = HashMap::new();
                    for (chunk, &row_index) in left_chunks.iter().zip(&left_indices) {
                        let ptr = Arc::as_ptr(chunk);
                        grouped
                            .entry(ptr)
                            .or_insert_with(|| (chunk.clone(), vec![]))
                            .1
                            .push(*row_index as u32);
                    }

                    let left_join_chunks: Vec<DataChunk> = grouped
                        .into_values()
                        .map(|(chunk, indices)| chunk.take(&UInt32Array::from(indices)))
                        .collect();
                    
                    let left_join_chunk = DataChunk::concat(left_join_chunks);
                    let right_join_chunk = chunk.take(&UInt32Array::from(right_indices));
                    let mut joined_chunk = left_join_chunk;
                    joined_chunk.append_columns(right_join_chunk.columns().iter().cloned());
                    yield Ok(joined_chunk)
                }
            }
        }
        .into_executor()
    }
}
