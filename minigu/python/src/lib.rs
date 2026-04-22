//! Python bindings for miniGU graph database
//!
//! This module provides Python bindings for the miniGU graph database using PyO3.

use std::path::Path;

use arrow::array::*;
use arrow::datatypes::DataType;
use minigu::common::data_chunk::DataChunk;
use minigu::database::{Database, DatabaseConfig};
use minigu::session::Session;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString};

// Define custom exception type checking functions
#[pyfunction]
fn is_syntax_error(e: &Bound<PyAny>) -> PyResult<bool> {
    let error_str: String = e.str()?.extract()?;
    Ok(error_str.to_lowercase().contains("syntax")
        || error_str.to_lowercase().contains("unexpected"))
}

#[pyfunction]
fn is_timeout_error(e: &Bound<PyAny>) -> PyResult<bool> {
    let error_str: String = e.str()?.extract()?;
    Ok(error_str.to_lowercase().contains("timeout"))
}

#[pyfunction]
fn is_transaction_error(e: &Bound<PyAny>) -> PyResult<bool> {
    let error_str: String = e.str()?.extract()?;
    let error_lower = error_str.to_lowercase();
    Ok(error_lower.contains("transaction")
        || error_lower.contains("txn")
        || error_lower.contains("commit")
        || error_lower.contains("rollback"))
}

#[pyfunction]
fn is_not_implemented_error(e: &Bound<PyAny>) -> PyResult<bool> {
    let error_str: String = e.str()?.extract()?;
    let error_lower = error_str.to_lowercase();
    Ok(error_lower.contains("not implemented") || error_lower.contains("not yet implemented"))
}

// Helper function to sanitize file paths
fn sanitize_file_path(path: &str) -> String {
    path.replace(['\'', '"', ';', '\n', '\r'], "")
        .replace("..", "")
}

