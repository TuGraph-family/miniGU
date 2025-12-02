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
    let mut cmd = common::run_cli();
    let dir = tempfile::tempdir().unwrap();
    let prompt = format!(":cd {}", dir.path().display());

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
    let mut cmd = common::run_cli();
    let file = tempfile::NamedTempFile::new().unwrap();
    let prompt = format!(":cd {}", file.path().display());

    insta::with_settings!({
        filters => vec![
            (r#"(\s+× )[^:\n]+(: not a directory)"#, r#"$1[TEMP_DIR]$2"#),
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
    let mut cmd = common::run_cli();
    let dirname = "😅";
    let dir = tempfile::tempdir().unwrap();
    let prompt = format!(":cd {}", dir.path().join(dirname).display());

    insta::with_settings!({
        filters => vec![
            (r#"(\s+× )[^:\n]+(: No such file or directory \(os error 2\))"#, r#"$1[TEMP_DIR]$2"#),
        ],
        redactions => vec![
          (".stdin", shell_command_cd_redaction())
        ]
    },{
        assert_cmd_snapshot!(cmd.arg("shell").pass_stdin(prompt));
    });
}

// TODO add test for `:cd "foo bar"` and `:cd foo\ bar`
