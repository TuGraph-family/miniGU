//! AST definitions for *object references*.

use super::Ident;
use crate::imports::Vec;
use crate::macros::{base, ext};
use crate::span::Span;

#[apply(ext)]
pub enum PredefinedSchemaRef {
    Home,
    Current,
}

#[apply(base)]
pub struct SchemaRef {
    pub kind: SchemaRefKind,
    pub span: Span,
}

#[apply(base)]
pub enum SchemaRefKind {
    Absolute(SchemaPath),
    Relative(SchemaPath),
    Predefined(PredefinedSchemaRef),
    Parameter(Ident),
}

#[apply(base)]
pub struct SchemaPath {
    pub components: Vec<SchemaPathComponent>,
    pub span: Span,
}

#[apply(base)]
pub enum SchemaPathComponent {
    /// A named object (schema or directory), e.g., `a`.
    Name(Ident),
    /// Parent directory, e.g., `..`.
    Parent,
}

#[apply(base)]
pub struct GraphRef {
    pub kind: GraphRefKind,
    pub span: Span,
}

#[apply(base)]
pub enum GraphRefKind {
    Name(Ident),
    Parameter(Ident),
    Ref(CatalogObjectRef),
    Home,
}

#[apply(base)]
pub struct ProcedureRef {
    pub kind: ProcedureRefKind,
    pub span: Span,
}

#[apply(base)]
pub enum ProcedureRefKind {
    Ref(CatalogObjectRef),
    Parameter(Ident),
}

#[apply(base)]
pub struct GraphTypeRef {
    pub kind: GraphTypeRefKind,
    pub span: Span,
}

#[apply(base)]
pub enum GraphTypeRefKind {
    Ref(CatalogObjectRef),
    Parameter(Ident),
}

#[apply(base)]
pub struct CatalogObjectRef {
    pub schema: Option<SchemaRef>,
    pub objects: Vec<Ident>,
    pub span: Span,
}
