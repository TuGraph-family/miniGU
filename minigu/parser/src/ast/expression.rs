//! AST definitions for *Value expressions and specifications*.

use super::{BooleanLiteral, GraphExpr, Ident, ListTypeName, Literal};
use crate::imports::{Box, Vec};
use crate::macros::{base, ext};

#[apply(base)]
pub enum Expr<'a> {
    Binary {
        op: BinaryOp,
        left: Box<Expr<'a>>,
        right: Box<Expr<'a>>,
    },
    Unary {
        op: UnaryOp,
        child: Box<Expr<'a>>,
    },
    #[cfg_attr(feature = "serde", serde(borrow))]
    BuiltinFunction(BuiltinFunction<'a>),
    DurationBetween {
        arg1: Box<Expr<'a>>,
        arg2: Box<Expr<'a>>,
    },
    Is {
        left: Box<Expr<'a>>,
        right: BooleanLiteral,
    },
    IsNot {
        left: Box<Expr<'a>>,
        right: BooleanLiteral,
    },
    Variable(Ident<'a>),
    Value(Value<'a>),
    Path(Box<PathConstructor<'a>>),
    Property(Box<PropertyRef<'a>>),
    Graph(GraphExpr<'a>),
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
    CharLength(Box<Expr<'a>>),
    ByteLength(Box<Expr<'a>>),
    OctetLength(Box<Expr<'a>>),
    Cardinality(Box<Expr<'a>>),
    Size(Box<Expr<'a>>),
}

#[apply(base)]
pub struct PathConstructor<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub start: Expr<'a>,
    pub steps: Vec<PathStep<'a>>,
}

#[apply(base)]
pub struct PathStep<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub edge: Expr<'a>,
    pub node: Expr<'a>,
}

#[apply(base)]
pub struct ListConstructor<'a> {
    pub type_name: Option<ListTypeName>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub values: Vec<Expr<'a>>,
}

#[apply(base)]
pub struct RecordConstructor<'a>(#[cfg_attr(feature = "serde", serde(borrow))] pub Vec<Field<'a>>);

#[apply(base)]
pub struct Field<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: Ident<'a>,
    pub value: Expr<'a>,
}

#[apply(base)]
pub enum Value<'a> {
    SessionUser,
    #[cfg_attr(feature = "serde", serde(borrow))]
    Parameter(Ident<'a>),
    Literal(Literal<'a>),
}

#[apply(ext)]
pub enum SetQuantifier {
    Distinct,
    All,
}

#[apply(base)]
pub struct PropertyRef<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub source: Expr<'a>,
    pub name: Ident<'a>,
}
