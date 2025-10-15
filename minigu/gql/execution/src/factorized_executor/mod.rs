pub mod factorized_expand;
pub mod factorized_simple_aggregate;

use factorized_expand::FactorizedExpandBuilder;
use factorized_simple_aggregate::{FactorizedAggregateBuilder, SimpleAggregateSpec};
use minigu_common::result_set::ResultSet;

use crate::error::ExecutionResult;
use crate::source::ExpandSource;

pub trait FactorizedExecutor {
    fn next_resultset(&mut self) -> Option<ExecutionResult<ResultSet>>;

    #[inline]
    fn into_iter(self) -> FactorizedBridge<Self>
    where
        Self: Sized,
    {
        FactorizedBridge(self)
    }

    fn factorized_expand<S>(
        self,
        source: S,
        expand_chunk_pos: minigu_common::result_set::DataChunkPos,
        expand_col_idx: usize,
    ) -> impl FactorizedExecutor
    where
        Self: Sized,
        S: ExpandSource,
    {
        FactorizedExpandBuilder::new(self, source, expand_chunk_pos, expand_col_idx)
            .into_factorized_executor()
    }

    fn factorized_simple_aggregate(self, specs: Vec<SimpleAggregateSpec>) -> impl FactorizedExecutor
    where
        Self: Sized,
    {
        FactorizedAggregateBuilder::new_simple(self, specs).into_factorized_executor()
    }
}

/// A bridge between `Iterator` and [`FactorizedExecutor`].
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct FactorizedBridge<T>(T);

impl<E: FactorizedExecutor> Iterator for FactorizedBridge<E> {
    type Item = ExecutionResult<ResultSet>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_resultset()
    }
}

impl<I> FactorizedExecutor for FactorizedBridge<I>
where
    I: Iterator<Item = ExecutionResult<ResultSet>>,
{
    fn next_resultset(&mut self) -> Option<ExecutionResult<ResultSet>> {
        self.0.next()
    }
}

pub trait IntoFactorizedExecutor {
    type IntoFactorizedExecutor: FactorizedExecutor;

    fn into_factorized_executor(self) -> Self::IntoFactorizedExecutor;
}

impl<I> IntoFactorizedExecutor for I
where
    I: IntoIterator<Item = ExecutionResult<ResultSet>>,
{
    type IntoFactorizedExecutor = FactorizedBridge<I::IntoIter>;

    fn into_factorized_executor(self) -> Self::IntoFactorizedExecutor {
        FactorizedBridge(self.into_iter())
    }
}
