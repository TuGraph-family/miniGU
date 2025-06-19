use clap::Command;
use gql_parser::error::TokenErrorKind;
use gql_parser::tokenize_full;
use miette::{IntoDiagnostic, Result};
use minigu::common::data_chunk::display::{TableBuilder, TableOptions};
use minigu::session::Session;
use rustyline::error::ReadlineError;

use super::OutputMode;
use super::command::ShellCommand;
use super::editor::ShellEditor;

const PROLOGUE: &str = r#"Enter ":help" for usage hints."#;

pub struct ShellContext {
    pub session: Session,
    pub editor: ShellEditor,
    pub command: Command,
    pub should_quit: bool,
    pub mode: OutputMode,
    pub header: bool,
    pub column_type: bool,
}

impl ShellContext {
    pub fn run(mut self) -> Result<()> {
        println!("{}", PROLOGUE);
        while !self.should_quit {
            let result = match self.editor.readline("minigu> ") {
                Ok(line) => {
                    let trimmed = line.trim_start();
                    if trimmed.is_empty() {
                        continue;
                    } else if trimmed.starts_with(":") {
                        self.execute_command(trimmed)
                    } else {
                        self.execute_query(trimmed)
                    }
                }
                Err(ReadlineError::Interrupted) => continue,
                Err(ReadlineError::Eof) => return Ok(()),
                Err(e) => return Err(e).into_diagnostic(),
            };
            // Handle recoverable errors.
            if let Err(e) = result {
                println!("{e:?}");
            }
        }
        Ok(())
    }

    fn execute_query(&mut self, input: &str) -> Result<()> {
        let segments = split_query(input);
        for segment in segments {
            // Print error for each segment
            if let Err(e) = self.execute_query_segment(segment) {
                println!("{e:?}");
            }
        }
        Ok(())
    }

    fn execute_query_segment(&mut self, segment: &str) -> Result<()> {
        let result = self.session.query(segment)?;
        let options = TableOptions::new()
            .with_style(self.mode.into())
            .with_type_info(self.column_type);
        let mut builder = if self.header {
            TableBuilder::new(result.schema().cloned(), options)
        } else {
            TableBuilder::new(None, options)
        };
        let mut num_rows = 0;
        for chunk in result {
            let chunk = chunk;
            num_rows += chunk.cardinality();
            builder = builder.append_chunk(&chunk);
        }
        let table = builder.build();
        println!("{table}");
        println!("({} rows)", num_rows);
        Ok(())
    }

    fn execute_command(&mut self, input: &str) -> Result<()> {
        ShellCommand::execute_from_input(self, input)
    }
}

fn split_query(input: &str) -> Vec<&str> {
    let mut offset = 0;
    let mut segments = Vec::new();
    let tokens = tokenize_full(input);
    // The validator guarantees that the last token is a semicolon.
    assert!(
        matches!(tokens.last(), Some(Err(e)) if *e.kind() == TokenErrorKind::InvalidToken && e.slice() == ";"),
        "`tokens` should be terminated with a semicolon"
    );
    for token in tokens {
        match token {
            Err(e) if *e.kind() == TokenErrorKind::InvalidToken && e.slice() == ";" => {
                let segment = &input[offset..e.span().start];
                if !segment.trim().is_empty() {
                    segments.push(segment);
                }
                offset = e.span().end;
            }
            _ => (),
        }
    }
    segments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_query_1() {
        let input = "match (n) return n; ";
        let segments = split_query(input);
        assert_eq!(segments, vec!["match (n) return n"]);
    }

    #[test]
    fn test_split_query_2() {
        let input = " match (n) return n;; ; commit;";
        let segments = split_query(input);
        assert_eq!(segments, vec![" match (n) return n", " commit"]);
    }
}
