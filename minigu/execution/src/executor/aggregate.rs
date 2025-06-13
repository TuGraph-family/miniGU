use std::collections::HashMap;
use std::sync::Arc;

use arrow::array::{ArrayRef, Float32Array, Float64Array, Int64Array, StringArray};
use minigu_common::data_chunk::DataChunk;
use minigu_common::value::{ScalarValue, ScalarValueAccessor};

use super::utils::gen_try;
use super::{Executor, IntoExecutor};
use crate::error::ExecutionResult;
use crate::evaluator::BoxedEvaluator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AggregateFunction {
    /// COUNT(*)
    Count,
    /// COUNT(expr)
    CountExpression,
    /// SUM(expr)
    Sum,
    /// AVG(expr)
    Avg,
    /// MIN(expr)
    Min,
    /// MAX(expr)
    Max,
}

/// Aggregate specification, defines the aggregate function and its parameters
#[derive(Debug)]
pub struct AggregateSpec {
    function: AggregateFunction,
    expression: Option<BoxedEvaluator>,
    distinct: bool,
}

impl AggregateSpec {
    /// Create COUNT(*) aggregate specification
    pub fn count() -> Self {
        Self {
            function: AggregateFunction::Count,
            expression: None,
            distinct: false,
        }
    }

    /// Create COUNT(expr) aggregate specification
    pub fn count_expression(expr: BoxedEvaluator, distinct: bool) -> Self {
        Self {
            function: AggregateFunction::CountExpression,
            expression: Some(expr),
            distinct,
        }
    }

    /// Create SUM(expr) aggregate specification
    pub fn sum(expr: BoxedEvaluator, distinct: bool) -> Self {
        Self {
            function: AggregateFunction::Sum,
            expression: Some(expr),
            distinct,
        }
    }

    /// Create AVG(expr) aggregate specification
    pub fn avg(expr: BoxedEvaluator, distinct: bool) -> Self {
        Self {
            function: AggregateFunction::Avg,
            expression: Some(expr),
            distinct,
        }
    }

    /// Create MIN(expr) aggregate specification
    pub fn min(expr: BoxedEvaluator) -> Self {
        Self {
            function: AggregateFunction::Min,
            expression: Some(expr),
            distinct: false,
        }
    }

    /// Create MAX(expr) aggregate specification
    pub fn max(expr: BoxedEvaluator) -> Self {
        Self {
            function: AggregateFunction::Max,
            expression: Some(expr),
            distinct: false,
        }
    }
}

/// Aggregate state for storing intermediate results during aggregation
#[derive(Debug)]
struct AggregateState {
    count: i64,
    sum_i64: Option<i64>,
    sum_f64: Option<f64>,
    min_i64: Option<i64>,
    max_i64: Option<i64>,
    min_f64: Option<f64>,
    max_f64: Option<f64>,
    min_string: Option<String>,
    max_string: Option<String>,
    distinct_values: Option<HashMap<String, bool>>,
}

impl AggregateState {
    /// Create a new aggregate state
    fn new(distinct: bool) -> Self {
        Self {
            count: 0,
            sum_i64: None,
            sum_f64: None,
            min_i64: None,
            max_i64: None,
            min_f64: None,
            max_f64: None,
            min_string: None,
            max_string: None,
            distinct_values: if distinct { Some(HashMap::new()) } else { None },
        }
    }

