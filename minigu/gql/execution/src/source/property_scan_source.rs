use std::sync::Arc;

use arrow::array::{
    ArrayRef, BooleanArray, Float32Array, Float64Array, Int8Array, Int16Array, Int32Array,
    Int64Array, StringArray, UInt8Array, UInt16Array, UInt32Array, UInt64Array,
};
use minigu_common::types::{PropertyId, VertexIdArray};
use minigu_common::value::ScalarValue;
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_transaction::{GraphTxnManager, IsolationLevel};

use crate::error::{ExecutionError, ExecutionResult};
use crate::source::VertexPropertySource;

/// Map a vector of scalar values to a arrayref.
///
/// Usage：
/// - `convert_scalar_values_to_array!(values, ScalarValue::Int32, Int32Array, i32)`
/// - `convert_scalar_values_to_array!(values, ScalarValue::Float32, Float32Array, f32, |v|
///   v.into_inner())`
macro_rules! convert_scalar_values_to_array {
    ($values:expr, $variant:path, $array_type:ty, $rust_type:ty) => {{
        let typed_values: Vec<Option<$rust_type>> = $values
            .into_iter()
            .map(|v| match v {
                $variant(val) => val,
                ScalarValue::Null => None,
                _ => None,
            })
            .collect();
        Arc::new(<$array_type>::from(typed_values)) as ArrayRef
    }};
    ($values:expr, $variant:path, $array_type:ty, $rust_type:ty, $transform:expr) => {{
        let typed_values: Vec<Option<$rust_type>> = $values
            .into_iter()
            .map(|v| match v {
                $variant(val) => val.map($transform),
                ScalarValue::Null => None,
                _ => None,
            })
            .collect();
        Arc::new(<$array_type>::from(typed_values)) as ArrayRef
    }};
}

