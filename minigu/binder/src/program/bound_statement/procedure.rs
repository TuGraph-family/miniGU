use gql_parser::ast::Yield;
use macro_rules_attribute::apply;

use crate::macros::base;
use crate::program::bound_statement::expr::BoundExpr;
use crate::program::bound_statement::object_ref::BoundProcedureRef;

#[apply(base)]
pub struct BoundCallProcedureStatement {
    pub optional: bool,
    pub procedure: BoundProcedureCall,
}

#[apply(base)]
pub enum BoundProcedureCall {
    Inline(BoundInlineProcedureCall),
    Named(BoundNamedProcedureCall),
}

#[apply(base)]
pub struct BoundInlineProcedureCall {}

#[apply(base)]
pub struct BoundNamedProcedureCall {
    pub name: BoundProcedureRef,
    pub args: Vec<BoundExpr>,
    // TODO: Bind Yield
    pub yield_clause: Option<Yield>,
}
