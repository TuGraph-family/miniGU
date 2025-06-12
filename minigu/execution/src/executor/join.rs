use std::collections::HashMap;
use std::sync::Arc;

use arrow::array::{Array, ArrayRef};
use arrow::compute::concat;
use itertools::Itertools;
use minigu_common::data_chunk::DataChunk;
use minigu_common::value::{ScalarValue, ScalarValueAccessor};

use super::{Executor, IntoExecutor};
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
            let (left_evals, right_evals): (Vec<_>, Vec<_>) =
                conds.into_iter().map(|c| (c.left_key, c.right_key)).unzip();

            // build
            let mut hash_table: HashMap<JoinKey, Vec<(Arc<DataChunk>, usize)>> = HashMap::new();

            for chunk in left.into_iter() {
                let chunk = Arc::new(gen_try!(chunk));
                let key_cols: Vec<_> = gen_try!(
                    left_evals
                        .iter()
                        .map(|e| e.evaluate(&chunk).map(|d| d.as_array().clone()))
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
                    right_evals
                        .iter()
                        .map(|e| e.evaluate(&chunk).map(|d| d.as_array().clone()))
                        .try_collect()
                );
                let mut out_rows = vec![];
                for row in 0..chunk.len() {
                    let key = make_join_key(&key_cols, row);
                    if let Some(match_rows) = hash_table.get(&key) {
                        let right_row: Vec<ArrayRef> =
                            chunk.columns().iter().map(|c| c.slice(row, 1)).collect();
                        for (build_chunk, build_row_index) in match_rows {
                            let left_row: Vec<ArrayRef> = build_chunk
                                .columns()
                                .iter()
                                .map(|c| c.slice(*build_row_index, 1))
                                .collect();
                            let mut join_row = left_row;
                            join_row.extend(right_row.clone());
                            out_rows.push(join_row)
                        }
                    }
                }
                if !out_rows.is_empty() {
                    let cols = out_rows[0].len();
                    let mut buffers = vec![vec![]; cols];
                    for row in out_rows {
                        for (i, arr) in row.into_iter().enumerate() {
                            buffers[i].push(arr);
                        }
                    }
                    let result_cols: Vec<ArrayRef> = buffers
                        .into_iter()
                        .map(|vec_of_arc| {
                            let slice: Vec<&dyn Array> = vec_of_arc.iter().map(|a| a.as_ref()).collect();
                            concat(&slice).unwrap()
                        })
                        .collect();
                    yield Ok(DataChunk::new(result_cols))
                }
            }
        }
        .into_executor()
    }
}
