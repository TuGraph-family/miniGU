//! AST definitions for *catalog-modifying statements*.

use super::{
    CallProcedureStatement, CatalogObjectRef, GraphElementType, GraphExpr, GraphTypeRef, SchemaPath,
};
use crate::macros::base;
use crate::span::{OptSpanned, Spanned, VecSpanned};

pub type LinearCatalogModifyingStatement = VecSpanned<CatalogModifyingStatement>;

#[apply(base)]
pub enum CatalogModifyingStatement {
    Call(CallProcedureStatement),
    CreateSchema(CreateSchemaStatement),
    DropSchema(DropSchemaStatement),
    CreateGraph(CreateGraphStatement),
    DropGraph(DropGraphStatement),
    CreateGraphType(CreateGraphTypeStatement),
    DropGraphType(DropGraphTypeStatement),
}

#[apply(base)]
pub struct CreateSchemaStatement {
    pub path: Spanned<SchemaPath>,
    pub if_not_exists: bool,
}

#[apply(base)]
pub struct DropSchemaStatement {
    pub path: Spanned<SchemaPath>,
    pub if_exists: bool,
}

#[apply(base)]
pub struct CreateGraphStatement {
    pub path: Spanned<CatalogObjectRef>,
    pub kind: Spanned<CreateGraphOrGraphTypeStatementKind>,
    pub graph_type: Spanned<OfGraphType>,
    pub source: OptSpanned<GraphExpr>,
}

#[apply(base)]
pub enum CreateGraphOrGraphTypeStatementKind {
    Create,
    CreateIfNotExists,
    CreateOrReplace,
}

#[apply(base)]
pub enum OfGraphType {
    Like(Spanned<GraphExpr>),
    Ref(Spanned<GraphTypeRef>),
    Nested(VecSpanned<GraphElementType>),
    Any,
}

#[apply(base)]
pub struct DropGraphStatement {
    pub path: Spanned<CatalogObjectRef>,
    pub if_exists: bool,
}

#[apply(base)]
pub struct DropGraphTypeStatement {
    pub path: Spanned<CatalogObjectRef>,
    pub if_exists: bool,
}

#[apply(base)]
pub struct CreateGraphTypeStatement {
    pub path: Spanned<CatalogObjectRef>,
    pub kind: Spanned<CreateGraphOrGraphTypeStatementKind>,
    pub source: Spanned<GraphTypeSource>,
}

#[apply(base)]
pub enum GraphTypeSource {
    Copy(Spanned<GraphTypeRef>),
    Like(Spanned<GraphExpr>),
    Nested(VecSpanned<GraphElementType>),
}
