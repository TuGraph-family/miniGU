//! Sqllogictest for MiniGU.

use std::path::Path;
use std::sync::Arc;

use libtest_mimic::{Arguments, Trial};
use minigu_test::slt_adapter::MiniGuDB;
use sqllogictest::{DBOutput, DefaultColumnType};
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

/// Run all sqllogictest files in the `sql` directory.
///
/// This function:
/// - Recursively finds all `.slt` files in the `sql` directory
/// - Skips files that start with '_'
/// - Skips directories listed in `BLOCKLIST`
/// - Creates a test runner for each file
/// - Runs all tests in parallel using `libtest-mimic`
/// - Reports test results and fails if any test fails
///
/// # Panics
///
/// Panics if:
/// - No test files are found
/// - Any test fails to execute
/// - Any test assertion fails
///
/// # Usage
///     
/// Run `cargo test --test sqllogictest -- --nocapture` to run all tests.
#[tokio::test]
async fn run_all_sqllogictest_files() {
    // Ignore files starting with '_'
    const PATTERN: &str = "sql/**/[!_]*.slt";
    // Skip specific tests
    const BLOCKLIST: &[&str] = &["finbench/", "gql_on_one_page/", "misc/", "opengql/", "snb/"];

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
    let conclusion = libtest_mimic::run(&args, tests);

    // Check if any test failed
    let exit_code = conclusion.exit_code();
    if exit_code != std::process::ExitCode::SUCCESS {
        panic!("Some tests failed with exit code: {:?}", exit_code);
    }
}

/// Run a test file
async fn test(filename: impl AsRef<Path>) -> Result<()> {
    let db = MiniGuDB::new().map_err(|e| format!("Failed to create database: {}", e))?;
    let db = Arc::new(Mutex::new(db));
    let mut tester = sqllogictest::Runner::new(|| async { Ok(DatabaseWrapper { db: db.clone() }) });

    // Run test file and check result
    match tester.run_file_async(filename.as_ref()).await {
        Ok(_) => Ok(()),
        Err(e) => {
            // Provide more detailed error information
            eprintln!(
                "\nTest failed for file: {}\nError: {}\n",
                filename.as_ref().display(),
                e
            );
            Err(format!(
                "Test failed for file: {}\nError: {}",
                filename.as_ref().display(),
                e
            )
            .into())
        }
    }
}

/// Wrapper type, for implementing sqllogictest driver trait
struct DatabaseWrapper {
    db: Arc<Mutex<MiniGuDB>>,
}

#[async_trait::async_trait]
impl sqllogictest::AsyncDB for DatabaseWrapper {
    type ColumnType = DefaultColumnType;
    type Error = minigu_test::slt_adapter::SqlLogicTestError;

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
