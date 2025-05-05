use clap::Parser;

use crate::shell::Shell;

#[derive(Debug, Parser)]
pub enum Cli {
    Shell(Shell),
    Execute { file: String },
}
