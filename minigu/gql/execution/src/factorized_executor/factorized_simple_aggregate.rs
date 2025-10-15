use minigu_common::data_chunk::DataChunk;
use minigu_common::result_set;
use minigu_common::result_set::{DataChunkPos, ResultSet};
use minigu_common::value::ScalarValueAccessor;

use super::{FactorizedExecutor, IntoFactorizedExecutor};
use crate::error::ExecutionResult;
use crate::evaluator::BoxedEvaluator;
use crate::executor::aggregate::{AggregateFunction, AggregateState};
use crate::executor::utils::gen_try;

/// Aggregate specification for factorized execution without GROUP BY.
#[derive(Debug)]
pub struct SimpleAggregateSpec {
    pub function: AggregateFunction,
    pub distinct: bool,
    pub expression: FactorizedExpression,
}

/// Expression that operates on a specific data chunk in factorized format.
#[derive(Debug)]
pub struct FactorizedExpression {
    /// Which data chunk this expression operates on
    pub chunk_pos: DataChunkPos,
    /// Expression to evaluate, None for COUNT(*)
    pub expression: Option<BoxedEvaluator>,
}

impl SimpleAggregateSpec {
    pub fn new(
        function: AggregateFunction,
        expression: FactorizedExpression,
        distinct: bool,
    ) -> Self {
        Self {
            function,
            distinct,
            expression,
        }
    }

    pub fn count() -> Self {
        Self::new(
            AggregateFunction::Count,
            FactorizedExpression {
                chunk_pos: DataChunkPos(0),
                expression: None,
            },
            false,
        )
    }

    pub fn count_expression(
        chunk_pos: DataChunkPos,
        expression: Option<BoxedEvaluator>,
        distinct: bool,
    ) -> Self {
        Self::new(
            AggregateFunction::CountExpression,
            FactorizedExpression {
                chunk_pos,
                expression,
            },
            distinct,
        )
    }

    pub fn sum(
        chunk_pos: DataChunkPos,
        expression: Option<BoxedEvaluator>,
        distinct: bool,
    ) -> Self {
        Self::new(
            AggregateFunction::Sum,
            FactorizedExpression {
                chunk_pos,
                expression,
            },
            distinct,
        )
    }

    pub fn avg(
        chunk_pos: DataChunkPos,
        expression: Option<BoxedEvaluator>,
        distinct: bool,
    ) -> Self {
        Self::new(
            AggregateFunction::Avg,
            FactorizedExpression {
                chunk_pos,
                expression,
            },
            distinct,
        )
    }

    pub fn min(chunk_pos: DataChunkPos, expression: Option<BoxedEvaluator>) -> Self {
        Self::new(
            AggregateFunction::Min,
            FactorizedExpression {
                chunk_pos,
                expression,
            },
            false,
        )
    }

    pub fn max(chunk_pos: DataChunkPos, expression: Option<BoxedEvaluator>) -> Self {
        Self::new(
            AggregateFunction::Max,
            FactorizedExpression {
                chunk_pos,
                expression,
            },
            false,
        )
    }
}

/// Builder for factorized simple aggregate operations without GROUP BY.
#[derive(Debug)]
pub struct FactorizedAggregateBuilder<E> {
    child: E,
    simple_aggregate_specs: Vec<SimpleAggregateSpec>,
}

impl<E> FactorizedAggregateBuilder<E> {
    /// Create a new factorized aggregate builder
    pub fn new_simple(child: E, simple_aggregate_specs: Vec<SimpleAggregateSpec>) -> Self {
        Self {
            child,
            simple_aggregate_specs,
        }
    }
}

