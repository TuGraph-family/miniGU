//! AST definitions for *object references*.

use crate::ast::Ident;
use crate::macros::{base, ext};
use crate::span::Span;

#[apply(ext)]
pub enum PredefinedSchemaRef {
    Home,
    Current,
}

#[apply(base)]
pub enum SchemaRef<'a> {
    Path(SchemaPath<'a>),
    Predefined(PredefinedSchemaRef),
    Parameter(Ident<'a>),
}

#[apply(base)]
pub struct SchemaPath<'a> {
    pub components: Vec<SchemaPathComponent<'a>>,
    pub span: Span,
}

#[apply(base)]
pub enum SchemaPathComponent<'a> {
    /// Named component, e.g., `a`.
    Name(Ident<'a>),
    /// Parent directory, e.g., `..`.
    Parent,
    /// The root indicator, e.g., `/`.
    ///
    /// # NOTE
    /// This should only appear as the first element of path components. Accordingly, this can be
    /// used to distinguish between absolute path and relative path.
    Root,
}

#[apply(base)]
pub struct ObjectRef<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub schema: Option<SchemaRef<'a>>,
    pub name: Vec<Ident<'a>>,
}

#[apply(base)]
pub enum GraphRef<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Name(Ident<'a>),
    Parameter(Ident<'a>),
    Ref(ObjectRef<'a>),
    Home,
}
