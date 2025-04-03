use winnow::Parser;

use super::impls::gql_program;
use super::token::{build_token_stream, tokenize};
use crate::ast::Program;
use crate::error::Error;
use crate::span::Spanned;

/// Options which can be used to configure the behavior of the parser.
///
/// # Examples
/// Parsing a GQL query with default options:
/// ```no_run
/// use gql_parser::ParseOptions;
///
/// let parsed = ParseOptions::new().parse("match (: Person) -> (b: Person) return b");
/// ```
#[derive(Debug, Clone, Default)]
pub struct ParseOptions(ParseOptionsInner);

impl ParseOptions {
    /// Create a default set of parse options for configuration.  
    ///
    /// # Examples
    /// ```no_run
    /// use gql_parser::ParseOptions;
    ///
    /// let mut options = ParseOptions::new();
    /// let parsed = options.unescape(true).parse("CREATE GRAPH mygraph ANY");
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether quoted character sequences should be unescaped by the parser.
    ///
    /// If set to `true` (default), the parser will unescape quoted sequences in the input query.
    /// For example, `""` in GQL is unescaped to `"` in a double quoted character sequence.
    ///
    /// Otherwise, the parser will leave the raw strings unchanged in the output, and the
    /// caller should handle them manually.
    ///
    /// # Examples
    /// // TODO: Fill this part.
    ///
    /// Parsing a GQL query with quoted character sequences unescaped:
    /// ```no_run
    /// use gql_parser::ParseOptions;
    ///
    /// let parsed = ParseOptions::new()
    ///     .unescape(true)
    ///     .parse(r"session set graph /`my\ngraph`");
    /// ```
    pub fn unescape(&mut self, unescape: bool) -> &mut Self {
        self.0.unescape = unescape;
        self
    }

    /// Parses a GQL query `gql` into a spanned abstract syntax tree with the options specified by
    /// `self`.
    ///
    /// # Errors
    /// This function will return an error if `gql` is not a valid GQL query. The error will carry
    /// fancy diagnostics if feature `miette` is enabled.
    ///
    /// Currently, we provide only simple and non-informative errors as defined in [`Error`]. More
    /// specific errors will be introduced in the future.
    ///
    /// # Examples
    /// ```
    /// use gql_parser::ParseOptions;
    ///
    /// let program = ParseOptions::new().parse("SESSION CLOSE");
    /// assert!(program.is_ok());
    /// assert_eq!(program.unwrap().span(), 0..13);
    /// ```
    pub fn parse(&self, gql: &str) -> Result<Spanned<Program>, Error> {
        let tokens = tokenize(gql)?;
        let stream = build_token_stream(&tokens, self.0.clone());
        gql_program
            .parse(stream)
            .map_err(|e| match tokens.get(e.offset()) {
                Some(token) => Error::unexpected(gql, token.span.clone()),
                None => Error::UnexpectedEof,
            })
    }
}

#[derive(Debug, Clone)]
pub(super) struct ParseOptionsInner {
    unescape: bool,
}

impl Default for ParseOptionsInner {
    fn default() -> Self {
        Self { unescape: true }
    }
}

impl ParseOptionsInner {
    pub(super) fn unescape(&self) -> bool {
        self.unescape
    }
}
