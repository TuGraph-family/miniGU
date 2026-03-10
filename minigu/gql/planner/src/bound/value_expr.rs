use std::fmt::Display;

use minigu_common::data_type::LogicalType;
use minigu_common::types::VectorMetric;
use minigu_common::value::ScalarValue;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum BoundExprKind {
    Value(ScalarValue),
    Variable(String),
    Binary {
        op: BoundBinaryOp,
        left: Box<BoundExpr>,
        right: Box<BoundExpr>,
    },
    Property {
        source: String,
        property: String,
    },
    VectorDistance {
        lhs: Box<BoundExpr>,
        rhs: Box<BoundExpr>,
        metric: VectorMetric,
        dimension: usize,
    },
}

impl Display for BoundExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // TODO: Use `Display` rather than `Debug` representation for `value`.
            BoundExprKind::Value(value) => write!(f, "{value:?}"),
            BoundExprKind::Variable(variable) => write!(f, "{variable}"),
            BoundExprKind::Binary { op, left, right } => {
                write!(f, "({} {:?} {})", left, op, right)
            }
            BoundExprKind::Property { source, property } => {
                write!(f, "{}.{}", source, property)
            }
            BoundExprKind::VectorDistance {
                lhs, rhs, metric, ..
            } => {
                write!(f, "VECTOR_DISTANCE({}, {}, {})", lhs, rhs, metric)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundExpr {
    pub kind: BoundExprKind,
    pub logical_type: LogicalType,
    pub nullable: bool,
}

impl BoundExpr {
    pub fn value(value: ScalarValue, logical_type: LogicalType, nullable: bool) -> Self {
        Self {
            kind: BoundExprKind::Value(value),
            logical_type,
            nullable,
        }
    }

    pub fn variable(name: String, logical_type: LogicalType, nullable: bool) -> Self {
        Self {
            kind: BoundExprKind::Variable(name),
            logical_type,
            nullable,
        }
    }

    pub fn binary(op: BoundBinaryOp, left: BoundExpr, right: BoundExpr) -> Self {
        let nullable = left.nullable || right.nullable;
        let logical_type = match &op {
            BoundBinaryOp::Lt
            | BoundBinaryOp::Le
            | BoundBinaryOp::Gt
            | BoundBinaryOp::Ge
            | BoundBinaryOp::Eq
            | BoundBinaryOp::Ne
            | BoundBinaryOp::And
            | BoundBinaryOp::Or
            | BoundBinaryOp::Xor => LogicalType::Boolean,
            BoundBinaryOp::Concat => LogicalType::String,
            BoundBinaryOp::Add | BoundBinaryOp::Sub | BoundBinaryOp::Mul | BoundBinaryOp::Div => {
                left.logical_type.clone()
            }
        };
        Self {
            kind: BoundExprKind::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            logical_type,
            nullable,
        }
    }

    pub fn property(source: String, property: String, logical_type: LogicalType) -> Self {
        Self {
            kind: BoundExprKind::Property { source, property },
            logical_type,
            nullable: true,
        }
    }

    pub fn vector_distance(
        lhs: BoundExpr,
        rhs: BoundExpr,
        metric: VectorMetric,
        dimension: usize,
    ) -> Self {
        let nullable = lhs.nullable || rhs.nullable;
        Self {
            kind: BoundExprKind::VectorDistance {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                metric,
                dimension,
            },
            logical_type: LogicalType::Float32,
            nullable,
        }
    }

    pub fn evaluate_scalar(self) -> Option<ScalarValue> {
        match self.kind {
            BoundExprKind::Value(value) => Some(value),
            _ => None,
        }
    }
}

impl Display for BoundExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum BoundBinaryOp {
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
pub enum BoundUnaryOp {
    Plus,
    Minus,
    Not,
}

#[derive(Debug, Clone, Serialize)]
pub enum BoundSetQuantifier {
    Distinct,
    All,
}
