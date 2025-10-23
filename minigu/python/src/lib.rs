//! Python bindings for miniGU graph database
//!
//! This module provides Python bindings for the miniGU graph database using PyO3.

use std::path::Path;

use arrow::array::*;
use arrow::datatypes::DataType;
use minigu::database::{Database, DatabaseConfig};
use minigu::session::Session;
use minigu_common::data_chunk::DataChunk;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyList, PyString};

/// PyMiniGU class that wraps the Rust Database
#[pyclass]
#[allow(clippy::upper_case_acronyms)]
pub struct PyMiniGU {
    database: Option<Database>,
    session: Option<Session>,
    current_graph: Option<String>, // Track current graph name
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
        let db = Database::open_in_memory(&config).map_err(|e| {
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
        Ok(())
    }

    /// Execute a GQL query
    fn execute(&mut self, query_str: &str, py: Python) -> PyResult<PyObject> {
        // Get the session
        let session = self.session.as_mut().expect("Session not initialized");

        // Execute the query
        let query_result = session.query(query_str).expect("Query execution failed");

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
            // Convert DataChunk to Python list of lists
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
        metrics_dict.set_item(
            "planning_time_ms",
            metrics.planning_time().as_millis() as f64,
        )?;
        metrics_dict.set_item(
            "execution_time_ms",
            metrics.execution_time().as_millis() as f64,
        )?;

        dict.set_item("metrics", metrics_dict)?;

        Ok(dict.into())
    }

    /// Load data from a file
    fn load_from_file(&mut self, file_path: &str) -> PyResult<()> {
        // Get the session
        let session = self.session.as_mut().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyException, _>("Session not initialized")
        })?;

