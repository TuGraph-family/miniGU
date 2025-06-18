use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error, Diagnostic)]
pub enum Error {
    #[error("failed to parse the query")]
    #[diagnostic(transparent)]
    Parser(#[from] gql_parser::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
