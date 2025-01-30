//! AST definitions for *GQL-program*.

use crate::ast::{SessionActivity, TransactionActivity};
use crate::macros::base;

#[apply(base)]
pub struct Program<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub activity: Option<ProgramActivity<'a>>,
    pub session_close: bool,
}

#[apply(base)]
pub enum ProgramActivity<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Session(SessionActivity<'a>),
    Transaction(TransactionActivity<'a>),
}
