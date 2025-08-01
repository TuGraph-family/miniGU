use miette::Diagnostic;
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
}

pub type PlanResult<T> = std::result::Result<T, PlanError>;
