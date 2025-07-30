//! AST definitions for *session management*.

use super::{
    GraphExpr, Ident, SchemaRef, StringLiteral, TypedGraphInitializer, TypedValueInitializer,
};
use crate::macros::base;
use crate::span::{OptSpanned, Spanned};

#[apply(base)]
pub enum SessionSet {
    Schema(Spanned<SchemaRef>),
    Graph(Spanned<GraphExpr>),
    TimeZone(Spanned<StringLiteral>),
    Parameter(Spanned<SessionSetParameter>),
}

#[apply(base)]
pub struct SessionSetParameter {
    pub name: Spanned<Ident>,
    pub if_not_exists: bool,
    pub kind: SessionSetParameterKind,
}

#[apply(base)]
pub enum SessionSetParameterKind {
    Graph(Spanned<TypedGraphInitializer>),
    Value(Spanned<TypedValueInitializer>),
}

#[apply(base)]
pub struct SessionReset(pub OptSpanned<SessionResetArgs>);

#[apply(base)]
pub enum SessionResetArgs {
    AllCharacteristics,
    AllParameters,
    Schema,
    Graph,
    TimeZone,
    Parameter(Spanned<Ident>),
}
