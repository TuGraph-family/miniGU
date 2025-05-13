use clap::Parser;
use miette::{IntoDiagnostic, Result};
use minigu::Database;

#[derive(Debug, Parser, Clone)]
pub struct ScriptExecutor {}

impl ScriptExecutor {
    pub fn execute_file(&self, file: String) -> Result<()> {
        let db = Database::open_in_memory().unwrap();
        let session = db.session().unwrap();
        let content = std::fs::read_to_string(&file).into_diagnostic()?;
        for line in content.lines() {
            let line = line.trim();
            match line {
                "" => continue,
                ":quit" => break,
                line => session.query(line)?,
            };
        }
        Ok(())
    }
}
