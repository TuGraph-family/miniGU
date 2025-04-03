//! AST definitions for *GQL-program*.

use super::{EndTransaction, Procedure, SessionReset, SessionSet, StartTransaction};
use crate::macros::base;
use crate::span::{OptSpanned, VecSpanned};

#[apply(base)]
pub struct Program {
    pub activity: OptSpanned<ProgramActivity>,
    pub session_close: bool,
}

#[apply(base)]
pub enum ProgramActivity {
    Session(SessionActivity),
    Transaction(TransactionActivity),
}

#[apply(base)]
pub struct SessionActivity {
    pub set: VecSpanned<SessionSet>,
    pub reset: VecSpanned<SessionReset>,
}

#[apply(base)]
pub struct TransactionActivity {
    pub start: OptSpanned<StartTransaction>,
    pub procedure: OptSpanned<Procedure>,
    pub end: OptSpanned<EndTransaction>,
}
