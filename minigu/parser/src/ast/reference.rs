//! AST definitions for *object references*.

use super::Ident;
use crate::macros::{base, ext};

#[apply(ext)]
pub enum PredefinedSchemaRef {
    Home,
    Current,
}

#[apply(base)]
pub enum SchemaRef<'a> {
    Path(Vec<SchemaPathComponent<'a>>),
    Predefined(PredefinedSchemaRef),
    Parameter(Ident<'a>),
}

#[apply(base)]
pub enum SchemaPathComponent<'a> {
    /// A named object (schema or directory), e.g., `a`.
    Name(Ident<'a>),
    /// Parent directory, e.g., `..`.
    Parent,
    /// The root indicator, e.g., `/`.
    ///
    /// # Note
    /// This should only appear as the first component of an object path. Accordingly, this can be
    /// used to distinguish between absolute path and relative path.
    Root,
}

impl<'a> SchemaPathComponent<'a> {
    pub(crate) fn into_name(self) -> Option<Ident<'a>> {
        if let Self::Name(name) = self {
            Some(name)
        } else {
            None
        }
    }
}

#[apply(base)]
pub struct ObjectRef<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub schema: Option<SchemaRef<'a>>,
    pub objects: Vec<Ident<'a>>,
}

#[apply(base)]
pub enum GraphRef<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Name(Ident<'a>),
    Parameter(Ident<'a>),
    Ref(ObjectRef<'a>),
    Home,
}

#[apply(base)]
pub enum ProcedureRef<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Ref(ObjectRef<'a>),
    Parameter(Ident<'a>),
}

#[apply(base)]
pub enum GraphTypeRef<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Ref(ObjectRef<'a>),
    Parameter(Ident<'a>),
}
