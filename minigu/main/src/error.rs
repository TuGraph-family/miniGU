use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("failed to parse the query")]
    #[diagnostic(transparent)]
    Parser(#[from] gql_parser::error::Error),

    #[error("failed to bind the procedure")]
    Bind(#[from] minigu_binder::error::BindError),
}

pub type Result<T> = std::result::Result<T, Error>;
