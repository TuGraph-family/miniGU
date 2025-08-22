use std::collections::HashMap;
use std::sync::Arc;

use arrow::array::{
    Array, ArrayRef, AsArray, Float32Array, Float64Array, Int64Array, ListArray, StringArray,
};
use arrow::datatypes::DataType;
use minigu_common::data_chunk::DataChunk;
use minigu_common::value::{ScalarValue, ScalarValueAccessor};

use super::utils::gen_try;
use super::{Executor, IntoExecutor};
use crate::error::ExecutionResult;
use crate::evaluator::BoxedEvaluator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FactorizedAggregateFunction {
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
pub struct FactorizedAggregateSpec {
    function: FactorizedAggregateFunction,
    expression: Option<BoxedEvaluator>,
    distinct: bool,
}

impl FactorizedAggregateSpec {
    /// Create COUNT(*) aggregate specification
    pub fn count() -> Self {
        Self {
            function: FactorizedAggregateFunction::Count,
            expression: None,
            distinct: false,
        }
    }

    /// Create COUNT(expr) aggregate specification
    pub fn count_expression(expr: BoxedEvaluator, distinct: bool) -> Self {
        Self {
            function: FactorizedAggregateFunction::CountExpression,
            expression: Some(expr),
            distinct,
        }
    }

    /// Create SUM(expr) aggregate specification
    pub fn sum(expr: BoxedEvaluator, distinct: bool) -> Self {
        Self {
            function: FactorizedAggregateFunction::Sum,
            expression: Some(expr),
            distinct,
        }
    }

    /// Create AVG(expr) aggregate specification
    pub fn avg(expr: BoxedEvaluator, distinct: bool) -> Self {
        Self {
            function: FactorizedAggregateFunction::Avg,
            expression: Some(expr),
            distinct,
        }
    }

    /// Create MIN(expr) aggregate specification
    pub fn min(expr: BoxedEvaluator) -> Self {
        Self {
            function: FactorizedAggregateFunction::Min,
            expression: Some(expr),
            distinct: false,
        }
    }

    /// Create MAX(expr) aggregate specification
    pub fn max(expr: BoxedEvaluator) -> Self {
        Self {
            function: FactorizedAggregateFunction::Max,
            expression: Some(expr),
            distinct: false,
        }
    }
}

/// Aggregate state for storing intermediate results during aggregation
#[derive(Debug)]
enum FactorizedAggregateState {
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

impl FactorizedAggregateState {
    /// Create a new aggregate state
    fn new(func: &FactorizedAggregateFunction, distinct: bool) -> Self {
        match func {
            FactorizedAggregateFunction::Count => Self::Count { count: 0 },
            FactorizedAggregateFunction::CountExpression => Self::CountExpression {
                count: 0,
                distinct_values: if distinct { Some(HashMap::new()) } else { None },
            },
            FactorizedAggregateFunction::Sum => Self::Sum {
                sum_i64: None,
                sum_f64: None,
                distinct_values: if distinct { Some(HashMap::new()) } else { None },
            },
            FactorizedAggregateFunction::Avg => Self::Avg {
                sum_f64: 0.0,
                count: 0,
                distinct_values: if distinct { Some(HashMap::new()) } else { None },
            },
            FactorizedAggregateFunction::Min => Self::Min {
                min_i64: None,
                min_f64: None,
                min_string: None,
            },
            FactorizedAggregateFunction::Max => Self::Max {
                max_i64: None,
                max_f64: None,
                max_string: None,
            },
        }
    }

    /// Update the aggregate state with a new value
    fn update(&mut self, value: Option<ScalarValue>) -> ExecutionResult<()> {
        match self {
            FactorizedAggregateState::Count { count } => {
                *count += 1;
            }
            FactorizedAggregateState::CountExpression {
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
            FactorizedAggregateState::Sum {
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
            FactorizedAggregateState::Avg {
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
            FactorizedAggregateState::Min { .. } => {
                if let Some(val) = value {
                    if !is_null_value(&val) {
                        self.update_min_aggregate(&val)?;
                    }
                }
            }
            FactorizedAggregateState::Max { .. } => {
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
            FactorizedAggregateState::Sum {
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
            FactorizedAggregateState::Avg { sum_f64, count, .. } => {
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
        if let FactorizedAggregateState::Min {
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
        if let FactorizedAggregateState::Max {
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
            FactorizedAggregateState::Count { count } => Ok(ScalarValue::Int64(Some(*count))),

            FactorizedAggregateState::CountExpression {
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

            FactorizedAggregateState::Sum {
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

            FactorizedAggregateState::Avg {
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

            FactorizedAggregateState::Min {
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

            FactorizedAggregateState::Max {
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
pub struct FactorizedAggregateBuilder<E> {
    child: E,
    aggregate_specs: Vec<FactorizedAggregateSpec>,
    group_by_expressions: Vec<BoxedEvaluator>,
    output_expressions: Vec<BoxedEvaluator>, // Expressions like `1 + COUNT(*)`
    unflat_col_idx: Option<usize>,           // an unflat column index for COUNT(*)
}

impl<E> FactorizedAggregateBuilder<E> {
    /// Create a new aggregate builder
    pub fn new(
        child: E,
        aggregate_specs: Vec<FactorizedAggregateSpec>,
        group_by_expressions: Vec<BoxedEvaluator>,
        output_expressions: Vec<BoxedEvaluator>,
        unflat_col_idx: Option<usize>,
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
            unflat_col_idx,
        }
    }
}

impl<E> IntoExecutor for FactorizedAggregateBuilder<E>
where
    E: Executor,
{
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let FactorizedAggregateBuilder {
                child,
                aggregate_specs,
                group_by_expressions,
                output_expressions,
                unflat_col_idx,
            } = self;

            // If there is no grouping expression, perform simple aggregation
            if group_by_expressions.is_empty() {
                // Create aggregate states for each aggregate spec
                let mut states: Vec<FactorizedAggregateState> = aggregate_specs
                    .iter()
                    .map(|spec| FactorizedAggregateState::new(&spec.function, spec.distinct))
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
                            if let Some(ref expr) = spec.expression {
                                // Create a single row data chunk for the current row
                                let row_columns: Vec<ArrayRef> = chunk
                                    .columns()
                                    .iter()
                                    .map(|col| col.slice(row.row_index(), 1))
                                    .collect();
                                let row_chunk = DataChunk::new(row_columns);
                                let result_datum = gen_try!(expr.evaluate(&row_chunk));
                                let result_array = result_datum.as_array();

                                assert!(
                                    matches!(result_array.data_type(), DataType::List(_)),
                                    "Factorized aggregate expects list data"
                                );
                                let list_array: &ListArray = result_array.as_list();
                                let inner_array = list_array.value(0);
                                // For unflat columns, iterate through all elements in the list
                                for j in 0..inner_array.len() {
                                    let scalar_value = inner_array.index(j);
                                    gen_try!(states[i].update(Some(scalar_value)));
                                }
                            } else {
                                // COUNT(*)
                                let idx = unflat_col_idx.expect(
                                    "COUNT(*) in factorized aggregate requires an unflat column index",
                                );
                                let unflat_column = &chunk.columns()[idx];
                                // Found unflat column, count each element
                                let list_array: &ListArray = unflat_column.as_list();
                                let inner_array = list_array.value(row.row_index());
                                for _ in 0..inner_array.len() {
                                    let value = Some(ScalarValue::Int64(Some(1)));
                                    gen_try!(states[i].update(value));
                                }
                            };
                        }
                    }
                }

                // If there is no data, return the default aggregate result
                if !has_data {
                    let mut result_columns = Vec::new();
                    for spec in &aggregate_specs {
                        let default_value = match spec.function {
                            FactorizedAggregateFunction::Count | FactorizedAggregateFunction::CountExpression => {
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
                let mut groups: HashMap<Vec<ScalarValue>, Vec<FactorizedAggregateState>> = HashMap::new();
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
                                .map(|spec| FactorizedAggregateState::new(&spec.function, spec.distinct))
                                .collect()
                        });

                        // Update the aggregate state for the current row
                        for (i, spec) in aggregate_specs.iter().enumerate() {
                            if let Some(ref expr) = spec.expression {
                                // Create a single row data chunk for the current row
                                let row_columns: Vec<ArrayRef> = chunk
                                    .columns()
                                    .iter()
                                    .map(|col| col.slice(row.row_index(), 1))
                                    .collect();
                                let row_chunk = DataChunk::new(row_columns);
                                let result_datum = gen_try!(expr.evaluate(&row_chunk));
                                let result_array = result_datum.as_array();

                                assert!(
                                    matches!(result_array.data_type(), DataType::List(_)),
                                    "Factorized aggregate expects list data"
                                );
                                let list_array: &ListArray = result_array.as_list();
                                let inner_array = list_array.value(0);
                                // For unflat columns, iterate through all elements in the list
                                for j in 0..inner_array.len() {
                                    let scalar_value = inner_array.index(j);
                                    gen_try!(states[i].update(Some(scalar_value)));
                                }
                            } else {
                                // COUNT(*)
                                let idx = unflat_col_idx.expect(
                                    "COUNT(*) in factorized aggregate requires an unflat column index",
                                );
                                let unflat_column = &chunk.columns()[idx];
                                // Found unflat column, count each element
                                let list_array: &ListArray = unflat_column.as_list();
                                let inner_array = list_array.value(row.row_index());
                                for _ in 0..inner_array.len() {
                                    let value = Some(ScalarValue::Int64(Some(1)));
                                    gen_try!(states[i].update(value));
                                }
                            };
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
    use arrow::array::{Array, Float64Array, Int32Array, Int32Builder, Int64Array, ListBuilder};
    use arrow::datatypes::{DataType as ArrowDataType, Field};
    use itertools::Itertools;
    use minigu_common::data_chunk;
    use minigu_common::data_chunk::DataChunk;
    use minigu_common::value::ScalarValue;

    use super::*;
    use crate::evaluator::Evaluator;
    use crate::evaluator::column_ref::ColumnRef;
    use crate::evaluator::constant::Constant;

    /// Helper to build a ListArray of Int32 values
    fn build_list_i32(data: &[&[Option<i32>]]) -> ArrayRef {
        let field = Field::new_list_field(ArrowDataType::Int32, true);
        let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
        for sublist in data {
            for &item in *sublist {
                builder.values().append_option(item);
            }
            builder.append(true);
        }
        Arc::new(builder.finish())
    }

    /// Helper to build a ListArray of Int64 values
    fn build_list_i64(data: &[&[Option<i64>]]) -> ArrayRef {
        let field = Field::new_list_field(ArrowDataType::Int64, true);
        let mut builder = ListBuilder::new(Int64Array::builder(0)).with_field(Arc::new(field));
        for sublist in data {
            for &item in *sublist {
                builder.values().append_option(item);
            }
            builder.append(true);
        }
        Arc::new(builder.finish())
    }

    /// Helper to build a ListArray of Float64 values
    fn build_list_f64(data: &[&[Option<f64>]]) -> ArrayRef {
        let field = Field::new_list_field(ArrowDataType::Float64, true);
        let mut builder = ListBuilder::new(Float64Array::builder(0)).with_field(Arc::new(field));
        for sublist in data {
            for &item in *sublist {
                builder.values().append_option(item);
            }
            builder.append(true);
        }
        Arc::new(builder.finish())
    }

    #[test]
    fn test_factorized_count_star() {
        let chunk1 = DataChunk::new(vec![build_list_i32(&[&[Some(1)], &[Some(2), Some(3)]])]);
        let chunk2 = DataChunk::new(vec![build_list_i32(&[&[Some(4), Some(5)]])]);

        let result: DataChunk = [Ok(chunk1), Ok(chunk2)]
            .into_executor()
            .factorized_aggregate(
                vec![FactorizedAggregateSpec::count()],
                vec![],
                vec![],
                Some(0),
            )
            .into_iter()
            .try_collect()
            .unwrap();

        let expected = data_chunk!((Int64, [5]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_factorized_count_expression() {
        let chunk = DataChunk::new(vec![build_list_i32(&[&[Some(1), Some(2)], &[
            Some(3),
            Some(4),
            Some(5),
        ]])]);

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .factorized_aggregate(
                vec![FactorizedAggregateSpec::count_expression(
                    Box::new(ColumnRef::new(0)),
                    false,
                )],
                vec![],
                vec![],
                None,
            )
            .into_iter()
            .try_collect()
            .unwrap();

        let expected = data_chunk!((Int64, [5]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_factorized_count_with_nulls() {
        let chunk = DataChunk::new(vec![build_list_i32(&[&[Some(1), None], &[Some(3)], &[
            None,
            Some(5),
        ]])]);

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .factorized_aggregate(
                vec![FactorizedAggregateSpec::count_expression(
                    Box::new(ColumnRef::new(0)),
                    false,
                )],
                vec![],
                vec![],
                None,
            )
            .into_iter()
            .try_collect()
            .unwrap();

        let expected = data_chunk!((Int64, [3]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_factorized_sum() {
        let chunk = DataChunk::new(vec![build_list_i32(&[&[Some(1), Some(2)], &[
            Some(3),
            Some(4),
            Some(5),
        ]])]);

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .factorized_aggregate(
                vec![FactorizedAggregateSpec::sum(
                    Box::new(ColumnRef::new(0)),
                    false,
                )],
                vec![],
                vec![],
                None,
            )
            .into_iter()
            .try_collect()
            .unwrap();

        let expected = data_chunk!((Int64, [15]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_factorized_min_max() {
        let chunk = DataChunk::new(vec![build_list_i32(&[&[Some(5), Some(1)], &[Some(3)], &[
            Some(9),
            Some(2),
        ]])]);

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .factorized_aggregate(
                vec![
                    FactorizedAggregateSpec::min(Box::new(ColumnRef::new(0))),
                    FactorizedAggregateSpec::max(Box::new(ColumnRef::new(0))),
                ],
                vec![],
                vec![],
                None,
            )
            .into_iter()
            .try_collect()
            .unwrap();

        let expected = data_chunk!((Int64, [1]), (Int64, [9]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_factorized_avg() {
        let chunk = DataChunk::new(vec![build_list_i32(&[&[Some(1), Some(2)], &[
            Some(3),
            Some(4),
            Some(5),
        ]])]);

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .factorized_aggregate(
                vec![FactorizedAggregateSpec::avg(
                    Box::new(ColumnRef::new(0)),
                    false,
                )],
                vec![],
                vec![],
                None,
            )
            .into_iter()
            .try_collect()
            .unwrap();

        let expected = data_chunk!((Float64, [3.0]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_factorized_avg_with_nulls() {
        let chunk = DataChunk::new(vec![build_list_i32(&[&[Some(2), None], &[Some(4)], &[
            None,
            Some(6),
        ]])]);

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .factorized_aggregate(
                vec![FactorizedAggregateSpec::avg(
                    Box::new(ColumnRef::new(0)),
                    false,
                )],
                vec![],
                vec![],
                None,
            )
            .into_iter()
            .try_collect()
            .unwrap();

        let expected = data_chunk!((Float64, [4.0]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_factorized_avg_float_values() {
        let chunk = DataChunk::new(vec![build_list_f64(&[&[Some(1.5), Some(2.5)], &[
            Some(3.5),
            Some(4.5),
        ]])]);

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .factorized_aggregate(
                vec![FactorizedAggregateSpec::avg(
                    Box::new(ColumnRef::new(0)),
                    false,
                )],
                vec![],
                vec![],
                None,
            )
            .into_iter()
            .try_collect()
            .unwrap();

        let expected = data_chunk!((Float64, [3.0]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_factorized_group_by_aggregate() {
        let group_col = Arc::new(Int32Array::from(vec![1, 1, 2, 2, 1]));
        let agg_col = build_list_i32(&[
            &[Some(5000), Some(6000)],
            &[Some(5500)],
            &[Some(4000), Some(4500)],
            &[],
            &[],
        ]);
        let chunk = DataChunk::new(vec![group_col, agg_col]);

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .factorized_aggregate(
                vec![
                    FactorizedAggregateSpec::count(),
                    FactorizedAggregateSpec::sum(Box::new(ColumnRef::new(1)), false),
                ],
                vec![Box::new(ColumnRef::new(0))],
                vec![],
                Some(1),
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
        // department 1: COUNT=3, SUM=16500
        // department 2: COUNT=2, SUM=8500

        assert_eq!(result.len(), 2);
        assert_eq!(result.columns().len(), 3);

        // Get the result data for verification
        let dept_column = result.columns()[0]
            .as_any()
            .downcast_ref::<Int32Array>()
            .unwrap();
        let count_column = result.columns()[1]
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();
        let sum_column = result.columns()[2]
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();

        let mut found_dept1 = false;
        let mut found_dept2 = false;

        for i in 0..result.len() {
            let dept = dept_column.value(i);
            match dept {
                1 => {
                    assert!(!found_dept1, "Found duplicate row for department 1");
                    assert_eq!(count_column.value(i), 3);
                    assert_eq!(sum_column.value(i), 16500);
                    found_dept1 = true;
                }
                2 => {
                    assert!(!found_dept2, "Found duplicate row for department 2");
                    assert_eq!(count_column.value(i), 2);
                    assert_eq!(sum_column.value(i), 8500);
                    found_dept2 = true;
                }
                _ => panic!("unexpected department value: {}", dept),
            }
        }
        assert!(
            found_dept1 && found_dept2,
            "Did not find results for all departments"
        );
    }

    #[test]
    fn test_factorized_group_by_multiple_keys() {
        let group_col1 = Arc::new(Int32Array::from(vec![1, 1, 1, 2, 2]));
        let group_col2 = Arc::new(Int32Array::from(vec![1, 2, 1, 1, 2]));
        let agg_col = build_list_i32(&[
            &[Some(5000), Some(5500)],
            &[Some(8000)],
            &[],
            &[Some(4000)],
            &[Some(7000)],
        ]);
        let chunk = DataChunk::new(vec![group_col1, group_col2, agg_col]);

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .factorized_aggregate(
                vec![
                    FactorizedAggregateSpec::count(),
                    FactorizedAggregateSpec::avg(Box::new(ColumnRef::new(2)), false),
                ],
                vec![
                    Box::new(ColumnRef::new(0)), // GROUP BY department
                    Box::new(ColumnRef::new(1)), // GROUP BY position
                ],
                vec![],
                Some(2),
            )
            .into_iter()
            .try_collect()
            .unwrap();

        // The expected result:
        // (department 1, position 1): COUNT=2, AVG=5250
        // (department 1, position 2): COUNT=1, AVG=8000
        // (department 2, position 1): COUNT=1, AVG=4000
        // (department 2, position 2): COUNT=1, AVG=7000

        assert_eq!(result.len(), 4);
        assert_eq!(result.columns().len(), 4);

        let dept_col = result.columns()[0]
            .as_any()
            .downcast_ref::<Int32Array>()
            .unwrap();
        let pos_col = result.columns()[1]
            .as_any()
            .downcast_ref::<Int32Array>()
            .unwrap();
        let count_col = result.columns()[2]
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();
        let avg_col = result.columns()[3]
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap();

        let mut found = [false; 4]; // To track if we found each of the 4 expected groups

        for i in 0..result.len() {
            let dept = dept_col.value(i);
            let pos = pos_col.value(i);

            match (dept, pos) {
                (1, 1) => {
                    assert!(!found[0]);
                    assert_eq!(count_col.value(i), 2);
                    assert!((avg_col.value(i) - 5250.0).abs() < 0.01);
                    found[0] = true;
                }
                (1, 2) => {
                    assert!(!found[1]);
                    assert_eq!(count_col.value(i), 1);
                    assert!((avg_col.value(i) - 8000.0).abs() < 0.01);
                    found[1] = true;
                }
                (2, 1) => {
                    assert!(!found[2]);
                    assert_eq!(count_col.value(i), 1);
                    assert!((avg_col.value(i) - 4000.0).abs() < 0.01);
                    found[2] = true;
                }
                (2, 2) => {
                    assert!(!found[3]);
                    assert_eq!(count_col.value(i), 1);
                    assert!((avg_col.value(i) - 7000.0).abs() < 0.01);
                    found[3] = true;
                }
                _ => panic!("unexpected group key combination: ({}, {})", dept, pos),
            }
        }
        assert!(found.iter().all(|&f| f), "Did not find all expected groups");
    }

    #[test]
    fn test_factorized_output_expressions_simple() {
        let chunk = DataChunk::new(vec![build_list_i32(&[&[Some(1), Some(2)], &[
            Some(3),
            Some(4),
            Some(5),
        ]])]);

        let add_ten = ColumnRef::new(0).add(Constant::new(ScalarValue::Int64(Some(10))));
        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .factorized_aggregate(
                vec![FactorizedAggregateSpec::count()],
                vec![],
                vec![Box::new(add_ten)],
                Some(0),
            )
            .into_iter()
            .try_collect()
            .unwrap();

        let expected = data_chunk!((Int64, [15]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_factorized_output_expressions_with_grouping() {
        let group_col = Arc::new(Int32Array::from(vec![1, 1, 2, 2, 1]));
        let agg_col = build_list_i32(&[
            &[Some(5000), Some(6000)],
            &[Some(5500)],
            &[Some(4000), Some(4500)],
            &[],
            &[],
        ]);
        let chunk = DataChunk::new(vec![group_col, agg_col]);

        let count_times_100 = ColumnRef::new(1).mul(Constant::new(ScalarValue::Int64(Some(100))));

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .factorized_aggregate(
                vec![
                    FactorizedAggregateSpec::count(),
                    FactorizedAggregateSpec::sum(Box::new(ColumnRef::new(1)), false),
                ],
                vec![Box::new(ColumnRef::new(0))],
                vec![
                    Box::new(ColumnRef::new(0)),
                    Box::new(count_times_100),
                    Box::new(ColumnRef::new(2)),
                ],
                Some(1),
            )
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

        let mut found_dept1 = false;
        let mut found_dept2 = false;

        for i in 0..result.len() {
            let dept = result.columns()[0]
                .as_any()
                .downcast_ref::<Int32Array>()
                .unwrap()
                .value(i);
            let count_val = result.columns()[1]
                .as_any()
                .downcast_ref::<Int64Array>()
                .unwrap()
                .value(i);
            let sum_val = result.columns()[2]
                .as_any()
                .downcast_ref::<Int64Array>()
                .unwrap()
                .value(i);

            match dept {
                1 => {
                    assert!(!found_dept1);
                    assert_eq!(count_val, 300);
                    assert_eq!(sum_val, 16500);
                    found_dept1 = true;
                }
                2 => {
                    assert!(!found_dept2);
                    assert_eq!(count_val, 200);
                    assert_eq!(sum_val, 8500);
                    found_dept2 = true;
                }
                _ => panic!("Unexpected department"),
            }
        }
        assert!(found_dept1 && found_dept2);
    }

    #[test]
    fn test_factorized_output_expressions_with_multiple_aggregates() {
        let group_col = Arc::new(Int32Array::from(vec![1, 1, 2, 2, 1]));
        let agg_col = build_list_i32(&[
            &[Some(5000), Some(6000)],
            &[Some(5500)],
            &[Some(4000), Some(4500)],
            &[],
            &[],
        ]);
        let chunk = DataChunk::new(vec![group_col, agg_col]);

        let sum_plus_count = ColumnRef::new(2).add(ColumnRef::new(1));

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .factorized_aggregate(
                vec![
                    FactorizedAggregateSpec::count(),
                    FactorizedAggregateSpec::sum(Box::new(ColumnRef::new(1)), false),
                ],
                vec![Box::new(ColumnRef::new(0))],
                vec![Box::new(ColumnRef::new(0)), Box::new(sum_plus_count)],
                Some(1),
            )
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

        let mut found_dept1 = false;
        let mut found_dept2 = false;

        for i in 0..result.len() {
            let dept = result.columns()[0]
                .as_any()
                .downcast_ref::<Int32Array>()
                .unwrap()
                .value(i);
            let sum_plus_count_val = result.columns()[1]
                .as_any()
                .downcast_ref::<Int64Array>()
                .unwrap()
                .value(i);

            match dept {
                1 => {
                    assert!(!found_dept1);
                    assert_eq!(sum_plus_count_val, 16503); // 16500 + 3
                    found_dept1 = true;
                }
                2 => {
                    assert!(!found_dept2);
                    assert_eq!(sum_plus_count_val, 8502); // 8500 + 2
                    found_dept2 = true;
                }
                _ => panic!("Unexpected department"),
            }
        }
        assert!(found_dept1 && found_dept2);
    }

    #[test]
    fn test_factorized_sum_float_values_with_div_float() {
        let chunk = DataChunk::new(vec![
            build_list_f64(&[
                &[Some(1.5), Some(2.5)], // row 1, list 1
                &[Some(3.5), Some(4.5)], // row 2, list 2
            ]), // Total SUM = 12.0
            build_list_f64(&[
                &[Some(2.0)],                       // row 1, list 1
                &[Some(2.0), Some(2.0), Some(2.0)], // row 2, list 2
            ]), // Total SUM = 8.0
        ]);

        // SUM(col0) / SUM(col1) -> 12.0 / 8.0 = 1.5
        let sum_div_sum = ColumnRef::new(0).div(ColumnRef::new(1));

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .factorized_aggregate(
                vec![
                    FactorizedAggregateSpec::sum(Box::new(ColumnRef::new(0)), false),
                    FactorizedAggregateSpec::sum(Box::new(ColumnRef::new(1)), false),
                ],
                vec![],
                vec![Box::new(sum_div_sum)],
                None,
            )
            .into_iter()
            .try_collect()
            .unwrap();

        let expected = data_chunk!((Float64, [1.5]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_factorized_sum_float_values_with_div_int_err() {
        let chunk = DataChunk::new(vec![
            build_list_f64(&[&[Some(1.5), Some(2.5)], &[Some(3.5), Some(4.5)]]), /* SUM = 12.0
                                                                                  * (Float64) */
            build_list_i64(&[&[Some(2), Some(2)], &[Some(2), Some(2)]]), // SUM = 8 (Int64)
        ]);

        // SUM(col0) / SUM(col1) -> a float / an int -> should error
        let sum_div_sum = ColumnRef::new(0).div(ColumnRef::new(1));

        let result: Result<Vec<DataChunk>, _> = [Ok(chunk)]
            .into_executor()
            .factorized_aggregate(
                vec![
                    FactorizedAggregateSpec::sum(Box::new(ColumnRef::new(0)), false),
                    FactorizedAggregateSpec::sum(Box::new(ColumnRef::new(1)), false),
                ],
                vec![],
                vec![Box::new(sum_div_sum)],
                None,
            )
            .into_iter()
            .try_collect();

        assert!(result.is_err());
    }

    #[test]
    fn test_factorized_avg_unified_f64_precision() {
        // Test that AVG always uses f64 precision for all numeric types
        let chunk = DataChunk::new(vec![
            build_list_i32(&[&[Some(1)], &[Some(2)], &[Some(3)]]), // avg = 2.0
            build_list_i64(&[&[Some(1_000_000_000_001)], &[Some(1_000_000_000_002)], &[
                Some(1_000_000_000_003),
            ]]), // avg = 1_000_000_000_002.0
        ]);

        // Test AVG with Int32 values
        let result_int32: DataChunk = [Ok(chunk.clone())]
            .into_executor()
            .factorized_aggregate(
                vec![FactorizedAggregateSpec::avg(
                    Box::new(ColumnRef::new(0)),
                    false,
                )],
                vec![],
                vec![],
                None,
            )
            .into_iter()
            .try_collect()
            .unwrap();

        let avg_int32 = result_int32.columns()[0]
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap()
            .value(0);

        assert_eq!(avg_int32, 2.0, "AVG of Int32 should be a precise f64");

        // Test AVG with Int64 values
        let result_int64: DataChunk = [Ok(chunk)]
            .into_executor()
            .factorized_aggregate(
                vec![FactorizedAggregateSpec::avg(
                    Box::new(ColumnRef::new(1)),
                    false,
                )],
                vec![],
                vec![],
                None,
            )
            .into_iter()
            .try_collect()
            .unwrap();

        let avg_int64 = result_int64.columns()[0]
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap()
            .value(0);

        assert_eq!(
            avg_int64, 1_000_000_000_002.0,
            "AVG of Int64 should be a precise f64"
        );
    }
}
