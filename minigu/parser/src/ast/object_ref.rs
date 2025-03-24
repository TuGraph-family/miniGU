//! AST definitions for *object references*.

use super::Ident;
use crate::macros::base;
use crate::span::{OptSpanned, VecSpanned};

#[apply(base)]
pub enum PredefinedSchemaRef {
    Home,
    Current,
}

#[apply(base)]
pub enum SchemaRef {
    Absolute(SchemaPath),
    Relative(SchemaPath),
    Predefined(PredefinedSchemaRef),
    Parameter(Ident),
}

pub type SchemaPath = VecSpanned<SchemaPathSegment>;

#[apply(base)]
pub enum SchemaPathSegment {
    /// A named object (schema or directory), e.g., `a`.
    Name(Ident),
    /// Parent directory, e.g., `..`.
    Parent,
}

#[apply(base)]
pub enum GraphRef {
    Name(Ident),
    Parameter(Ident),
    Ref(CatalogObjectRef),
    Home,
}

#[apply(base)]
pub enum ProcedureRef {
    Ref(CatalogObjectRef),
    Parameter(Ident),
}

#[apply(base)]
pub enum GraphTypeRef {
    Ref(CatalogObjectRef),
    Parameter(Ident),
}

#[apply(base)]
pub enum BindingTableRef {
    Name(Ident),
    Ref(CatalogObjectRef),
    Parameter(Ident),
}

#[apply(base)]
pub struct CatalogObjectRef {
    pub schema: OptSpanned<SchemaRef>,
    pub objects: VecSpanned<Ident>,
}
