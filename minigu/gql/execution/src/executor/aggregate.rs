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
enum AggregateState {
    Count {
        count: i64,
    },
    CountExpression {
        count: i64,
        distinct_values: Option<HashMap<String, bool>>,
    },
    Sum {
        sum_i64: Option<i64>,
        sum_f64: Option<f64>,
        distinct_values: Option<HashMap<String, bool>>,
    },
    Avg {
        sum_f64: f64,
        count: i64,
        distinct_values: Option<HashMap<String, bool>>,
    },
    Min {
        min_i64: Option<i64>,
        min_f64: Option<f64>,
        min_string: Option<String>,
    },
    Max {
        max_i64: Option<i64>,
        max_f64: Option<f64>,
        max_string: Option<String>,
    },
}

impl AggregateState {
    /// Create a new aggregate state
    fn new(func: &AggregateFunction, distinct: bool) -> Self {
        match func {
            AggregateFunction::Count => Self::Count { count: 0 },
            AggregateFunction::CountExpression => Self::CountExpression {
                count: 0,
                distinct_values: if distinct { Some(HashMap::new()) } else { None },
            },
            AggregateFunction::Sum => Self::Sum {
                sum_i64: None,
                sum_f64: None,
                distinct_values: if distinct { Some(HashMap::new()) } else { None },
            },
            AggregateFunction::Avg => Self::Avg {
                sum_f64: 0.0,
                count: 0,
                distinct_values: if distinct { Some(HashMap::new()) } else { None },
            },
            AggregateFunction::Min => Self::Min {
                min_i64: None,
                min_f64: None,
                min_string: None,
            },
            AggregateFunction::Max => Self::Max {
                max_i64: None,
                max_f64: None,
                max_string: None,
            },
        }
    }

