//! AST definitions for *GQL-program*.

use super::{EndTransaction, Procedure, SessionReset, SessionSet, StartTransaction};
use crate::imports::Vec;
use crate::macros::base;

#[apply(base)]
pub struct Program {
    pub activity: Option<ProgramActivity>,
    pub session_close: bool,
}

#[apply(base)]
pub enum ProgramActivity {
    Session(SessionActivity),
    Transaction(TransactionActivity),
}

#[apply(base)]
pub struct SessionActivity {
    pub set: Vec<SessionSet>,
    pub reset: Vec<SessionReset>,
}

#[apply(base)]
pub struct TransactionActivity {
    pub start: Option<StartTransaction>,
    pub procedure: Option<Procedure>,
    pub end: Option<EndTransaction>,
}
