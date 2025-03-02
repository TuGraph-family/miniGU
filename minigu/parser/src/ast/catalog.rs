//! AST definitions for *catalog-modifying statements*.

use super::{
    CallProcedureStatement, CatalogObjectRef, GraphElementType, GraphExpr, GraphTypeRef, SchemaPath,
};
use crate::imports::Vec;
use crate::macros::{base, ext};
use crate::span::Span;

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
    pub path: SchemaPath,
    pub if_not_exists: bool,
    pub span: Span,
}

#[apply(base)]
pub struct DropSchemaStatement {
    pub path: SchemaPath,
    pub if_exists: bool,
    pub span: Span,
}

#[apply(base)]
pub struct CreateGraphStatement {
    pub path: CatalogObjectRef,
    pub kind: CreateGraphOrGraphTypeStatementKind,
    pub graph_type: OfGraphType,
    pub source: Option<GraphExpr>,
    pub span: Span,
}

#[apply(ext)]
pub enum CreateGraphOrGraphTypeStatementKind {
    Create,
    CreateIfNotExists,
    CreateOrReplace,
}

#[apply(base)]
pub enum OfGraphType {
    Like(GraphExpr),
    Ref(GraphTypeRef),
    Nested(Vec<GraphElementType>),
    Any,
}

#[apply(base)]
pub struct DropGraphStatement {
    pub path: CatalogObjectRef,
    pub if_exists: bool,
    pub span: Span,
}

#[apply(base)]
pub struct DropGraphTypeStatement {
    pub path: CatalogObjectRef,
    pub if_exists: bool,
    pub span: Span,
}

#[apply(base)]
pub struct CreateGraphTypeStatement {
    pub path: CatalogObjectRef,
    pub kind: CreateGraphOrGraphTypeStatementKind,
    pub source: GraphTypeSource,
    pub span: Span,
}

#[apply(base)]
pub enum GraphTypeSource {
    Copy(GraphTypeRef),
    Like(GraphExpr),
    Nested(GraphElementType),
}
