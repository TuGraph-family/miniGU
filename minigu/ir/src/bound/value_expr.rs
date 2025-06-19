use std::sync::Arc;

use minigu_common::data_type::LogicalType;
use minigu_common::value::ScalarValue;
use serde::Serialize;

use crate::bound::{BoundEdgePattern, BoundPathPattern, BoundSubpathPattern, BoundVertexPattern};

#[derive(Debug, Clone, Serialize)]
pub enum BoundExprKind {
    Value(ScalarValue),
    ColumnRef(usize),
    Path(Arc<BoundPathPattern>),
    Subpath(Arc<BoundSubpathPattern>),
    Vertex(Arc<BoundVertexPattern>),
    Edge(Arc<BoundEdgePattern>),
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundExpr {
    pub name: String,
    pub kind: BoundExprKind,
    pub logical_type: LogicalType,
}

impl BoundExpr {
    pub fn column_ref(index: usize, logical_type: LogicalType) -> Self {
        Self {
            name: format!("_col{0}", index),
            kind: BoundExprKind::ColumnRef(index),
            logical_type,
        }
    }

    pub fn value(value: ScalarValue, logical_type: LogicalType) -> Self {
        Self {
            name: format!("{value:?}"),
            kind: BoundExprKind::Value(value),
            logical_type,
        }
    }

    pub fn evaluate_scalar(self) -> Option<ScalarValue> {
        match self.kind {
            BoundExprKind::Value(value) => Some(value),
            _ => None,
        }
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

// where Vertex(v) + 1
