use serde::Serialize;

use super::value_expr::BoundExpr;
use crate::named_ref::NamedProcedureRef;

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
    pub procedure_ref: NamedProcedureRef,
    pub args: Vec<BoundExpr>,
    pub yield_column_indices: Vec<usize>,
}
