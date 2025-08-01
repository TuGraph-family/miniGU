//! AST definitions for *variable definitions*.

use super::{Expr, GraphExpr, Ident, ValueType};
use crate::macros::base;
use crate::span::{OptSpanned, Spanned};

#[apply(base)]
pub struct TypedValueInitializer {
    pub value_type: OptSpanned<ValueType>,
    pub init: Spanned<Expr>,
}

#[apply(base)]
pub struct TypedGraphInitializer {
    pub value_type: OptSpanned<ValueType>,
    pub init: Spanned<GraphExpr>,
}

#[apply(base)]
pub struct TypedBindingTableInitializer {}

#[apply(base)]
pub struct GraphVariableDef {
    pub name: Spanned<Ident>,
    pub init: Spanned<TypedGraphInitializer>,
}

#[apply(base)]
pub struct ValueVariableDef {
    pub name: Spanned<Ident>,
    pub init: Spanned<TypedValueInitializer>,
}

#[apply(base)]
pub struct BindingTableVariableDef {
    pub name: Spanned<Ident>,
    pub init: Spanned<TypedBindingTableInitializer>,
}
