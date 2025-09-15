use std::sync::Arc;

use arrow::array::{Array, FixedSizeListArray, Float32Array};
use minigu_common::data_chunk::DataChunk;
use minigu_common::types::VectorMetric;
use vector::distance_l2_vector_f32;

use super::{BoxedEvaluator, DatumRef, Evaluator};
use crate::error::{ExecutionError, ExecutionResult};

#[derive(Debug)]
pub struct VectorDistanceEvaluator {
    query_vector: BoxedEvaluator,
    target_vector: BoxedEvaluator,
    metric: VectorMetric,
}

impl VectorDistanceEvaluator {
    pub fn new(
        query_vector: BoxedEvaluator,
        target_vector: BoxedEvaluator,
        metric: VectorMetric,
    ) -> Self {
        Self {
            query_vector,
            target_vector,
            metric,
        }
    }

    fn evaluate_similarity_search(
        &self,
        query_datum: DatumRef,
        target_datum: DatumRef,
    ) -> ExecutionResult<DatumRef> {
        let query_vec = Self::extract_single_vector(&query_datum)?;
        let target_vectors = Self::extract_vector_batch(&target_datum)?;

        let mut distances = Vec::with_capacity(target_vectors.len());

        for (row_idx, target_vec) in target_vectors.iter().enumerate() {
            if query_vec.len() != target_vec.len() {
                return Err(ExecutionError::Custom(
                    format!(
                        "Vector dimension mismatch at row {}: query={}, target={}",
                        row_idx,
                        query_vec.len(),
                        target_vec.len()
                    )
                    .into(),
                ));
            }

            let distance = Self::compute_distance(&query_vec, target_vec, self.metric)?;
            distances.push(distance);
        }
        let result_array = Arc::new(Float32Array::from(distances));
        Ok(DatumRef::new(result_array, false))
    }

    fn evaluate_scalar_pair(
        &self,
        query_datum: DatumRef,
        target_datum: DatumRef,
    ) -> ExecutionResult<DatumRef> {
        let query_vec = Self::extract_single_vector(&query_datum)?;
        let target_vec = Self::extract_single_vector(&target_datum)?;
        let distance = Self::compute_distance(&query_vec, &target_vec, self.metric)?;

        let result_array = Arc::new(Float32Array::from(vec![distance]));
        Ok(DatumRef::new(result_array, true)) // is_scalar=true
    }

    fn extract_single_vector(datum: &DatumRef) -> ExecutionResult<Vec<f32>> {
        let array = datum.as_array();

        if let Some(list_array) = array.as_any().downcast_ref::<FixedSizeListArray>() {
            let values = list_array.values();
            if let Some(float_array) = values.as_any().downcast_ref::<Float32Array>() {
                Ok(float_array.values().to_vec())
            } else {
                Err(ExecutionError::Custom(
                    format!(
                        "Expected Float32Array in vector, got {:?}",
                        values.data_type()
                    )
                    .into(),
                ))
            }
        } else {
            Err(ExecutionError::Custom(
                format!(
                    "Expected FixedSizeListArray for vector, got {:?}",
                    array.data_type()
                )
                .into(),
            ))
        }
    }

    fn extract_vector_batch(datum: &DatumRef) -> ExecutionResult<Vec<Vec<f32>>> {
        let array = datum.as_array();

        if let Some(list_array) = array.as_any().downcast_ref::<FixedSizeListArray>() {
            let mut vectors = Vec::with_capacity(list_array.len());

            for i in 0..list_array.len() {
                if list_array.is_null(i) {
                    return Err(ExecutionError::Custom(
                        format!("Null vector at row {}", i).into(),
                    ));
                }

                let vector_array = list_array.value(i);
                if let Some(float_array) = vector_array.as_any().downcast_ref::<Float32Array>() {
                    vectors.push(float_array.values().to_vec());
                } else {
                    return Err(ExecutionError::Custom(
                        format!("Invalid vector type at row {}", i).into(),
                    ));
                }
            }
            Ok(vectors)
        } else {
            Err(ExecutionError::Custom(
                format!(
                    "Expected FixedSizeListArray for vector batch, got {:?}",
                    array.data_type()
                )
                .into(),
            ))
        }
    }

    fn compute_distance(vec1: &[f32], vec2: &[f32], metric: VectorMetric) -> ExecutionResult<f32> {
        if vec1.len() != vec2.len() {
            return Err(ExecutionError::Custom(
                format!(
                    "Vector dimension mismatch: {} vs {}",
                    vec1.len(),
                    vec2.len()
                )
                .into(),
            ));
        }

        match metric {
            VectorMetric::L2 => {
                // Use dynamic dispatch for different dimensions
                let distance = match vec1.len() {
                    104 => unsafe {
                        let query_array = &*(vec1.as_ptr() as *const [f32; 104]);
                        let stored_array = &*(vec2.as_ptr() as *const [f32; 104]);
                        distance_l2_vector_f32::<104>(query_array, stored_array)
                    },
                    128 => unsafe {
                        let query_array = &*(vec1.as_ptr() as *const [f32; 128]);
                        let stored_array = &*(vec2.as_ptr() as *const [f32; 128]);
                        distance_l2_vector_f32::<128>(query_array, stored_array)
                    },
                    256 => unsafe {
                        let query_array = &*(vec1.as_ptr() as *const [f32; 256]);
                        let stored_array = &*(vec2.as_ptr() as *const [f32; 256]);
                        distance_l2_vector_f32::<256>(query_array, stored_array)
                    },
                    // TODO: Fallback to manual calculation for other dimensions
                    _ => {
                        let mut distance = 0.0f32;
                        for (v1, v2) in vec1.iter().zip(vec2.iter()) {
                            let diff = v1 - v2;
                            distance += diff * diff;
                        }
                        distance.sqrt()
                    }
                };
                Ok(distance)
            }
        }
    }
}

