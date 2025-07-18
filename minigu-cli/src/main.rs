#![feature(duration_millis_float)]

mod cli;
mod script_executor;
mod shell;

use clap::Parser;

fn main() -> miette::Result<()> {
    cli::Cli::parse().run()
}
