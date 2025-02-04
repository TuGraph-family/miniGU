//! AST definitions for *catalog-modifying statements*.

use super::{CallProcedureStatement, GraphExpr, GraphTypeRef, ObjectRef, SchemaPathComponent};
use crate::macros::{base, ext};

#[apply(base)]
pub enum CatalogModifyingStatement<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Call(CallProcedureStatement<'a>),
    CreateSchema(CreateSchemaStatement<'a>),
    DropSchema(DropSchemaStatement<'a>),
    CreateGraph(CreateGraphStatement<'a>),
    DropGraph(DropGraphStatement<'a>),
    CreateGraphType(CreateGraphTypeStatement<'a>),
    DropGraphType(DropGraphTypeStatement<'a>),
}

#[apply(base)]
pub struct CreateSchemaStatement<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub path: Vec<SchemaPathComponent<'a>>,
    pub if_not_exists: bool,
}

#[apply(base)]
pub struct DropSchemaStatement<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub path: Vec<SchemaPathComponent<'a>>,
    pub if_exists: bool,
}

#[apply(base)]
pub struct CreateGraphStatement<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub path: ObjectRef<'a>,
    pub kind: CreateGraphOrGraphTypeStatementKind,
    pub graph_type: OfGraphType<'a>,
    pub source: Option<GraphExpr<'a>>,
}

#[apply(ext)]
pub enum CreateGraphOrGraphTypeStatementKind {
    Create,
    CreateIfNotExists,
    CreateOrReplace,
}

#[apply(base)]
pub enum OfGraphType<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Like(GraphExpr<'a>),
    Ref(GraphTypeRef<'a>),
    Any,
}

#[apply(base)]
pub struct DropGraphStatement<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub path: ObjectRef<'a>,
    pub if_exists: bool,
}

#[apply(base)]
pub struct DropGraphTypeStatement<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub path: ObjectRef<'a>,
    pub if_exists: bool,
}

#[apply(base)]
pub struct CreateGraphTypeStatement<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub path: ObjectRef<'a>,
    pub kind: CreateGraphOrGraphTypeStatementKind,
    pub source: GraphTypeSource<'a>,
}

#[apply(base)]
pub enum GraphTypeSource<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Copy(GraphTypeRef<'a>),
    Like(GraphExpr<'a>),
}
