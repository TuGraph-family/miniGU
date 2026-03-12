use std::time::Instant;

use clap::Parser;
use miette::{IntoDiagnostic, Result};
use minigu::database::{Database, DatabaseConfig};

#[derive(Debug, Parser, Clone)]
pub struct ScriptExecutor {}

impl ScriptExecutor {
    pub fn execute_file(&self, file: String, path: Option<String>) -> Result<()> {
        let db = if let Some(path) = path {
            Database::open(path, DatabaseConfig::default()).unwrap()
        } else {
            Database::open_in_memory(DatabaseConfig::default()).unwrap()
        };
        let mut session = db.session().unwrap();
        let content = std::fs::read_to_string(&file).into_diagnostic()?;
        let mut timing = false;
        for line in content.lines() {
            let line = line.trim();
            match line {
                "" => continue,
                ":quit" => break,
                ":timing" => {
                    timing = !timing;
                    eprintln!("Timing is {}", if timing { "on" } else { "off" });
                }
                line => {
                    let (timed, query) = if let Some(rest) = line.strip_prefix(":time ") {
                        (true, rest)
                    } else {
                        (timing, line)
                    };
                    if timed {
                        let start = Instant::now();
                        session.query(query)?;
                        let elapsed = start.elapsed();
                        eprintln!("Time: {:.3}s", elapsed.as_secs_f64());
                    } else {
                        session.query(line)?;
                    }
                }
            };
        }
        Ok(())
    }
}
