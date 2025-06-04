use minigu_common::data_type::LogicalType;
use minigu_common::value::ScalarValue;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum BoundExprKind {
    Value(ScalarValue),
    ColumnRef(usize),
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundExpr {
    pub kind: BoundExprKind,
    pub logical_type: LogicalType,
}

impl BoundExpr {
    #[inline]
    pub fn column_ref(index: usize, logical_type: LogicalType) -> Self {
        Self {
            kind: BoundExprKind::ColumnRef(index),
            logical_type,
        }
    }

    #[inline]
    pub fn value(value: ScalarValue, logical_type: LogicalType) -> Self {
        Self {
            kind: BoundExprKind::Value(value),
            logical_type,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Concat,
    Or,
    Xor,
    And,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

#[derive(Debug, Clone, Serialize)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
}

#[derive(Debug, Clone, Serialize)]
pub enum SetQuantifier {
    Distinct,
    All,
}
