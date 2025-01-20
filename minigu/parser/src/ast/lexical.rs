//! AST definitions for *lexical elements*.

use super::expression::RecordConstructor;
use super::ListConstructor;
use crate::macros::{base, ext};
use crate::span::Span;
use crate::Cow;

/// An identifier in the query string.
#[apply(base)]
pub struct Ident<'a> {
    pub name: Cow<'a, str>,
    pub span: Span,
}

#[apply(base)]
pub enum Literal<'a> {
    Numeric(&'a str),
    Boolean(BooleanLiteral),
    String(StringLiteral<'a>),
    Temporal(TemporalLiteral<'a>),
    Duration(DurationLiteral<'a>),
    Null,
    List(ListConstructor<'a>),
    Record(RecordConstructor<'a>),
}

#[apply(base)]
pub struct StringLiteral<'a> {
    pub kind: StringLiteralKind,
    pub literal: Cow<'a, str>,
}

#[apply(ext)]
pub enum StringLiteralKind {
    Char,
    Byte,
}

#[apply(ext)]
pub enum BooleanLiteral {
    True,
    False,
    Unknown,
}

#[apply(base)]
pub struct TemporalLiteral<'a> {
    pub kind: TemporalLiteralKind,
    pub literal: Cow<'a, str>,
}

#[apply(ext)]
pub enum TemporalLiteralKind {
    Date,
    Time,
    Datetime,
    Timestamp,
    SqlDatetime,
}

#[apply(base)]
pub struct DurationLiteral<'a> {
    pub kind: DurationLiteralKind,
    pub literal: Cow<'a, str>,
}

#[apply(ext)]
pub enum DurationLiteralKind {
    Duration,
    SqlInterval,
}
