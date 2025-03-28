use clap::Parser;
use cli::Cli;

mod cli;
mod shell;

pub fn run_cli() {
    let cli = Cli::parse();
    let result = match cli {
        Cli::Shell(shell) => shell.run(),
    };
    // Handle unrecoverable errors.
    if let Err(e) = result {
        println!("{e:?}")
    }
}
