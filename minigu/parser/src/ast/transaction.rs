//! AST definitions for *transaction management*.

use crate::macros::base;
use crate::span::VecSpanned;

#[apply(base)]
pub struct StartTransaction(pub VecSpanned<TransactionMode>);

#[apply(base)]
pub enum EndTransaction {
    Rollback,
    Commit,
}

#[apply(base)]
pub enum TransactionMode {
    ReadOnly,
    ReadWrite,
}
