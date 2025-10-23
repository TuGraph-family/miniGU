use std::sync::Arc;

use arrow::array::{Array, AsArray, FixedSizeListArray, Float32Array, Float32Builder};
use arrow::datatypes::Float32Type;
use minigu_common::data_chunk::DataChunk;
use minigu_common::types::VectorMetric;
use thiserror::Error;

use super::{BoxedEvaluator, DatumRef, Evaluator};
use crate::error::{ExecutionError, ExecutionResult};

#[derive(Debug, Error)]
#[error("{0}")]
struct VectorDistanceEvalError(String);

/// Compute L2 distance between two vectors starting at given offsets
fn compute_l2_distance(
    lhs_values: &Float32Array,
    rhs_values: &Float32Array,
    lhs_offset: usize,
    rhs_offset: usize,
    dimension: usize,
) -> f32 {
    let mut sum = 0.0f32;
    for dim in 0..dimension {
        let l = lhs_values.value(lhs_offset + dim);
        let r = rhs_values.value(rhs_offset + dim);
        let diff = l - r;
        sum += diff * diff;
    }
    sum.sqrt()
}

/// Validate vector arrays: check row count and dimension consistency
fn validate_vector_arrays(
    lhs_array: &FixedSizeListArray,
    rhs_array: &FixedSizeListArray,
    expected_dimension: usize,
    row_count: usize,
    lhs_is_scalar: bool,
    rhs_is_scalar: bool,
) -> ExecutionResult<()> {
    // Validate row counts for non-scalar arrays
    if !lhs_is_scalar && lhs_array.len() != row_count {
        return Err(ExecutionError::Custom(Box::new(VectorDistanceEvalError(
            format!(
                "left vector input has length {} but chunk has {} rows",
                lhs_array.len(),
                row_count
            ),
        ))));
    }
    if !rhs_is_scalar && rhs_array.len() != row_count {
        return Err(ExecutionError::Custom(Box::new(VectorDistanceEvalError(
            format!(
                "right vector input has length {} but chunk has {} rows",
                rhs_array.len(),
                row_count
            ),
        ))));
    }

    // Validate dimensions
    let lhs_dim = lhs_array.value_length() as usize;
    let rhs_dim = rhs_array.value_length() as usize;

    if lhs_dim != expected_dimension {
        return Err(ExecutionError::Custom(Box::new(VectorDistanceEvalError(
            format!(
                "left vector dimension {} doesn't match expected dimension {}",
                lhs_dim, expected_dimension
            ),
        ))));
    }
    if rhs_dim != expected_dimension {
        return Err(ExecutionError::Custom(Box::new(VectorDistanceEvalError(
            format!(
                "right vector dimension {} doesn't match expected dimension {}",
                rhs_dim, expected_dimension
            ),
        ))));
    }

    Ok(())
}

#[derive(Debug)]
pub struct VectorDistanceEvaluator {
    lhs: BoxedEvaluator,
    rhs: BoxedEvaluator,
    metric: VectorMetric,
    dimension: usize,
}

impl VectorDistanceEvaluator {
    pub fn new(
        lhs: BoxedEvaluator,
        rhs: BoxedEvaluator,
        metric: VectorMetric,
        dimension: usize,
    ) -> Self {
        Self {
            lhs,
            rhs,
            metric,
            dimension,
        }
    }
}

impl Evaluator for VectorDistanceEvaluator {
    fn evaluate(&self, chunk: &DataChunk) -> ExecutionResult<DatumRef> {
        let lhs = self.lhs.evaluate(chunk)?;
        let rhs = self.rhs.evaluate(chunk)?;

        let row_count = chunk.len();
        let lhs_array = lhs.as_array().as_fixed_size_list();
        let rhs_array = rhs.as_array().as_fixed_size_list();

        // Validate input arrays
        validate_vector_arrays(
            lhs_array,
            rhs_array,
            self.dimension,
            row_count,
            lhs.is_scalar(),
            rhs.is_scalar(),
        )?;

        let lhs_values = lhs_array.values().as_primitive::<Float32Type>();
        let rhs_values = rhs_array.values().as_primitive::<Float32Type>();
        let stride = self.dimension;

        let mut builder = Float32Builder::with_capacity(row_count);

        // Process each row in the chunk: compute vector distances with support for scalar
        // broadcasting
        for row in 0..row_count {
            // Handle scalar broadcasting: if input is scalar, use index 0, otherwise use current
            // row index
            let lhs_index = if lhs.is_scalar() { 0 } else { row };
            let rhs_index = if rhs.is_scalar() { 0 } else { row };

            // Skip NULL values: if either vector is NULL, append NULL to result and continue to
            // next row
            if lhs_array.is_null(lhs_index) || rhs_array.is_null(rhs_index) {
                builder.append_null();
                continue;
            }

            let lhs_offset = lhs_array.value_offset(lhs_index) as usize;
            let rhs_offset = rhs_array.value_offset(rhs_index) as usize;

            let distance = match self.metric {
                VectorMetric::L2 => {
                    compute_l2_distance(lhs_values, rhs_values, lhs_offset, rhs_offset, stride)
                }
            };

            builder.append_value(distance);
        }

        let is_scalar = lhs.is_scalar() && rhs.is_scalar() && row_count <= 1;
        let array = Arc::new(builder.finish());
        Ok(DatumRef::new(array, is_scalar))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use arrow::array::{ArrayRef, FixedSizeListArray, Float32Array};
    use arrow::datatypes::{DataType, Field};
    use minigu_common::data_chunk::DataChunk;
    use minigu_common::value::{F32, ScalarValue, VectorValue};

    use super::*;
    use crate::evaluator::column_ref::ColumnRef;
    use crate::evaluator::constant::Constant;

    #[test]
    fn test_vector_distance_l2() {
        let values = Float32Array::from(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let field = Arc::new(Field::new("item", DataType::Float32, false));
        let list = FixedSizeListArray::new(field, 3, Arc::new(values) as ArrayRef, None);
        let chunk = DataChunk::new(vec![Arc::new(list) as ArrayRef]);

        let query =
            VectorValue::new(vec![F32::from(1.0), F32::from(2.0), F32::from(3.0)], 3).unwrap();
        let query_scalar = ScalarValue::new_vector(3, Some(query));

        let evaluator = VectorDistanceEvaluator::new(
            Box::new(Constant::new(query_scalar)),
            Box::new(ColumnRef::new(0)),
            VectorMetric::L2,
            3,
        );

        let result = evaluator.evaluate(&chunk).unwrap();
        let array = result.into_array();
        let distances = array.as_primitive::<Float32Type>();

        assert!((distances.value(0) - 0.0).abs() < 1e-6);
        assert!((distances.value(1) - 5.196_152).abs() < 1e-6);
    }
}
