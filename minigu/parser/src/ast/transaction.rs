//! AST definitions for *transaction management*.

use crate::imports::Vec;
use crate::macros::{base, ext};

#[apply(base)]
pub struct StartTransaction(pub Vec<TransactionMode>);

#[apply(ext)]
pub enum EndTransaction {
    Rollback,
    Commit,
}

#[apply(ext)]
pub enum TransactionMode {
    ReadOnly,
    ReadWrite,
}
