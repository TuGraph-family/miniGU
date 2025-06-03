use minigu_common::data_type::LogicalType;
use minigu_common::value::ScalarValue;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum BoundExprKind {
    Value(ScalarValue),
    ColumnRef(usize),
}

#[derive(Debug, Serialize)]
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
}

#[derive(Debug, Serialize)]
pub enum SetQuantifier {
    Distinct,
    All,
}
