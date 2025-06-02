use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum BoundExpr {}

#[derive(Debug, Serialize)]
pub enum SetQuantifier {
    Distinct,
    All,
}