// Helper function to sanitize graph names
fn sanitize_graph_name(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

/// PyMiniGU class that wraps the Rust Database
#[pyclass]
#[allow(clippy::upper_case_acronyms)]
pub struct PyMiniGU {
    database: Option<Database>,
    session: Option<Session>,
    current_graph: Option<String>,
}

#[pymethods]
impl PyMiniGU {
    /// Create a new PyMiniGU instance
    #[new]
    fn new() -> PyResult<Self> {
        Ok(PyMiniGU {
            database: None,
            session: None,
            current_graph: None,
        })
    }

    /// Initialize the database
    fn init(&mut self) -> PyResult<()> {
        let config = DatabaseConfig::default();
        let db = Database::open_in_memory(config).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                "Failed to initialize database: {}",
                e
            ))
        })?;
        let session = db.session().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                "Failed to create session: {}",
                e
            ))
        })?;

        self.database = Some(db);
        self.session = Some(session);
        self.current_graph = None;
        Ok(())
    }

    /// Check if the database is connected
    #[getter]
    fn is_connected(&self) -> bool {
        self.session.is_some()
    }

    /// Execute a GQL query
    fn execute(&mut self, query_str: &str, py: Python) -> PyResult<PyObject> {
        let session = self.session.as_mut().expect("Session not initialized");

        let query_result = session.query(query_str).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyException, _>(format!("Query execution failed: {}", e))
        })?;

        // Convert QueryResult to Python dict
        let dict = PyDict::new(py);

        // Convert schema
        let schema_list = PyList::empty(py);
        if let Some(schema_ref) = query_result.schema() {
            for field in schema_ref.fields() {
                let field_dict = PyDict::new(py);
                field_dict.set_item("name", field.name())?;
                field_dict.set_item("data_type", format!("{:?}", field.ty()))?;
                schema_list.append(field_dict)?;
            }
        }
        dict.set_item("schema", schema_list)?;

        // Convert data
        let data_list = PyList::empty(py);
        for chunk in query_result.iter() {
            let chunk_data = convert_data_chunk(chunk)?;
            for row in chunk_data {
                let row_list = PyList::empty(py);
                for value in row {
                    row_list.append(value)?;
                }
                data_list.append(row_list)?;
            }
        }
        dict.set_item("data", data_list)?;

        // Convert metrics
        let metrics = query_result.metrics();
        let metrics_dict = PyDict::new(py);
        metrics_dict.set_item("parsing_time_ms", metrics.parsing_time().as_millis() as f64)?;
        metrics_dict.set_item("planning_time_ms", metrics.planning_time().as_millis() as f64)?;
        metrics_dict.set_item("execution_time_ms", metrics.execution_time().as_millis() as f64)?;
        dict.set_item("metrics", metrics_dict)?;

        Ok(dict.into())
    }

    /// Create a new graph with optional test data
    fn create_graph(&mut self, graph_name: &str, num_vertices: Option<i64>) -> PyResult<bool> {
        let session = self.session.as_mut().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyException, _>("Session not initialized")
        })?;

        if graph_name.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyException, _>(
                "Graph name cannot be empty",
            ));
        }

        let sanitized_name = sanitize_graph_name(graph_name);
        if sanitized_name.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyException, _>(
                "Graph name contains only invalid characters",
            ));
        }

        // Create graph with test data if num_vertices is specified, otherwise create empty graph
        let query = if let Some(n) = num_vertices {
            format!("CALL create_test_graph_data('{}', {})", sanitized_name, n)
        } else {
            format!("CALL create_test_graph('{}')", sanitized_name)
        };

        session.query(&query).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                "Failed to create graph '{}': {}",
                sanitized_name, e
            ))
        })?;

        // Set the current graph
        let set_graph_query = format!("session set graph {}", sanitized_name);
        session.query(&set_graph_query).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                "Failed to set current graph '{}': {}",
                sanitized_name, e
            ))
        })?;

        self.current_graph = Some(sanitized_name);
        Ok(true)
    }

    /// Load data from a list of dictionaries
    fn load(&mut self, data: &Bound<'_, PyAny>) -> PyResult<bool> {
        let session = self.session.as_mut().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyException, _>("Session not initialized")
        })?;

        // Check if data is a string (file path) or a list
        if let Ok(path) = data.extract::<String>() {
            // Load from file
            let path_obj = Path::new(&path);
            if !path_obj.exists() {
                return Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                    "File not found: {}",
                    path
                )));
            }
            let sanitized_path = sanitize_file_path(&path);
            let graph_name = self.current_graph.as_deref().unwrap_or("default_graph");
            let query = format!(
                "CALL import('{}', '{}', 'manifest.json')",
                graph_name, sanitized_path
            );
            session.query(&query).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                    "Failed to load data from file: {}",
                    e
                ))
            })?;
            return Ok(true);
        }

        // Load from list of dictionaries
        let list = data.downcast::<PyList>().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyException, _>("Expected a list of dictionaries or a file path")
        })?;

        let _graph_name = self.current_graph.as_deref().unwrap_or("default_graph");

        for (index, item) in list.iter().enumerate() {
            let dict = item.downcast::<PyDict>().map_err(|_| {
                PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                    "Item {} is not a dictionary",
                    index
                ))
            })?;

            let mut label = "Node".to_string();
            let mut properties = Vec::new();

            for (key, value) in dict.iter() {
                let key_str = key.downcast::<PyString>()?.to_string();

                if key_str.is_empty() {
                    return Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                        "Empty key found in item {}",
                        index
                    )));
                }

                let value_str = value.str()?.to_string();

                if key_str == "label" {
                    label = value_str;
                } else if let Ok(int_val) = value_str.parse::<i64>() {
                    properties.push(format!("{}: {}", key_str, int_val));
                } else if let Ok(float_val) = value_str.parse::<f64>() {
                    properties.push(format!("{}: {}", key_str, float_val));
                } else if value_str.eq_ignore_ascii_case("true") {
                    properties.push(format!("{}: true", key_str));
                } else if value_str.eq_ignore_ascii_case("false") {
                    properties.push(format!("{}: false", key_str));
                } else {
                    let escaped_value = value_str.replace('\'', "\\'");
                    properties.push(format!("{}: '{}'", key_str, escaped_value));
                }
            }

            let statement = if !properties.is_empty() {
                let props_str = properties.join(", ");
                format!("INSERT (n:{} {{ {} }});", label, props_str)
            } else {
                format!("INSERT (n:{});", label)
            };
            session.query(&statement).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                    "Failed to insert data: {}",
                    e
                ))
            })?;
        }

        Ok(true)
    }

    /// Save database to a file
    fn save(&mut self, file_path: &str) -> PyResult<bool> {
        let session = self.session.as_mut().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyException, _>("Session not initialized")
        })?;

        let graph_name = self.current_graph.as_deref().unwrap_or("default_graph");
        let sanitized_path = sanitize_file_path(file_path);

        let query = format!(
            "CALL export('{}', '{}', 'manifest.json')",
            graph_name, sanitized_path
        );
        session.query(&query).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyException, _>(format!("Export failed: {}", e))
        })?;

        Ok(true)
    }

    /// Close the database connection
    fn close(&mut self) -> PyResult<()> {
        self.database = None;
        self.session = None;
        self.current_graph = None;
        Ok(())
    }
}

