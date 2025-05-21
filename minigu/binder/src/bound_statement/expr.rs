use macro_rules_attribute::apply;
use serde::Serialize;
use crate::binder::bound_statement::expr::BoundExpr;
use crate::catalog_ref::GraphCatalog;


#[derive(Debug, Serialize)]
pub enum BoundGraphExpr {
    Ref(GraphCatalog),
    Object(BoundObjectExpr),
}

#[derive(Debug, Serialize)]
pub enum BoundObjectExpr {
    Variable(BoundExpr),
    Expr(BoundExpr),
}