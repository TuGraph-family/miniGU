//! AST definitions for *transaction management*.

use crate::imports::Vec;
use crate::macros::{base, ext};

#[apply(base)]
pub struct TransactionActivity<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub start_transaction: Option<StartTransaction<'a>>,
    pub end_transaction: Option<EndTransaction>,
}

#[apply(base)]
pub struct StartTransaction<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub modes: Vec<TransactionAccessMode<'a>>,
}

#[apply(ext)]
pub enum EndTransaction {
    Rollback,
    Commit,
}

#[apply(ext)]
pub enum TransactionAccessMode<'a> {
    ReadOnly,
    ReadWrite,
    Custom(&'a str),
}
