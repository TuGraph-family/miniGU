use insta_cmd::assert_cmd_snapshot;

mod common;

#[test]
fn test_cli_help() {
    let mut cmd = common::run_cli();
    assert_cmd_snapshot!(cmd.arg("--help"));
}
