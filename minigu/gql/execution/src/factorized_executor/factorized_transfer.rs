use minigu_common::result_set;

use super::{FactorizedExecutor, IntoFactorizedExecutor};
use crate::executor::Executor;
use crate::executor::utils::gen_try;

/// Builder for the FactorizedTransfer operator.
///
/// This operator acts as an adapter between the traditional Executor (producing DataChunk)
/// and the FactorizedExecutor (producing ResultSet). It converts each row from the upstream
/// DataChunk into an independent ResultSet.
#[derive(Debug)]
pub struct FactorizedTransferBuilder<E> {
    child: E,
}

impl<E> FactorizedTransferBuilder<E> {
    pub fn new(child: E) -> Self {
        Self { child }
    }
}

impl<E> IntoFactorizedExecutor for FactorizedTransferBuilder<E>
where
    E: Executor,
{
    type IntoFactorizedExecutor = impl FactorizedExecutor;

    fn into_factorized_executor(self) -> Self::IntoFactorizedExecutor {
        gen move {
            let FactorizedTransferBuilder { child } = self;

            for chunk in child.into_iter() {
                let mut chunk = gen_try!(chunk);

                chunk.compact();

                // For each row, yield a ResultSet with the chunk and cur_idx pointing to that row
                for i in 0..chunk.len() {
                    let mut row_chunk = chunk.clone();
                    row_chunk.set_cur_idx(Some(i));
                    yield Ok(result_set!(row_chunk));
                }
            }
        }
        .into_factorized_executor()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use arrow::array::{ArrayRef, Int64Array};
    use minigu_common::data_chunk::DataChunk;

    use super::*;
    use crate::executor::IntoExecutor;

    /// Test basic factorized transfer: converts a DataChunk with 3 rows into 3 ResultSets.
    /// Each ResultSet shares the same chunk but with different cur_idx.
    #[test]
    fn test_factorized_transfer_basic() {
        let chunk = DataChunk::new(vec![Arc::new(Int64Array::from(vec![1, 2, 3])) as ArrayRef]);

        let executor = std::iter::once(Ok(chunk)).into_executor();
        let mut transfer = FactorizedTransferBuilder::new(executor).into_factorized_executor();

        let mut result_sets = Vec::new();
        while let Some(result) = transfer.next_resultset() {
            result_sets.push(result.unwrap());
        }

        assert_eq!(result_sets.len(), 3);

        for (i, rs) in result_sets.iter().enumerate() {
            assert_eq!(rs.factor, 1);
            let chunk = rs
                .get_data_chunk(minigu_common::result_set::DataChunkPos(0))
                .unwrap();

            assert_eq!(chunk.len(), 3);
            assert_eq!(chunk.cur_idx(), Some(i));

            let col: &Int64Array = chunk.columns()[0].as_any().downcast_ref().unwrap();
            assert_eq!(col.value(i), (i + 1) as i64);
        }
    }
}
