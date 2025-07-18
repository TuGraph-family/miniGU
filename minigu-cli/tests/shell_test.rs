use insta_cmd::assert_cmd_snapshot;

mod common;

#[test]
fn test_shell_command_help() {
    let mut cmd = common::run_cli();
    assert_cmd_snapshot!(cmd.arg("shell").pass_stdin(":help"));
}

#[test]
fn test_shell_command_help_mode() {
    let mut cmd = common::run_cli();
    assert_cmd_snapshot!(cmd.arg("shell").pass_stdin(":help :mode"));
}
