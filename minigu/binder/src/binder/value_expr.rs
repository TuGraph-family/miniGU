use gql_parser::ast::Expr;
use minigu_ir::bound::BoundExpr;

use super::Binder;
use crate::error::{BindResult, not_implemented};

impl Binder {
    pub fn bind_value_expression(&self, expr: &Expr) -> BindResult<BoundExpr> {
        not_implemented("value expression".into(), None)
    }
}
