#![cfg(target_arch = "wasm32")]

use minigu_wasm::MiniGuDb;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn query_json_returns_struct_rows() {
    let mut db = MiniGuDb::new().unwrap();

    db.query_json("CALL create_test_graph_data(\"g\", 5)")
        .unwrap();
    db.query_json("SESSION SET GRAPH g").unwrap();

    let out = db.query_json("MATCH (n:PERSON) RETURN n").unwrap();
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();

    let rows = v.get("rows").unwrap().as_array().unwrap();
    assert!(!rows.is_empty());

    let first_row = rows[0].as_array().unwrap();
    let first_vertex = first_row[0].as_object().unwrap();
    assert!(first_vertex.get("_vid").is_some());
}

#[wasm_bindgen_test]
fn import_graph_is_unavailable() {
    let mut db = MiniGuDb::new().unwrap();

    let err = db
        .query_json("CALL import_graph(\"g\", \"manifest.json\")")
        .unwrap_err();

    let msg = err.as_string().unwrap_or_else(|| format!("{err:?}"));
    assert!(msg.contains("import_graph"));
}