        // Validate file path
        let path_obj = Path::new(file_path);
        if !path_obj.exists() {
            return Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                "File not found: {}",
                file_path
            )));
        }

        // Use current graph or default to "default_graph"
        let graph_name = self.current_graph.as_deref().unwrap_or("default_graph");

        // Sanitize the path to prevent injection attacks
        let sanitized_path = file_path.replace(['\'', '"', ';', '\n', '\r'], "");

        // Execute the import procedure with correct syntax (no semicolon)
        let query = format!(
            "CALL import('{}', '{}', 'manifest.json')",
            graph_name, sanitized_path
        );
        match session.query(&query) {
            Ok(_) => {
                println!("Data loaded successfully from: {}", file_path);
                Ok(())
            }
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                "Failed to load data from file: {}",
                e
            ))),
        }
    }

    /// Load data directly with batch support
    fn load_data(&mut self, data: &Bound<'_, PyAny>) -> PyResult<()> {
        // Get the session
        let session = self.session.as_mut().expect("Session not initialized");

        // Convert Python data to Rust data structures
        let list = data.downcast::<PyList>().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyException, _>("Expected a list of dictionaries")
        })?;

        println!("Loading {} records", list.len());

        // Use current graph or default to "default_graph"
        let graph_name = self.current_graph.as_deref().unwrap_or("default_graph");

        // Process data in batches for better performance
        const BATCH_SIZE: usize = 1000;
        let mut batch_statements = Vec::new();
        let mut current_batch = Vec::new();

        for (index, item) in list.iter().enumerate() {
            let dict = item.downcast::<PyDict>().map_err(|_| {
                PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                    "Expected a list of dictionaries, but item {} is not a dictionary",
                    index
                ))
            })?;

            // Extract label and properties
            let mut label = "Node".to_string();
            let mut properties = Vec::new();

            for (key, value) in dict.iter() {
                let key_str = key
                    .downcast::<PyString>()
                    .map_err(|_| {
                        PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                            "Dictionary keys must be strings, but key in item {} is not a string",
                            index
                        ))
                    })?
                    .to_string();

                // Validate key is not empty
                if key_str.is_empty() {
                    return Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                        "Empty key found in item {}",
                        index
                    )));
                }

                let value_str = value
                    .str()
                    .map_err(|_| PyErr::new::<pyo3::exceptions::PyException, _>(
                        format!("Dictionary values must be convertible to strings, but value for key '{}' in item {} is not convertible", key_str, index)
                    ))?
                    .to_string();

                if key_str == "label" {
                    if value_str.is_empty() {
                        return Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                            "Empty label found in item {}",
                            index
                        )));
                    }
                    label = value_str;
                } else {
                    // Format property value appropriately
                    if let Ok(int_val) = value_str.parse::<i64>() {
                        properties.push(format!("{}: {}", key_str, int_val));
                    } else if let Ok(float_val) = value_str.parse::<f64>() {
                        properties.push(format!("{}: {}", key_str, float_val));
                    } else if value_str.eq_ignore_ascii_case("true") {
                        properties.push(format!("{}: true", key_str));
                    } else if value_str.eq_ignore_ascii_case("false") {
                        properties.push(format!("{}: false", key_str));
                    } else if value_str.eq_ignore_ascii_case("null") {
                        properties.push(format!("{}: null", key_str));
                    } else {
                        // It's a string, remove the extra quotes if they exist and escape single
                        // quotes
                        let clean_value = if value_str.starts_with('\'')
                            && value_str.ends_with('\'')
                            && value_str.len() > 1
                        {
                            &value_str[1..value_str.len() - 1]
                        } else {
                            &value_str
                        };
                        // Escape single quotes in string values
                        let escaped_value = clean_value.replace('\'', "\\'");
                        properties.push(format!("{}: '{}'", key_str, escaped_value));
                    }
                }
            }

            // Validate label is not empty
            if label.is_empty() {
                return Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                    "Empty label found in item {}",
                    index
                )));
            }

            // Create INSERT statement using correct GQL syntax
            if !properties.is_empty() {
                let props_str = properties.join(", ");
                // Use (:Label { properties }) syntax according to GQL specification
                let statement = format!(
                    "INSERT (:{} {{ {} }}) INTO {}",
                    label, props_str, graph_name
                );
                current_batch.push(statement);
            }

            // If batch is full, add it to batch_statements and start a new batch
            if current_batch.len() >= BATCH_SIZE {
                batch_statements.push(current_batch);
                current_batch = Vec::new();
            }
        }

        // Add the last batch if it's not empty
        if !current_batch.is_empty() {
            batch_statements.push(current_batch);
        }

        // Execute all batches
        for (batch_index, batch) in batch_statements.iter().enumerate() {
            // Create a transaction for this batch
            let transaction_query = format!("BEGIN TRANSACTION INTO {}", graph_name);
            session.query(&transaction_query).unwrap_or_else(|_| {
                panic!("Failed to begin transaction for batch {}", batch_index)
            });

            for statement in batch {
                session
                    .query(statement)
                    .unwrap_or_else(|_| panic!("Failed to execute statement '{}'", statement));
            }

            // Commit the transaction
            let commit_query = "COMMIT";
            session.query(commit_query).unwrap_or_else(|_| {
                panic!("Failed to commit transaction for batch {}", batch_index)
            });

            println!(
                "Successfully executed batch {} with {} statements",
                batch_index,
                batch.len()
            );
        }

        println!("All data loaded successfully");
        Ok(())
    }

    /// Save database to a file
    fn save_to_file(&mut self, file_path: &str) -> PyResult<()> {
        // Get the session
        let session = self.session.as_mut().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyException, _>("Session not initialized")
        })?;

        // Use current graph or default to "default_graph"
        let graph_name = self.current_graph.as_deref().unwrap_or("default_graph");

        // Sanitize the path to prevent injection attacks
        let sanitized_path = file_path.replace(['\'', '"', ';', '\n', '\r'], "");

        // Execute export procedure with correct syntax (no semicolon)
        let query = format!(
            "CALL export('{}', '{}', 'manifest.json')",
            graph_name, sanitized_path
        );
        session.query(&query).expect("Export failed");

        println!("Database saved successfully to: {}", file_path);
        Ok(())
    }

    /// Create a new graph
    fn create_graph(&mut self, graph_name: &str) -> PyResult<()> {
        let session = self.session.as_mut().expect("Session not initialized");

        // Validate graph name
        if graph_name.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyException, _>(
                "Graph name cannot be empty",
            ));
        }

        // Sanitize graph name - replace invalid characters with underscore
        let sanitized_name = graph_name.replace(|c: char| !c.is_alphanumeric() && c != '_', "_");

        // Validate graph name after sanitization
        if sanitized_name.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyException, _>(
                "Graph name contains only invalid characters",
            ));
        }

        // Create the graph using the create_test_graph procedure
        let query = format!("CALL create_test_graph('{}')", sanitized_name);
        match session.query(&query) {
            Ok(_) => {
                println!("Graph '{}' created successfully", sanitized_name);
                self.current_graph = Some(sanitized_name);

                Ok(())
            }
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                "Failed to create graph '{}': {}",
                sanitized_name, e
            ))),
        }
    }

    /// Close the database connection
    fn close(&mut self) -> PyResult<()> {
        self.database = None;
        self.session = None;
        self.current_graph = None;
        Ok(())
    }

    /// Load data from a CSV file
    fn load_csv(&mut self, path: &str) -> PyResult<()> {
        let session = self.session.as_mut().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyException, _>("Session not initialized")
        })?;

        // Validate file path
        let path_obj = Path::new(path);
        if !path_obj.exists() {
            return Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                "File not found: {}",
                path
            )));
        }

        // Use current graph or default to "default_graph"
        let graph_name = self.current_graph.as_deref().unwrap_or("default_graph");

        // Sanitize the path to prevent injection attacks
        let sanitized_path = path.replace(['\'', '"', ';', '\n', '\r'], "");

        let query = format!("LOAD CSV FROM \"{}\" INTO {}", sanitized_path, graph_name);
        match session.query(&query) {
            Ok(_) => {
                println!("CSV data loaded successfully from: {}", path);
                Ok(())
            }
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                "Failed to load CSV from file: {}",
                e
            ))),
        }
    }

    /// Load data from a JSON file
    fn load_json(&mut self, path: &str) -> PyResult<()> {
        let session = self.session.as_mut().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyException, _>("Session not initialized")
        })?;

        // Validate file path
        let path_obj = Path::new(path);
        if !path_obj.exists() {
            return Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                "File not found: {}",
                path
            )));
        }

        // Use current graph or default to "default_graph"
        let graph_name = self.current_graph.as_deref().unwrap_or("default_graph");

        // Sanitize the path to prevent injection attacks
        let sanitized_path = path.replace(['\'', '"', ';', '\n', '\r'], "");

        let query = format!("LOAD JSON FROM \"{}\" INTO {}", sanitized_path, graph_name);
        match session.query(&query) {
            Ok(_) => {
                println!("JSON data loaded successfully from: {}", path);
                Ok(())
            }
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                "Failed to load JSON from file: {}",
                e
            ))),
        }
    }

    /// Drop a graph
    fn drop_graph(&mut self, graph_name: &str) -> PyResult<()> {
        let session = self.session.as_mut().expect("Session not initialized");

        // Validate graph name
        if graph_name.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyException, _>(
                "Graph name cannot be empty",
            ));
        }

        // Sanitize graph name
        let sanitized_name = graph_name.replace(|c: char| !c.is_alphanumeric() && c != '_', "");

        // Validate graph name after sanitization
        if sanitized_name.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyException, _>(
                "Graph name contains only invalid characters",
            ));
        }

        let query = format!("DROP GRAPH {}", sanitized_name);
        match session.query(&query) {
            Ok(_) => {
                // Clear current graph if it's the one being dropped
                if self.current_graph.as_deref() == Some(&sanitized_name) {
                    self.current_graph = None;
                }
                println!("Graph '{}' dropped successfully", sanitized_name);
                Ok(())
            }
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                "Failed to drop graph '{}': {}",
                sanitized_name, e
            ))),
        }
    }

    /// Use a graph
    fn use_graph(&mut self, graph_name: &str) -> PyResult<()> {
        let session = self.session.as_mut().expect("Session not initialized");

        // Validate graph name
        if graph_name.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyException, _>(
                "Graph name cannot be empty",
            ));
        }

        // Sanitize graph name
        let sanitized_name = graph_name.replace(['\'', '"', ';', '\n', '\r'], "");

        // Validate graph name after sanitization
        if sanitized_name.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyException, _>(
                "Graph name contains only invalid characters",
            ));
        }

        let query = format!("USE GRAPH {}", sanitized_name);
        session.query(&query).expect("Failed to use graph");
        self.current_graph = Some(sanitized_name);
        Ok(())
    }

    /// Begin a transaction
    fn begin_transaction(&mut self) -> PyResult<()> {
        let session = self.session.as_mut().expect("Session not initialized");

        // Use current graph or default to "default_graph"
        let graph_name = self.current_graph.as_deref().unwrap_or("default_graph");

        let query = format!("BEGIN TRANSACTION INTO {}", graph_name);
        session.query(&query).expect("Failed to begin transaction");
        Ok(())
    }

    /// Commit current transaction
    fn commit(&mut self) -> PyResult<()> {
        let session = self.session.as_mut().expect("Session not initialized");
        session
            .query("COMMIT")
            .expect("Failed to commit transaction");
        Ok(())
    }

    /// Rollback current transaction
    fn rollback(&mut self) -> PyResult<()> {
        let session = self.session.as_mut().expect("Session not initialized");
        session
            .query("ROLLBACK")
            .expect("Failed to rollback transaction");
        Ok(())
    }
}

