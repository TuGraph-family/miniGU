use std::sync::Arc;

use minigu_catalog::memory::MemoryCatalog;
use minigu_catalog::memory::directory::MemoryDirectoryCatalog;
use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_catalog::provider::{
    CatalogProvider, DirectoryOrSchema, DirectoryProvider, SchemaProvider,
};
use minigu_context::database::DatabaseContext;
use minigu_context::session::SessionContext;
use minigu_execution::executor::Executor;
use minigu_execution::executor::catalog::CatalogDdlBuilder;
use minigu_planner::bound::{
    BoundCatalogModifyingStatement, BoundCreateGraphTypeStatement, BoundCreateSchemaStatement,
    BoundGraphType, CreateKind,
};
use minigu_transaction::Transaction;
use rayon::ThreadPoolBuilder;
use smol_str::SmolStr;

fn make_db() -> Arc<DatabaseContext> {
    // Root directory catalog
    let root = Arc::new(MemoryDirectoryCatalog::new(None)) as Arc<dyn DirectoryProvider>;
    let catalog = MemoryCatalog::new(DirectoryOrSchema::Directory(root));
    let pool = ThreadPoolBuilder::new().num_threads(2).build().unwrap();
    Arc::new(DatabaseContext::new(catalog, pool))
}

fn run_ddl(session: SessionContext, stmt: BoundCatalogModifyingStatement) {
    let mut exec = CatalogDdlBuilder::new(session, stmt).into_executor();
    while let Some(res) = exec.next_chunk() {
        res.unwrap();
    }
}

#[test]
fn two_sessions_auto_commit_visibility() {
    let db = make_db();
    let s1 = SessionContext::new(db.clone());
    let mut s2 = SessionContext::new(db.clone());

    // s2 starts a txn before s1 creates schema
    let t2_old = s2.get_or_begin_txn().unwrap();

    // s1: create schema `s` via DDL executor
    let stmt = BoundCatalogModifyingStatement::CreateSchema(BoundCreateSchemaStatement {
        schema_path: vec![SmolStr::new("s")],
        if_not_exists: false,
    });
    run_ddl(s1.clone(), stmt);

    // s2 old txn should not see it (SI)
    {
        let root = db.catalog().get_root().unwrap();
        let dir = root.as_directory().unwrap();
        assert!(dir.get_child("s", &t2_old).unwrap().is_none());
        // end old txn
        t2_old.abort().ok();
        s2.clear_current_txn();
    }

    // s2 new txn should see it
    let t2_new = s2.get_or_begin_txn().unwrap();
    {
        let root = db.catalog().get_root().unwrap();
        let dir = root.as_directory().unwrap();
        assert!(dir.get_child("s", &t2_new).unwrap().is_some());
    }
}

#[test]
fn two_sessions_graph_type_visibility_and_current_schema() {
    let db = make_db();
    let mut s1 = SessionContext::new(db.clone());
    let mut s2 = SessionContext::new(db.clone());

    // Prepare a standalone MemorySchemaCatalog and set as current for both sessions
    let schema = Arc::new(MemorySchemaCatalog::new(None));
    s1.current_schema = Some(schema.clone());
    s2.current_schema = Some(schema.clone());

    // s2 begins a txn before s1 creates graph type
    let t2_old = s2.get_or_begin_txn().unwrap();

    // s1: create a graph type `gt` via DDL executor (Nested -> new empty graph type)
    let stmt = BoundCatalogModifyingStatement::CreateGraphType(BoundCreateGraphTypeStatement {
        name: SmolStr::new("gt"),
        kind: CreateKind::Create,
        source: BoundGraphType::Nested(vec![]),
    });
    run_ddl(s1.clone(), stmt);

    // s2 old txn should not see it
    assert!(schema.get_graph_type("gt", &t2_old).unwrap().is_none());
    t2_old.abort().ok();
    s2.clear_current_txn();

    // s2 new txn should see it
    let t2_new = s2.get_or_begin_txn().unwrap();
    assert!(schema.get_graph_type("gt", &t2_new).unwrap().is_some());
}
