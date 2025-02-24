//! AST definitions for *Value expressions and specifications*.

use super::{BooleanLiteral, GraphExpr, Ident, ListTypeName, Literal};
use crate::imports::{Box, Vec};
use crate::macros::{base, ext};
use crate::span::Span;

#[apply(base)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[apply(base)]
pub enum ExprKind {
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        child: Box<Expr>,
    },
    // // BuiltinFunction(BuiltinFunction),
    DurationBetween {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Is {
        left: Box<Expr>,
        right: BooleanLiteral,
    },
    IsNot {
        left: Box<Expr>,
        right: BooleanLiteral,
    },
    Variable(Ident),
    Value(Value),
    // Path(Box<PathConstructor<'a>>),
    Property {
        source: Box<Expr>,
        name: Ident,
    },
    Graph(GraphExpr),
    // Invalid,
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

// #[apply(base)]
// pub enum BuiltinFunction<'a> {
//     #[cfg_attr(feature = "serde", serde(borrow))]
//     CharLength(Box<Expr<'a>>),
//     ByteLength(Box<Expr<'a>>),
//     OctetLength(Box<Expr<'a>>),
//     Cardinality(Box<Expr<'a>>),
//     Size(Box<Expr<'a>>),
// }

#[apply(base)]
pub struct PathConstructor {
    pub start: Expr,
    pub steps: Vec<PathStep>,
    pub span: Span,
}

#[apply(base)]
pub struct PathStep {
    pub edge: Expr,
    pub node: Expr,
}

#[apply(base)]
pub struct ListConstructor {
    pub type_name: Option<ListTypeName>,
    pub values: Vec<Expr>,
    pub span: Span,
}

#[apply(base)]
pub struct RecordConstructor {
    pub fields: Vec<Field>,
    pub span: Span,
}

#[apply(base)]
pub struct Field {
    pub name: Ident,
    pub value: Expr,
    pub span: Span,
}

#[apply(base)]
pub struct Value {
    pub kind: ValueKind,
    pub span: Span,
}

#[apply(base)]
pub enum ValueKind {
    SessionUser,
    Parameter(Ident),
    Literal(Literal),
}

#[apply(ext)]
pub enum SetQuantifier {
    Distinct,
    All,
}
