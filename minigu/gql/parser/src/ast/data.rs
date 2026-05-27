//! AST definitions for *data-modifying statements*.

use super::ElementPattern;
use crate::macros::base;
use crate::span::{Spanned, VecSpanned};

pub type LinearDataModifyingStatement = VecSpanned<DataModifyingStatement>;

#[apply(base)]
pub enum DataModifyingStatement {
    Insert(InsertStatement),
}

/// `INSERT <path> (, <path>)*`
#[apply(base)]
pub struct InsertStatement {
    pub paths: VecSpanned<InsertPath>,
}

/// A single insert path: alternating node and edge patterns,
/// e.g. `(a:L{...})` or `(a)-[:R{...}]->(b)`.
///
/// Elements are reused from `ElementPattern` so we share the existing parser and
/// label/property/variable AST.
#[apply(base)]
pub struct InsertPath {
    pub elements: VecSpanned<ElementPattern>,
}
