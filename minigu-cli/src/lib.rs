use clap::Parser;
use cli::Cli;

mod cli;
mod script_executor;
mod shell;

pub fn run_cli() {
    let cli = Cli::parse();
    let result = match cli {
        Cli::Shell(shell) => shell.run(),
        Cli::Execute { file } => {
            let executor = script_executor::ScriptExecutor {};
            executor.execute_file(file)
        }
    };
    // Handle unrecoverable errors.
    if let Err(e) = result {
        println!("{e:?}")
    }
}
