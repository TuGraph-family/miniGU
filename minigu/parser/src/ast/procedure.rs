//! AST definitions for *procedure calling*.

use super::{Expr, ProcedureRef, Yield};
use crate::macros::base;

#[apply(base)]
pub struct CallProcedureStatement<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub procedure: ProcedureCall<'a>,
    pub optional: bool,
}

#[apply(base)]
pub enum ProcedureCall<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Named(NamedProcedureCall<'a>),
}

#[apply(base)]
pub struct NamedProcedureCall<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: ProcedureRef<'a>,
    pub args: Vec<Expr<'a>>,
    pub yield_clause: Option<Yield<'a>>,
}
