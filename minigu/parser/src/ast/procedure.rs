//! AST definitions for *procedure specification*.

use super::{CatalogModifyingStatement, SchemaRef, Yield};
use crate::imports::Vec;
use crate::macros::base;
use crate::span::Span;

#[apply(base)]
pub struct Procedure {
    pub at: Option<SchemaRef>,
    pub statement: Statement,
    pub next_statements: Vec<NextStatement>,
    pub span: Span,
}

#[apply(base)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

#[apply(base)]
pub enum StatementKind {
    Catalog(Vec<CatalogModifyingStatement>),
}

#[apply(base)]
pub struct NextStatement {
    pub yield_clause: Option<Yield>,
    pub statement: Statement,
    pub span: Span,
}
