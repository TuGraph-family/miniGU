use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("failed to parse the query")]
    #[diagnostic(transparent)]
    Parser(#[from] gql_parser::error::Error),

    #[error("failed to execute the procedure")]
    Procedure(#[from] Box<dyn std::error::Error>),
}

pub type Result<T> = std::result::Result<T, Error>;
