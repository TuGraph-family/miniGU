//! AST definitions for *object expressions*.

use super::{Expr, GraphRef, Ident};
use crate::macros::base;
use crate::span::Spanned;

#[apply(base)]
pub enum GraphExpr {
    Name(Ident),
    Ref(GraphRef),
    Object(ObjectExpr),
    Current,
}

#[apply(base)]
pub enum ObjectExpr {
    Variable(Spanned<Expr>),
    Expr(Spanned<Expr>),
}
