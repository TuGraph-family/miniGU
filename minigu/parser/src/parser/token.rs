use logos::Logos;
use winnow::error::ParserError;
use winnow::stream::{ContainsToken, Stream, StreamIsPartial, TokenSlice};
use winnow::token::one_of;
use winnow::{Parser, Stateful};

use crate::lexer::{LexerError, TokenKind};
use crate::span::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Token<'a> {
    pub(super) kind: TokenKind<'a>,
    pub(super) slice: &'a str,
    pub(super) span: Span,
}

impl<'a> Token<'a> {
    #[inline(always)]
    pub(super) fn kind(&self) -> &TokenKind<'a> {
        &self.kind
    }

    #[inline(always)]
    pub(super) fn span(&self) -> Span {
        self.span
    }
}

impl<'a> ContainsToken<&Token<'a>> for &TokenKind<'a> {
    #[inline(always)]
    fn contains_token(&self, token: &Token<'a>) -> bool {
        token.kind.eq(self)
    }
}

impl<I, E> Parser<I, I::Token, E> for TokenKind<'_>
where
    I: Stream + StreamIsPartial,
    I::Token: Clone,
    E: ParserError<I>,
    for<'b> &'b Self: ContainsToken<I::Token>,
{
    fn parse_next(&mut self, i: &mut I) -> Result<I::Token, E> {
        one_of(&*self).parse_next(i)
    }
}

#[derive(Debug, Default)]
pub(super) struct State {
    recursion: usize,
}

pub(super) type TokenStream<'a, 'b> = Stateful<TokenSlice<'b, Token<'a>>, State>;

pub(super) fn tokenize(input: &str) -> Result<Vec<Token<'_>>, LexerError> {
    let mut lexer = TokenKind::lexer(input).spanned();
    let mut tokens = Vec::new();
    while let Some((kind, span)) = lexer.next() {
        let kind = kind?;
        let slice = lexer.slice();
        let span = span.try_into().map_err(|_| LexerError::InvalidToken)?;
        tokens.push(Token { kind, slice, span });
    }
    Ok(tokens)
}

pub(super) fn build_token_stream<'a, 'b>(input: &'b [Token<'a>]) -> TokenStream<'a, 'b> {
    let input = TokenSlice::new(input);
    Stateful {
        input,
        state: State::default(),
    }
}
