//! AST definitions for *object expressions*.

use super::{GraphRef, Ident};
use crate::macros::base;

#[apply(base)]
pub enum GraphExpr {
    Object(Ident),
    Ref(GraphRef),
    Current,
}
