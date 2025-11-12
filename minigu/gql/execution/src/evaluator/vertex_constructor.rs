use std::sync::Arc;

use arrow::array::{ArrayRef, AsArray, StructArray, UInt32Array, UInt64Array};
use arrow::datatypes::Fields;
use minigu_common::constants::{LABEL_FIELD_NAME, VID_FIELD_NAME};
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::LogicalType;
use minigu_common::types::{LabelId, VertexId, VertexIdArray};
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_storage::tp::MemoryGraph;
use minigu_transaction::{GraphTxnManager, IsolationLevel};

use super::{DatumRef, Evaluator};
use crate::error::{ExecutionError, ExecutionResult};

/// An evaluator that constructs a Vertex struct from vertex ID and property columns.
/// 
/// This evaluator:
/// 1. Takes a vertex ID column (VertexIdArray)
/// 2. Retrieves label_id from storage for each vertex
/// 3. Takes property columns (already scanned)
/// 4. Constructs a StructArray with vid, label_id, and properties
#[derive(Debug)]
pub struct VertexConstructor {
    /// Index of the vertex ID column
    vid_column_index: usize,
    /// Indices of property columns (in order)
    property_column_indices: Vec<usize>,
    /// Property names (in order, matching property_column_indices)
    property_names: Vec<String>,
    /// Graph container for retrieving vertex labels
    graph_container: Arc<GraphContainer>,
    /// Vertex type fields (for constructing the struct)
    vertex_fields: Vec<(String, LogicalType)>,
}

impl VertexConstructor {
    pub fn new(
        vid_column_index: usize,
        property_column_indices: Vec<usize>,
        property_names: Vec<String>,
        graph_container: Arc<GraphContainer>,
        vertex_fields: Vec<(String, LogicalType)>,
    ) -> Self {
        Self {
            vid_column_index,
            property_column_indices,
            property_names,
            graph_container,
            vertex_fields,
        }
    }
}

impl Evaluator for VertexConstructor {
    fn evaluate(&self, chunk: &DataChunk) -> ExecutionResult<DatumRef> {
        // Get the vertex ID column
        let vid_column = chunk
            .columns()
            .get(self.vid_column_index)
            .ok_or_else(|| ExecutionError::Custom(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Vertex ID column at index {} not found", self.vid_column_index),
            ))))?;
        
        let vid_array: &VertexIdArray = vid_column.as_primitive();
        let len = vid_array.len();

        // Get property columns
        let mut property_arrays = Vec::new();
        for &prop_idx in &self.property_column_indices {
            let prop_column = chunk
                .columns()
                .get(prop_idx)
                .ok_or_else(|| ExecutionError::Custom(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Property column at index {} not found", prop_idx),
                ))))?;
            property_arrays.push(prop_column.clone());
        }

        // Retrieve label_id for each vertex
        let mem = match self.graph_container.graph_storage() {
            GraphStorage::Memory(m) => Arc::clone(m),
        };
        let txn = mem
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .map_err(|e| ExecutionError::Custom(Box::new(e)))?;

        let mut label_ids = Vec::with_capacity(len);
        for vid in vid_array.values().iter().copied() {
            let vertex = mem
                .get_vertex(&txn, vid)
                .map_err(|e| ExecutionError::Custom(Box::new(e)))?;
            label_ids.push(u32::from(vertex.label_id));
        }

        // Create label_id array
        let label_array: ArrayRef = Arc::new(UInt32Array::from_iter_values(label_ids.iter().copied()));

        // Build struct fields: [vid, label_id, ...properties]
        let mut struct_arrays = Vec::new();
        
        // Add vid field (convert from VertexIdArray to UInt64Array)
        let vid_values: Vec<u64> = vid_array.values().iter().copied().collect();
        let vid_u64_array: ArrayRef = Arc::new(UInt64Array::from_iter_values(vid_values));
        struct_arrays.push(vid_u64_array);
        
        // Add label_id field
        struct_arrays.push(label_array);
        
        // Add property fields
        struct_arrays.extend(property_arrays);

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
        
        // Add property fields
        for (name, logical_type) in &self.vertex_fields {
            let arrow_field = arrow::datatypes::Field::new(
                name.clone(),
                logical_type.to_arrow_data_type(),
                true, // properties can be null
            );
            struct_fields.push(Arc::new(arrow_field));
        }

        let struct_array = StructArray::new(
            Fields::from(struct_fields),
            struct_arrays,
            None, // null buffer - all rows are valid
        );

        Ok(DatumRef::new(Arc::new(struct_array), false))
    }
}

