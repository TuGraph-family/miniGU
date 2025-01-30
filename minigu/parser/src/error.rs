use crate::span::Span;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum Error {}

#[derive(Debug)]
pub(crate) enum UserErrorKind {
    InvalidToken,
    InvalidEscapeSequence,
    IncompleteComment,
    InvalidObjectRef,
}

#[derive(Debug)]
pub(crate) struct UserError {
    pub(crate) kind: UserErrorKind,
    pub(crate) span: Span,
}

impl UserError {
    pub(crate) fn invalid_token<S: Into<Span>>(span: S) -> Self {
        Self {
            kind: UserErrorKind::InvalidToken,
            span: span.into(),
        }
    }

    pub(crate) fn invalid_escape_sequence<S: Into<Span>>(span: S) -> Self {
        Self {
            kind: UserErrorKind::InvalidEscapeSequence,
            span: span.into(),
        }
    }

    pub(crate) fn incomplete_comment<S: Into<Span>>(span: S) -> Self {
        Self {
            kind: UserErrorKind::IncompleteComment,
            span: span.into(),
        }
    }

    pub(crate) fn invalid_object_ref<S: Into<Span>>(span: S) -> Self {
        Self {
            kind: UserErrorKind::InvalidObjectRef,
            span: span.into(),
        }
    }
}
