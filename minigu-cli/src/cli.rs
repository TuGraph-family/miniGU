use clap::Parser;
use miette::Result;

use crate::script_executor;
use crate::shell::ShellArgs;

#[derive(Debug, Parser)]
pub enum Cli {
    Shell(ShellArgs),
    Execute { file: String },
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self {
            Cli::Shell(shell) => shell.run(),
            Cli::Execute { file } => {
                let executor = script_executor::ScriptExecutor {};
                executor.execute_file(file)
            }
        }
    }
}
