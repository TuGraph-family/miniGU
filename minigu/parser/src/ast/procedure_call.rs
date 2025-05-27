//! AST definitions for *procedure calling*.

use super::{Expr, ProcedureRef, Yield};
use crate::macros::base;
use crate::span::{OptSpanned, Spanned, VecSpanned};

#[apply(base)]
pub struct CallProcedureStatement {
    pub optional: bool,
    pub procedure: Spanned<ProcedureCall>,
}

#[apply(base)]
pub enum ProcedureCall {
    Inline(InlineProcedureCall),
    Named(NamedProcedureCall),
}

#[apply(base)]
pub struct InlineProcedureCall {}

#[apply(base)]
pub struct NamedProcedureCall {
    pub name: Spanned<ProcedureRef>,
    pub args: VecSpanned<Expr>,
    pub yield_clause: OptSpanned<Yield>,
}
