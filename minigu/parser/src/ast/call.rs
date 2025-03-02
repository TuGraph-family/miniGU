//! AST definitions for *procedure calling*.

use super::{Expr, ProcedureRef, Yield};
use crate::imports::Vec;
use crate::macros::base;

#[apply(base)]
pub struct CallProcedureStatement {
    pub procedure: ProcedureCall,
    pub optional: bool,
}

#[apply(base)]
pub enum ProcedureCall {
    Named(NamedProcedureCall),
}

#[apply(base)]
pub struct NamedProcedureCall {
    pub name: ProcedureRef,
    pub args: Vec<Expr>,
    pub yield_clause: Option<Yield>,
}
