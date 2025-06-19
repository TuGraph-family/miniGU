use miette::Diagnostic;
use minigu_common::error::NotImplemented;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum PlanError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    NotImplemented(#[from] NotImplemented),
}

pub type PlanResult<T> = std::result::Result<T, PlanError>;
