//! AST definitions for *lexical elements*.

use super::ListConstructor;
use super::expression::RecordConstructor;
use crate::Cow;
use crate::macros::{base, ext};
use crate::span::Span;

/// An identifier or parameter in the query string.
#[apply(base)]
pub struct Ident<'a> {
    pub name: Cow<'a, str>,
    pub span: Span,
}

#[apply(base)]
pub enum Literal<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Numeric(UnsignedNumericLiteral<'a>),
    Boolean(BooleanLiteral),
    String(StringLiteral<'a>),
    Temporal(TemporalLiteral<'a>),
    Duration(DurationLiteral<'a>),
    List(ListConstructor<'a>),
    Record(RecordConstructor<'a>),
    Null,
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

#[apply(ext)]
pub enum UnsignedNumericLiteral<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Integer(UnsignedInteger<'a>),
}

#[apply(ext)]
pub enum UnsignedIntegerKind {
    Binary,
    Octal,
    Decimal,
    Hex,
}

#[apply(ext)]
pub struct UnsignedInteger<'a> {
    pub kind: UnsignedIntegerKind,
    pub integer: &'a str,
}