    /// Update the aggregate state with a new value
    fn update(&mut self, value: Option<ScalarValue>) -> ExecutionResult<()> {
        match self {
            AggregateState::Count { count } => {
                *count += 1;
            }
            AggregateState::CountExpression {
                count,
                distinct_values,
            } => {
                if let Some(val) = value {
                    if !is_null_value(&val) {
                        if let Some(distinct_set) = distinct_values {
                            let key = format!("{:?}", val);
                            distinct_set.insert(key, true);
                        } else {
                            *count += 1;
                        }
                    }
                }
            }
            AggregateState::Sum {
                distinct_values, ..
            } => {
                if let Some(val) = value {
                    if !is_null_value(&val) {
                        if let Some(distinct_set) = distinct_values {
                            let key = format!("{:?}", val);
                            distinct_set.insert(key, true);
                        } else {
                            self.update_sum_aggregate(&val)?;
                        }
                    }
                }
            }
            AggregateState::Avg {
                distinct_values, ..
            } => {
                if let Some(val) = value {
                    if !is_null_value(&val) {
                        if let Some(distinct_set) = distinct_values {
                            let key = format!("{:?}", val);
                            distinct_set.insert(key, true);
                        } else {
                            self.update_sum_aggregate(&val)?;
                        }
                    }
                }
            }
            AggregateState::Min { .. } => {
                if let Some(val) = value {
                    if !is_null_value(&val) {
                        self.update_min_aggregate(&val)?;
                    }
                }
            }
            AggregateState::Max { .. } => {
                if let Some(val) = value {
                    if !is_null_value(&val) {
                        self.update_max_aggregate(&val)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn update_sum_aggregate(&mut self, val: &ScalarValue) -> ExecutionResult<()> {
        match self {
            AggregateState::Sum {
                sum_i64, sum_f64, ..
            } => {
                match val {
                    ScalarValue::Int8(Some(v)) => {
                        let v = *v as i64;
                        if let Some(current) = sum_i64 {
                            *sum_i64 = Some(*current + v);
                        } else {
                            *sum_i64 = Some(v);
                        }
                    }
                    ScalarValue::Int16(Some(v)) => {
                        let v = *v as i64;
                        if let Some(current) = sum_i64 {
                            *sum_i64 = Some(*current + v);
                        } else {
                            *sum_i64 = Some(v);
                        }
                    }
                    ScalarValue::Int32(Some(v)) => {
                        let v = *v as i64;
                        if let Some(current) = sum_i64 {
                            *sum_i64 = Some(*current + v);
                        } else {
                            *sum_i64 = Some(v);
                        }
                    }
                    ScalarValue::Int64(Some(v)) => {
                        let v = *v;
                        if let Some(current) = sum_i64 {
                            *sum_i64 = Some(*current + v);
                        } else {
                            *sum_i64 = Some(v);
                        }
                    }
                    ScalarValue::UInt8(Some(v)) => {
                        let v = *v as i64;
                        if let Some(current) = sum_i64 {
                            *sum_i64 = Some(*current + v);
                        } else {
                            *sum_i64 = Some(v);
                        }
                    }
                    ScalarValue::UInt16(Some(v)) => {
                        let v = *v as i64;
                        if let Some(current) = sum_i64 {
                            *sum_i64 = Some(*current + v);
                        } else {
                            *sum_i64 = Some(v);
                        }
                    }
                    ScalarValue::UInt32(Some(v)) => {
                        let v = *v as i64;
                        if let Some(current) = sum_i64 {
                            *sum_i64 = Some(*current + v);
                        } else {
                            *sum_i64 = Some(v);
                        }
                    }
                    ScalarValue::UInt64(Some(v)) => {
                        let v = *v as i64;
                        if let Some(current) = sum_i64 {
                            *sum_i64 = Some(*current + v);
                        } else {
                            *sum_i64 = Some(v);
                        }
                    }
                    ScalarValue::Float32(Some(v)) => {
                        let v = v.into_inner() as f64;
                        if let Some(current) = sum_f64 {
                            *sum_f64 = Some(*current + v);
                        } else {
                            *sum_f64 = Some(v);
                        }
                    }
                    ScalarValue::Float64(Some(v)) => {
                        let v = v.into_inner();
                        if let Some(current) = sum_f64 {
                            *sum_f64 = Some(*current + v);
                        } else {
                            *sum_f64 = Some(v);
                        }
                    }
                    _ => todo!(), // TODO: handle other types
                }
            }
            AggregateState::Avg { sum_f64, count, .. } => {
                match val {
                    ScalarValue::Int8(Some(v)) => {
                        let v = *v as f64;
                        *sum_f64 += v;
                        *count += 1;
                    }
                    ScalarValue::Int16(Some(v)) => {
                        let v = *v as f64;
                        *sum_f64 += v;
                        *count += 1;
                    }
                    ScalarValue::Int32(Some(v)) => {
                        let v = *v as f64;
                        *sum_f64 += v;
                        *count += 1;
                    }
                    ScalarValue::Int64(Some(v)) => {
                        let v = *v as f64;
                        *sum_f64 += v;
                        *count += 1;
                    }
                    ScalarValue::UInt8(Some(v)) => {
                        let v = *v as f64;
                        *sum_f64 += v;
                        *count += 1;
                    }
                    ScalarValue::UInt16(Some(v)) => {
                        let v = *v as f64;
                        *sum_f64 += v;
                        *count += 1;
                    }
                    ScalarValue::UInt32(Some(v)) => {
                        let v = *v as f64;
                        *sum_f64 += v;
                        *count += 1;
                    }
                    ScalarValue::UInt64(Some(v)) => {
                        let v = *v as f64;
                        *sum_f64 += v;
                        *count += 1;
                    }
                    ScalarValue::Float32(Some(v)) => {
                        let v = v.into_inner() as f64;
                        *sum_f64 += v;
                        *count += 1;
                    }
                    ScalarValue::Float64(Some(v)) => {
                        let v = v.into_inner();
                        *sum_f64 += v;
                        *count += 1;
                    }
                    _ => todo!(), // TODO: handle other types
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn update_min_aggregate(&mut self, val: &ScalarValue) -> ExecutionResult<()> {
        if let AggregateState::Min {
            min_i64,
            min_f64,
            min_string,
        } = self
        {
            match val {
                ScalarValue::Int8(Some(v)) => {
                    let v = *v as i64;
                    if let Some(current) = min_i64 {
                        *min_i64 = Some((*current).min(v));
                    } else {
                        *min_i64 = Some(v);
                    }
                }
                ScalarValue::Int16(Some(v)) => {
                    let v = *v as i64;
                    if let Some(current) = min_i64 {
                        *min_i64 = Some((*current).min(v));
                    } else {
                        *min_i64 = Some(v);
                    }
                }
                ScalarValue::Int32(Some(v)) => {
                    let v = *v as i64;
                    if let Some(current) = min_i64 {
                        *min_i64 = Some((*current).min(v));
                    } else {
                        *min_i64 = Some(v);
                    }
                }
                ScalarValue::Int64(Some(v)) => {
                    let v = *v;
                    if let Some(current) = min_i64 {
                        *min_i64 = Some((*current).min(v));
                    } else {
                        *min_i64 = Some(v);
                    }
                }
                ScalarValue::UInt8(Some(v)) => {
                    let v = *v as i64;
                    if let Some(current) = min_i64 {
                        *min_i64 = Some((*current).min(v));
                    } else {
                        *min_i64 = Some(v);
                    }
                }
                ScalarValue::UInt16(Some(v)) => {
                    let v = *v as i64;
                    if let Some(current) = min_i64 {
                        *min_i64 = Some((*current).min(v));
                    } else {
                        *min_i64 = Some(v);
                    }
                }
                ScalarValue::UInt32(Some(v)) => {
                    let v = *v as i64;
                    if let Some(current) = min_i64 {
                        *min_i64 = Some((*current).min(v));
                    } else {
                        *min_i64 = Some(v);
                    }
                }
                ScalarValue::UInt64(Some(v)) => {
                    let v = *v as i64;
                    if let Some(current) = min_i64 {
                        *min_i64 = Some((*current).min(v));
                    } else {
                        *min_i64 = Some(v);
                    }
                }
                ScalarValue::Float32(Some(v)) => {
                    let v = v.into_inner() as f64;
                    if let Some(current) = min_f64 {
                        *min_f64 = Some(current.min(v));
                    } else {
                        *min_f64 = Some(v);
                    }
                }
                ScalarValue::Float64(Some(v)) => {
                    let v = v.into_inner();
                    if let Some(current) = min_f64 {
                        *min_f64 = Some(current.min(v));
                    } else {
                        *min_f64 = Some(v);
                    }
                }
                ScalarValue::String(Some(s)) => {
                    if let Some(current) = min_string {
                        if s < current {
                            *min_string = Some(s.clone());
                        }
                    } else {
                        *min_string = Some(s.clone());
                    }
                }
                _ => todo!(), // TODO: handle other types
            }
        }
        Ok(())
    }

    fn update_max_aggregate(&mut self, val: &ScalarValue) -> ExecutionResult<()> {
        if let AggregateState::Max {
            max_i64,
            max_f64,
            max_string,
        } = self
        {
            match val {
                ScalarValue::Int8(Some(v)) => {
                    let v = *v as i64;
                    if let Some(current) = max_i64 {
                        *max_i64 = Some((*current).max(v));
                    } else {
                        *max_i64 = Some(v);
                    }
                }
                ScalarValue::Int16(Some(v)) => {
                    let v = *v as i64;
                    if let Some(current) = max_i64 {
                        *max_i64 = Some((*current).max(v));
                    } else {
                        *max_i64 = Some(v);
                    }
                }
                ScalarValue::Int32(Some(v)) => {
                    let v = *v as i64;
                    if let Some(current) = max_i64 {
                        *max_i64 = Some((*current).max(v));
                    } else {
                        *max_i64 = Some(v);
                    }
                }
                ScalarValue::Int64(Some(v)) => {
                    let v = *v;
                    if let Some(current) = max_i64 {
                        *max_i64 = Some((*current).max(v));
                    } else {
                        *max_i64 = Some(v);
                    }
                }
                ScalarValue::UInt8(Some(v)) => {
                    let v = *v as i64;
                    if let Some(current) = max_i64 {
                        *max_i64 = Some((*current).max(v));
                    } else {
                        *max_i64 = Some(v);
                    }
                }
                ScalarValue::UInt16(Some(v)) => {
                    let v = *v as i64;
                    if let Some(current) = max_i64 {
                        *max_i64 = Some((*current).max(v));
                    } else {
                        *max_i64 = Some(v);
                    }
                }
                ScalarValue::UInt32(Some(v)) => {
                    let v = *v as i64;
                    if let Some(current) = max_i64 {
                        *max_i64 = Some((*current).max(v));
                    } else {
                        *max_i64 = Some(v);
                    }
                }
                ScalarValue::UInt64(Some(v)) => {
                    let v = *v as i64;
                    if let Some(current) = max_i64 {
                        *max_i64 = Some((*current).max(v));
                    } else {
                        *max_i64 = Some(v);
                    }
                }
                ScalarValue::Float32(Some(v)) => {
                    let v = v.into_inner() as f64;
                    if let Some(current) = max_f64 {
                        *max_f64 = Some(current.max(v));
                    } else {
                        *max_f64 = Some(v);
                    }
                }
                ScalarValue::Float64(Some(v)) => {
                    let v = v.into_inner();
                    if let Some(current) = max_f64 {
                        *max_f64 = Some(current.max(v));
                    } else {
                        *max_f64 = Some(v);
                    }
                }
                ScalarValue::String(Some(s)) => {
                    if let Some(current) = max_string {
                        if s > current {
                            *max_string = Some(s.clone());
                        }
                    } else {
                        *max_string = Some(s.clone());
                    }
                }
                _ => todo!(), // TODO: handle other types
            }
        }
        Ok(())
    }

    /// Finalize the aggregate state and return the result
    fn finalize(&self) -> ExecutionResult<ScalarValue> {
        match self {
            AggregateState::Count { count } => Ok(ScalarValue::Int64(Some(*count))),

            AggregateState::CountExpression {
                count,
                distinct_values,
            } => {
                let count = if let Some(distinct_set) = distinct_values {
                    distinct_set.len() as i64
                } else {
                    *count
                };
                Ok(ScalarValue::Int64(Some(count)))
            }

            AggregateState::Sum {
                sum_i64, sum_f64, ..
            } => {
                // Check sum_i64 first, then sum_f64
                if let Some(value) = sum_i64 {
                    return Ok(ScalarValue::Int64(Some(*value)));
                }
                if let Some(value) = sum_f64 {
                    return Ok(ScalarValue::Float64(Some(minigu_common::value::F64::from(
                        *value,
                    ))));
                }
                Ok(ScalarValue::Null)
            }

            AggregateState::Avg {
                sum_f64,
                count,
                distinct_values,
            } => {
                let effective_count = if let Some(distinct_set) = distinct_values {
                    distinct_set.len() as i64
                } else {
                    *count
                };

                if effective_count > 0 {
                    return Ok(ScalarValue::Float64(Some(minigu_common::value::F64::from(
                        *sum_f64 / effective_count as f64,
                    ))));
                }
                Ok(ScalarValue::Null)
            }

            AggregateState::Min {
                min_i64,
                min_f64,
                min_string,
            } => {
                // Check numeric minimums first
                if let Some(value) = min_i64 {
                    return Ok(ScalarValue::Int64(Some(*value)));
                }
                if let Some(value) = min_f64 {
                    return Ok(ScalarValue::Float64(Some(minigu_common::value::F64::from(
                        *value,
                    ))));
                }
                // Check string minimum
                if let Some(value) = min_string {
                    return Ok(ScalarValue::String(Some(value.clone())));
                }
                Ok(ScalarValue::Null)
            }

            AggregateState::Max {
                max_i64,
                max_f64,
                max_string,
            } => {
                // Check numeric maximums first
                if let Some(value) = max_i64 {
                    return Ok(ScalarValue::Int64(Some(*value)));
                }
                if let Some(value) = max_f64 {
                    return Ok(ScalarValue::Float64(Some(minigu_common::value::F64::from(
                        *value,
                    ))));
                }
                // Check string maximum
                if let Some(value) = max_string {
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
                ScalarValue::Float32(_) => {
                    let typed_values: Vec<Option<f32>> = values
                        .into_iter()
                        .map(|v| match v {
                            ScalarValue::Float32(val) => val.map(|f| f.into_inner()),
                            ScalarValue::Null => None,
                            _ => None, // Type mismatch, treat as NULL
                        })
                        .collect();
                    Arc::new(Float32Array::from(typed_values)) as ArrayRef
                }
                ScalarValue::Float64(_) => {
                    let typed_values: Vec<Option<f64>> = values
                        .into_iter()
                        .map(|v| match v {
                            ScalarValue::Float64(val) => val.map(|f| f.into_inner()),
                            ScalarValue::Null => None,
                            _ => None, // Type mismatch, treat as NULL
                        })
                        .collect();
                    Arc::new(Float64Array::from(typed_values)) as ArrayRef
                }
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
        (String, String, StringArray),
    )
}

/// Aggregate operator builder
#[derive(Debug)]
pub struct AggregateBuilder<E> {
    child: E,
    aggregate_specs: Vec<AggregateSpec>,
    group_by_expressions: Vec<BoxedEvaluator>,
    output_expressions: Vec<BoxedEvaluator>, // Expressions like `1 + COUNT(*)`
}

impl<E> AggregateBuilder<E> {
    /// Create a new aggregate builder
    pub fn new(
        child: E,
        aggregate_specs: Vec<AggregateSpec>,
        group_by_expressions: Vec<BoxedEvaluator>,
        output_expressions: Vec<BoxedEvaluator>,
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
                    .map(|spec| AggregateState::new(&spec.function, spec.distinct))
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
                            gen_try!(states[i].update(value));
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
                for (i, _spec) in aggregate_specs.iter().enumerate() {
                    let final_value = gen_try!(states[i].finalize());
                    result_columns.push(final_value.to_scalar_array());
                }

                // Apply output expressions if any
                if !output_expressions.is_empty() {
                    let mut output_columns: Vec<ArrayRef> = Vec::new();
                    for expr in output_expressions {
                        // Create a data chunk with the aggregate results
                        let agg_chunk = DataChunk::new(result_columns.clone());
                        // Evaluate the output expression
                        let result = gen_try!(expr.evaluate(&agg_chunk));
                        output_columns.push(result.as_array().clone());
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
                                .map(|spec| AggregateState::new(&spec.function, spec.distinct))
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

                            gen_try!(states[i].update(value));
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
                        for (i, _spec) in aggregate_specs.iter().enumerate() {
                            let final_value = gen_try!(states[i].finalize());
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
                            // Create a data chunk with the aggregate results
                            let agg_chunk = DataChunk::new(arrays.clone());
                            // Evaluate the output expression
                            let result = gen_try!(expr.evaluate(&agg_chunk));
                            output_arrays.push(result.as_array().clone());
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
    use minigu_common::value::F64;

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
    fn test_sum_float_values_with_div_float() {
        // Test AVG with floating point values
        let chunk = data_chunk!((Float64, [1.5, 2.5, 3.5, 4.5]));

        let sum_div_f64_5 =
            ColumnRef::new(0).div(Constant::new(ScalarValue::Float64(Some(F64::from(5.0)))));

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .aggregate(
                vec![AggregateSpec::sum(Box::new(ColumnRef::new(0)), false)],
                vec![],
                vec![Box::new(sum_div_f64_5)],
            )
            .into_iter()
            .try_collect()
            .unwrap();

        // Expect: (1.5+2.5+3.5+4.5)/5.0 = 12.0/5.0 = 2.4
        let expected = data_chunk!((Float64, [2.4]));
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "chunks must not be empty")]
    fn test_sum_float_values_with_div_int_panic() {
        // Test AVG with floating point values
        let chunk = data_chunk!((Float64, [1.5, 2.5, 3.5, 4.5]));

        // inconsistent type (Float64 / Int64), will panic
        let sum_div_i64_5 = ColumnRef::new(0).div(Constant::new(ScalarValue::Int64(Some(10))));

        let _: DataChunk = [Ok(chunk)]
            .into_executor()
            .aggregate(
                vec![AggregateSpec::sum(Box::new(ColumnRef::new(0)), false)],
                vec![],
                vec![Box::new(sum_div_i64_5)],
            )
            .into_iter()
            .try_collect()
            .unwrap();
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
                vec![AggregateSpec::count()], // COUNT(*)
                vec![],                       // No grouping
                vec![Box::new(add_ten)],      // Output: COUNT(*) + 10
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
                    Box::new(ColumnRef::new(0)), // Keep department as-is
                    Box::new(count_times_100),   // COUNT(*) * 100
                    Box::new(ColumnRef::new(2)), // Keep SUM(salary) as-is
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
    fn test_output_expressions_with_multiple_aggregates() {
        // Test output expressions combining multiple aggregates (SUM + COUNT)
        let chunk = data_chunk!(
            (Int32, [1, 1, 2, 2, 1]),                // department
            (Int32, [5000, 6000, 4000, 4500, 5500])  // salary
        );

        // Create expression: SUM(salary) + COUNT(*)
        let sum_plus_count = ColumnRef::new(2).add(ColumnRef::new(1));

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .aggregate(
                vec![
                    AggregateSpec::count(),                                 // COUNT(*)
                    AggregateSpec::sum(Box::new(ColumnRef::new(1)), false), // SUM(salary)
                ],
                vec![Box::new(ColumnRef::new(0))], // GROUP BY department
                vec![
                    Box::new(ColumnRef::new(0)), // Keep department as-is
                    Box::new(sum_plus_count),    // SUM(salary) + COUNT(*)
                ],
            )
            .into_iter()
            .try_collect()
            .unwrap();

        // The result should be:
        // - The first column: department (group key)
        // - The second column: SUM(salary) + COUNT(*)
        //
        // The expected result:
        // department 1: COUNT=3, SUM=16500, so SUM+COUNT=16503
        // department 2: COUNT=2, SUM=8500, so SUM+COUNT=8502

        assert_eq!(result.len(), 2);
        assert_eq!(result.columns().len(), 2);

        // Get the result data for verification
        let dept_column = &result.columns()[0];
        let sum_plus_count_column = &result.columns()[1];

        let dept_values: Vec<i32> = dept_column
            .as_any()
            .downcast_ref::<arrow::array::Int32Array>()
            .unwrap()
            .iter()
            .map(|v| v.unwrap())
            .collect();

        let sum_plus_count_values: Vec<i64> = sum_plus_count_column
            .as_any()
            .downcast_ref::<arrow::array::Int64Array>()
            .unwrap()
            .iter()
            .map(|v| v.unwrap())
            .collect();

        // Check if the result contains the correct group data
        for i in 0..2 {
            let dept = dept_values[i];
            let sum_plus_count = sum_plus_count_values[i];

            match dept {
                1 => {
                    assert_eq!(
                        sum_plus_count, 16503,
                        "department 1 should have SUM(salary) + COUNT(*) = 16500 + 3 = 16503"
                    );
                }
                2 => {
                    assert_eq!(
                        sum_plus_count, 8502,
                        "department 2 should have SUM(salary) + COUNT(*) = 8500 + 2 = 8502"
                    );
                }
                _ => panic!("unexpected department value: {}", dept),
            }
        }
    }

    #[test]
    fn test_avg_unified_f64_precision() {
        // Test that AVG always uses f64 precision for all numeric types
        let chunk = data_chunk!(
            (Int32, [1, 2, 3]), // These values when averaged should be 2.0 exactly
            (Int64, [1000000000001, 1000000000002, 1000000000003])  /* Large integers that test
                                 * f64 precision */
        );

        // Test AVG with Int32 values
        let result_int32: DataChunk = [Ok(chunk.clone())]
            .into_executor()
            .aggregate(
                vec![AggregateSpec::avg(Box::new(ColumnRef::new(0)), false)],
                vec![],
                vec![],
            )
            .into_iter()
            .try_collect()
            .unwrap();

        // Expected AVG: (1+2+3)/3 = 6/3 = 2.0
        let expected_int32 = data_chunk!((Float64, [2.0]));
        assert_eq!(result_int32, expected_int32);

        // Test AVG with Int64 values (large integers)
        let result_int64: DataChunk = [Ok(chunk)]
            .into_executor()
            .aggregate(
                vec![AggregateSpec::avg(Box::new(ColumnRef::new(1)), false)],
                vec![],
                vec![],
            )
            .into_iter()
            .try_collect()
            .unwrap();

        // Expected AVG: (1000000000001+1000000000002+1000000000003)/3 = 3000000000006/3 =
        // 1000000000002.0
        let expected_int64 = data_chunk!((Float64, [1000000000002.0]));
        assert_eq!(result_int64, expected_int64);

        // Verify that the result type is consistently Float64
        let result_columns = result_int32.columns();
        assert_eq!(result_columns.len(), 1);
        assert!(result_columns[0].as_any().is::<Float64Array>());
    }
}
