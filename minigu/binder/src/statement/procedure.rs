use serde::Serialize;
use crate::object_ref::ProcedureRef;
use crate::statement::common::Yield;

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
    pub procedure_ref: ProcedureRef,
    // pub args: Vec<BoundExpr>,
    pub yield_clause: Option<Yield>,
}
