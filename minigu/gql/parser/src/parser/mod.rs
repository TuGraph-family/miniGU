pub use options::ParseOptions;
pub use token::{Token, tokenize, tokenize_full};

use crate::ast::Program;
use crate::error::Error;
use crate::span::Spanned;

mod impls;
mod options;
mod precedence;
mod token;
mod utils;

/// Parses a GQL query into a spanned abstract syntax tree with default options.
///
/// See [`ParseOptions`] for more information on how to configure the parser.
///
/// # Errors
///
/// This function will return an error if `gql` is not a valid GQL query. The error will carry
/// fancy diagnostics if feature `miette` is enabled.
///
/// # Examples
///
/// ```
/// # use gql_parser::parse_gql;
/// let program = parse_gql("SESSION CLOSE");
/// assert!(program.is_ok());
/// assert_eq!(program.unwrap().span(), 0..13);
/// ```
pub fn parse_gql(gql: &str) -> Result<Spanned<Program>, Error> {
    ParseOptions::new().parse(gql)
}
