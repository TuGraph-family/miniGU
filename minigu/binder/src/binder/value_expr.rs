use gql_parser::ast::Expr;

use super::Binder;
use crate::error::{BindResult, not_implemented};
use crate::procedure::value_expr::BoundExpr;

impl Binder {
    pub fn bind_value_expression(&self, expr: &Expr) -> BindResult<BoundExpr> {
        not_implemented("value expression".into(), None)
    }
}
