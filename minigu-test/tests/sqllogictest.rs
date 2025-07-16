// Copyright 2024 MiniGU Project Authors. Licensed under Apache-2.0.

//! Sqllogictest for MiniGU.

use std::path::Path;
use std::sync::Arc;

use libtest_mimic::{Arguments, Trial};
use minigu_test::sqllogictest_adapter::MiniGuDB;
use sqllogictest::{DBOutput, DefaultColumnType};
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::test]
async fn run_all_sqllogictest_files() {
    const PATTERN: &str = "sql/**/[!_]*.slt"; // Ignore files starting with '_'
    const BLOCKLIST: &[&str] = &[]; // No files to skip for now

    let mut tests = vec![];

    // Use glob pattern to find all test files
    let paths = glob::glob(PATTERN).expect("failed to find test files");

    for entry in paths {
        let path = entry.expect("failed to read glob entry");
        let subpath = path.strip_prefix("sql").unwrap().to_str().unwrap();

        // Check if in blacklist
        if !BLOCKLIST.iter().any(|p| subpath.contains(p)) {
            let path_clone = path.clone();
            tests.push(Trial::test(format!("minigu::{}", subpath), move || {
                Ok(build_runtime().block_on(test(&path_clone))?)
            }));
        }
    }

    if tests.is_empty() {
        panic!(
            "no test found for sqllogictest! pwd: {:?}",
            std::env::current_dir().unwrap()
        );
    }

    fn build_runtime() -> Runtime {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
    }

    // Use libtest-mimic to run tests
    let args = Arguments::from_args();
    let _result = libtest_mimic::run(&args, tests);
}

async fn test(filename: impl AsRef<Path>) -> Result<()> {
    let db = MiniGuDB::new().map_err(|e| format!("Failed to create database: {}", e))?;
    let db = Arc::new(Mutex::new(db));
    let mut tester = sqllogictest::Runner::new(|| async { Ok(DatabaseWrapper { db: db.clone() }) });

    // Run test file
    tester.run_file_async(filename).await?;
    Ok(())
}

/// Wrapper type, for implementing sqllogictest driver trait
struct DatabaseWrapper {
    db: Arc<Mutex<MiniGuDB>>,
}

#[async_trait::async_trait]
impl sqllogictest::AsyncDB for DatabaseWrapper {
    type ColumnType = DefaultColumnType;
    type Error = minigu_test::sqllogictest_adapter::SqlLogicTestError;

    async fn run(
        &mut self,
        sql: &str,
    ) -> core::result::Result<DBOutput<DefaultColumnType>, Self::Error> {
        // Get database instance lock and execute query
        let mut db = self.db.lock().await;
        db.run(sql).await
    }

    async fn shutdown(&mut self) {}
}
