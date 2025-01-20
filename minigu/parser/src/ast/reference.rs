//! AST definitions for *object references*.

use crate::macros::ext;


pub struct DirectoryPath {}

pub enum PathComponent {
    Parent,
}

#[apply(ext)]
pub enum PredefinedSchemaReference {
    Home,
    Current,
}