/// Convert a DataChunk to a Python list of lists
fn convert_data_chunk(chunk: &DataChunk) -> PyResult<Vec<Vec<PyObject>>> {
    let mut result = Vec::new();

    // Get the number of rows
    let num_rows = chunk.len();

    // For each row, create a list of values
    for row_idx in 0..num_rows {
        let mut row_vec = Vec::new();

        // For each column, get the value at this row
        for col in chunk.columns() {
            let value = extract_value_from_array(col, row_idx)?;
            row_vec.push(value);
        }

        result.push(row_vec);
    }

    Ok(result)
}

/// Extract a value from an Arrow array at a specific index
fn extract_value_from_array(array: &ArrayRef, index: usize) -> PyResult<PyObject> {
    Python::with_gil(|py| match array.data_type() {
        DataType::Int32 => {
            let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
            if arr.is_null(index) {
                Ok(py.None())
            } else {
                Ok(arr.value(index).into_pyobject(py)?.into_any().unbind())
            }
        }
        DataType::Utf8 => {
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
                let value = pyo3::types::PyBool::new(py, arr.value(index));
                Ok(value.into_pyobject(py).map(|v| {
                    <pyo3::Bound<'_, PyBool> as Clone>::clone(&v)
                        .into_any()
                        .unbind()
                })?)
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
        _ => Ok(py.None()),
    })
}

/// Python module for miniGU
#[pymodule]
fn minigu_python(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyMiniGU>()?;
    Ok(())
}
