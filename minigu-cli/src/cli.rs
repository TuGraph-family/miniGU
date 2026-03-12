use clap::Parser;
use miette::Result;

use crate::script_executor;
use crate::shell::ShellArgs;

#[derive(Debug, Parser)]
pub enum Cli {
    Shell(ShellArgs),
    Execute {
        file: String,
        /// Path to the database directory. If not provided, an in-memory database will be opened.
        #[arg(long)]
        path: Option<String>,
    },
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self {
            Cli::Shell(shell) => shell.run(),
            Cli::Execute { file, path } => {
                let executor = script_executor::ScriptExecutor {};
                executor.execute_file(file, path)
            }
        }
    }
}
