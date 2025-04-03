use core::fmt::{self, Debug, Display, Formatter};
use core::marker::PhantomData;
use core::ops::Range;

#[cfg(feature = "miette")]
use miette::Diagnostic;
use thiserror::Error;

use crate::imports::Arc;
use crate::lexer::LexerError;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "miette", derive(Diagnostic))]
pub enum Error {
    #[error("syntax error: unexpected eof")]
    UnexpectedEof,

    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    InvalidToken(TokenError<InvalidToken>),

    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    IncompleteComment(TokenError<IncompleteComment>),

    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Unexpected(UnexpectedError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TokenError<T> {
    input: Arc<str>,
    span: Range<usize>,
    position: (usize, usize),
    #[cfg_attr(feature = "serde", serde(skip))]
    _marker: PhantomData<T>,
}

impl<T> TokenError<T> {
    pub fn input(&self) -> &Arc<str> {
        &self.input
    }

    pub fn span(&self) -> &Range<usize> {
        &self.span
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidToken;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncompleteComment;

impl Display for TokenError<InvalidToken> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let token = &self.input[self.span.clone()];
        let (line, column) = self.position;
        write!(
            f,
            "syntax error at or near line {line}, column {column}: invalid token \"{token}\""
        )
    }
}

impl Display for TokenError<IncompleteComment> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (line, column) = self.position;
        write!(
            f,
            "syntax error at or near line {line}, column {column}: incomplete comment"
        )
    }
}

impl<T> core::error::Error for TokenError<T> where TokenError<T>: Debug + Display {}

#[cfg(feature = "miette")]
impl<T> Diagnostic for TokenError<T>
where
    TokenError<T>: core::error::Error,
{
    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        Some(&self.input)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        Some(Box::new(core::iter::once(
            miette::LabeledSpan::new_with_span(Some("here".into()), self.span.clone()),
        )))
    }
}

// TODO: This is a temporary error type for the parser. Remove this once concrete parser errors are
// implemented.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "miette", derive(Diagnostic))]
pub struct UnexpectedError {
    #[cfg_attr(feature = "miette", source_code)]
    input: Arc<str>,
    #[cfg_attr(feature = "miette", label("here"))]
    span: Range<usize>,
    position: (usize, usize),
}

impl Display for UnexpectedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (line, column) = self.position;
        write!(
            f,
            "syntax error at or near line {line}, column {column}: unexpected error"
        )
    }
}

impl core::error::Error for UnexpectedError {}

impl Error {
    pub(crate) fn unexpected(input: &str, span: Range<usize>) -> Self {
        let offset = span.start;
        Error::Unexpected(UnexpectedError {
            input: input.into(),
            span,
            position: translate_offset_to_line_column(input, offset),
        })
    }

    pub(crate) fn from_lexer_error(err: LexerError, input: &str, span: Range<usize>) -> Self {
        let offset = span.start;
        let position = translate_offset_to_line_column(input, offset);
        let input = input.into();
        match err {
            LexerError::InvalidToken => Self::InvalidToken(TokenError {
                input,
                span,
                position,
                _marker: PhantomData,
            }),
            LexerError::IncompleteComment => Self::IncompleteComment(TokenError {
                input,
                span,
                position,
                _marker: PhantomData,
            }),
        }
    }
}

fn translate_offset_to_line_column(input: &str, offset: usize) -> (usize, usize) {
    assert!(!input.is_empty(), "`input` should not be empty");
    assert!(
        input.is_char_boundary(offset),
        "`offset` must be a valid character boundary"
    );
    for (line_idx, line) in input.lines().enumerate() {
        // SAFETY: `line` and `input` are both derived from the same allocated string.
        let start_offset = unsafe { line.as_ptr().byte_offset_from(input.as_ptr()) as usize };
        let end_offset = start_offset + line.len();
        if offset <= end_offset {
            let column_idx = line
                .char_indices()
                .enumerate()
                .find_map(|(column_idx, (byte_idx, _))| {
                    (byte_idx == offset - start_offset).then_some(column_idx)
                })
                .expect("`column_idx` should be found successfully");
            return (line_idx + 1, column_idx + 1);
        }
    }
    unreachable!("`offset` should be within the range of `input`");
}

#[cfg(test)]
mod tests {
    use crate::error::translate_offset_to_line_column;

    #[test]
    fn test_translate_1() {
        let input = r"
This
is
a
multi-line
string.
";
        let position = translate_offset_to_line_column(input, 7);
        assert_eq!(position, (3, 2));
    }

    #[test]
    fn test_translate_2() {
        let input = "This\nis\na\r\nmulti-line\n\nstring\r.";
        let position = translate_offset_to_line_column(input, 24);
        assert_eq!(position, (6, 2));
    }

    #[test]
    fn test_translate_3() {
        let input = "这是\n一个\r\n多行\n\n字符串\r.";
        let offset = input.find("串").unwrap();
        let position = translate_offset_to_line_column(input, offset);
        assert_eq!(position, (5, 3));
    }
}
