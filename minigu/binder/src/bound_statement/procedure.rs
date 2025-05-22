use gql_parser::ast::Yield;
use macro_rules_attribute::apply;
use serde::Serialize;
use crate::bound_statement::expr::BoundExpr;
use crate::bound_statement::object_ref::BoundProcedureRef;
use crate::catalog_ref::CallProcedureCatalogRef;

#[derive(Debug, Serialize)]
pub struct BoundCallProcedureStatement {
    pub optional: bool,
    pub procedure: BoundProcedureCall,
}

#[derive(Debug, Serialize)]
pub enum BoundProcedureCall {
    Inline(BoundInlineProcedureCall),
    Named(BoundNamedProcedureCall),
}

#[derive(Debug, Serialize)]
pub struct BoundInlineProcedureCall {}

#[derive(Debug, Serialize)]
pub struct BoundNamedProcedureCall {
    pub name: BoundProcedureRef,
    pub args: Vec<BoundExpr>,
    // TODO: Bind Yield
    pub yield_clause: Option<Yield>,
}
