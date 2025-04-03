//! AST definitions for *lexical elements*.

use smol_str::SmolStr;

use super::{ListConstructor, RecordConstructor};
use crate::macros::base;
use crate::span::Spanned;

pub type Ident = SmolStr;

#[apply(base)]
pub enum Literal {
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
}

#[apply(base)]
pub enum StringLiteralKind {
    Char,
    Byte,
}

#[apply(base)]
pub enum BooleanLiteral {
    True,
    False,
    Unknown,
}

#[apply(base)]
pub struct TemporalLiteral {
    pub kind: TemporalLiteralKind,
    pub literal: Spanned<SmolStr>,
}

#[apply(base)]
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
    pub literal: Spanned<SmolStr>,
}

#[apply(base)]
pub enum DurationLiteralKind {
    Duration,
    SqlInterval,
}

#[apply(base)]
pub enum UnsignedNumericLiteral {
    Integer(Spanned<UnsignedInteger>),
}

#[apply(base)]
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
}
