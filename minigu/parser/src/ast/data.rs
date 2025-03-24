//! AST definitions for *data-modifying statements*.

use super::{Expr, Ident};
use crate::imports::{Box, Vec};
use crate::macros::base;
use crate::span::Spanned;

#[apply(base)]
pub struct LinearDataModifyingStatement {}
