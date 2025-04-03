use core::ops::Range;

use logos::Logos;
use winnow::error::ParserError;
use winnow::stream::{ContainsToken, Location, Stream, StreamIsPartial, TokenSlice};
use winnow::{Parser, Stateful};

use super::options::ParseOptionsInner;
use crate::error::Error;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Token<'a> {
    pub(super) kind: TokenKind<'a>,
    pub(super) slice: &'a str,
    pub(super) span: Range<usize>,
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

pub(super) fn tokenize(input: &str) -> Result<Vec<Token<'_>>, Error> {
    let mut lexer = TokenKind::lexer(input).spanned();
    let mut tokens = Vec::new();
    while let Some((kind, span)) = lexer.next() {
        match kind {
            Ok(kind) => {
                let slice = lexer.slice();
                tokens.push(Token { kind, slice, span });
            }
            Err(e) => {
                return Err(Error::from_lexer_error(e, input, span));
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
