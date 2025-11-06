//! AST definitions for *Value expressions and specifications*.

use super::{BooleanLiteral, GraphExpr, Ident, ListTypeName, Literal, UnsignedInteger};
use crate::imports::Box;
use crate::macros::base;
use crate::span::{BoxSpanned, OptSpanned, Spanned, VecSpanned};

#[apply(base)]
pub enum Expr {
    Binary {
        op: Spanned<BinaryOp>,
        left: BoxSpanned<Expr>,
        right: BoxSpanned<Expr>,
    },
    Unary {
        op: Spanned<UnaryOp>,
        child: BoxSpanned<Expr>,
    },
    // // BuiltinFunction(BuiltinFunction),
    DurationBetween {
        left: BoxSpanned<Expr>,
        right: BoxSpanned<Expr>,
    },
    Is {
        left: BoxSpanned<Expr>,
        right: Spanned<BooleanLiteral>,
    },
    IsNot {
        left: BoxSpanned<Expr>,
        right: Spanned<BooleanLiteral>,
    },
    Function(Function),
    Aggregate(AggregateFunction),
    Variable(Ident),
    Value(Value),
    Path(PathConstructor),
    Property {
        source: BoxSpanned<Expr>,
        trailing_names: VecSpanned<Ident>,
    },
    Graph(Box<GraphExpr>),
}

/// Binary operators.
#[apply(base)]
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
#[apply(base)]
pub enum UnaryOp {
    /// Plus, e.g., `+a`.
    Plus,
    /// Minus, e.g., `-a`.
    Minus,
    /// Not, e.g., `NOT a`.
    Not,
}

#[apply(base)]
pub enum Function {
    Generic(GenericFunction),
    Numeric(NumericFunction),
    Case(CaseFunction),
    Vector(VectorDistance),
}

#[apply(base)]
pub struct GenericFunction {
    pub name: Spanned<Ident>,
    pub args: VecSpanned<Expr>,
}

#[apply(base)]
pub enum NumericFunction {
    CharLength(BoxSpanned<Expr>),
    ByteLength(BoxSpanned<Expr>),
    PathLength(BoxSpanned<Expr>),
    Absolute(BoxSpanned<Expr>),
}

#[apply(base)]
pub enum CaseFunction {
    NullIf(BoxSpanned<Expr>, BoxSpanned<Expr>),
    Coalesce(VecSpanned<Expr>),
}

#[apply(base)]
pub struct VectorDistance {
    pub lhs: BoxSpanned<Expr>,
    pub rhs: BoxSpanned<Expr>,
    pub metric: OptSpanned<Ident>,
}

#[apply(base)]
pub struct PathConstructor {
    pub start: BoxSpanned<Expr>,
    pub steps: VecSpanned<PathStep>,
}

#[apply(base)]
pub struct PathStep {
    pub edge: Spanned<Expr>,
    pub node: Spanned<Expr>,
}

#[apply(base)]
pub struct ListConstructor {
    pub type_name: OptSpanned<ListTypeName>,
    pub values: VecSpanned<Expr>,
}

pub type RecordConstructor = VecSpanned<FieldOrProperty>;

#[apply(base)]
pub struct FieldOrProperty {
    pub name: Spanned<Ident>,
    pub value: Spanned<Expr>,
}

#[apply(base)]
pub enum Value {
    SessionUser,
    Parameter(Ident),
    Literal(Literal),
}

#[apply(base)]
pub enum NonNegativeInteger {
    Integer(UnsignedInteger),
    Parameter(Ident),
}

#[apply(base)]
pub enum SetQuantifier {
    Distinct,
    All,
}

#[apply(base)]
pub enum AggregateFunction {
    Count,
    General(GeneralSetFunction),
    Binary(BinarySetFunction),
}

#[apply(base)]
pub struct GeneralSetFunction {
    pub kind: Spanned<GeneralSetFunctionKind>,
    pub quantifier: OptSpanned<SetQuantifier>,
    pub expr: BoxSpanned<Expr>,
}

#[apply(base)]
pub enum GeneralSetFunctionKind {
    Avg,
    Count,
    Max,
    Min,
    Sum,
    CollectList,
    StddevSamp,
    StddevPop,
}

#[apply(base)]
pub struct BinarySetFunction {
    pub kind: Spanned<BinarySetFunctionKind>,
    pub quantifier: OptSpanned<SetQuantifier>,
    pub dependent: BoxSpanned<Expr>,
    pub independent: BoxSpanned<Expr>,
}

#[apply(base)]
pub enum BinarySetFunctionKind {
    PercentileCont,
    PercentileDisc,
}
