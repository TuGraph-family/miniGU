use clap::Parser;
use minigu_cli::Cli;

fn main() -> miette::Result<()> {
    Cli::parse().run()
}