/// Retrieves property values for the given vertex IDs and property IDs,
/// returning them as ArrayRef columns.
/// Each ArrayRef must be length-aligned with the input VertexIdArray.
impl VertexPropertySource for GraphContainer {
    fn scan_vertex_properties(
        &self,
        vertices: &VertexIdArray,
        property_list: &[PropertyId],
    ) -> ExecutionResult<Vec<ArrayRef>> {
        let mem = match self.graph_storage() {
            GraphStorage::Memory(mem) => Arc::clone(mem),
        };
        let txn = mem
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .map_err(|e| ExecutionError::Custom(Box::new(e)))?;

        // If property_list is empty, get all properties from the first vertex
        let property_list = if property_list.is_empty() {
            if let Some(&first_vid) = vertices.values().first() {
                let sample_vertex = mem
                    .get_vertex(&txn, first_vid)
                    .map_err(|e| ExecutionError::Custom(Box::new(e)))?;
                let num_properties = sample_vertex.properties().len();
                (0..num_properties as u32).collect()
            } else {
                // No vertices, return empty list
                Vec::new()
            }
        } else {
            Vec::from(property_list)
        };

        let mut results = Vec::new();

        for prop_id in property_list.iter() {
            let idx = *prop_id as usize;
            let mut values = Vec::new();

            for vid in vertices.values().iter().copied() {
                let v = mem
                    .get_vertex(&txn, vid)
                    .map_err(|e| ExecutionError::Custom(Box::new(e)))?;
                if v.is_tombstone {
                    values.push(ScalarValue::Null);
                } else {
                    let sv = v.properties.get(idx).unwrap_or(&ScalarValue::Null);
                    values.push(sv.clone());
                }
            }

            let sample_value = values
                .iter()
                .find(|v| !matches!(v, ScalarValue::Null))
                .unwrap_or(&ScalarValue::Null);

            let array_ref = match sample_value {
                ScalarValue::Int8(_) => {
                    convert_scalar_values_to_array!(values, ScalarValue::Int8, Int8Array, i8)
                }
                ScalarValue::Int16(_) => {
                    convert_scalar_values_to_array!(values, ScalarValue::Int16, Int16Array, i16)
                }
                ScalarValue::Int32(_) => {
                    convert_scalar_values_to_array!(values, ScalarValue::Int32, Int32Array, i32)
                }
                ScalarValue::Int64(_) => {
                    convert_scalar_values_to_array!(values, ScalarValue::Int64, Int64Array, i64)
                }
                ScalarValue::UInt8(_) => {
                    convert_scalar_values_to_array!(values, ScalarValue::UInt8, UInt8Array, u8)
                }
                ScalarValue::UInt16(_) => {
                    convert_scalar_values_to_array!(values, ScalarValue::UInt16, UInt16Array, u16)
                }
                ScalarValue::UInt32(_) => {
                    convert_scalar_values_to_array!(values, ScalarValue::UInt32, UInt32Array, u32)
                }
                ScalarValue::UInt64(_) => {
                    convert_scalar_values_to_array!(values, ScalarValue::UInt64, UInt64Array, u64)
                }
                ScalarValue::Float32(_) => {
                    convert_scalar_values_to_array!(
                        values,
                        ScalarValue::Float32,
                        Float32Array,
                        f32,
                        |f| f.into_inner()
                    )
                }
                ScalarValue::Float64(_) => {
                    convert_scalar_values_to_array!(
                        values,
                        ScalarValue::Float64,
                        Float64Array,
                        f64,
                        |f| f.into_inner()
                    )
                }
                ScalarValue::Boolean(_) => {
                    convert_scalar_values_to_array!(
                        values,
                        ScalarValue::Boolean,
                        BooleanArray,
                        bool
                    )
                }
                ScalarValue::String(_) => {
                    convert_scalar_values_to_array!(
                        values,
                        ScalarValue::String,
                        StringArray,
                        String
                    )
                }
                ScalarValue::Null => {
                    Arc::new(Int64Array::from(vec![None::<i64>; values.len()])) as ArrayRef
                }
                _ => Arc::new(Int64Array::from(vec![None::<i64>; values.len()])) as ArrayRef,
            };

            results.push(array_ref);
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use arrow::array::{BooleanArray, Float32Array, Float64Array, Int32Array, Int64Array};
    use minigu_catalog::memory::graph_type::MemoryGraphTypeCatalog;
    use minigu_common::types::{LabelId, PropertyId, VertexId, VertexIdArray};
    use minigu_common::value::ScalarValue;
    use minigu_context::graph::{GraphContainer, GraphStorage};
    use minigu_storage::common::{PropertyRecord, Vertex};
    use minigu_storage::tp::MemoryGraph;
    use minigu_transaction::{IsolationLevel, Transaction};

    use super::*;
    use crate::source::VertexPropertySource;

    const PERSON_LABEL_ID: LabelId = LabelId::new(1).unwrap();

    fn create_test_graph_container() -> GraphContainer {
        let graph = MemoryGraph::in_memory();
        let graph_type = Arc::new(MemoryGraphTypeCatalog::new());
        GraphContainer::new(graph_type, GraphStorage::Memory(graph))
    }

    fn create_test_vertices_with_properties(container: &GraphContainer) {
        let mem = match container.graph_storage() {
            GraphStorage::Memory(mem) => Arc::clone(mem),
        };
        let txn = mem
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        // vertex 0: Int32(10), Int64(100), Float32(1.5), Float64(2.5), Boolean(true)
        let vertex0 = Vertex::new(
            VertexId::from(0u64),
            PERSON_LABEL_ID,
            PropertyRecord::new(vec![
                ScalarValue::Int32(Some(10)),
                ScalarValue::Int64(Some(100)),
                ScalarValue::Float32(Some(ordered_float::OrderedFloat(1.5))),
                ScalarValue::Float64(Some(ordered_float::OrderedFloat(2.5))),
                ScalarValue::Boolean(Some(true)),
            ]),
        );
        mem.create_vertex(&txn, vertex0).unwrap();

        // vertex 1: Int32(20), Int64(200), Float32(3.5), Float64(4.5), Boolean(false)
        let vertex1 = Vertex::new(
            VertexId::from(1u64),
            PERSON_LABEL_ID,
            PropertyRecord::new(vec![
                ScalarValue::Int32(Some(20)),
                ScalarValue::Int64(Some(200)),
                ScalarValue::Float32(Some(ordered_float::OrderedFloat(3.5))),
                ScalarValue::Float64(Some(ordered_float::OrderedFloat(4.5))),
                ScalarValue::Boolean(Some(false)),
            ]),
        );
        mem.create_vertex(&txn, vertex1).unwrap();

        // vertex 2: NULL
        let vertex2 = Vertex::new(
            VertexId::from(2u64),
            PERSON_LABEL_ID,
            PropertyRecord::new(vec![
                ScalarValue::Int32(None),
                ScalarValue::Int64(Some(300)),
                ScalarValue::Float32(Some(ordered_float::OrderedFloat(5.5))),
                ScalarValue::Float64(None),
                ScalarValue::Boolean(Some(true)),
            ]),
        );
        mem.create_vertex(&txn, vertex2).unwrap();

        txn.commit().unwrap();
    }

    #[test]
    fn test_scan_int32_properties() {
        let container = create_test_graph_container();
        create_test_vertices_with_properties(&container);

        let vertices = VertexIdArray::from_iter_values([0u64, 1u64, 2u64].iter().copied());
        let property_list = vec![PropertyId::from(0u32)];

        let results = container
            .scan_vertex_properties(&vertices, &property_list)
            .unwrap();
        assert_eq!(results.len(), 1);

        let int32_array = results[0].as_any().downcast_ref::<Int32Array>().unwrap();
        let values: Vec<Option<i32>> = int32_array.iter().collect();
        assert_eq!(values, vec![Some(10), Some(20), None]);
    }

    #[test]
    fn test_scan_int64_properties() {
        let container = create_test_graph_container();
        create_test_vertices_with_properties(&container);
        let vertices = VertexIdArray::from_iter_values([0u64, 1u64, 2u64].iter().copied());
        let property_list = vec![PropertyId::from(1u32)];

        let results = container
            .scan_vertex_properties(&vertices, &property_list)
            .unwrap();
        assert_eq!(results.len(), 1);

        let int64_array = results[0].as_any().downcast_ref::<Int64Array>().unwrap();
        let values: Vec<Option<i64>> = int64_array.iter().collect();
        assert_eq!(values, vec![Some(100), Some(200), Some(300)]);
    }

    #[test]
    fn test_scan_float32_properties() {
        let container = create_test_graph_container();
        create_test_vertices_with_properties(&container);
        let vertices = VertexIdArray::from_iter_values([0u64, 1u64, 2u64].iter().copied());
        let property_list = vec![PropertyId::from(2u32)];

        let results = container
            .scan_vertex_properties(&vertices, &property_list)
            .unwrap();
        assert_eq!(results.len(), 1);

        let float32_array = results[0].as_any().downcast_ref::<Float32Array>().unwrap();
        let values: Vec<Option<f32>> = float32_array.iter().collect();
        assert_eq!(values, vec![Some(1.5), Some(3.5), Some(5.5)]);
    }

    #[test]
    fn test_scan_float64_properties() {
        let container = create_test_graph_container();
        create_test_vertices_with_properties(&container);
        let vertices = VertexIdArray::from_iter_values([0u64, 1u64, 2u64].iter().copied());
        let property_list = vec![PropertyId::from(3u32)];

        let results = container
            .scan_vertex_properties(&vertices, &property_list)
            .unwrap();
        assert_eq!(results.len(), 1);

        let float64_array = results[0].as_any().downcast_ref::<Float64Array>().unwrap();
        let values: Vec<Option<f64>> = float64_array.iter().collect();
        assert_eq!(values, vec![Some(2.5), Some(4.5), None]);
    }

    #[test]
    fn test_scan_boolean_properties() {
        let container = create_test_graph_container();
        create_test_vertices_with_properties(&container);
        let vertices = VertexIdArray::from_iter_values([0u64, 1u64, 2u64].iter().copied());
        let property_list = vec![PropertyId::from(4u32)];

        let results = container
            .scan_vertex_properties(&vertices, &property_list)
            .unwrap();
        assert_eq!(results.len(), 1);

        let boolean_array = results[0].as_any().downcast_ref::<BooleanArray>().unwrap();
        let values: Vec<Option<bool>> = boolean_array.iter().collect();
        assert_eq!(values, vec![Some(true), Some(false), Some(true)]);
    }

    #[test]
    fn test_scan_multiple_properties() {
        let container = create_test_graph_container();
        create_test_vertices_with_properties(&container);
        let vertices = VertexIdArray::from_iter_values([0u64, 1u64].iter().copied());
        let property_list = vec![
            PropertyId::from(0u32),
            PropertyId::from(1u32),
            PropertyId::from(4u32),
        ];

        let results = container
            .scan_vertex_properties(&vertices, &property_list)
            .unwrap();
        assert_eq!(results.len(), 3);

        let int32_array = results[0].as_any().downcast_ref::<Int32Array>().unwrap();
        let int32_values: Vec<Option<i32>> = int32_array.iter().collect();
        assert_eq!(int32_values, vec![Some(10), Some(20)]);

        let int64_array = results[1].as_any().downcast_ref::<Int64Array>().unwrap();
        let int64_values: Vec<Option<i64>> = int64_array.iter().collect();
        assert_eq!(int64_values, vec![Some(100), Some(200)]);

        let boolean_array = results[2].as_any().downcast_ref::<BooleanArray>().unwrap();
        let boolean_values: Vec<Option<bool>> = boolean_array.iter().collect();
        assert_eq!(boolean_values, vec![Some(true), Some(false)]);
    }

    #[test]
    fn test_scan_with_null_values() {
        let container = create_test_graph_container();
        create_test_vertices_with_properties(&container);
        let vertices = VertexIdArray::from_iter_values([0u64, 2u64].iter().copied());
        let property_list = vec![PropertyId::from(0u32)];
        let results = container
            .scan_vertex_properties(&vertices, &property_list)
            .unwrap();
        assert_eq!(results.len(), 1);

        let int32_array = results[0].as_any().downcast_ref::<Int32Array>().unwrap();
        let values: Vec<Option<i32>> = int32_array.iter().collect();
        assert_eq!(values, vec![Some(10), None]);
    }

    #[test]
    fn test_scan_single_vertex() {
        let container = create_test_graph_container();
        create_test_vertices_with_properties(&container);
        let vertices = VertexIdArray::from_iter_values([1u64].iter().copied());
        let property_list = vec![PropertyId::from(0u32), PropertyId::from(1u32)];

        let results = container
            .scan_vertex_properties(&vertices, &property_list)
            .unwrap();
        assert_eq!(results.len(), 2);
        let int32_array = results[0].as_any().downcast_ref::<Int32Array>().unwrap();
        let int32_values: Vec<Option<i32>> = int32_array.iter().collect();
        assert_eq!(int32_values, vec![Some(20)]);
        let int64_array = results[1].as_any().downcast_ref::<Int64Array>().unwrap();
        let int64_values: Vec<Option<i64>> = int64_array.iter().collect();
        assert_eq!(int64_values, vec![Some(200)]);
    }

    #[test]
    fn test_scan_empty_vertex_list() {
        let container = create_test_graph_container();
        create_test_vertices_with_properties(&container);
        let vertices = VertexIdArray::from_iter_values(std::iter::empty::<u64>());
        let property_list = vec![PropertyId::from(0u32)];

        let results = container
            .scan_vertex_properties(&vertices, &property_list)
            .unwrap();
        assert_eq!(results.len(), 1);

        let array = &results[0];
        assert_eq!(array.len(), 0);
    }
}
