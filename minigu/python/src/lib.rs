//! Python bindings for miniGU graph database
//!
//! This module provides Python bindings for the miniGU graph database using PyO3.

use arrow::array::*;
use arrow::datatypes::DataType;
use minigu::database::{Database, DatabaseConfig};
use minigu::session::Session;
use minigu_common::data_chunk::DataChunk;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyList, PyString};

/// PyMiniGU class that wraps the Rust Database
#[pyclass]
pub struct PyMiniGu {
    database: Option<Database>,
    session: Option<Session>,
}

#[pymethods]
impl PyMiniGu {
    /// Create a new PyMiniGU instance
    #[new]
    fn new() -> PyResult<Self> {
        Ok(PyMiniGu {
            database: None,
            session: None,
        })
    }

    /// Initialize the database
    fn init(&mut self) -> PyResult<()> {
        let config = DatabaseConfig::default();
        let db = Database::open_in_memory(&config).expect("Failed to initialize database");
        let session = db.session().expect("Failed to create session");
        self.database = Some(db);
        self.session = Some(session);
        Ok(())
    }

    /// Execute a GQL query
    fn execute(&mut self, query: &str, py: Python) -> PyResult<PyObject> {
        // Get the session
        let session = self.session.as_mut().expect("Session not initialized");

        // Execute the query
        let query_result = session.query(query).expect("Query execution failed");

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
    fn load_from_file(&mut self, path: &str) -> PyResult<()> {
        // Get the session
        let session = self.session.as_mut().expect("Session not initialized");

        // Execute the import procedure with correct syntax (no semicolon)
        let query = format!("CALL import('test_graph', '{}', 'manifest.json')", path);
        session
            .query(&query)
            .expect("Failed to load data from file");

        println!("Data loaded successfully from: {}", path);
        Ok(())
    }

    /// Load data directly
    fn load_data(&mut self, data: &Bound<'_, PyAny>) -> PyResult<()> {
        // Get the session
        let session = self.session.as_mut().expect("Session not initialized");

        // Convert Python data to Rust data structures
        let list = data
            .downcast::<PyList>()
            .expect("Expected a list of dictionaries");

        println!("Loading {} records", list.len());

        // Build GQL INSERT statements from the Python data
        let mut insert_statements = Vec::new();

        for item in list.iter() {
            let dict = item
                .downcast::<PyDict>()
                .expect("Expected a list of dictionaries");

            // Extract label and properties
            let mut label = "Node".to_string();
            let mut properties = Vec::new();

            for (key, value) in dict.iter() {
                let key_str = key
                    .downcast::<PyString>()
                    .expect("Dictionary keys must be strings")
                    .to_string();

                let value_str = value
                    .str()
                    .expect("Dictionary values must be convertible to strings")
                    .to_string();

                if key_str == "label" {
                    label = value_str;
                } else {
                    // Format property value appropriately
                    // Based on GQL examples, we need to handle different types
                    // correctly. For now, we'll try to determine if it's a number
                    // or string
                    if let Ok(int_val) = value_str.parse::<i64>() {
                        properties.push(format!("{}: {}", key_str, int_val));
                    } else if let Ok(float_val) = value_str.parse::<f64>() {
                        properties.push(format!("{}: {}", key_str, float_val));
                    } else {
                        // It's a string, remove the extra quotes if they exist
                        let clean_value = if value_str.starts_with('\'')
                            && value_str.ends_with('\'')
                            && value_str.len() > 1
                        {
                            &value_str[1..value_str.len() - 1]
                        } else {
                            &value_str
                        };
                        properties.push(format!("{}: '{}'", key_str, clean_value));
                    }
                }
            }

            // Create INSERT statement using correct GQL syntax
            if !properties.is_empty() {
                let props_str = properties.join(", ");
                // Use (:Label { properties }) syntax according to GQL specification
                let statement = format!("INSERT (:{} {{ {} }})", label, props_str);
                insert_statements.push(statement);
            }
        }

        // Execute all INSERT statements
        for statement in insert_statements {
            session
                .query(&statement)
                .unwrap_or_else(|_| panic!("Failed to execute statement '{}'", statement));
            println!("Successfully executed: {}", statement);
        }

        println!("All data loaded successfully");
        Ok(())
    }

    /// Save database to a file
    fn save_to_file(&mut self, path: &str) -> PyResult<()> {
        // Get the session
        let session = self.session.as_mut().expect("Session not initialized");

        // Execute the export procedure with correct syntax (no semicolon)
        let query = format!("CALL export('test_graph', '{}', 'manifest.json')", path);
        session
            .query(&query)
            .expect("Failed to save database to file");

        println!("Database saved successfully to: {}", path);
        Ok(())
    }

    /// Create a graph
    fn create_graph(&mut self, name: &str, schema: Option<&str>) -> PyResult<()> {
        // Get the session
        let session = self.session.as_mut().expect("Session not initialized");

        // Create the graph using the create_test_graph procedure
        let query = format!("CALL create_test_graph('{}');", name);
        session
            .query(&query)
            .unwrap_or_else(|_| panic!("Failed to create graph '{}'", name));

        println!("Graph '{}' created successfully", name);

        // If schema is provided, we could process it here
        if let Some(schema_str) = schema {
            println!("Schema provided but not yet implemented: {}", schema_str);
            // In a full implementation, we would parse the schema and add vertex/edge types
        }

        Ok(())
    }

    /// Insert data
    fn insert_data(&mut self, data: &str) -> PyResult<()> {
        // Get the session
        let session = self.session.as_mut().expect("Session not initialized");

        // Execute the INSERT statement
        session.query(data).expect("Failed to insert data");

        println!("Data inserted successfully: {}", data);
        Ok(())
    }

    /// Update data
    fn update_data(&mut self, query: &str) -> PyResult<()> {
        // Get the session
        let session = self.session.as_mut().expect("Session not initialized");

        // Execute the UPDATE statement
        session.query(query).expect("Failed to update data");

        println!("Data updated successfully with query: {}", query);
        Ok(())
    }

    /// Delete data
    fn delete_data(&mut self, query: &str) -> PyResult<()> {
        // Get the session
        let session = self.session.as_mut().expect("Session not initialized");

        // Execute the DELETE statement
        session.query(query).expect("Failed to delete data");

        println!("Data deleted successfully with query: {}", query);
        Ok(())
    }

    /// Close the database connection
    fn close(&mut self) -> PyResult<()> {
        self.database = None;
        self.session = None;
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