impl Evaluator for VectorDistanceEvaluator {
    fn evaluate(&self, chunk: &DataChunk) -> ExecutionResult<DatumRef> {
        let query_datum = self.query_vector.evaluate(chunk)?;
        let target_datum = self.target_vector.evaluate(chunk)?;

        if query_datum.is_scalar() && !target_datum.is_scalar() {
            self.evaluate_similarity_search(query_datum, target_datum)
        } else if query_datum.is_scalar() && target_datum.is_scalar() {
            self.evaluate_scalar_pair(query_datum, target_datum)
        } else {
            Err(ExecutionError::Custom(
                "VectorDistance currently supports: (scalar_query, vector_column) or (scalar, scalar)".into()
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use arrow::array::{ArrayRef, Float32Array};
    use minigu_common::data_chunk;
    use minigu_common::value::{F32, ScalarValue};

    use super::*;
    use crate::evaluator::column_ref::ColumnRef;
    use crate::evaluator::constant::Constant;

    fn create_vector_scalar(values: Vec<f32>) -> ScalarValue {
        let ordered_floats: Vec<F32> = values.into_iter().map(F32::from).collect();
        ScalarValue::Vector(Some(ordered_floats))
    }

    fn create_vector_column_data(vectors: Vec<Vec<f32>>) -> ArrayRef {
        use arrow::datatypes::{DataType, Field};

        let vector_dim = vectors.first().map(|v| v.len()).unwrap_or(0);

        let mut all_values = Vec::new();
        for vector in &vectors {
            all_values.extend_from_slice(vector);
        }

        let float_array = Arc::new(Float32Array::from(all_values));
        let field = Arc::new(Field::new("item", DataType::Float32, false));

        Arc::new(FixedSizeListArray::new(
            field,
            vector_dim as i32,
            float_array,
            None,
        ))
    }

    #[test]
    fn test_l2_distance() {
        // Test vectors: [1.0, 2.0] and [4.0, 6.0]
        // Expected L2 distance = sqrt((4-1)^2 + (6-2)^2) = sqrt(9 + 16) = 5.0
        let vec1 = create_vector_scalar(vec![1.0, 2.0]);
        let vec2 = create_vector_scalar(vec![4.0, 6.0]);

        let query_eval = Box::new(Constant::new(vec1));
        let target_eval = Box::new(Constant::new(vec2));

        let evaluator = VectorDistanceEvaluator::new(query_eval, target_eval, VectorMetric::L2);

        let chunk = data_chunk!((Int32, [1]));
        let result = evaluator.evaluate(&chunk).unwrap();

        let result_array = result.into_array();
        let float_result = result_array
            .as_any()
            .downcast_ref::<Float32Array>()
            .unwrap();

        assert_eq!(float_result.len(), 1);
        assert!((float_result.value(0) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_dimension_mismatch_error() {
        let vec1 = create_vector_scalar(vec![1.0, 2.0]);
        let vec2 = create_vector_scalar(vec![4.0, 6.0, 8.0]); // Different dimension

        let query_eval = Box::new(Constant::new(vec1));
        let target_eval = Box::new(Constant::new(vec2));

        let evaluator = VectorDistanceEvaluator::new(query_eval, target_eval, VectorMetric::L2);

        let chunk = data_chunk!((Int32, [1]));
        let result = evaluator.evaluate(&chunk);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dimension mismatch")
        );
    }

    #[test]
    fn test_similarity_search() {
        let query_vec = create_vector_scalar(vec![0.0, 0.0]);
        let query_eval = Box::new(Constant::new(query_vec));

        let target_vectors = create_vector_column_data(vec![
            vec![1.0, 0.0], // distance = 1.0
            vec![0.0, 1.0], // distance = 1.0
            vec![3.0, 4.0], // distance = 5.0
            vec![0.1, 0.2], // distance ≈ 0.224
        ]);

        let chunk = DataChunk::new(vec![target_vectors]);
        let target_eval = Box::new(ColumnRef::new(0));

        let evaluator = VectorDistanceEvaluator::new(query_eval, target_eval, VectorMetric::L2);
        let result = evaluator.evaluate(&chunk).unwrap();

        assert!(!result.is_scalar());

        let distances = result.into_array();
        let float_distances = distances.as_any().downcast_ref::<Float32Array>().unwrap();

        assert_eq!(float_distances.len(), 4);
        assert!((float_distances.value(0) - 1.0).abs() < 0.001);
        assert!((float_distances.value(1) - 1.0).abs() < 0.001);
        assert!((float_distances.value(2) - 5.0).abs() < 0.001);
        assert!((float_distances.value(3) - 0.224).abs() < 0.001);
    }
}
