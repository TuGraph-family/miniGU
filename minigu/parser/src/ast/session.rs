//! AST definitions for *session management*.

use super::{
    GraphExpr, Ident, SchemaRef, StringLiteral, TypedGraphInitializer, TypedValueInitializer,
};
use crate::macros::base;
use crate::span::Span;

#[apply(base)]
pub struct SessionSet {
    pub kind: SessionSetKind,
    pub span: Span,
}

#[apply(base)]
pub enum SessionSetKind {
    Schema(SchemaRef),
    Graph(GraphExpr),
    TimeZone(StringLiteral),
    Parameter(SessionSetParameter),
}

#[apply(base)]
pub struct SessionSetParameter {
    pub name: Ident,
    pub if_not_exists: bool,
    pub kind: SessionSetParameterKind,
    pub span: Span,
}

#[apply(base)]
pub enum SessionSetParameterKind {
    Graph(TypedGraphInitializer),
    Value(TypedValueInitializer),
}

#[apply(base)]
pub struct SessionReset {
    pub kind: SessionResetKind,
    pub span: Span,
}

#[apply(base)]
pub enum SessionResetKind {
    AllCharacteristics,
    AllParameters,
    Schema,
    Graph,
    TimeZone,
    Parameter(Ident),
}
