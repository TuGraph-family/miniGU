use crate::span::Span;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum Error {}

#[derive(Debug)]
pub(crate) enum UserError {
    InvalidToken(Span),
    InvalidEscapeSequence(Span),
}
