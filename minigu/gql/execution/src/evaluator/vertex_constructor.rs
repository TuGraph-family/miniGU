use std::sync::Arc;

use arrow::array::{ArrayRef, AsArray, StructArray, UInt32Array, UInt64Array};
use arrow::datatypes::Fields;
use minigu_common::constants::{LABEL_FIELD_NAME, VID_FIELD_NAME};
use minigu_common::data_chunk::DataChunk;
use minigu_common::types::{LabelId, VertexIdArray};

use super::{DatumRef, Evaluator};
use crate::error::{ExecutionError, ExecutionResult};

/// An evaluator that constructs a Vertex struct from vertex ID and property columns.
/// This evaluator:
/// 1. Takes a vertex ID column (VertexIdArray)
/// 2. Retrieves label_id
/// 3. Takes property columns (already scanned)
/// 4. Constructs a StructArray with vid, label_id, and its properties
#[derive(Debug)]
pub struct VertexConstructor {
    /// Index of the vertex ID column
    vid_column_index: usize,
    /// Indices of property columns
    property_column_indices: Vec<usize>,
    /// Property names corresponding to property columns
    property_names: Vec<String>,
    /// Label specifications from schema (if available)
    label_specs: Option<Vec<Vec<LabelId>>>,
}

impl VertexConstructor {
    pub fn new(
        vid_column_index: usize,
        property_column_indices: Vec<usize>,
        property_names: Vec<String>,
        label_specs: Option<Vec<Vec<LabelId>>>,
    ) -> Self {
        Self {
            vid_column_index,
            property_column_indices,
            property_names,
            label_specs,
        }
    }
}

impl Evaluator for VertexConstructor {
    fn evaluate(&self, chunk: &DataChunk) -> ExecutionResult<DatumRef> {
        // Get the vertex ID column
        let vid_column = chunk.columns().get(self.vid_column_index).ok_or_else(|| {
            ExecutionError::Custom(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "Vertex ID column at index {} not found",
                    self.vid_column_index
                ),
            )))
        })?;

        let vid_array: &VertexIdArray = vid_column.as_primitive();
        let len = vid_array.len();

        // Get property columns
        let mut property_arrays = Vec::new();
        for &prop_idx in &self.property_column_indices {
            let prop_column = chunk.columns().get(prop_idx).ok_or_else(|| {
                ExecutionError::Custom(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Property column at index {} not found", prop_idx),
                )))
            })?;
            property_arrays.push(prop_column.clone());
        }

        // TODO: Get label set from catalog if the label_specs is none.
        let label_id = if let Some(label_specs) = &self.label_specs {
            if let Some(first_label_set) = label_specs.first() {
                if let Some(&id) = first_label_set.first() {
                    u32::from(id)
                } else {
                    0
                }
            } else {
                0
            }
        } else {
            0
        };

        let label_array: ArrayRef = Arc::new(UInt32Array::from_iter_values(
            std::iter::repeat(label_id).take(len),
        ));

        let mut struct_arrays = Vec::new();

        let vid_values: Vec<u64> = vid_array.values().iter().copied().collect();

        let vid_u64_array: ArrayRef = Arc::new(UInt64Array::from_iter_values(vid_values));
        struct_arrays.push(vid_u64_array);

        struct_arrays.push(label_array);

        // Create StructArray
        // Build field definitions matching the LogicalType::Vertex structure
        // [vid (UInt64), label_id (UInt32), ...properties]
        let mut struct_fields = vec![
            Arc::new(arrow::datatypes::Field::new(
                VID_FIELD_NAME.to_string(),
                arrow::datatypes::DataType::UInt64,
                false,
            )),
            Arc::new(arrow::datatypes::Field::new(
                LABEL_FIELD_NAME.to_string(),
                arrow::datatypes::DataType::UInt32,
                false,
            )),
        ];

        for (idx, prop_array) in property_arrays.iter().enumerate() {
            let prop_name = if idx < self.property_names.len() {
                self.property_names[idx].clone()
            } else {
                format!("prop_{}", idx)
            };
            let arrow_field =
                arrow::datatypes::Field::new(prop_name, prop_array.data_type().clone(), true);
            struct_fields.push(Arc::new(arrow_field));
        }

        struct_arrays.extend(property_arrays);

        let struct_array = StructArray::new(Fields::from(struct_fields), struct_arrays, None);

        Ok(DatumRef::new(Arc::new(struct_array), false))
    }
}
