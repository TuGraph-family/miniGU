use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashSet;
use std::num::NonZeroUsize;

use gql_parser::error::TokenErrorKind;
use gql_parser::tokenize_full;
use itertools::Itertools;
use lru::LruCache;
use miette::IntoDiagnostic;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::HistoryHinter;
use rustyline::history::FileHistory;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{
    Completer, CompletionType, Config, Context, Editor, Helper, Highlighter, Hinter, Result,
    Validator,
};
use strum::VariantNames;

use super::command::ShellCommand;

pub type ShellEditor = Editor<ShellHelper, FileHistory>;

pub fn build_editor() -> miette::Result<ShellEditor> {
    let config = Config::builder()
        .history_ignore_space(true)
        .auto_add_history(true)
        .completion_type(CompletionType::List)
        .build();
    let mut editor = Editor::with_config(config).into_diagnostic()?;
    let helper = ShellHelper {
        completer: ShellCompleter::new(),
        highlighter: ShellHighlighter::new(),
        hinter: HistoryHinter::new(),
        validator: ShellValidator,
    };
    editor.set_helper(Some(helper));
    Ok(editor)
}

#[derive(Helper, Completer, Highlighter, Hinter, Validator)]
pub struct ShellHelper {
    #[rustyline(Completer)]
    completer: ShellCompleter,

    #[rustyline(Highlighter)]
    highlighter: ShellHighlighter,

    #[rustyline(Hinter)]
    hinter: HistoryHinter,

    #[rustyline(Validator)]
    validator: ShellValidator,
}

struct ShellHighlighter {
    query_cache: RefCell<LruCache<String, String>>,
    command_cache: RefCell<LruCache<String, String>>,
    commands: HashSet<&'static str>,
}

const DEFAULT_CACHE_CAP: usize = 256;

impl ShellHighlighter {
    fn new() -> Self {
        Self {
            query_cache: RefCell::new(LruCache::new(NonZeroUsize::new(DEFAULT_CACHE_CAP).unwrap())),
            command_cache: RefCell::new(LruCache::new(
                NonZeroUsize::new(DEFAULT_CACHE_CAP).unwrap(),
            )),
            commands: ShellCommand::VARIANTS.iter().copied().collect(),
        }
    }
}

impl Highlighter for ShellHighlighter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        if line.is_empty() {
            return line.into();
        }
        if line.trim_start().starts_with(":") {
            let cmd = if let Some(cmd) = line.split_whitespace().next() {
                cmd
            } else {
                return line.into();
            };
            let actual_cmd = cmd
                .strip_prefix(":")
                .expect("`cmd` should be prefixed with `:`");
            if !self.commands.contains(actual_cmd) {
                return line.into();
            }
            self.command_cache
                .borrow_mut()
                .get_or_insert_ref(line, || {
                    let mut highlighted = String::new();
                    // SAFETY: `cmd` is a substring of `line`.
                    let offset = unsafe { cmd.as_ptr().offset_from(line.as_ptr()) };
                    highlighted.push_str(&line[..offset as usize]);
                    highlighted.push_str(&format!("\x1b[1;33m{}\x1b[0m", cmd));
                    highlighted.push_str(&line[offset as usize + cmd.len()..]);
                    highlighted
                })
                .clone()
                .into()
        } else {
            self.query_cache
                .borrow_mut()
                .get_or_insert_ref(line, || {
                    let spans = tokenize_full(line)
                        .into_iter()
                        .filter_map(|t| match t {
                            Ok(token) if token.kind().is_reserved_word() => Some(token.span()),
                            _ => None,
                        })
                        .collect_vec();
                    let mut highlighted = String::new();
                    let mut offset = 0;
                    for span in spans {
                        let prefix = &line[offset..span.start];
                        if !prefix.is_empty() {
                            highlighted.push_str(prefix);
                        }
                        // Highlight the keyword with green color and make it bold.
                        highlighted.push_str(&format!("\x1b[1;32m{}\x1b[0m", &line[span.clone()]));
                        offset = span.end;
                    }
                    if offset < line.len() {
                        highlighted.push_str(&line[offset..]);
                    }
                    highlighted
                })
                .clone()
                .into()
        }
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        // Make the prompt bold.
        format!("\x1b[1m{prompt}\x1b[0m").into()
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        // Highlight the hint with Grey46 color.
        format!("\x1b[38;5;243m{hint}\x1b[0m").into()
    }

    fn highlight_char(&self, line: &str, _pos: usize, _kind: CmdKind) -> bool {
        !line.is_empty()
    }
}

/// Custom validator for the shell to support multi-line inputs.
pub struct ShellValidator;

fn is_query_complete(input: &str) -> bool {
    let tokens = tokenize_full(input);
    match tokens.last() {
        Some(Err(e)) if *e.kind() == TokenErrorKind::InvalidToken && e.slice() == ";" => true,
        Some(_) => false,
        None => true,
    }
}

impl Validator for ShellValidator {
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult> {
        if ctx.input().trim().starts_with(":") || is_query_complete(ctx.input()) {
            Ok(ValidationResult::Valid(None))
        } else {
            Ok(ValidationResult::Incomplete)
        }
    }
}

/// Custom helper for the shell. Only completer customized.
pub struct ShellCompleter {
    filename_completer: FilenameCompleter,
}

impl ShellCompleter {
    fn new() -> Self {
        Self {
            filename_completer: FilenameCompleter::new(),
        }
    }
}
impl Completer for ShellCompleter {
    type Candidate = Pair;

    // Command completion first, otherwise fallback to filename completion
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context,
    ) -> Result<(usize, Vec<Self::Candidate>)> {
        if line.trim_start().starts_with(":") {
            let cmd = line
                .trim_start()
                .strip_prefix(":")
                .expect("`line` should be prefixed with `:`");
            let candidates = ShellCommand::VARIANTS
                .iter()
                .filter(|candidate| candidate.starts_with(cmd))
                .map(|candidate| Pair {
                    display: ":".to_string() + candidate,
                    replacement: ":".to_string() + candidate,
                })
                .collect();
            Ok((0, candidates))
        } else {
            self.filename_completer.complete(line, pos, ctx)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_query_complete() {
        assert!(is_query_complete(""));
        assert!(is_query_complete("  "));
        assert!(!is_query_complete("MATCH (n) return n"));
        assert!(is_query_complete("MATCH (n) return n;"));
    }

    #[test]
    fn test_is_query_complete_multiple_lines() {
        assert!(!is_query_complete("MATCH (n) return n; \ncommit"));
        assert!(is_query_complete("MATCH (n) return n; \ncommit;"));
    }

    #[test]
    fn test_is_query_complete_with_comments() {
        assert!(is_query_complete("MATCH (n) return n; -- comment"));
        assert!(!is_query_complete("MATCH (n) return n -- comment;"));
        assert!(is_query_complete("MATCH (n) return n -- comment;\n;"));
    }
}
