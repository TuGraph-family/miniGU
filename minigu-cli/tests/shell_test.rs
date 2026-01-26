use std::path::Path;

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

#[test]
fn test_shell_command_cd_dir() {
    // Create a dir, <temp_dir>/foo
    let temp_dir = tempfile::tempdir().unwrap();
    let dirname = Path::new("foo");
    std::fs::create_dir(temp_dir.path().join(dirname)).unwrap();

    let mut cmd = common::run_cli();
    cmd.current_dir(temp_dir.path());
    let prompt = format!(":cd {}", dirname.display());

    assert_cmd_snapshot!(cmd.arg("shell").pass_stdin(prompt));
}

#[test]
fn test_shell_command_cd_no_arg() {
    let mut cmd = common::run_cli();

    assert_cmd_snapshot!(cmd.arg("shell").pass_stdin(":cd"));
}

#[test]
fn test_shell_command_cd_too_many_arg() {
    let mut cmd = common::run_cli();

    assert_cmd_snapshot!(cmd.arg("shell").pass_stdin(":cd foo bar"));
}

#[test]
fn test_shell_command_cd_file() {
    // Create a file, <temp_dir>/foo
    let temp_dir = tempfile::tempdir().unwrap();

    let filename = Path::new("foo");
    std::fs::File::create(temp_dir.path().join(filename)).unwrap();

    let mut cmd = common::run_cli();
    cmd.current_dir(temp_dir.path());
    let prompt = format!(":cd {}", filename.display());

    assert_cmd_snapshot!(cmd.arg("shell").pass_stdin(prompt));
}

#[test]
fn test_shell_command_cd_non_existent_dir() {
    // Create a file, <temp_dir>/foo
    let temp_dir = tempfile::tempdir().unwrap();

    let non_existent_dir = Path::new("foo");
    assert!(!temp_dir.path().join(non_existent_dir).exists());

    let mut cmd = common::run_cli();
    cmd.current_dir(temp_dir.path());
    let prompt = format!(":cd {}", non_existent_dir.display());

    // Makes windows happy
    let suffix = if cfg!(windows) { "windows" } else { "unix" };

    insta::with_settings!({
        snapshot_suffix => suffix,
    },{
        assert_cmd_snapshot!(cmd.arg("shell").pass_stdin(prompt));
    });
}

// TODO(support paths that include whitespace like `:cd "foo bar"` or `:cd foo\ bar`)
