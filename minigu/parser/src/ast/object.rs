//! AST definitions for *object expressions*.

use super::{GraphRef, Ident};
use crate::macros::base;

#[apply(base)]
pub enum GraphExpr<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Object(Ident<'a>),
    Ref(GraphRef<'a>),
    Current,
}
