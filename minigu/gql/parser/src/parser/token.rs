use core::ops::Range;

use logos::Logos;
use winnow::error::ParserError;
use winnow::stream::{ContainsToken, Location, Stream, StreamIsPartial, TokenSlice};
use winnow::{Parser, Stateful};

use super::options::ParseOptionsInner;
use crate::error::{Error, TokenizeError};
use crate::imports::Vec;
use crate::lexer::TokenKind;

/// A wrapper around [`winnow::token::any`] to return [`TokenKind`] directly.
///
/// If the matched slice is needed, use [`winnow::token::any`] instead.
#[inline(always)]
pub(super) fn any<'a: 'b, 'b, I, E>(input: &mut I) -> Result<&'b TokenKind<'a>, E>
where
    I: Stream<Token = &'b Token<'a>> + StreamIsPartial,
    E: ParserError<I>,
{
    winnow::token::any
        .map(|t: &Token| &t.kind)
        .parse_next(input)
}

/// A token with its kind, slice, and span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub(super) kind: TokenKind<'a>,
    pub(super) slice: &'a str,
    pub(super) span: Range<usize>,
}

impl<'a> Token<'a> {
    #[inline]
    pub fn new(kind: TokenKind<'a>, slice: &'a str, span: Range<usize>) -> Self {
        Self { kind, slice, span }
    }

    #[inline]
    pub fn kind(&self) -> &TokenKind<'a> {
        &self.kind
    }

    #[inline]
    pub fn slice(&self) -> &'a str {
        self.slice
    }

    #[inline]
    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }
}

impl<'a> ContainsToken<&Token<'a>> for &TokenKind<'a> {
    #[inline(always)]
    fn contains_token(&self, token: &Token<'a>) -> bool {
        token.kind.eq(self)
    }
}

impl<'a> ContainsToken<&Token<'a>> for TokenKind<'a> {
    #[inline(always)]
    fn contains_token(&self, token: &Token<'a>) -> bool {
        token.kind.eq(self)
    }
}

impl<'a, 'b, I, E> Parser<I, I::Token, E> for TokenKind<'a>
where
    I: Stream<Token = &'b Token<'a>> + StreamIsPartial,
    E: ParserError<I>,
{
    #[inline(always)]
    fn parse_next(&mut self, i: &mut I) -> Result<I::Token, E> {
        winnow::token::any
            .verify(|t: &Token| t.kind.eq(self))
            .parse_next(i)
    }
}

impl Location for Token<'_> {
    #[inline(always)]
    fn previous_token_end(&self) -> usize {
        self.span.end
    }

    #[inline(always)]
    fn current_token_start(&self) -> usize {
        self.span.start
    }
}

#[derive(Debug)]
pub(super) struct State {
    recursion: usize,
    options: ParseOptionsInner,
}

impl State {
    pub(super) fn unescape(&self) -> bool {
        self.options.unescape()
    }
}

pub(super) type TokenStream<'a, 'b> = Stateful<TokenSlice<'b, Token<'a>>, State>;

/// Tokenizes the input string and returns a vector of tokens or errors.
///
/// This is different from [`tokenize`] in that this collects all errors to the resulting vector
/// rather than returning early.
///
/// # Examples
///
/// ```
/// # use gql_parser::{tokenize_full, Token, TokenKind};
/// # use gql_parser::error::{TokenizeError, TokenErrorKind};
/// let tokens = tokenize_full("COMMIT;");
/// assert_eq!(tokens, vec![
///     Ok(Token::new(TokenKind::Commit, "COMMIT", 0..6)),
///     Err(TokenizeError::new(TokenErrorKind::InvalidToken, ";", 6..7))
/// ]);
/// ```
pub fn tokenize_full(input: &str) -> Vec<Result<Token<'_>, TokenizeError<'_>>> {
    let mut lexer = TokenKind::lexer(input).spanned();
    let mut tokens = Vec::new();
    while let Some((kind, span)) = lexer.next() {
        match kind {
            Ok(kind) => {
                let slice = lexer.slice();
                tokens.push(Ok(Token { kind, slice, span }));
            }
            Err(e) => {
                let slice = lexer.slice();
                tokens.push(Err(TokenizeError::new(e, slice, span)));
            }
        }
    }
    tokens
}

/// Tokenizes the input string and returns a vector of tokens.
///
/// This can be used as the building block of a GQL parser/analyzer/syntax highlighter, etc.
///
/// # Errors
///
/// This returns a [`TokenizeError`] if the input string cannot be tokenized successfully.
///
/// # Examples
///
/// ```
/// # use gql_parser::{tokenize, Token, TokenKind};
/// let tokens = tokenize("COMMIT").unwrap();
/// assert_eq!(tokens, vec![Token::new(TokenKind::Commit, "COMMIT", 0..6)]);
/// ```
pub fn tokenize(input: &str) -> Result<Vec<Token<'_>>, TokenizeError<'_>> {
    let mut lexer = TokenKind::lexer(input).spanned();
    let mut tokens = Vec::new();
    while let Some((kind, span)) = lexer.next() {
        match kind {
            Ok(kind) => {
                let slice = lexer.slice();
                tokens.push(Token { kind, slice, span });
            }
            Err(e) => {
                return Err(TokenizeError::new(e, input, span));
            }
        }
    }
    Ok(tokens)
}

pub(super) fn build_token_stream<'a, 'b>(
    input: &'b [Token<'a>],
    options: ParseOptionsInner,
) -> TokenStream<'a, 'b> {
    let input = TokenSlice::new(input);
    Stateful {
        input,
        state: State {
            recursion: 0,
            options,
        },
    }
}
