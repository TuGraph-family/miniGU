use miette::Diagnostic;
use minigu_common::error::NotImplemented;
use minigu_context::runtime::RuntimeError;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("parse error")]
    #[diagnostic(transparent)]
    Parser(#[from] gql_parser::error::Error),

    #[error("plan error")]
    #[diagnostic(transparent)]
    Plan(#[from] minigu_planner::error::PlanError),

    #[error("catalog error")]
    Catalog(#[from] minigu_catalog::error::CatalogError),

    #[error("execution error")]
    #[diagnostic(transparent)]
    Execution(#[from] minigu_execution::error::ExecutionError),

    #[error("runtime error")]
    #[diagnostic(transparent)]
    Runtime(#[from] RuntimeError),

    #[error("session error")]
    #[diagnostic(transparent)]
    Session(#[from] minigu_context::error::Error),

    #[error("current session is closed")]
    SessionClosed,

    #[error(transparent)]
    #[diagnostic(transparent)]
    NotImplemented(#[from] NotImplemented),
}

pub type Result<T> = std::result::Result<T, Error>;