impl<E> IntoFactorizedExecutor for FactorizedAggregateBuilder<E>
where
    E: FactorizedExecutor,
{
    type IntoFactorizedExecutor = impl FactorizedExecutor;

    fn into_factorized_executor(self) -> Self::IntoFactorizedExecutor {
        gen move {
            let FactorizedAggregateBuilder {
                child,
                simple_aggregate_specs,
            } = self;

            // Create aggregate states
            let mut states: Vec<AggregateState> = simple_aggregate_specs
                .iter()
                .map(|spec| AggregateState::new(&spec.function, spec.distinct))
                .collect();

            let mut has_data = false;

            // Process input from child
            for result_set in child.into_iter() {
                let result_set = gen_try!(result_set);

                if result_set.is_empty() {
                    continue;
                }

                has_data = true;

                // Process each aggregate spec
                for (spec_idx, spec) in simple_aggregate_specs.iter().enumerate() {
                    let state = &mut states[spec_idx];

                    match &spec.function {
                        AggregateFunction::Count => {
                            // COUNT(*) counts total factor across all chunks
                            let all_chunks_in_scope: std::collections::HashSet<DataChunkPos> = (0
                                ..result_set.num_data_chunks())
                                .map(DataChunkPos)
                                .collect();
                            let total_factor = result_set.get_num_tuples(&all_chunks_in_scope);

                            for _ in 0..total_factor {
                                gen_try!(state.update(None));
                            }
                        }
                        AggregateFunction::CountExpression
                        | AggregateFunction::Sum
                        | AggregateFunction::Avg
                        | AggregateFunction::Min
                        | AggregateFunction::Max => {
                            // Use the expression from spec (which handles both column and
                            // expression cases)
                            gen_try!(process_aggregate(
                                state,
                                &result_set,
                                &spec.expression.expression,
                                spec.expression.chunk_pos,
                            ));
                        }
                    }
                }
            }

            // Generate result
            if has_data {
                let mut result_columns = Vec::new();
                for (i, _spec) in simple_aggregate_specs.iter().enumerate() {
                    let final_value = gen_try!(states[i].finalize());
                    result_columns.push(final_value.to_scalar_array());
                }
                let mut result_chunk = DataChunk::new(result_columns);
                result_chunk.set_cur_idx(Some(0));
                let result_set = result_set!(result_chunk);
                yield Ok(result_set);
            } else {
                // Return default values for empty input
                let mut result_columns = Vec::new();
                for spec in &simple_aggregate_specs {
                    let default_value = match spec.function {
                        AggregateFunction::Count | AggregateFunction::CountExpression => {
                            use std::sync::Arc;

                            use arrow::array::{ArrayRef, Int64Array};
                            Arc::new(Int64Array::from(vec![Some(0i64)])) as ArrayRef
                        }
                        _ => {
                            use std::sync::Arc;

                            use arrow::array::{ArrayRef, Int64Array};
                            Arc::new(Int64Array::from(vec![None::<i64>])) as ArrayRef
                        }
                    };
                    result_columns.push(default_value);
                }
                if !result_columns.is_empty() {
                    let mut result_chunk = DataChunk::new(result_columns);
                    result_chunk.set_cur_idx(Some(0));
                    let result_set = result_set!(result_chunk);
                    yield Ok(result_set);
                }
            }
        }
        .into_factorized_executor()
    }
}

