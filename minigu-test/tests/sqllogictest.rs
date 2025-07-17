//! Sqllogictest for MiniGU.

use std::path::{Path, PathBuf};

use libtest_mimic::{Arguments, Trial};
use minigu_test::slt_adapter::MiniGuDb;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

/// Glob pattern to find all test files
const PATTERN: &str = "sql/**/[!_]*.slt";
/// Blocklist of directories to skip
const BLOCKLIST: &[&str] = &["finbench/", "gql_on_one_page/", "misc/", "opengql/", "snb/"];
// const BLOCKLIST: &[&str] = &[];

fn discover_tests() -> Vec<PathBuf> {
    glob::glob(PATTERN)
        .expect("failed to read glob pattern")
        .filter_map(|p| p.ok())
        .filter(|p| {
            let sub = p.strip_prefix("sql").unwrap().to_string_lossy();
            !BLOCKLIST.iter().any(|b| sub.contains(b))
        })
        .collect()
}

/// Run all sqllogictest files in the `sql` directory.
///
/// This function:
/// - Recursively finds all `.slt` files in the `sql` directory
/// - Skips files that start with '_'
/// - Skips directories listed in `BLOCKLIST`
/// - Runs all tests in parallel using `libtest-mimic` (local mode)
/// - Runs all tests sequentially (CI mode)
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
#[test]
fn run_sqllogictest() {
    let files = discover_tests();
    if files.is_empty() {
        panic!("No sql logic test files found by pattern `{PATTERN}`");
    }

    if std::env::var("CI").is_ok() {
        run_ci(&files);
    } else {
        run_locally(&files);
    }
}

/// CI mode: run tests sequentially.
fn run_ci(files: &[PathBuf]) {
    let mut failures = Vec::new();
    println!("Running {} SQL Logic Test files (CI mode)", files.len());

    for f in files {
        let name = f.strip_prefix("sql").unwrap().display();
        print!("→ {name} ... ");
        match run_one(f) {
            Ok(_) => println!("ok"),
            Err(e) => {
                println!("FAILED");
                eprintln!("{name}: {e}");
                failures.push((name.to_string(), e));
            }
        }
    }

    if !failures.is_empty() {
        for (n, e) in &failures {
            eprintln!("{n}: {e}");
        }
        panic!("{} SQL Logic Test(s) failed", failures.len());
    }
}

/// Local mode: libtest-mimic runs tests in parallel.
fn run_locally(files: &[PathBuf]) {
    let trials: Vec<_> = files
        .iter()
        .map(|p| {
            let name = p.strip_prefix("sql").unwrap().display().to_string();
            let p = p.clone();
            Trial::test(format!("minigu::{name}"), move || {
                run_one(&p).map_err(|e| libtest_mimic::Failed::from(e.to_string()))
            })
        })
        .collect();

    if libtest_mimic::run(&Arguments::from_args(), trials).exit_code()
        != std::process::ExitCode::SUCCESS
    {
        panic!("Some SQL Logic Test cases failed");
    }
}

/// Run a single .slt file.
fn run_one(path: impl AsRef<Path>) -> Result<()> {
    let db = MiniGuDb::new()?;
    let records = sqllogictest::parse_file(path.as_ref())?;

    for record in records {
        let mut runner = sqllogictest::Runner::new(|| async { Ok(db.clone()) });
        runner.run(record)?;
    }

    Ok(())
}
