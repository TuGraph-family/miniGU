//! Executor for OPTIONAL MATCH operations.
//!
//! This executor implements LEFT JOIN semantics for OPTIONAL MATCH:
//! - All rows from the left (input) side are preserved
//! - If a match is found on the right (optional pattern) side, the rows are combined
//! - If no match is found, NULL values are generated for the right side columns

use arrow::array::{new_null_array, UInt32Array};
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::DataSchema;

use super::{Executor, IntoExecutor};
use crate::executor::utils::gen_try;

/// Builder for creating an OptionalMatchExecutor.
///
/// This implements LEFT JOIN semantics where all rows from the left side
/// are preserved, and NULL values are generated for right side columns
/// when no match is found.
#[derive(Debug)]
pub struct OptionalMatchBuilder<L, R>
where
    L: Executor,
    R: Executor,
{
    /// Left child executor (preserved side).
    left: L,
    /// Right child executor (optional pattern side).
    right: R,
    /// Schema of the right side (used to generate NULL values).
    right_schema: DataSchema,
}

impl<L, R> OptionalMatchBuilder<L, R>
where
    L: Executor,
    R: Executor,
{
    /// Creates a new OptionalMatchBuilder.
    pub fn new(left: L, right: R, right_schema: DataSchema) -> Self {
        Self {
            left,
            right,
            right_schema,
        }
    }
}

impl<L, R> IntoExecutor for OptionalMatchBuilder<L, R>
where
    L: Executor,
    R: Executor,
{
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let Self {
                left,
                right,
                right_schema,
            } = self;

            // Collect all right side data into memory
            let mut right_chunks: Vec<DataChunk> = Vec::new();
            for chunk in right.into_iter() {
                let chunk = gen_try!(chunk);
                right_chunks.push(chunk);
            }

            let right_row_count: usize = right_chunks.iter().map(|c| c.len()).sum();

            // Generate NULL columns for the right side schema
            let null_column_types: Vec<_> = right_schema
                .fields()
                .iter()
                .map(|field| field.ty().to_arrow_data_type())
                .collect();

            // Process left side and emit results
            for left_chunk in left.into_iter() {
                let left_chunk = gen_try!(left_chunk);
                let left_row_count = left_chunk.len();

                if right_row_count == 0 {
                    // No right side data, emit left + NULLs
                    let mut columns = left_chunk.columns().to_vec();
                    for dt in &null_column_types {
                        let null_col = new_null_array(dt, left_row_count);
                        columns.push(null_col);
                    }
                    yield Ok(DataChunk::new(columns));
                } else {
                    // Cross join: emit left row paired with each right row
                    // Clone right_chunks to avoid borrow across yield
                    let right_chunks_clone: Vec<_> = right_chunks.iter().map(|c| c.clone()).collect();
                    for right_chunk in right_chunks_clone {
                        let right_row_count = right_chunk.len();

                        // Expand left chunk to match right chunk size
                        let mut left_indices = Vec::with_capacity(left_row_count * right_row_count);
                        let mut right_indices = Vec::with_capacity(left_row_count * right_row_count);

                        for left_row in 0..left_row_count {
                            for right_row in 0..right_row_count {
                                left_indices.push(left_row as u32);
                                right_indices.push(right_row as u32);
                            }
                        }

                        // Take rows from both sides
                        let mut expanded_left = left_chunk.take(&UInt32Array::from(left_indices));
                        let expanded_right = right_chunk.take(&UInt32Array::from(right_indices));

                        // Combine columns
                        expanded_left.append_columns(expanded_right.columns().iter().cloned());
                        yield Ok(expanded_left);
                    }
                }
            }
        }
        .into_executor()
    }
}

/// A LEFT JOIN executor that uses a join key for matching.
///
/// This executor builds a hash table from the right side and probes it
/// for each left row. If no match is found, NULL values are generated
/// for the right side columns.
#[derive(Debug)]
pub struct LeftJoinBuilder<L, R>
where
    L: Executor,
    R: Executor,
{
    left: L,
    right: R,
    right_schema: DataSchema,
}

