//! AST definitions for *variable definitions*.

use super::{Expr, GraphExpr, ValueType};
use crate::macros::base;
use crate::span::Span;

#[apply(base)]
pub struct TypedValueInitializer {
    pub value_type: Option<ValueType>,
    pub init: Expr,
    pub span: Span,
}

#[apply(base)]
pub struct TypedGraphInitializer {
    pub value_type: Option<ValueType>,
    pub init: GraphExpr,
    pub span: Span,
}
