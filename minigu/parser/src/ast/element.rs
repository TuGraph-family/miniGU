//! AST definitions for *type elements*.

use crate::macros::ext;

#[apply(ext)]
pub struct ListTypeName {
    pub group: bool,
    pub synonym: ListTypeNameSynonym,
}

#[apply(ext)]
pub enum ListTypeNameSynonym {
    List,
    Array,
}
