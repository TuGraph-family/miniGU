//! AST definitions for *Value expressions and specifications*.

use crate::ast::element::ListTypeName;
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
    Value(Value),
    Invalid,
}

/// Binary operators.
#[apply(ext)]
pub enum BinaryOp {
    /// Addition, e.g., `a + b`.
    Add,
    /// Subtraction, e.g., `a - b`.
    Sub,
    /// Multiplication, e.g., `a * b`.
    Mul,
    /// Division, e.g., `a / b`.
    Div,
    /// Concatenation, e.g., `a || b`.
    Concat,
    /// OR, e.g., `a OR b`.
    Or,
    /// XOR, e.g., `a XOR b`.
    Xor,
    /// AND, e.g., `a AND b`.
    And,
    /// Less than, e.g., `a < b`.
    Lt,
    /// Less than or equal, e.g., `a <= b`.
    Le,
    /// Greater than, e.g., `a > b`.
    Gt,
    /// Greater than or equal, e.g., `a >= b`.
    Ge,
    /// Equal, e.g., `a = b`.
    Eq,
    /// Not equal, e.g., `a <> b`.
    Ne,
}

/// Unary operators.
#[apply(ext)]
pub enum UnaryOp {
    /// Plus, e.g., `+a`.
    Plus,
    /// Minus, e.g., `-a`.
    Minus,
    /// Not, e.g., `NOT a`.
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
pub struct ListConstructor<'a> {
    pub type_name: Option<ListTypeName>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub values: Vec<Expression<'a>>,
}

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

#[apply(ext)]
pub enum SetQuantifier {
    Distinct,
    All,
}
