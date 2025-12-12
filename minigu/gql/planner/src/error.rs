use miette::Diagnostic;
use minigu_catalog::txn::error::CatalogTxnError;
use minigu_common::error::NotImplemented;
use thiserror::Error;

use crate::binder::error::BindError;

#[derive(Debug, Error, Diagnostic)]
pub enum PlanError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Bind(#[from] BindError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    NotImplemented(#[from] NotImplemented),

    #[error(transparent)]
    #[diagnostic(code(planner::transaction))]
    Transaction(#[from] CatalogTxnError),

    #[error("internal error: {0}")]
    InvalidOperation(String),
}

pub type PlanResult<T> = std::result::Result<T, PlanError>;

impl From<minigu_catalog::error::CatalogError> for PlanError {
    fn from(err: minigu_catalog::error::CatalogError) -> Self {
        PlanError::InvalidOperation(err.to_string())
    }
}