/// Process aggregate for non-COUNT(*) functions
fn process_aggregate(
    state: &mut AggregateState,
    input: &ResultSet,
    expression: &Option<BoxedEvaluator>,
    base_chunk_pos: DataChunkPos,
) -> ExecutionResult<()> {
    use crate::executor::aggregate::is_null_value;

    let payload_chunk = input.get_data_chunk(base_chunk_pos).unwrap();

    // Calculate factor from all chunks except the payload chunk
    let payload_chunk_idx = base_chunk_pos.0;
    let mut chunks_in_scope = std::collections::HashSet::new();
    for i in 0..input.num_data_chunks() {
        if i != payload_chunk_idx {
            chunks_in_scope.insert(DataChunkPos(i));
        }
    }

    let factor = if chunks_in_scope.is_empty() {
        input.factor
    } else {
        input
            .get_num_tuples_without_factor(&chunks_in_scope)
            .checked_mul(input.factor)
            .ok_or_else(|| {
                arrow::error::ArrowError::ComputeError(
                    "Factor multiplication overflow during aggregation".to_string(),
                )
            })?
    };

    // All non-COUNT(*) aggregates must provide an expression
    // Upper layer guarantees that expression is Some for all cases except COUNT(*)
    let expr = expression
        .as_ref()
        .expect("Expression must be provided for all non-COUNT(*) aggregates");

    if payload_chunk.is_unflat() {
        // Evaluate the expression on the entire chunk at once for better performance
        let result = expr.evaluate(payload_chunk)?;
        let result_array = result.as_array();

        for row_idx in 0..payload_chunk.len() {
            let scalar = result_array.as_ref().index(row_idx);

            if !is_null_value(&scalar) {
                for _ in 0..factor {
                    state.update(Some(scalar.clone()))?;
                }
            }
        }
    } else {
        // For flat chunk, use cur_idx
        let cur_idx = payload_chunk
            .cur_idx()
            .expect("Flat chunk should have cursor index");

        // create a single-row chunk to avoid evaluating the entire column
        let single_row_chunk = payload_chunk.slice(cur_idx, 1);

        let result = expr.evaluate(&single_row_chunk)?;
        let scalar = result.as_array().as_ref().index(0);

        if !is_null_value(&scalar) {
            for _ in 0..factor {
                state.update(Some(scalar.clone()))?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use arrow::array::{Float64Array, Int64Array};
    use itertools::Itertools;
    use minigu_common::data_chunk;
    use minigu_common::value::ScalarValue;

    use super::*;
    use crate::error::ExecutionResult;
    use crate::evaluator::Evaluator;
    use crate::evaluator::column_ref::ColumnRef;
    use crate::evaluator::constant::Constant;

    // Mock executor for testing
    #[derive(Debug)]
    struct MockFactorizedExecutor {
        result: ResultSet,
        consumed: bool,
    }

    impl FactorizedExecutor for MockFactorizedExecutor {
        fn next_resultset(&mut self) -> Option<ExecutionResult<ResultSet>> {
            if !self.consumed {
                self.consumed = true;
                Some(Ok(std::mem::replace(&mut self.result, ResultSet::new())))
            } else {
                None
            }
        }
    }

    #[test]
    fn test_flat_and_unflat_count() {
        // ResultSet
        //     // Cartesian product relationship between flat and unflat chunks, and between unflat
        //     Chunk (flat, idx = 0)
        //         [1]
        //     Chunk (unflat)
        //         [10, 20, 30]
        // Test COUNT(*) with one flat chunk (1 value) and one unflat chunk (3 values)
        // Expected result: 1 * 3 = 3
        let mut flat_chunk = data_chunk!((Int32, [1]));
        flat_chunk.set_cur_idx(Some(0));
        let mut unflat_chunk = data_chunk!((Int32, [10, 20, 30]));
        unflat_chunk.set_unflat();
        let result_set = result_set!(flat_chunk, unflat_chunk);

        let mock_input = MockFactorizedExecutor {
            result: result_set,
            consumed: false,
        };
        let aggregate_executor =
            FactorizedAggregateBuilder::new_simple(mock_input, vec![SimpleAggregateSpec::count()])
                .into_factorized_executor();

        let results: Vec<ResultSet> = aggregate_executor.into_iter().try_collect().unwrap();
        let result = &results[0];

        let result_chunk = result.get_data_chunk(DataChunkPos(0)).unwrap();
        let count_array = result_chunk.columns()[0]
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();

        assert_eq!(count_array.value(0), 3);
    }

    #[test]
    fn test_two_unflat_count() {
        // ResultSet
        //     Chunk (flat, idx = 0)
        //         [100, 200, 300, 400]  // Only uses value 100 at index 0
        //     Chunk (unflat)
        //         [1, 2, 3]
        //     Chunk (unflat)
        //         [10, 20]
        // Test COUNT(*) with flat chunk (1 value at cursor 0) and two unflat chunks (3 values) x (2
        // values) Expected result: 1 * 3 * 2 = 6 (Cartesian product)
        // Add a flat chunk first
        let mut flat_chunk = data_chunk!((Int32, [100, 200, 300, 400]));
        flat_chunk.set_cur_idx(Some(0)); // Set cursor to first element, so only contributes 1 to factor
        let mut unflat_chunk1 = data_chunk!((Int32, [1, 2, 3]));
        unflat_chunk1.set_unflat();
        let mut unflat_chunk2 = data_chunk!((Int32, [10, 20]));
        unflat_chunk2.set_unflat();
        let result_set = result_set!(flat_chunk, unflat_chunk1, unflat_chunk2);

        let mock_input = MockFactorizedExecutor {
            result: result_set,
            consumed: false,
        };
        let aggregate_executor =
            FactorizedAggregateBuilder::new_simple(mock_input, vec![SimpleAggregateSpec::count()])
                .into_factorized_executor();

        let results: Vec<ResultSet> = aggregate_executor.into_iter().try_collect().unwrap();
        let result = &results[0];

        let result_chunk = result.get_data_chunk(DataChunkPos(0)).unwrap();
        let count_array = result_chunk.columns()[0]
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();

        assert_eq!(count_array.value(0), 6);
    }

    #[test]
    fn test_two_unflat_sum() {
        // ResultSet
        //     Chunk (flat, idx = 0)
        //         [1]
        //     Chunk (unflat)
        //         [1, 2, 3]  // Target column for SUM aggregation
        //     Chunk (unflat)
        //         [10, 20]
        // Test SUM with flat chunk (1 value), unflat chunk with values [1,2,3], unflat chunk (2
        // values) Direct SUM: (1+2+3) * 2 = 12, Expression SUM: (2+3+4) * 2 = 18
        let mut flat_chunk = data_chunk!((Int32, [1]));
        flat_chunk.set_cur_idx(Some(0));
        let mut unflat_chunk1 = data_chunk!((Int32, [1, 2, 3]));
        unflat_chunk1.set_unflat();
        let mut unflat_chunk2 = data_chunk!((Int32, [10, 20]));
        unflat_chunk2.set_unflat();
        let result_set = result_set!(flat_chunk, unflat_chunk1, unflat_chunk2);

        let column_ref = ColumnRef::new(0);
        let add_one_expr = ColumnRef::new(0).add(Constant::new(ScalarValue::Int32(Some(1))));

        let mock_input = MockFactorizedExecutor {
            result: result_set,
            consumed: false,
        };
        let aggregate_executor = FactorizedAggregateBuilder::new_simple(mock_input, vec![
            SimpleAggregateSpec::sum(DataChunkPos(1), Some(Box::new(column_ref)), false),
            SimpleAggregateSpec::sum(DataChunkPos(1), Some(Box::new(add_one_expr)), false),
        ])
        .into_factorized_executor();

        let results: Vec<ResultSet> = aggregate_executor.into_iter().try_collect().unwrap();
        let result = &results[0];

        let result_chunk = result.get_data_chunk(DataChunkPos(0)).unwrap();
        assert_eq!(result_chunk.columns().len(), 2);

        let direct_sum_array = result_chunk.columns()[0]
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();
        let expr_sum_array = result_chunk.columns()[1]
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();

        assert_eq!(direct_sum_array.value(0), 12);
        assert_eq!(expr_sum_array.value(0), 18);
    }

    #[test]
    fn test_two_unflat_min() {
        // ResultSet
        //     Chunk (flat, idx = 0)
        //         [1]
        //     Chunk (unflat)
        //         [5, 2, 8, 1]  // Target column for MIN aggregation
        //     Chunk (unflat)
        //         [10, 20, 30]
        // Test MIN with values [5,2,8,1] and expression (column*2)
        // Direct MIN: 1, Expression MIN: 2
        let mut flat_chunk = data_chunk!((Int32, [1]));
        flat_chunk.set_cur_idx(Some(0));
        let mut unflat_chunk1 = data_chunk!((Int32, [5, 2, 8, 1]));
        unflat_chunk1.set_unflat();
        let mut unflat_chunk2 = data_chunk!((Int32, [10, 20, 30]));
        unflat_chunk2.set_unflat();
        let result_set = result_set!(flat_chunk, unflat_chunk1, unflat_chunk2);

        let column_ref = ColumnRef::new(0);
        let mul_two_expr = ColumnRef::new(0).mul(Constant::new(ScalarValue::Int32(Some(2))));

        let mock_input = MockFactorizedExecutor {
            result: result_set,
            consumed: false,
        };
        let aggregate_executor = FactorizedAggregateBuilder::new_simple(mock_input, vec![
            SimpleAggregateSpec::min(DataChunkPos(1), Some(Box::new(column_ref))),
            SimpleAggregateSpec::min(DataChunkPos(1), Some(Box::new(mul_two_expr))),
        ])
        .into_factorized_executor();

        let results: Vec<ResultSet> = aggregate_executor.into_iter().try_collect().unwrap();
        let result = &results[0];

        let result_chunk = result.get_data_chunk(DataChunkPos(0)).unwrap();
        assert_eq!(result_chunk.columns().len(), 2);

        let direct_min_array = result_chunk.columns()[0]
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();
        let expr_min_array = result_chunk.columns()[1]
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();

        assert_eq!(direct_min_array.value(0), 1);
        assert_eq!(expr_min_array.value(0), 2);
    }

    #[test]
    fn test_two_unflat_max() {
        // ResultSet
        //     Chunk (flat, idx = 0)
        //         [1]
        //     Chunk (unflat)
        //         [5, 2, 8, 1]  // Target column for MAX aggregation
        //     Chunk (unflat)
        //         [10, 20]
        // Test MAX with values [5,2,8,1] and expression (column+5)
        // Direct MAX: 8, Expression MAX: 13
        let mut flat_chunk = data_chunk!((Int32, [1]));
        flat_chunk.set_cur_idx(Some(0));
        let mut unflat_chunk1 = data_chunk!((Int32, [5, 2, 8, 1]));
        unflat_chunk1.set_unflat();
        let mut unflat_chunk2 = data_chunk!((Int32, [10, 20]));
        unflat_chunk2.set_unflat();
        let result_set = result_set!(flat_chunk, unflat_chunk1, unflat_chunk2);

        let column_ref = ColumnRef::new(0);
        let add_five_expr = ColumnRef::new(0).add(Constant::new(ScalarValue::Int32(Some(5))));

        let mock_input = MockFactorizedExecutor {
            result: result_set,
            consumed: false,
        };
        let aggregate_executor = FactorizedAggregateBuilder::new_simple(mock_input, vec![
            SimpleAggregateSpec::max(DataChunkPos(1), Some(Box::new(column_ref))),
            SimpleAggregateSpec::max(DataChunkPos(1), Some(Box::new(add_five_expr))),
        ])
        .into_factorized_executor();

        let results: Vec<ResultSet> = aggregate_executor.into_iter().try_collect().unwrap();
        let result = &results[0];

        let result_chunk = result.get_data_chunk(DataChunkPos(0)).unwrap();
        assert_eq!(result_chunk.columns().len(), 2);

        let direct_max_array = result_chunk.columns()[0]
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();
        let expr_max_array = result_chunk.columns()[1]
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();

        assert_eq!(direct_max_array.value(0), 8);
        assert_eq!(expr_max_array.value(0), 13);
    }

    #[test]
    fn test_two_unflat_avg() {
        // ResultSet
        //     Chunk (flat, idx = 0)
        //         [1]
        //     Chunk (unflat)
        //         [2, 4, 6]  // Target column for AVG aggregation
        //     Chunk (unflat)
        //         [10, 20]
        // Test AVG with values [2,4,6] and expression (column/2)
        // Direct AVG: (2+2+4+4+6+6)/6 = 4.0, Expression AVG: (1+1+2+2+3+3)/6 = 2.0
        let mut flat_chunk = data_chunk!((Int32, [1]));
        flat_chunk.set_cur_idx(Some(0));
        let mut unflat_chunk1 = data_chunk!((Int32, [2, 4, 6]));
        unflat_chunk1.set_unflat();
        let mut unflat_chunk2 = data_chunk!((Int32, [10, 20]));
        unflat_chunk2.set_unflat();
        let result_set = result_set!(flat_chunk, unflat_chunk1, unflat_chunk2);

        let column_ref = ColumnRef::new(0);
        let div_two_expr = ColumnRef::new(0).div(Constant::new(ScalarValue::Int32(Some(2))));

        let mock_input = MockFactorizedExecutor {
            result: result_set,
            consumed: false,
        };
        let aggregate_executor = FactorizedAggregateBuilder::new_simple(mock_input, vec![
            SimpleAggregateSpec::avg(DataChunkPos(1), Some(Box::new(column_ref)), false),
            SimpleAggregateSpec::avg(DataChunkPos(1), Some(Box::new(div_two_expr)), false),
        ])
        .into_factorized_executor();

        let results: Vec<ResultSet> = aggregate_executor.into_iter().try_collect().unwrap();
        let result = &results[0];

        let result_chunk = result.get_data_chunk(DataChunkPos(0)).unwrap();
        assert_eq!(result_chunk.columns().len(), 2);

        let direct_avg_array = result_chunk.columns()[0]
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap();
        let expr_avg_array = result_chunk.columns()[1]
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap();

        assert_eq!(direct_avg_array.value(0), 4.0);
        assert_eq!(expr_avg_array.value(0), 2.0);
    }

    #[test]
    fn test_two_unflat_multiple_aggregates() {
        // ResultSet
        //     Chunk (flat, idx = 0)
        //         [1]
        //     Chunk (unflat)
        //         [1, 2, 3]  // Target column for multiple aggregate functions
        //     Chunk (unflat)
        //         [10, 20, 30]
        // Test multiple aggregates with values [1,2,3] and various expressions
        // COUNT: 9, SUM: 18, SUM(col+10): 108, MIN: 1, MAX(col*5): 15, AVG: 2.0
        let mut flat_chunk = data_chunk!((Int32, [1]));
        flat_chunk.set_cur_idx(Some(0));
        let mut unflat_chunk1 = data_chunk!((Int32, [1, 2, 3]));
        unflat_chunk1.set_unflat();
        let mut unflat_chunk2 = data_chunk!((Int32, [10, 20, 30]));
        unflat_chunk2.set_unflat();
        let result_set = result_set!(flat_chunk, unflat_chunk1, unflat_chunk2);

        let column_ref = ColumnRef::new(0);
        let add_ten_expr = ColumnRef::new(0).add(Constant::new(ScalarValue::Int32(Some(10))));
        let mul_five_expr = ColumnRef::new(0).mul(Constant::new(ScalarValue::Int32(Some(5))));

        let mock_input = MockFactorizedExecutor {
            result: result_set,
            consumed: false,
        };
        let aggregate_executor = FactorizedAggregateBuilder::new_simple(mock_input, vec![
            SimpleAggregateSpec::count(),
            SimpleAggregateSpec::sum(DataChunkPos(1), Some(Box::new(column_ref.clone())), false),
            SimpleAggregateSpec::sum(DataChunkPos(1), Some(Box::new(add_ten_expr)), false),
            SimpleAggregateSpec::min(DataChunkPos(1), Some(Box::new(column_ref.clone()))),
            SimpleAggregateSpec::max(DataChunkPos(1), Some(Box::new(mul_five_expr))),
            SimpleAggregateSpec::avg(DataChunkPos(1), Some(Box::new(column_ref)), false),
        ])
        .into_factorized_executor();

        let results: Vec<ResultSet> = aggregate_executor.into_iter().try_collect().unwrap();
        let result = &results[0];

        let result_chunk = result.get_data_chunk(DataChunkPos(0)).unwrap();
        assert_eq!(result_chunk.columns().len(), 6);

        let count_array = result_chunk.columns()[0]
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();
        let direct_sum_array = result_chunk.columns()[1]
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();
        let expr_sum_array = result_chunk.columns()[2]
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();
        let direct_min_array = result_chunk.columns()[3]
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();
        let expr_max_array = result_chunk.columns()[4]
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();
        let direct_avg_array = result_chunk.columns()[5]
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap();

        assert_eq!(count_array.value(0), 9);
        assert_eq!(direct_sum_array.value(0), 18);
        assert_eq!(expr_sum_array.value(0), 108);
        assert_eq!(direct_min_array.value(0), 1);
        assert_eq!(expr_max_array.value(0), 15);
        assert_eq!(direct_avg_array.value(0), 2.0);
    }
}
