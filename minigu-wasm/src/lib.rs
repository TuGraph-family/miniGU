use std::fmt::Write;

use arrow::array::ArrayRef;
use minigu::common::data_chunk::display::{TableBuilder, TableOptions};
use minigu::database::{Database, DatabaseConfig};
use minigu::result::QueryResult;
use minigu::session::Session;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct MiniGuDb {
    session: Session,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl MiniGuDb {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<MiniGuDb, JsValue> {
        let db = Database::open_in_memory(DatabaseConfig::default())
            .map_err(|e| JsValue::from_str(&format!("{e:#?}")))?;
        let session = db
            .session()
            .map_err(|e| JsValue::from_str(&format!("{e:#?}")))?;
        Ok(MiniGuDb { session })
    }

    pub fn query_table(&mut self, query: &str) -> Result<String, JsValue> {
        let result = self
            .session
            .query(query)
            .map_err(|e| JsValue::from_str(&format!("{e:#?}")))?;
        Ok(result_to_table_string(&result))
    }

    pub fn query_json(&mut self, query: &str) -> Result<String, JsValue> {
        let result = self
            .session
            .query(query)
            .map_err(|e| JsValue::from_str(&format!("{e:#?}")))?;
        Ok(result_to_json_string(&result))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl MiniGuDb {
    pub fn new() -> Result<MiniGuDb, String> {
        let db =
            Database::open_in_memory(DatabaseConfig::default()).map_err(|e| format!("{e:#?}"))?;
        let session = db.session().map_err(|e| format!("{e:#?}"))?;
        Ok(MiniGuDb { session })
    }

    pub fn query_table(&mut self, query: &str) -> Result<String, String> {
        let result = self.session.query(query).map_err(|e| format!("{e:#?}"))?;
        Ok(result_to_table_string(&result))
    }

    pub fn query_json(&mut self, query: &str) -> Result<String, String> {
        let result = self.session.query(query).map_err(|e| format!("{e:#?}"))?;
        Ok(result_to_json_string(&result))
    }
}

fn result_to_table_string(result: &QueryResult) -> String {
    let mut out = String::new();
    let options = TableOptions::style_for_test();
    if let Some(schema) = result.schema() {
        let mut builder = TableBuilder::new(Some(schema.clone()), options);
        let mut num_rows = 0;
        for chunk in result.iter() {
            num_rows += chunk.cardinality();
            builder = builder.append_chunk(chunk);
        }
        let table = builder.build();
        writeln!(&mut out, "{table}").unwrap();
        writeln!(&mut out, "{num_rows} rows").unwrap();
        return out;
    }

    let mut has_rows = false;
    for chunk in result.iter() {
        let row_count = chunk.cardinality();
        let columns = chunk.columns();

        for row_idx in 0..row_count {
            has_rows = true;
            let row_values: Vec<String> = columns
                .iter()
                .map(|column| extract_string_value(column, row_idx))
                .collect();
            out.push_str(&row_values.join("\t"));
            out.push('\n');
        }
    }

    if !has_rows {
        return "Statement OK. No results".to_string();
    }

    out
}

fn extract_string_value(array: &ArrayRef, row_idx: usize) -> String {
    use arrow::array::*;

    if array.is_null(row_idx) {
        return String::new();
    }

    if let Some(string_array) = array.as_any().downcast_ref::<StringArray>() {
        string_array.value(row_idx).to_string()
    } else if let Some(string_array) = array.as_any().downcast_ref::<LargeStringArray>() {
        string_array.value(row_idx).to_string()
    } else if let Some(int_array) = array.as_any().downcast_ref::<Int64Array>() {
        int_array.value(row_idx).to_string()
    } else if let Some(int_array) = array.as_any().downcast_ref::<Int32Array>() {
        int_array.value(row_idx).to_string()
    } else if let Some(uint_array) = array.as_any().downcast_ref::<UInt32Array>() {
        uint_array.value(row_idx).to_string()
    } else if let Some(uint_array) = array.as_any().downcast_ref::<UInt64Array>() {
        uint_array.value(row_idx).to_string()
    } else if let Some(double_array) = array.as_any().downcast_ref::<Float64Array>() {
        double_array.value(row_idx).to_string()
    } else if let Some(float_array) = array.as_any().downcast_ref::<Float32Array>() {
        float_array.value(row_idx).to_string()
    } else if let Some(bool_array) = array.as_any().downcast_ref::<BooleanArray>() {
        bool_array.value(row_idx).to_string()
    } else {
        format!("{:?}", array.to_data())
    }
}

fn result_to_json_string(result: &QueryResult) -> String {
    use serde_json::json;

    let schema_json = result.schema().map(|schema| {
        schema
            .fields()
            .iter()
            .map(|f| {
                json!({
                    "name": f.name(),
                    "type": f.ty().to_string(),
                    "nullable": f.is_nullable(),
                })
            })
            .collect::<Vec<_>>()
    });

    let mut rows: Vec<Vec<serde_json::Value>> = Vec::new();
    for chunk in result.iter() {
        let num_rows = chunk.cardinality();
        let columns = chunk.columns();

        for row_idx in 0..num_rows {
            let mut row = Vec::with_capacity(columns.len());
            for col in columns {
                row.push(extract_json_value(col, row_idx));
            }
            rows.push(row);
        }
    }

    let metrics = result.metrics();
    json!({
        "schema": schema_json,
        "rows": rows,
        "metrics_ms": {
            "parsing": metrics.parsing_time().as_secs_f64() * 1000.0,
            "planning": metrics.planning_time().as_secs_f64() * 1000.0,
            "compiling": metrics.compiling_time().as_secs_f64() * 1000.0,
            "execution": metrics.execution_time().as_secs_f64() * 1000.0,
            "total": metrics.total_time().as_secs_f64() * 1000.0,
        }
    })
    .to_string()
}

fn extract_json_value(array: &ArrayRef, row_idx: usize) -> serde_json::Value {
    use arrow::array::*;
    use serde_json::Value;

    if array.is_null(row_idx) {
        return Value::Null;
    }

    if let Some(a) = array.as_any().downcast_ref::<StringArray>() {
        Value::String(a.value(row_idx).to_string())
    } else if let Some(a) = array.as_any().downcast_ref::<LargeStringArray>() {
        Value::String(a.value(row_idx).to_string())
    } else if let Some(a) = array.as_any().downcast_ref::<Int64Array>() {
        Value::Number(a.value(row_idx).into())
    } else if let Some(a) = array.as_any().downcast_ref::<Int32Array>() {
        Value::Number(a.value(row_idx).into())
    } else if let Some(a) = array.as_any().downcast_ref::<UInt32Array>() {
        Value::Number(a.value(row_idx).into())
    } else if let Some(a) = array.as_any().downcast_ref::<UInt64Array>() {
        Value::Number(a.value(row_idx).into())
    } else if let Some(a) = array.as_any().downcast_ref::<Float64Array>() {
        serde_json::Number::from_f64(a.value(row_idx))
            .map(Value::Number)
            .unwrap_or(Value::Null)
    } else if let Some(a) = array.as_any().downcast_ref::<Float32Array>() {
        serde_json::Number::from_f64(a.value(row_idx) as f64)
            .map(Value::Number)
            .unwrap_or(Value::Null)
    } else if let Some(a) = array.as_any().downcast_ref::<BooleanArray>() {
        Value::Bool(a.value(row_idx))
    } else if let Some(a) = array.as_any().downcast_ref::<StructArray>() {
        let mut obj = serde_json::Map::with_capacity(a.num_columns());
        for (idx, field) in a.fields().iter().enumerate() {
            let v = extract_json_value(a.column(idx), row_idx);
            obj.insert(field.name().clone(), v);
        }
        Value::Object(obj)
    } else {
        Value::String(format!("{:?}", array))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    #[test]
    fn wasm_wrapper_smoke_query_json() {
        let mut db = MiniGuDb::new().unwrap();
        db.query_json("CALL create_test_graph_data(\"g\", 5)")
            .unwrap();
        db.query_json("SESSION SET GRAPH g").unwrap();

        let out = db.query_json("MATCH (n:PERSON) RETURN n").unwrap();
        let v: Value = serde_json::from_str(&out).unwrap();

        let schema = v.get("schema").unwrap().as_array().unwrap();
        assert_eq!(schema.len(), 1);

        let rows = v.get("rows").unwrap().as_array().unwrap();
        assert!(!rows.is_empty());

        let first_row = rows[0].as_array().unwrap();
        let first_vertex = first_row[0].as_object().unwrap();
        assert!(first_vertex.get("_vid").is_some());
    }

    #[test]
    fn wasm_wrapper_query_table_keeps_header_for_empty_result() {
        let mut db = MiniGuDb::new().unwrap();
        db.query_json("CALL create_test_graph_data(\"g\", 5)")
            .unwrap();
        db.query_json("SESSION SET GRAPH g").unwrap();

        let out = db.query_table("MATCH (n:PERSON) RETURN n LIMIT 0").unwrap();

        assert!(out.contains("vertex {"));
        assert!(out.contains("0 rows"));
    }

    #[test]
    fn wasm_wrapper_query_table_formats_schema_less_rows() {
        let mut db = MiniGuDb::new().unwrap();
        let out = db.query_table("EXPLAIN RETURN 1").unwrap();

        assert!(!out.contains("Statement OK. No results"));
        assert!(!out.contains("Statement OK. No schema"));
        assert!(!out.trim().is_empty());
    }
}
