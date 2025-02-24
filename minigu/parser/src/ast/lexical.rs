//! AST definitions for *lexical elements*.

use smol_str::SmolStr;

use super::{ListConstructor, RecordConstructor};
use crate::macros::{base, ext};
use crate::span::Span;

/// An identifier or parameter in the query string.
#[apply(base)]
pub struct Ident {
    pub name: SmolStr,
    pub span: Span,
}

#[apply(base)]
pub struct Literal {
    pub kind: LiteralKind,
    pub span: Span,
}

#[apply(base)]
pub enum LiteralKind {
    Numeric(UnsignedNumericLiteral),
    Boolean(BooleanLiteral),
    String(StringLiteral),
    Temporal(TemporalLiteral),
    Duration(DurationLiteral),
    List(ListConstructor),
    Record(RecordConstructor),
    Null,
}

#[apply(base)]
pub struct StringLiteral {
    pub kind: StringLiteralKind,
    pub literal: SmolStr,
    pub span: Span,
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
pub struct TemporalLiteral {
    pub kind: TemporalLiteralKind,
    pub literal: SmolStr,
    pub span: Span,
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
pub struct DurationLiteral {
    pub kind: DurationLiteralKind,
    pub literal: SmolStr,
    pub span: Span,
}

#[apply(ext)]
pub enum DurationLiteralKind {
    Duration,
    SqlInterval,
}

#[apply(base)]
pub enum UnsignedNumericLiteral {
    Integer(UnsignedInteger),
}

#[apply(ext)]
pub enum UnsignedIntegerKind {
    Binary,
    Octal,
    Decimal,
    Hex,
}

#[apply(base)]
pub struct UnsignedInteger {
    pub kind: UnsignedIntegerKind,
    pub integer: SmolStr,
    pub span: Span,
}
