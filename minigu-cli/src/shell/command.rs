use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{ColorChoice, Command, CommandFactory, FromArgMatches, Parser, ValueEnum};
use itertools::Itertools;
use miette::{IntoDiagnostic, Result};
use strum::{Display, VariantNames};

use super::context::ShellContext;
use crate::shell::output::OutputMode;

pub fn build_command() -> Command {
    ShellCommand::command()
        .multicall(true)
        .help_template("{before-help}{subcommands}{after-help}")
        .before_help("Usage hints:")
        .after_help("Enter \":help <COMMAND>\" for more information about a command.")
        .color(ColorChoice::Never)
        .disable_colored_help(true)
        .disable_help_flag(true)
        .disable_help_subcommand(true)
}

#[derive(Debug, Parser, VariantNames)]
#[strum(serialize_all = "kebab-case")]
pub enum ShellCommand {
    /// Show usage hints.
    #[command(name = ":help")]
    Help {
        /// The command to show help for.
        /// If not provided, the help for all commands will be shown.
        command: Option<String>,
    },

    /// Exit the shell.
    #[command(name = ":quit")]
    Quit,

    /// Show command history.
    #[command(name = ":history")]
    History,

    /// Set output mode.
    #[command(name = ":mode")]
    Mode {
        /// The output mode to change to.
        /// If not provided, the current output mode will be printed.
        mode_to_change: Option<OutputMode>,
    },

    /// Set if query metrics should be printed.
    #[command(name = ":metrics")]
    Metrics {
        /// The status to change to.
        /// If not provided, the current status will be printed.
        status: Option<CliStatus>,
    },
}

#[derive(Debug, Clone, ValueEnum, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum CliStatus {
    On,
    Off,
}

impl From<CliStatus> for bool {
    fn from(status: CliStatus) -> Self {
        matches!(status, CliStatus::On)
    }
}

impl From<bool> for CliStatus {
    fn from(status: bool) -> Self {
        if status {
            CliStatus::On
        } else {
            CliStatus::Off
        }
    }
}

impl ShellCommand {
    pub fn execute_from_input(ctx: &mut ShellContext, input: &str) -> Result<()> {
        assert!(input.starts_with(":"));
        let input = input.split_whitespace().collect_vec();
        let matches = ctx
            .command
            .try_get_matches_from_mut(input)
            .map_err(|e| match e.kind() {
                ErrorKind::InvalidSubcommand => {
                    let invalid = e
                        .get(ContextKind::InvalidSubcommand)
                        .expect("invalid subcommand should be provided");
                    let diag = miette::diagnostic!("unknown command: \"{}\"", invalid);
                    let help = match e.get(ContextKind::SuggestedSubcommand) {
                        Some(ContextValue::Strings(s)) if s.len() == 1 => {
                            format!("did you mean \"{}\"?", s[0])
                        }
                        _ => "enter \":help\" for usage hints".into(),
                    };
                    diag.with_help(help)
                }
                ErrorKind::UnknownArgument => {
                    let arg = e
                        .get(ContextKind::InvalidArg)
                        .expect("invalid arg should be provided");
                    miette::diagnostic!("unknown argument: \"{}\"", arg)
                        .with_help("enter \":help\" for usage hints")
                }
                ErrorKind::InvalidValue => {
                    let invalid_arg = e
                        .get(ContextKind::InvalidArg)
                        .expect("invalid arg should be provided");
                    let invalid_value = e
                        .get(ContextKind::InvalidValue)
                        .expect("invalid value should be provided");
                    let diag = miette::diagnostic!(
                        "invalid value for argument {}: \"{}\"",
                        invalid_arg,
                        invalid_value
                    );
                    let mut help = match e.get(ContextKind::ValidValue) {
                        Some(ContextValue::Strings(s)) => {
                            let values = s.iter().map(|s| format!("\"{}\"", s)).join(", ");
                            format!("possible values: {}", values)
                        }
                        _ => String::new(),
                    };
                    if let Some(ContextValue::String(s)) = e.get(ContextKind::SuggestedValue) {
                        help.push_str(&format!("\ndid you mean \"{}\"?", s));
                    }
                    if help.is_empty() {
                        diag
                    } else {
                        diag.with_help(help)
                    }
                }
                // TODO: Handle other error kinds.
                _ => miette::diagnostic!("{e:?}"),
            })?;
        let cmd = Self::from_arg_matches(&matches).into_diagnostic()?;

        match cmd {
            ShellCommand::Help { command } => help(ctx, command),
            ShellCommand::Quit => quit(ctx),
            ShellCommand::History => history(ctx),
            Self::Mode { mode_to_change } => mode(ctx, mode_to_change),
            Self::Metrics { status } => metrics(ctx, status),
        }
    }
}

fn help(ctx: &mut ShellContext, command: Option<String>) -> Result<()> {
    if let Some(command) = command {
        let subcommand = ctx
            .command
            .find_subcommand_mut(&command)
            .ok_or_else(|| miette::diagnostic!("unknown command: \"{}\"", command))?;
        println!("{}", subcommand.render_long_help());
    } else {
        println!("{}", ctx.command.render_help());
    }
    Ok(())
}

fn quit(ctx: &mut ShellContext) -> Result<()> {
    ctx.should_quit = true;
    Ok(())
}

fn history(ctx: &mut ShellContext) -> Result<()> {
    for (index, line) in ctx.editor.history().iter().enumerate() {
        println!("{}\t{}", index + 1, line);
    }
    Ok(())
}

fn mode(ctx: &mut ShellContext, mode_to_change: Option<OutputMode>) -> Result<()> {
    if let Some(mode_to_change) = mode_to_change {
        ctx.mode = mode_to_change;
    } else {
        println!("current output mode: {}", ctx.mode);
    }
    Ok(())
}

fn metrics(ctx: &mut ShellContext, status: Option<CliStatus>) -> Result<()> {
    if let Some(status) = status {
        ctx.show_metrics = status.into()
    } else {
        let status = CliStatus::from(ctx.show_metrics);
        println!("show query metrics: {status}");
    }
    Ok(())
}
