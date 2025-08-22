pub mod aggregate;
pub mod expand;
pub mod factorized_filter;
pub mod filter;
pub mod flatten;
pub mod procedure_call;

// TODO: Implement join executor.
pub mod join;

// TODO: Implement intersect executor.
pub mod intersect;

// TODO: Implement limit executor.
pub mod limit;

pub mod project;
pub mod sort;
mod utils;
pub mod vertex_property_scan;
pub mod vertex_scan;

use std::fmt::Debug;

use aggregate::{AggregateBuilder, AggregateSpec};
use arrow::array::{BooleanArray, ListArray};
use expand::ExpandBuilder;
use factorized_filter::FactorizedFilterBuilder;
use filter::FilterBuilder;
use flatten::FlattenBuilder;
use minigu_common::data_chunk::DataChunk;
use project::ProjectBuilder;
use sort::{SortBuilder, SortSpec};
use vertex_property_scan::VertexPropertyScanBuilder;

use crate::error::ExecutionResult;
use crate::evaluator::BoxedEvaluator;
use crate::executor::join::{JoinBuilder, JoinCond};
use crate::executor::limit::LimitBuilder;
use crate::source::{ExpandSource, VertexPropertySource};

pub type BoxedExecutor = Box<dyn Executor>;

/// A trait for pull-based vectorized volcano executors.
///
/// This is generally equivalent to `Iterator<Item = ExecutionResult<DataChunk>>`. To avoid naming
/// conflicts between some executors and iterators, e.g., `Filter`, we define this trait rather than
/// using `Iterator` directly.
///
/// To convert an `Iterator<Item = ExecutionResult<DataChunk>>` to an `Executor`, use
/// [`IntoExecutor::into_executor`].
///
/// Reversely, an `Executor` can be converted to an `Iterator` by calling [`Executor::into_iter`].
pub trait Executor {
    /// Advances the executor and returns the next `ExecutionResult<DataChunk>`.
    ///
    /// The execution should be terminated when either of the following conditions is met:
    /// - `next_chunk()` returns `None`.
    /// - `next_chunk()` returns an `Some(Err(_))`.
    ///
    /// It is up to the implementors to decide whether the executor can be resumed or not after
    /// termination. Currently, all built-in executors are fused, i.e., they always return `None`
    /// after termination.
    fn next_chunk(&mut self) -> Option<ExecutionResult<DataChunk>>;

    #[inline]
    fn into_iter(self) -> Bridge<Self>
    where
        Self: Sized,
    {
        Bridge(self)
    }

    fn filter<P>(self, predicate: P) -> impl Executor
    where
        Self: Sized,
        P: FnMut(&DataChunk) -> ExecutionResult<BooleanArray>,
    {
        FilterBuilder::new(self, predicate).into_executor()
    }

    fn factorized_filter<P>(self, predicate: P, unflat_column_indices: Vec<usize>) -> impl Executor
    where
        Self: Sized,
        P: FnMut(&DataChunk) -> ExecutionResult<ListArray>,
    {
        FactorizedFilterBuilder::new(self, predicate, unflat_column_indices).into_executor()
    }

    fn expand<S>(self, input_column_index: usize, source: S) -> impl Executor
    where
        Self: Sized,
        S: ExpandSource,
    {
        ExpandBuilder::new(self, input_column_index, source).into_executor()
    }

    fn scan_vertex_property<S>(self, input_column_index: usize, source: S) -> impl Executor
    where
        Self: Sized,
        S: VertexPropertySource,
    {
        VertexPropertyScanBuilder::new(self, input_column_index, source).into_executor()
    }

    fn sort(self, specs: Vec<SortSpec>, max_chunk_size: usize) -> impl Executor
    where
        Self: Sized,
    {
        SortBuilder::new(self, specs, max_chunk_size).into_executor()
    }

    fn join<R>(self, right: R, conds: Vec<JoinCond>) -> impl Executor
    where
        Self: Sized,
        R: Executor,
    {
        JoinBuilder::new(self, right, conds).into_executor()
    }

    fn flatten(self, column_indices: Vec<usize>) -> impl Executor
    where
        Self: Sized,
    {
        FlattenBuilder::new(self, column_indices).into_executor()
    }

    fn project(self, evaluators: Vec<BoxedEvaluator>) -> impl Executor
    where
        Self: Sized,
    {
        ProjectBuilder::new(self, evaluators).into_executor()
    }

    fn aggregate(
        self,
        aggregate_specs: Vec<AggregateSpec>,
        group_by_expressions: Vec<BoxedEvaluator>,
        output_expressions: Vec<BoxedEvaluator>,
    ) -> impl Executor
    where
        Self: Sized,
    {
        AggregateBuilder::new(
            self,
            aggregate_specs,
            group_by_expressions,
            output_expressions,
        )
        .into_executor()
    }

    fn limit(self, limit: usize) -> impl Executor
    where
        Self: Sized,
    {
        LimitBuilder::new(self, limit).into_executor()
    }
}

/// Conversion into an [`Executor`].
///
/// This is typically used to convert an `Iterator<Item = ExecutionResult<DataChunk>>` to an
/// `Executor`.
///
/// # Examples
/// Basic usage:
/// ```
/// # use minigu_execution::executor::{Executor, IntoExecutor};
/// # use minigu_execution::error::ExecutionResult;
/// # use minigu_common::data_chunk::DataChunk;
/// fn convert_to_executor<I>(some_chunk_iter: I) -> impl Executor
/// where
///     I: Iterator<Item = ExecutionResult<DataChunk>>,
/// {
///     some_chunk_iter.into_executor()
/// }
/// ```
pub trait IntoExecutor {
    type IntoExecutor: Executor;

    fn into_executor(self) -> Self::IntoExecutor;
}

/// A bridge between `Iterator` and [`Executor`].
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Bridge<T>(T);

impl<I> Executor for Bridge<I>
where
    I: Iterator<Item = ExecutionResult<DataChunk>>,
{
    fn next_chunk(&mut self) -> Option<ExecutionResult<DataChunk>> {
        self.0.next()
    }
}

impl<E: Executor> Iterator for Bridge<E> {
    type Item = ExecutionResult<DataChunk>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_chunk()
    }
}

impl<I> IntoExecutor for I
where
    I: IntoIterator<Item = ExecutionResult<DataChunk>>,
{
    type IntoExecutor = Bridge<I::IntoIter>;

    fn into_executor(self) -> Self::IntoExecutor {
        Bridge(self.into_iter())
    }
}

impl<E> Executor for Box<E>
where
    E: Executor + ?Sized,
{
    fn next_chunk(&mut self) -> Option<ExecutionResult<DataChunk>> {
        (**self).next_chunk()
    }
}
