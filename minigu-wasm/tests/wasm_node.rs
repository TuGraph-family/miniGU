#![cfg(target_arch = "wasm32")]

use minigu_wasm::MiniGuDb;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn query_table_smoke() {
    let mut db = MiniGuDb::new().unwrap();

    db.query_json("CALL create_test_graph_data(\"g\", 5)")
        .unwrap();
    db.query_json("SESSION SET GRAPH g").unwrap();

    let out = db.query_table("MATCH (n:PERSON) RETURN n").unwrap();
    assert!(out.contains("rows"));
}
