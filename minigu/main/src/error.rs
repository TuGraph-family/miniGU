use std::fmt::Display;

use miette::{Diagnostic, LabeledSpan, Severity, SourceCode};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {
    #[error(transparent)]
    Parser(#[from] gql_parser::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! impl_diagnostic {
    ( $( [ $variant:ident, $code:expr ] ),* ) => {
        #[allow(unreachable_patterns)]
        impl Diagnostic for Error {
            fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
                match self {
                    $(
                        Self::$variant(_) => Some(Box::new($code)),
                    )*
                    _ => None,
                }
            }

            fn severity(&self) -> Option<Severity> {
                match self {
                    $(
                        Self::$variant(e) => e.severity(),
                    )*
                    _ => None,
                }
            }

            fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
                match self {
                    $(
                        Self::$variant(e) => e.help(),
                    )*
                    _ => None,
                }
            }

            fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
                match self {
                    $(
                        Self::$variant(e) => e.url(),
                    )*
                    _ => None,
                }
            }

            fn source_code(&self) -> Option<&dyn SourceCode> {
                match self {
                    $(
                        Self::$variant(e) => e.source_code(),
                    )*
                    _ => None,
                }
            }

            fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
                match self {
                    $(
                        Self::$variant(e) => e.labels(),
                    )*
                    _ => None,
                }
            }

            fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
                match self {
                    $(
                        Self::$variant(e) => e.related(),
                    )*
                    _ => None,
                }
            }

            fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
                match self {
                    $(
                        Self::$variant(e) => e.diagnostic_source(),
                    )*
                    _ => None,
                }
            }
        }
    }
}

impl_diagnostic! {
    [ Parser, "Parser error:" ]
}