    /// Update the aggregate state with a new value
    fn update(
        &mut self,
        value: Option<ScalarValue>,
        func: &AggregateFunction,
    ) -> ExecutionResult<()> {
        match func {
            AggregateFunction::Count => {
                self.count += 1;
            }
            AggregateFunction::CountExpression => {
                if let Some(val) = value {
                    if !is_null_value(&val) {
                        // If distinct is true, we need to track the distinct values
                        if let Some(ref mut distinct_set) = self.distinct_values {
                            let key = format!("{:?}", val);
                            distinct_set.insert(key, true);
                        } else {
                            self.count += 1;
                        }
                    }
                }
            }
            _ => {
                if let Some(val) = value {
                    if !is_null_value(&val) {
                        if let Some(ref mut distinct_set) = self.distinct_values {
                            let key = format!("{:?}", val);
                            distinct_set.insert(key, true);
                        } else {
                            self.update_aggregates(&val, func)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn update_aggregates(
        &mut self,
        val: &ScalarValue,
        func: &AggregateFunction,
    ) -> ExecutionResult<()> {
        // Handle Sum/Avg using macro
        macro_rules! handle_sum_avg {
            ($(($variant:pat, $field:ident, $convert:expr)),* $(,)?) => {
                match val {
                    $(
                        $variant => {
                            let v = $convert;
                            if let Some(current) = self.$field {
                                self.$field = Some(current + v);
                            } else {
                                self.$field = Some(v);
                            }
                            self.count += 1;
                        }
                    )*
                    _ => todo!(), // TODO: handle other types
                }
            };
        }

        // Handle Min using macro
        macro_rules! handle_min {
            ($(($variant:pat, $field:ident, $convert:expr)),* $(,)?) => {
                match val {
                    $(
                        $variant => {
                            let v = $convert;
                            if let Some(current) = self.$field {
                                self.$field = Some(current.min(v));
                            } else {
                                self.$field = Some(v);
                            }
                        }
                    )*
                    ScalarValue::String(Some(s)) => {
                        if let Some(ref current) = self.min_string {
                            if s < current {
                                self.min_string = Some(s.clone());
                            }
                        } else {
                            self.min_string = Some(s.clone());
                        }
                    }
                    _ => todo!(), // TODO: handle other types
                }
            };
        }

        // Handle Max using macro
        macro_rules! handle_max {
            ($(($variant:pat, $field:ident, $convert:expr)),* $(,)?) => {
                match val {
                    $(
                        $variant => {
                            let v = $convert;
                            if let Some(current) = self.$field {
                                self.$field = Some(current.max(v));
                            } else {
                                self.$field = Some(v);
                            }
                        }
                    )*
                    ScalarValue::String(Some(s)) => {
                        if let Some(ref current) = self.max_string {
                            if s > current {
                                self.max_string = Some(s.clone());
                            }
                        } else {
                            self.max_string = Some(s.clone());
                        }
                    }
                    _ => todo!(), // TODO: handle other types
                }
            };
        }

        match func {
            AggregateFunction::Sum | AggregateFunction::Avg => {
                handle_sum_avg!(
                    (ScalarValue::Int8(Some(v)), sum_i64, *v as i64),
                    (ScalarValue::Int16(Some(v)), sum_i64, *v as i64),
                    (ScalarValue::Int32(Some(v)), sum_i64, *v as i64),
                    (ScalarValue::Int64(Some(v)), sum_i64, *v),
                    (ScalarValue::UInt8(Some(v)), sum_i64, *v as i64),
                    (ScalarValue::UInt16(Some(v)), sum_i64, *v as i64),
                    (ScalarValue::UInt32(Some(v)), sum_i64, *v as i64),
                    (ScalarValue::UInt64(Some(v)), sum_i64, *v as i64),
                    (ScalarValue::Float32(Some(v)), sum_f64, *v as f64),
                    (ScalarValue::Float64(Some(v)), sum_f64, *v),
                );
            }
            AggregateFunction::Min => {
                handle_min!(
                    (ScalarValue::Int8(Some(v)), min_i64, *v as i64),
                    (ScalarValue::Int16(Some(v)), min_i64, *v as i64),
                    (ScalarValue::Int32(Some(v)), min_i64, *v as i64),
                    (ScalarValue::Int64(Some(v)), min_i64, *v),
                    (ScalarValue::UInt8(Some(v)), min_i64, *v as i64),
                    (ScalarValue::UInt16(Some(v)), min_i64, *v as i64),
                    (ScalarValue::UInt32(Some(v)), min_i64, *v as i64),
                    (ScalarValue::UInt64(Some(v)), min_i64, *v as i64),
                    (ScalarValue::Float32(Some(v)), min_f64, *v as f64),
                    (ScalarValue::Float64(Some(v)), min_f64, *v),
                );
            }
            AggregateFunction::Max => {
                handle_max!(
                    (ScalarValue::Int8(Some(v)), max_i64, *v as i64),
                    (ScalarValue::Int16(Some(v)), max_i64, *v as i64),
                    (ScalarValue::Int32(Some(v)), max_i64, *v as i64),
                    (ScalarValue::Int64(Some(v)), max_i64, *v),
                    (ScalarValue::UInt8(Some(v)), max_i64, *v as i64),
                    (ScalarValue::UInt16(Some(v)), max_i64, *v as i64),
                    (ScalarValue::UInt32(Some(v)), max_i64, *v as i64),
                    (ScalarValue::UInt64(Some(v)), max_i64, *v as i64),
                    (ScalarValue::Float32(Some(v)), max_f64, *v as f64),
                    (ScalarValue::Float64(Some(v)), max_f64, *v),
                );
            }
            _ => todo!(),
        }
        Ok(())
    }

    /// Finalize the aggregate state and return the result
    fn finalize(&self, func: &AggregateFunction) -> ExecutionResult<ScalarValue> {
        match func {
            AggregateFunction::Count => Ok(ScalarValue::Int64(Some(self.count))),

            AggregateFunction::CountExpression => {
                let count = if let Some(ref distinct_set) = self.distinct_values {
                    distinct_set.len() as i64
                } else {
                    self.count
                };
                Ok(ScalarValue::Int64(Some(count)))
            }

            AggregateFunction::Sum => {
                // Check sum_i64 first, then sum_f64
                if let Some(value) = self.sum_i64 {
                    return Ok(ScalarValue::Int64(Some(value)));
                }
                if let Some(value) = self.sum_f64 {
                    return Ok(ScalarValue::Float64(Some(value)));
                }
                Ok(ScalarValue::Null)
            }

            AggregateFunction::Avg => {
                if self.count > 0 {
                    if let Some(sum) = self.sum_i64 {
                        return Ok(ScalarValue::Float64(Some(sum as f64 / self.count as f64)));
                    }
                    if let Some(sum) = self.sum_f64 {
                        return Ok(ScalarValue::Float64(Some(sum / self.count as f64)));
                    }
                }
                Ok(ScalarValue::Null)
            }

            AggregateFunction::Min => {
                // Check numeric minimums first
                if let Some(value) = self.min_i64 {
                    return Ok(ScalarValue::Int64(Some(value)));
                }
                if let Some(value) = self.min_f64 {
                    return Ok(ScalarValue::Float64(Some(value)));
                }
                // Check string minimum
                if let Some(ref value) = self.min_string {
                    return Ok(ScalarValue::String(Some(value.clone())));
                }
                Ok(ScalarValue::Null)
            }

            AggregateFunction::Max => {
                // Check numeric maximums first
                if let Some(value) = self.max_i64 {
                    return Ok(ScalarValue::Int64(Some(value)));
                }
                if let Some(value) = self.max_f64 {
                    return Ok(ScalarValue::Float64(Some(value)));
                }
                // Check string maximum
                if let Some(ref value) = self.max_string {
                    return Ok(ScalarValue::String(Some(value.clone())));
                }
                Ok(ScalarValue::Null)
            }
        }
    }
}

/// Check if a scalar value is null
fn is_null_value(value: &ScalarValue) -> bool {
    matches!(
        value,
        ScalarValue::Null
            | ScalarValue::Boolean(None)
            | ScalarValue::Int8(None)
            | ScalarValue::Int16(None)
            | ScalarValue::Int32(None)
            | ScalarValue::Int64(None)
            | ScalarValue::UInt8(None)
            | ScalarValue::UInt16(None)
            | ScalarValue::UInt32(None)
            | ScalarValue::UInt64(None)
            | ScalarValue::Float32(None)
            | ScalarValue::Float64(None)
            | ScalarValue::String(None)
            | ScalarValue::Vertex(None)
            | ScalarValue::Edge(None)
    )
}

/// Convert a vector of scalar values to an array using macro to reduce code duplication
fn scalar_values_to_array(values: Vec<ScalarValue>) -> ArrayRef {
    if values.is_empty() {
        return Arc::new(Int64Array::from(Vec::<Option<i64>>::new())) as ArrayRef;
    }

    // Determine the type based on the first non-null value
    let sample_value = values
        .iter()
        .find(|v| !is_null_value(v))
        .unwrap_or(&values[0]);

    // Define a macro to handle all supported data types
    macro_rules! handle_scalar_types {
        ($(($variant:ident, $rust_type:ty, $array_type:ty)),* $(,)?) => {
            match sample_value {
                $(
                    ScalarValue::$variant(_) => {
                        let typed_values: Vec<Option<$rust_type>> = values
                            .into_iter()
                            .map(|v| match v {
                                ScalarValue::$variant(val) => val,
                                ScalarValue::Null => None,
                                _ => None, // Type mismatch, treat as NULL
                            })
                            .collect();
                        Arc::new(<$array_type>::from(typed_values)) as ArrayRef
                    }
                )*
                ScalarValue::Null => {
                    // All values are NULL, default to Int64Array with NULLs
                    Arc::new(Int64Array::from(vec![None::<i64>; values.len()])) as ArrayRef
                }
                _ => {
                    // For other types, default to Int64Array with NULLs
                    Arc::new(Int64Array::from(vec![None::<i64>; values.len()])) as ArrayRef
                }
            }
        };
    }

    // Call the macro to handle all supported data types
    handle_scalar_types!(
        (Boolean, bool, arrow::array::BooleanArray),
        (Int8, i8, arrow::array::Int8Array),
        (Int16, i16, arrow::array::Int16Array),
        (Int32, i32, arrow::array::Int32Array),
        (Int64, i64, Int64Array),
        (UInt8, u8, arrow::array::UInt8Array),
        (UInt16, u16, arrow::array::UInt16Array),
        (UInt32, u32, arrow::array::UInt32Array),
        (UInt64, u64, arrow::array::UInt64Array),
        (Float32, f32, Float32Array),
        (Float64, f64, Float64Array),
        (String, String, StringArray),
    )
}

/// Aggregate operator builder
#[derive(Debug)]
pub struct AggregateBuilder<E> {
    child: E,
    aggregate_specs: Vec<AggregateSpec>,
    group_by_expressions: Vec<BoxedEvaluator>,
    output_expressions: Vec<Option<BoxedEvaluator>>, // Expressions like `1 + COUNT(*)`
}

impl<E> AggregateBuilder<E> {
    /// Create a new aggregate builder
    pub fn new(
        child: E,
        aggregate_specs: Vec<AggregateSpec>,
        group_by_expressions: Vec<BoxedEvaluator>,
        output_expressions: Vec<Option<BoxedEvaluator>>,
    ) -> Self {
        assert!(
            !aggregate_specs.is_empty(),
            "At least one aggregate function is required"
        );
        Self {
            child,
            aggregate_specs,
            group_by_expressions,
            output_expressions,
        }
    }
}

impl<E> IntoExecutor for AggregateBuilder<E>
where
    E: Executor,
{
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let AggregateBuilder {
                child,
                aggregate_specs,
                group_by_expressions,
                output_expressions,
            } = self;

            // If there is no grouping expression, perform simple aggregation
            if group_by_expressions.is_empty() {
                // Create aggregate states for each aggregate spec
                let mut states: Vec<AggregateState> = aggregate_specs
                    .iter()
                    .map(|spec| AggregateState::new(spec.distinct))
                    .collect();

                let mut has_data = false;

                // Stream processing each chunk to avoid performance overhead of concat
                for chunk in child.into_iter() {
                    let chunk = gen_try!(chunk);
                    if chunk.is_empty() {
                        continue;
                    }

                    has_data = true;

                    // Process each row of the current chunk directly
                    for row in chunk.rows() {
                        for (i, spec) in aggregate_specs.iter().enumerate() {
                            // If there is an expression, evaluate it for the current row
                            let value = if let Some(ref expr) = spec.expression {
                                // Create a single row data chunk for the current row
                                let row_columns: Vec<ArrayRef> = chunk
                                    .columns()
                                    .iter()
                                    .map(|col| col.slice(row.row_index(), 1))
                                    .collect();
                                let row_chunk = DataChunk::new(row_columns);
                                // Evaluate the expression for the current row
                                let result = gen_try!(expr.evaluate(&row_chunk));
                                let scalar_value = result.as_array().as_ref().index(0);
                                Some(scalar_value)
                            } else {
                                Some(ScalarValue::Int64(Some(1))) // COUNT(*)
                            };
                            // Update the aggregate state for the current row
                            gen_try!(states[i].update(value, &spec.function));
                        }
                    }
                }

                // If there is no data, return the default aggregate result
                if !has_data {
                    let mut result_columns = Vec::new();
                    for spec in &aggregate_specs {
                        let default_value = match spec.function {
                            AggregateFunction::Count | AggregateFunction::CountExpression => {
                                // For COUNT(*) and COUNT(expr), return 0 if there is no data
                                Arc::new(Int64Array::from(vec![Some(0i64)])) as ArrayRef
                            }
                            // For other aggregate functions, return NULL if there is no data
                            _ => Arc::new(Int64Array::from(vec![None::<i64>])) as ArrayRef,
                        };
                        result_columns.push(default_value);
                    }
                    if !result_columns.is_empty() {
                        yield Ok(DataChunk::new(result_columns));
                    }
                    return;
                }

                // Generate the final result
                let mut result_columns = Vec::new();
                for (i, spec) in aggregate_specs.iter().enumerate() {
                    let final_value = gen_try!(states[i].finalize(&spec.function));
                    result_columns.push(final_value.to_scalar_array());
                }

                // Apply output expressions if any
                if !output_expressions.is_empty() {
                    let mut output_columns: Vec<ArrayRef> = Vec::new();
                    for expr in output_expressions {
                        if let Some(expr) = expr {
                            // Create a data chunk with the aggregate results
                            let agg_chunk = DataChunk::new(result_columns.clone());
                            // Evaluate the output expression
                            let result = gen_try!(expr.evaluate(&agg_chunk));
                            output_columns.push(result.as_array().clone());
                        } else {
                            // If no expression is provided, use the original aggregate result
                            output_columns.push(result_columns[output_columns.len()].clone());
                        }
                    }
                    result_columns = output_columns;
                }

                yield Ok(DataChunk::new(result_columns));
            } else {
                // Grouped aggregation
                let mut groups: HashMap<Vec<ScalarValue>, Vec<AggregateState>> = HashMap::new();
                let mut has_data = false;

                // Stream processing each chunk to avoid performance overhead of concat
                for chunk in child.into_iter() {
                    let chunk = gen_try!(chunk);
                    if chunk.is_empty() {
                        continue;
                    }

                    has_data = true;

                    for row in chunk.rows() {
                        // Calculate the group key using original ScalarValue
                        let mut group_key = Vec::new();
                        for group_expr in &group_by_expressions {
                            // Create a single row data chunk for the current row
                            let row_columns: Vec<ArrayRef> = chunk
                                .columns()
                                .iter()
                                .map(|col| col.slice(row.row_index(), 1))
                                .collect();
                            let row_chunk = DataChunk::new(row_columns);
                            let result = gen_try!(group_expr.evaluate(&row_chunk));
                            let scalar_value = result.as_array().as_ref().index(0);
                            // Push the original ScalarValue to the group key
                            group_key.push(scalar_value);
                        }

                        // Get or create the state for this group
                        let states = groups.entry(group_key).or_insert_with(|| {
                            aggregate_specs
                                .iter()
                                .map(|spec| AggregateState::new(spec.distinct))
                                .collect()
                        });

                        // Update the aggregate state for the current row
                        for (i, spec) in aggregate_specs.iter().enumerate() {
                            let value = if let Some(ref expr) = spec.expression {
                                // Create a single row data chunk for the current row
                                let row_columns: Vec<ArrayRef> = chunk
                                    .columns()
                                    .iter()
                                    .map(|col| col.slice(row.row_index(), 1))
                                    .collect();
                                let row_chunk = DataChunk::new(row_columns);
                                let result = gen_try!(expr.evaluate(&row_chunk));
                                let scalar_value = result.as_array().as_ref().index(0);
                                Some(scalar_value)
                            } else {
                                Some(ScalarValue::Int64(Some(1))) // COUNT(*)
                            };

                            gen_try!(states[i].update(value, &spec.function));
                        }
                    }
                }

                // Generate the final result
                if has_data && !groups.is_empty() {
                    // [0, group_by_expressions.len() - 1] is group by columns like `id`, `name`
                    // [group_by_expressions.len(), group_by_expressions.len() +
                    // aggregate_specs.len() - 1] is aggregate columns like `SUM(expr)`, `AVG(expr)`
                    let mut result_columns: Vec<Vec<ScalarValue>> =
                        vec![Vec::new(); group_by_expressions.len() + aggregate_specs.len()];

                    for (group_key, states) in groups {
                        // Add the original group key values directly
                        for (i, scalar_value) in group_key.into_iter().enumerate() {
                            result_columns[i].push(scalar_value);
                        }

                        // Add aggregate results
                        for (i, spec) in aggregate_specs.iter().enumerate() {
                            let final_value = gen_try!(states[i].finalize(&spec.function));
                            result_columns[group_by_expressions.len() + i].push(final_value);
                        }
                    }

                    // Convert to ArrayRef
                    let mut arrays: Vec<ArrayRef> = result_columns
                        .into_iter()
                        .map(|col| {
                            if col.is_empty() {
                                Arc::new(Int64Array::from(Vec::<Option<i64>>::new())) as ArrayRef
                            } else {
                                scalar_values_to_array(col)
                            }
                        })
                        .collect();

                    // Apply output expressions if any
                    if !output_expressions.is_empty() {
                        let mut output_arrays: Vec<ArrayRef> = Vec::new();
                        for expr in output_expressions {
                            if let Some(expr) = expr {
                                // Create a data chunk with the aggregate results
                                let agg_chunk = DataChunk::new(arrays.clone());
                                // Evaluate the output expression
                                let result = gen_try!(expr.evaluate(&agg_chunk));
                                output_arrays.push(result.as_array().clone());
                            } else {
                                // If no expression is provided, use the original aggregate result
                                output_arrays.push(arrays[output_arrays.len()].clone());
                            }
                        }
                        arrays = output_arrays;
                    }

                    yield Ok(DataChunk::new(arrays));
                }
            }
        }
        .into_executor()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use minigu_common::data_chunk;
    use minigu_common::data_chunk::DataChunk;

    use super::*;
    use crate::evaluator::Evaluator;
    use crate::evaluator::column_ref::ColumnRef;
    use crate::evaluator::constant::Constant;

    #[test]
    fn test_count_star() {
        let chunk1 = data_chunk!((Int32, [1, 2, 3]));
        let chunk2 = data_chunk!((Int32, [4, 5]));

        let result: DataChunk = [Ok(chunk1), Ok(chunk2)]
            .into_executor()
            .aggregate(vec![AggregateSpec::count()], vec![], vec![])
            .into_iter()
            .try_collect()
            .unwrap();

        let expected = data_chunk!((Int64, [5]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_expression() {
        let chunk = data_chunk!((Int32, [1, 2, 3, 4, 5]));

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .aggregate(
                vec![AggregateSpec::count_expression(
                    Box::new(ColumnRef::new(0)),
                    false,
                )],
                vec![],
                vec![],
            )
            .into_iter()
            .try_collect()
            .unwrap();

        let expected = data_chunk!((Int64, [5]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_with_nulls() {
        let chunk = data_chunk!((Int32, [Some(1), None, Some(3), None, Some(5)]));

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .aggregate(
                vec![AggregateSpec::count_expression(
                    Box::new(ColumnRef::new(0)),
                    false,
                )],
                vec![],
                vec![],
            )
            .into_iter()
            .try_collect()
            .unwrap();

        let expected = data_chunk!((Int64, [3]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sum() {
        let chunk = data_chunk!((Int32, [1, 2, 3, 4, 5]));

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .aggregate(
                vec![AggregateSpec::sum(Box::new(ColumnRef::new(0)), false)],
                vec![],
                vec![],
            )
            .into_iter()
            .try_collect()
            .unwrap();

        let expected = data_chunk!((Int64, [15]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_min_max() {
        let chunk = data_chunk!((Int32, [5, 1, 3, 9, 2]));

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .aggregate(
                vec![
                    AggregateSpec::min(Box::new(ColumnRef::new(0))),
                    AggregateSpec::max(Box::new(ColumnRef::new(0))),
                ],
                vec![],
                vec![],
            )
            .into_iter()
            .try_collect()
            .unwrap();

        let expected = data_chunk!((Int64, [1]), (Int64, [9]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_avg() {
        // Test basic AVG functionality
        let chunk = data_chunk!((Int32, [1, 2, 3, 4, 5]));

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .aggregate(
                vec![AggregateSpec::avg(Box::new(ColumnRef::new(0)), false)],
                vec![],
                vec![],
            )
            .into_iter()
            .try_collect()
            .unwrap();

        // Expected AVG: (1+2+3+4+5)/5 = 15/5 = 3.0
        let expected = data_chunk!((Float64, [3.0]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_avg_with_nulls() {
        // Test AVG with NULL values (should ignore NULLs)
        let chunk = data_chunk!((Int32, [Some(2), None, Some(4), None, Some(6)]));

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .aggregate(
                vec![AggregateSpec::avg(Box::new(ColumnRef::new(0)), false)],
                vec![],
                vec![],
            )
            .into_iter()
            .try_collect()
            .unwrap();

        // Expected AVG: (2+4+6)/3 = 12/3 = 4.0 (NULL values are ignored)
        let expected = data_chunk!((Float64, [4.0]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_avg_float_values() {
        // Test AVG with floating point values
        let chunk = data_chunk!((Float64, [1.5, 2.5, 3.5, 4.5]));

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .aggregate(
                vec![AggregateSpec::avg(Box::new(ColumnRef::new(0)), false)],
                vec![],
                vec![],
            )
            .into_iter()
            .try_collect()
            .unwrap();

        // Expected AVG: (1.5+2.5+3.5+4.5)/4 = 12.0/4 = 3.0
        let expected = data_chunk!((Float64, [3.0]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_group_by_aggregate() {
        // Create test data: department and salary
        // department: [1, 1, 2, 2, 1]
        // salary: [5000, 6000, 4000, 4500, 5500]
        let chunk = data_chunk!(
            (Int32, [1, 1, 2, 2, 1]),                // department
            (Int32, [5000, 6000, 4000, 4500, 5500])  // salary
        );

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .aggregate(
                vec![
                    AggregateSpec::count(),                                 // COUNT(*)
                    AggregateSpec::sum(Box::new(ColumnRef::new(1)), false), // SUM(salary)
                ],
                vec![Box::new(ColumnRef::new(0))], // GROUP BY department
                vec![],                            // No output expressions
            )
            .into_iter()
            .try_collect()
            .unwrap();

        // The result should be:
        // - The first column: department (group key)
        // - The second column: COUNT(*)
        // - The third column: SUM(salary)
        //
        // The expected result:
        // department 1: COUNT=3, SUM=16500 (5000+6000+5500)
        // department 2: COUNT=2, SUM=8500  (4000+4500)

        assert_eq!(result.len(), 2);
        assert_eq!(result.columns().len(), 3);

        // Get the result data for verification
        let dept_column = &result.columns()[0];
        let count_column = &result.columns()[1];
        let sum_column = &result.columns()[2];

        // Since HashMap's order is not guaranteed, we need to check both possible orders
        let dept_values: Vec<i32> = dept_column
            .as_any()
            .downcast_ref::<arrow::array::Int32Array>()
            .unwrap()
            .iter()
            .map(|v| v.unwrap())
            .collect();

        let count_values: Vec<i64> = count_column
            .as_any()
            .downcast_ref::<arrow::array::Int64Array>()
            .unwrap()
            .iter()
            .map(|v| v.unwrap())
            .collect();

        let sum_values: Vec<i64> = sum_column
            .as_any()
            .downcast_ref::<arrow::array::Int64Array>()
            .unwrap()
            .iter()
            .map(|v| v.unwrap())
            .collect();

        // Check if the result contains the correct group data
        for i in 0..2 {
            let dept = dept_values[i];
            let count = count_values[i];
            let sum = sum_values[i];

            match dept {
                1 => {
                    assert_eq!(count, 3, "department 1 should have 3 rows");
                    assert_eq!(
                        sum, 16500,
                        "the sum of salary for department 1 should be 16500"
                    );
                }
                2 => {
                    assert_eq!(count, 2, "department 2 should have 2 rows");
                    assert_eq!(
                        sum, 8500,
                        "the sum of salary for department 2 should be 8500"
                    );
                }
                _ => panic!("unexpected department value: {}", dept),
            }
        }
    }

    #[test]
    fn test_group_by_multiple_keys() {
        // Create test data: department, position, salary
        // department: [1, 1, 1, 2, 2]
        // position: [1, 2, 1, 1, 2]
        // salary: [5000, 8000, 5500, 4000, 7000]
        let chunk = data_chunk!(
            (Int32, [1, 1, 1, 2, 2]),                // department
            (Int32, [1, 2, 1, 1, 2]),                // position
            (Int32, [5000, 8000, 5500, 4000, 7000])  // salary
        );

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .aggregate(
                vec![
                    AggregateSpec::count(),                                 // COUNT(*)
                    AggregateSpec::avg(Box::new(ColumnRef::new(2)), false), // AVG(salary)
                ],
                vec![
                    Box::new(ColumnRef::new(0)), // GROUP BY department
                    Box::new(ColumnRef::new(1)), // GROUP BY position
                ],
                vec![], // No output expressions
            )
            .into_iter()
            .try_collect()
            .unwrap();

        // The result should be:
        // - The first column: department (group key 1)
        // - The second column: position (group key 2)
        // - The third column: COUNT(*)
        // - The fourth column: AVG(salary)
        //
        // The expected result:
        // (department 1, position 1): COUNT=2, AVG=5250  (5000+5500)/2
        // (department 1, position 2): COUNT=1, AVG=8000  8000/1
        // (department 2, position 1): COUNT=1, AVG=4000  4000/1
        // (department 2, position 2): COUNT=1, AVG=7000  7000/1

        assert_eq!(result.len(), 4);
        assert_eq!(result.columns().len(), 4);

        // Get the result data for verification
        let dept_column = &result.columns()[0];
        let pos_column = &result.columns()[1];
        let count_column = &result.columns()[2];
        let avg_column = &result.columns()[3];

        // Get the data for each column
        let dept_values: Vec<i32> = dept_column
            .as_any()
            .downcast_ref::<arrow::array::Int32Array>()
            .unwrap()
            .iter()
            .map(|v| v.unwrap())
            .collect();

        let pos_values: Vec<i32> = pos_column
            .as_any()
            .downcast_ref::<arrow::array::Int32Array>()
            .unwrap()
            .iter()
            .map(|v| v.unwrap())
            .collect();

        let count_values: Vec<i64> = count_column
            .as_any()
            .downcast_ref::<arrow::array::Int64Array>()
            .unwrap()
            .iter()
            .map(|v| v.unwrap())
            .collect();

        let avg_values: Vec<f64> = avg_column
            .as_any()
            .downcast_ref::<arrow::array::Float64Array>()
            .unwrap()
            .iter()
            .map(|v| v.unwrap())
            .collect();

        // Check if the result contains the correct group data
        for i in 0..4 {
            let dept = dept_values[i];
            let pos = pos_values[i];
            let count = count_values[i];
            let avg = avg_values[i];

            match (dept, pos) {
                (1, 1) => {
                    assert_eq!(count, 2, "department 1-position 1 should have 2 rows");
                    assert!(
                        (avg - 5250.0).abs() < 0.01,
                        "the average salary for department 1-position 1 should be 5250"
                    );
                }
                (1, 2) => {
                    assert_eq!(count, 1, "department 1-position 2 should have 1 row");
                    assert!(
                        (avg - 8000.0).abs() < 0.01,
                        "the average salary for department 1-position 2 should be 8000"
                    );
                }
                (2, 1) => {
                    assert_eq!(count, 1, "department 2-position 1 should have 1 row");
                    assert!(
                        (avg - 4000.0).abs() < 0.01,
                        "the average salary for department 2-position 1 should be 4000"
                    );
                }
                (2, 2) => {
                    assert_eq!(count, 1, "department 2-position 2 should have 1 row");
                    assert!(
                        (avg - 7000.0).abs() < 0.01,
                        "the average salary for department 2-position 2 should be 7000"
                    );
                }
                _ => panic!("unexpected group key combination: ({}, {})", dept, pos),
            }
        }
    }

    #[test]
    fn test_output_expressions_simple() {
        // Test with simple output expressions using constant evaluators
        let chunk = data_chunk!((Int32, [1, 2, 3, 4, 5]));

        let add_ten = ColumnRef::new(0).add(Constant::new(ScalarValue::Int64(Some(10))));
        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .aggregate(
                vec![AggregateSpec::count()],  // COUNT(*)
                vec![],                        // No grouping
                vec![Some(Box::new(add_ten))], // Output: COUNT(*) + 10
            )
            .into_iter()
            .try_collect()
            .unwrap();

        // The result should be COUNT(*) + 10 = 5 + 10 = 15
        let expected = data_chunk!((Int64, [15]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_output_expressions_with_grouping() {
        // Test output expressions with grouping
        let chunk = data_chunk!(
            (Int32, [1, 1, 2, 2, 1]),                // department
            (Int32, [5000, 6000, 4000, 4500, 5500])  // salary
        );

        let count_times_100 = ColumnRef::new(1).mul(Constant::new(ScalarValue::Int64(Some(100))));

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .aggregate(
                vec![
                    AggregateSpec::count(),                                 // COUNT(*)
                    AggregateSpec::sum(Box::new(ColumnRef::new(1)), false), // SUM(salary)
                ],
                vec![Box::new(ColumnRef::new(0))], // GROUP BY department
                vec![
                    None,                            // Keep department as-is
                    Some(Box::new(count_times_100)), // COUNT(*) * 100
                    None,                            // Keep SUM(salary) as-is
                ],
            )
            .into_iter()
            .try_collect()
            .unwrap();

        // The result should be:
        // - The first column: department (group key)
        // - The second column: COUNT(*) * 100
        // - The third column: SUM(salary)
        //
        // The expected result:
        // department 1: COUNT*100=300, SUM=16500
        // department 2: COUNT*100=200, SUM=8500

        assert_eq!(result.len(), 2);
        assert_eq!(result.columns().len(), 3);

        // Get the result data for verification
        let dept_column = &result.columns()[0];
        let count_times_100_column = &result.columns()[1];
        let sum_column = &result.columns()[2];

        let dept_values: Vec<i32> = dept_column
            .as_any()
            .downcast_ref::<arrow::array::Int32Array>()
            .unwrap()
            .iter()
            .map(|v| v.unwrap())
            .collect();

        let count_times_100_values: Vec<i64> = count_times_100_column
            .as_any()
            .downcast_ref::<arrow::array::Int64Array>()
            .unwrap()
            .iter()
            .map(|v| v.unwrap())
            .collect();

        let sum_values: Vec<i64> = sum_column
            .as_any()
            .downcast_ref::<arrow::array::Int64Array>()
            .unwrap()
            .iter()
            .map(|v| v.unwrap())
            .collect();

        // Check if the result contains the correct group data
        for i in 0..2 {
            let dept = dept_values[i];
            let count_times_100 = count_times_100_values[i];
            let sum = sum_values[i];

            match dept {
                1 => {
                    assert_eq!(
                        count_times_100, 300,
                        "department 1 should have COUNT(*) * 100 = 300"
                    );
                    assert_eq!(
                        sum, 16500,
                        "the sum of salary for department 1 should be 16500"
                    );
                }
                2 => {
                    assert_eq!(
                        count_times_100, 200,
                        "department 2 should have COUNT(*) * 100 = 200"
                    );
                    assert_eq!(
                        sum, 8500,
                        "the sum of salary for department 2 should be 8500"
                    );
                }
                _ => panic!("unexpected department value: {}", dept),
            }
        }
    }

    #[test]
    fn test_aggregate_operators_combination() {
        // Test the combination of two aggregate operators: COUNT(*) + SUM(salary) / 1000
        let chunk = data_chunk!(
            (Int32, [1, 1, 2, 2, 1]),                // department
            (Int32, [5000, 6000, 4000, 4500, 5500])  // salary
        );

        // Create expression: COUNT(*) + SUM(salary) / 1000
        // We need to reference the aggregate result columns:
        // - The first column: department (group key)
        // - The second column: COUNT(*)
        // - The third column: SUM(salary)
        let sum_div_1000 = ColumnRef::new(2).div(Constant::new(ScalarValue::Int64(Some(1000))));
        let count_plus_sum_div_1000 = ColumnRef::new(1).add(sum_div_1000);

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .aggregate(
                vec![
                    AggregateSpec::count(),                                 // COUNT(*)
                    AggregateSpec::sum(Box::new(ColumnRef::new(1)), false), // SUM(salary)
                ],
                vec![Box::new(ColumnRef::new(0))], // GROUP BY department
                vec![
                    None,                                    // Keep department as-is
                    Some(Box::new(count_plus_sum_div_1000)), // COUNT(*) + SUM(salary) / 1000
                ],
            )
            .into_iter()
            .try_collect()
            .unwrap();

        // Expected result:
        // department 1: COUNT(*) + SUM(salary) / 1000 = 3 + 16500 / 1000 = 3 + 16 = 19
        // department 2: COUNT(*) + SUM(salary) / 1000 = 2 + 8500 / 1000 = 2 + 8 = 10

        assert_eq!(result.len(), 2);
        assert_eq!(result.columns().len(), 2);

        // Get the result data for verification
        let dept_column = &result.columns()[0];
        let combined_column = &result.columns()[1];

        let dept_values: Vec<i32> = dept_column
            .as_any()
            .downcast_ref::<arrow::array::Int32Array>()
            .unwrap()
            .iter()
            .map(|v| v.unwrap())
            .collect();

        let combined_values: Vec<i64> = combined_column
            .as_any()
            .downcast_ref::<arrow::array::Int64Array>()
            .unwrap()
            .iter()
            .map(|v| v.unwrap())
            .collect();

        // Check if the result contains the correct combined data
        for i in 0..2 {
            let dept = dept_values[i];
            let combined = combined_values[i];

            match dept {
                1 => {
                    assert_eq!(
                        combined, 19,
                        "department 1 should have COUNT(*) + SUM(salary) / 1000 = 3 + 16 = 19"
                    );
                }
                2 => {
                    assert_eq!(
                        combined, 10,
                        "department 2 should have COUNT(*) + SUM(salary) / 1000 = 2 + 8 = 10"
                    );
                }
                _ => panic!("unexpected department value: {}", dept),
            }
        }
    }
}
