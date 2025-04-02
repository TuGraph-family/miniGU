use clap::Parser;
use miette::{IntoDiagnostic, Result, bail};
use minigu::{Database, Session};
use rustyline::{DefaultEditor, Config, CompletionType};
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;

/// Start the local interactive shell.
#[derive(Debug, Parser, Clone)]
pub struct Shell {}

impl Shell {
    pub fn run(&self) -> Result<()> {
        let db = Database::open_in_memory()?;
        let session = db.session()?;
        let config = Config::builder().auto_add_history(true).build();
        let mut editor = DefaultEditor::with_config(config).into_diagnostic()?;
        
        ShellContext {
            session,
            editor,
            should_quit: false,
        }
        .enter_loop()
    }

    pub fn execute_file(&self, file: String) -> Result<()> {
        let db = Database::open_in_memory()?;
        let session = db.session()?;
        let editor = DefaultEditor::new().into_diagnostic()?;
        ShellContext {
            session,
            editor,
            should_quit: false,
        }
        .execute_file(file)
    }
}

struct ShellContext {
    session: Session,
    editor: DefaultEditor,
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
        println!(
            r"Usage hints:
:help       Show usage hints.
:history    Print the command history.
:quit       Exit the shell.
"
        );
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
            "quit" => {
                self.should_quit = true;
            }
            "help" => self.print_help(),
            "history" => {
                for line in self.editor.history().iter() {
                    println!("{}", line);
                }
            }
            _ => bail!("unknown command: {command}"),
        }
        Ok(())
    }

    fn execute_file(mut self, file: String) -> Result<()> {
        use std::fs;
        let content = match fs::read_to_string(&file) {
            Ok(content) => content,
            Err(e) => {
                println!("Error reading file {}: {:?}", file, e);
                return Err(e).into_diagnostic();
            }
        };
        for line in content.lines() {
            let result = match line {
                line if line.is_empty() => continue,
                line if line.starts_with(":") => self.execute_command(line.to_string()),
                line => self.execute_query(line.to_string()),
            };
            // Handle recoverable errors.
            if let Err(e) = result {
                println!("{e:?}");
            }
            if self.should_quit {
                return Ok(());
            }
        }
        Ok(())
    }
}
