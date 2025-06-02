use minigu_common::value::ScalarValue;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum BoundExpr {
    Value(ScalarValue)
}

#[derive(Debug, Serialize)]
pub enum SetQuantifier {
    Distinct,
    All,
}
