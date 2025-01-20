//! AST definitions for *value expressions*.

use crate::ast::lexical::{BooleanLiteral, Ident};
use crate::macros::{base, ext};
use crate::{Box, Vec};

#[apply(base)]
pub enum Expression<'a> {
    Binary {
        op: BinaryOp,
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
    },
    Unary {
        op: UnaryOp,
        child: Box<Expression<'a>>,
    },
    #[cfg_attr(feature = "serde", serde(borrow))]
    BuiltinFunction(BuiltinFunction<'a>),
    DurationBetween {
        arg1: Box<Expression<'a>>,
        arg2: Box<Expression<'a>>,
    },
    Is {
        left: Box<Expression<'a>>,
        right: BooleanLiteral,
    },
    IsNot {
        left: Box<Expression<'a>>,
        right: BooleanLiteral,
    },
    Variable(Ident<'a>),
    Invalid,
}

/// Binary operator.
#[apply(ext)]
pub enum BinaryOp {
    /// numeric/datetime/duration addition operator, i.e., '+'.
    Add,
    /// numeric/datetime/duration subtraction operator, i.e., '-'.
    Sub,
    /// numeric/duration multiplication operator, i.e., '*'.
    Mul,
    /// numeric/duration division operator, i.e., '/'.
    Div,
    /// string/list/path concatenation operator, i.e., '||'.
    Concat,
    /// boolean OR operator.
    Or,
    /// boolean XOR operator.
    Xor,
    /// boolean AND operator.
    And,
}

/// Unary operator.
#[apply(ext)]
pub enum UnaryOp {
    /// The numeric negation operator, i.e., '-'.
    Neg,
    /// The boolean NOT operator.
    Not,
}

#[apply(base)]
pub enum BuiltinFunction<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    CharLength(Box<Expression<'a>>),
    ByteLength(Box<Expression<'a>>),
    OctetLength(Box<Expression<'a>>),
    Cardinality(Box<Expression<'a>>),
    Size(Box<Expression<'a>>),
}

#[apply(base)]
pub struct ListConstructor<'a>(
    #[cfg_attr(feature = "serde", serde(borrow))] pub Vec<Expression<'a>>,
);

#[apply(base)]
pub struct RecordConstructor<'a>(#[cfg_attr(feature = "serde", serde(borrow))] pub Vec<Field<'a>>);

#[apply(base)]
pub struct Field<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: Ident<'a>,
    pub value: Expression<'a>,
}

#[apply(base)]
pub enum Value {
    SessionUser,
}
