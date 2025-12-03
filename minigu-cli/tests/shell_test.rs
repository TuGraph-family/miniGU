use std::env;
use std::path::Path;

use insta::internals::Redaction;
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

fn shell_command_cd_redaction() -> Redaction {
    insta::dynamic_redaction(|value, _path| {
        assert!(value.as_str().unwrap().starts_with(":cd"));
        "[STDIN REDACTED]"
    })
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

    insta::with_settings!({
        redactions => vec![
          (".stdin", shell_command_cd_redaction())
        ]
    },{
        assert_cmd_snapshot!(cmd.arg("shell").pass_stdin(prompt));
    });
}

#[test]
fn test_shell_command_cd_no_arg() {
    let mut cmd = common::run_cli();

    insta::with_settings!({
        redactions => vec![
          (".stdin", shell_command_cd_redaction())
        ]
    },{
        assert_cmd_snapshot!(cmd.arg("shell").pass_stdin(":cd"));
    });
}

#[test]
fn test_shell_command_cd_too_many_arg() {
    let mut cmd = common::run_cli();

    insta::with_settings!({
        redactions => vec![
          (".stdin", shell_command_cd_redaction())
        ]
    },{
        assert_cmd_snapshot!(cmd.arg("shell").pass_stdin(":cd foo bar"));
    });
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

    insta::with_settings!({
        filters => vec![
            (r#"(\s+× )[^:\n]+(: Not a directory)"#, r#"$1[TEMP_DIR]$2"#),
        ],
        redactions => vec![
          (".stdin", shell_command_cd_redaction())
        ]
    },{
        assert_cmd_snapshot!(cmd.arg("shell").pass_stdin(prompt));
    });
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
    let (suffix, re) = if cfg!(windows) {
        (
            "windows",
            r#"(\s+× )[^:\n]+(: The system cannot find the file specified\. \(os error 2\))"#,
        )
    } else {
        (
            "unix",
            r#"(\s+× )[^:\n]+(: No such file or directory \(os error 2\))"#,
        )
    };

    insta::with_settings!({
        snapshot_suffix => suffix,
        filters => vec![
            (re, r#"$1[TEMP_DIR]$2"#),
        ],
        redactions => vec![
          (".stdin", shell_command_cd_redaction())
        ]
    },{
        assert_cmd_snapshot!(cmd.arg("shell").pass_stdin(prompt));
    });
}

// TODO add test for `:cd "foo bar"` and `:cd foo\ bar`