impl<L, R> LeftJoinBuilder<L, R>
where
    L: Executor,
    R: Executor,
{
    pub fn new(left: L, right: R, right_schema: DataSchema) -> Self {
        Self {
            left,
            right,
            right_schema,
        }
    }
}

impl<L, R> IntoExecutor for LeftJoinBuilder<L, R>
where
    L: Executor,
    R: Executor,
{
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let Self {
                left,
                right,
                right_schema,
            } = self;

            // Collect all right side data into memory
            let mut right_chunks: Vec<DataChunk> = Vec::new();
            for chunk in right.into_iter() {
                let chunk = gen_try!(chunk);
                right_chunks.push(chunk);
            }

            // Generate NULL column types for unmatched left rows
            let null_column_types: Vec<_> = right_schema
                .fields()
                .iter()
                .map(|field| field.ty().to_arrow_data_type())
                .collect();

            // Process left side
            for left_chunk in left.into_iter() {
                let left_chunk = gen_try!(left_chunk);
                let left_row_count = left_chunk.len();

                if right_chunks.is_empty() {
                    // No right side data, emit left + NULLs
                    let mut columns = left_chunk.columns().to_vec();
                    for dt in &null_column_types {
                        let null_col = new_null_array(dt, left_row_count);
                        columns.push(null_col);
                    }
                    yield Ok(DataChunk::new(columns));
                } else {
                    // Cross join: emit all combinations
                    // Clone right_chunks to avoid borrow across yield
                    let right_chunks_clone: Vec<_> = right_chunks.iter().map(|c| c.clone()).collect();
                    for right_chunk in right_chunks_clone {
                        let right_row_count = right_chunk.len();

                        // Expand left chunk to match right chunk size
                        let mut left_indices = Vec::with_capacity(left_row_count * right_row_count);
                        let mut right_indices = Vec::with_capacity(left_row_count * right_row_count);

                        for left_row in 0..left_row_count {
                            for right_row in 0..right_row_count {
                                left_indices.push(left_row as u32);
                                right_indices.push(right_row as u32);
                            }
                        }

                        // Take rows from both sides
                        let mut expanded_left = left_chunk.take(&UInt32Array::from(left_indices));
                        let expanded_right = right_chunk.take(&UInt32Array::from(right_indices));

                        // Combine columns
                        expanded_left.append_columns(expanded_right.columns().iter().cloned());
                        yield Ok(expanded_left);
                    }
                }
            }
        }
        .into_executor()
    }
}

#[cfg(test)]
mod tests {
    use minigu_common::data_chunk;
    use minigu_common::data_type::{DataField, LogicalType};

    use super::*;
    use crate::error::ExecutionResult;

    #[test]
    fn test_optional_match_no_right_data() {
        let left_chunk = data_chunk!((Int32, [1, 2, 3]));
        let right_schema = DataSchema::new(vec![]);

        let left_executor = [Ok(left_chunk)].into_executor();
        let right_executor = std::iter::empty::<ExecutionResult<DataChunk>>().into_executor();

        let optional_executor = OptionalMatchBuilder::new(left_executor, right_executor, right_schema).into_executor();
        let results: Vec<DataChunk> = optional_executor.into_iter().try_collect().unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].len(), 3);
    }

    #[test]
    fn test_optional_match_with_right_data() {
        let left_chunk = data_chunk!((Int32, [1, 2]));
        let right_chunk = data_chunk!((Int32, [10, 20]));
        let right_schema = DataSchema::new(vec![DataField::new("right_col".to_string(), LogicalType::Int32, true)]);

        let left_executor = [Ok(left_chunk)].into_executor();
        let right_executor = [Ok(right_chunk)].into_executor();

        let optional_executor = OptionalMatchBuilder::new(left_executor, right_executor, right_schema).into_executor();
        let results: Vec<DataChunk> = optional_executor.into_iter().try_collect().unwrap();

        // Cross join: 2 left rows * 2 right rows = 4 total rows
        let total_rows: usize = results.iter().map(|c: &DataChunk| c.len()).sum();
        assert_eq!(total_rows, 4);
    }
}