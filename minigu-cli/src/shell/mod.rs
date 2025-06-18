mod command;
mod context;
mod editor;
mod output;

use std::path::PathBuf;

use clap::Parser;
use command::build_command;
use context::ShellContext;
use editor::build_editor;
use miette::Result;
use minigu::database::Database;
use output::OutputMode;

/// Start local interactive shell.
#[derive(Debug, Parser, Clone)]
pub struct ShellArgs {
    /// Path to the database directory. If it does not exist, a new database directory will be
    /// created.
    ///
    /// If not provided, an in-memory database will be opened.
    path: Option<PathBuf>,

    /// Set output mode.
    #[arg(short, long, default_value = "sharp")]
    mode: OutputMode,

    /// If set, the column header will not be printed.
    #[arg(long)]
    no_header: bool,

    /// If set, column types (in the header) will not be printed.
    #[arg(long)]
    no_column_type: bool,

    /// If set, the database will be opened in read-only mode.
    ///
    /// Ignored if an in-memory database is opened.
    #[arg(short, long)]
    read_only: bool,
}

impl ShellArgs {
    pub fn run(self) -> Result<()> {
        let db = if let Some(path) = self.path {
            Database::open(path)?
        } else {
            Database::open_in_memory()?
        };
        let session = db.session()?;
        let editor = build_editor()?;
        let command = build_command();
        let context = ShellContext {
            session,
            editor,
            command,
            should_quit: false,
            mode: self.mode,
            header: !self.no_header,
            column_type: !self.no_column_type,
        };
        context.run()
    }
}