/// Extract a value from an Arrow array at a specific index
fn extract_value_from_array(array: &ArrayRef, index: usize) -> PyResult<PyObject> {
    Python::with_gil(|py| match array.data_type() {
        DataType::Int8 => {
            let arr = array.as_any().downcast_ref::<Int8Array>().unwrap();
            if arr.is_null(index) {
                Ok(py.None())
            } else {
                Ok(arr.value(index).into_pyobject(py)?.into_any().unbind())
            }
        }
        DataType::Int16 => {
            let arr = array.as_any().downcast_ref::<Int16Array>().unwrap();
            if arr.is_null(index) {
                Ok(py.None())
            } else {
                Ok(arr.value(index).into_pyobject(py)?.into_any().unbind())
            }
        }
        DataType::Int32 => {
            let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
            if arr.is_null(index) {
                Ok(py.None())
            } else {
                Ok(arr.value(index).into_pyobject(py)?.into_any().unbind())
            }
        }
        DataType::Int64 => {
            let arr = array.as_any().downcast_ref::<Int64Array>().unwrap();
            if arr.is_null(index) {
                Ok(py.None())
            } else {
                Ok(arr.value(index).into_pyobject(py)?.into_any().unbind())
            }
        }
        DataType::UInt8 => {
            let arr = array.as_any().downcast_ref::<UInt8Array>().unwrap();
            if arr.is_null(index) {
                Ok(py.None())
            } else {
                Ok(arr.value(index).into_pyobject(py)?.into_any().unbind())
            }
        }
        DataType::UInt16 => {
            let arr = array.as_any().downcast_ref::<UInt16Array>().unwrap();
            if arr.is_null(index) {
                Ok(py.None())
            } else {
                Ok(arr.value(index).into_pyobject(py)?.into_any().unbind())
            }
        }
        DataType::UInt32 => {
            let arr = array.as_any().downcast_ref::<UInt32Array>().unwrap();
            if arr.is_null(index) {
                Ok(py.None())
            } else {
                Ok(arr.value(index).into_pyobject(py)?.into_any().unbind())
            }
        }
        DataType::UInt64 => {
            let arr = array.as_any().downcast_ref::<UInt64Array>().unwrap();
            if arr.is_null(index) {
                Ok(py.None())
            } else {
                Ok(arr.value(index).into_pyobject(py)?.into_any().unbind())
            }
        }
        DataType::Float32 => {
            let arr = array.as_any().downcast_ref::<Float32Array>().unwrap();
            if arr.is_null(index) {
                Ok(py.None())
            } else {
                Ok(arr.value(index).into_pyobject(py)?.into_any().unbind())
            }
        }
        DataType::Float64 => {
            let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
            if arr.is_null(index) {
                Ok(py.None())
            } else {
                Ok(arr.value(index).into_pyobject(py)?.into_any().unbind())
            }
        }
        DataType::Utf8 | DataType::LargeUtf8 => {
            let arr = array.as_any().downcast_ref::<StringArray>().unwrap();
            if arr.is_null(index) {
                Ok(py.None())
            } else {
                Ok(arr.value(index).into_pyobject(py)?.into_any().unbind())
            }
        }
        DataType::Boolean => {
            let arr = array.as_any().downcast_ref::<BooleanArray>().unwrap();
            if arr.is_null(index) {
                Ok(py.None())
            } else {
                let bound = arr.value(index).into_pyobject(py)?;
                Ok(<pyo3::Bound<'_, pyo3::types::PyBool> as Clone>::clone(&bound).into_any().unbind())
            }
        }
        _ => Ok(py.None()),
    })
}

/// Convert a DataChunk to a Python list of lists
fn convert_data_chunk(chunk: &DataChunk) -> PyResult<Vec<Vec<PyObject>>> {
    let mut result = Vec::new();
    let num_rows = chunk.len();

    for row_idx in 0..num_rows {
        let mut row_vec = Vec::new();
        for col in chunk.columns() {
            let value = extract_value_from_array(col, row_idx)?;
            row_vec.push(value);
        }
        result.push(row_vec);
    }

    Ok(result)
}

/// Python module definition
#[pymodule]
fn minigu_python(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyMiniGU>()?;
    m.add_function(wrap_pyfunction!(is_syntax_error, m)?)?;
    m.add_function(wrap_pyfunction!(is_timeout_error, m)?)?;
    m.add_function(wrap_pyfunction!(is_transaction_error, m)?)?;
    m.add_function(wrap_pyfunction!(is_not_implemented_error, m)?)?;
    Ok(())
}