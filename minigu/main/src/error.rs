use miette::Diagnostic;
use minigu_common::error::NotImplemented;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("parse error")]
    #[diagnostic(transparent)]
    Parser(#[from] gql_parser::error::Error),

    #[error("bind error")]
    #[diagnostic(transparent)]
    Bind(#[from] minigu_binder::error::BindError),

    #[error("plan error")]
    #[diagnostic(transparent)]
    Plan(#[from] minigu_planner::error::PlanError),

    #[error("catalog error")]
    Catalog(#[from] minigu_catalog::error::CatalogError),

    #[error("execution error")]
    #[diagnostic(transparent)]
    Execution(#[from] minigu_execution::error::ExecutionError),

    #[error("rayon error")]
    Rayon(#[from] rayon::ThreadPoolBuildError),

    #[error("current session is closed")]
    SessionClosed,

    #[error(transparent)]
    #[diagnostic(transparent)]
    NotImplemented(#[from] NotImplemented),
}

pub type Result<T> = std::result::Result<T, Error>;
