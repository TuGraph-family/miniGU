use std::collections::HashMap;
use std::sync::LazyLock;

use clap::Parser;
use miette::{IntoDiagnostic, Result, bail};
use minigu::{Database, Session};
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::MatchingBracketHighlighter;
use rustyline::hint::HistoryHinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{
    Completer, CompletionType, Config, Context, Editor, Helper, Highlighter, Hinter, Validator,
};

// Supported commands
static COMMANDS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(":help", "Show usage hints");
    map.insert(":history", "Print the command history");
    map.insert(":quit", "Exit the shell");
    map
});

/// Custom helper for the CLI. Only completer customized
struct CliCompleter {
    filename_completer: FilenameCompleter,
    commands: Vec<String>,
}

impl CliCompleter {
    pub fn new() -> Self {
        Self {
            filename_completer: FilenameCompleter::new(),
            commands: COMMANDS.keys().map(|s| s.to_string()).collect(),
        }
    }
}

impl Completer for CliCompleter {
    type Candidate = Pair;

    // Command completion first, otherwise fallback to filename completion
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        if line.starts_with(":") {
            let candidates = self
                .commands
                .iter()
                .filter(|cmd| cmd.starts_with(line))
                .map(|cmd| Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                })
                .collect();
            Ok((0, candidates))
        } else {
            self.filename_completer.complete(line, pos, ctx)
        }
    }
}

#[derive(Helper, Completer, Highlighter, Hinter, Validator)]
struct CliHelper {
    #[rustyline(Completer)]
    completer: CliCompleter,
    #[rustyline(Highlighter)]
    highlighter: MatchingBracketHighlighter,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
}

/// Start the local interactive shell.
#[derive(Debug, Parser, Clone)]
pub struct Shell {}

impl Shell {
    pub fn run(&self) -> Result<()> {
        let db = Database::open_in_memory()?;
        let session = db.session()?;
        let config = Config::builder()
            .history_ignore_space(true)
            .auto_add_history(true)
            .completion_type(CompletionType::List)
            .build();
        let mut editor = Editor::with_config(config).into_diagnostic()?;
        let h = CliHelper {
            completer: CliCompleter::new(),
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter::new(),
            validator: MatchingBracketValidator::new(),
        };
        editor.set_helper(Some(h));

        ShellContext {
            session,
            editor,
            should_quit: false,
        }
        .enter_loop()
    }
}

struct ShellContext {
    session: Session,
    editor: Editor<CliHelper, DefaultHistory>,
    should_quit: bool,
}

impl ShellContext {
    fn enter_loop(mut self) -> Result<()> {
        self.print_prologue();
        loop {
            let result = match self.editor.readline("minigu> ") {
                Ok(line) if line.is_empty() => continue,
                Ok(line) if line.starts_with(":") => self.execute_command(line),
                Ok(line) => self.execute_query(line),
                Err(ReadlineError::Interrupted) => continue,
                Err(ReadlineError::Eof) => return Ok(()),
                Err(e) => return Err(e).into_diagnostic(),
            };
            // Handle recoverable errors.
            if let Err(e) = result {
                println!("{e:?}");
            }
            if self.should_quit {
                return Ok(());
            }
        }
    }

    fn print_prologue(&self) {
        println!(r#"Enter ":help" for usage hints."#);
    }

    fn print_help(&self) {
        println!(r"Usage hints:");
        let max_cmd_len = COMMANDS.keys().map(|cmd| cmd.len()).max().unwrap_or(0);
        for (cmd, desc) in &*COMMANDS {
            println!("{:<width$} {}", cmd, desc, width = max_cmd_len + 2);
        }
    }

    fn execute_query(&self, input: String) -> Result<()> {
        Ok(self.session.query(&input)?)
    }

    fn execute_command(&mut self, input: String) -> Result<()> {
        let command = input
            .strip_prefix(":")
            .expect("`input` should be prefixed with `:`")
            .trim();
        match command {
            "quit" => self.should_quit = true,
            "help" => self.print_help(),
            "history" => {
                for (index, line) in self.editor.history().iter().enumerate() {
                    println!("{}\t{}", index + 1, line);
                }
            }
            _ => bail!("unknown command: {command}"),
        }
        Ok(())
    }
}
