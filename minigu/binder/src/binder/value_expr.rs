use gql_parser::ast::{
    CallProcedureStatement, CatalogModifyingStatement, Expr, NamedProcedureCall, ProcedureCall,
};

use super::Binder;
use crate::error::{BindResult, not_implemented};
use crate::procedure::procedure_call::{
    BoundCallProcedureStatement, BoundNamedProcedureCall, BoundProcedureCall,
};
use crate::procedure::value_expr::BoundExpr;

impl Binder {
    pub fn bind_value_expression(&self, expr: &Expr) -> BindResult<BoundExpr> {
        todo!()
    }
}
