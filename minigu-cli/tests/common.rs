use std::process::Command;

use insta_cmd::get_cargo_bin;

pub fn run_cli() -> Command {
    Command::new(get_cargo_bin("minigu"))
}
