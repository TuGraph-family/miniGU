use macro_rules_attribute::apply;
use serde::Serialize;
use gql_parser::ast::{CreateGraphOrGraphTypeStatementKind, GraphElementType, Ident};
use crate::binder::bound_statement::catalog::{BoundCreateGraphStatement, BoundCreateGraphTypeStatement, BoundCreateSchemaStatement, BoundDropGraphStatement, BoundDropGraphTypeStatement, BoundDropSchemaStatement};
use crate::binder::bound_statement::object_ref::{BoundCatalogObjectRef, BoundGraphTypeRef};
use crate::bound_statement::object_ref::CanonicalSchemaPath;
use crate::catalog_ref::{GraphTypeCatalog, SchemaCatalog};
use crate::macros::base;

pub type LinearBoundCatalogModifyingStatement = Vec<BoundCatalogModifyingStatement>;

#[apply(base)]
pub enum BoundCatalogModifyingStatement {
    Call(BoundCallProcedureStatement),
    CreateSchema(BoundCreateSchemaStatement),
    DropSchema(BoundDropSchemaStatement),
    CreateGraph(BoundCreateGraphStatement),
    DropGraph(BoundDropGraphStatement),
    CreateGraphType(BoundCreateGraphTypeStatement),
    DropGraphType(BoundDropGraphTypeStatement),
}


#[apply(base)]
pub struct BoundCreateSchemaStatement {
    pub schema_path: CanonicalSchemaPath,
    pub if_not_exists: bool,
}

#[apply(base)]
pub struct BoundDropSchemaStatement {
    pub schema_path: CanonicalSchemaPath,
    pub if_exists: bool,
}

#[apply(base)]
pub struct BoundCreateGraphStatement {
    pub path: BoundSchemaRef,
    pub kind: CreateGraphOrGraphTypeStatementKind,
    pub label_id: Option<LabelId>,
    pub label_name: Option<Ident>,
    pub source: Option<BoundGraphExpr>,
}

#[apply(base)]
pub struct BoundCreateGraphTypeAndGraphStatement {
    pub create_graph_type: Option<BoundCreateGraphTypeStatement>,
    pub create_graph: Option<BoundCreateGraphStatement>,
}

#[apply(base)]
pub struct BoundDropGraphStatement {
    pub schema_id: SchemaId,
    pub graph_id: Vec<GraphId>,
    pub if_exists: bool,
}

#[apply(base)]
pub enum BoundCreateGraphTypeStatementOrNot {
    // Some statements don't need to be executed and
    // be skipped here, therefore a SKIP marker is required.
    Process(BoundCreateGraphTypeStatement),
    Skip,
}

#[derive(Debug, Serialize)]
pub struct BoundCreateGraphTypeStatement {
    pub schema: SchemaCatalog,
    pub name: Ident,
    pub kind: CreateGraphOrGraphTypeStatementKind,
    pub source: BoundGraphTypeSource,
}

#[derive(Debug, Serialize)]
pub enum BoundGraphTypeSource {
    Ref(BoundGraphTypeRef),
    Nested(Vec<GraphElementType>),
}


#[derive(Debug, Serialize)]
pub enum BoundGraphTypeRef {
    Ref(GraphTypeCatalog),
    Parameter(Ident),
}


#[apply(base)]
pub struct BoundDropGraphTypeStatement {
    // If path is none, just skip this drop stmt.
    pub path: Option<BoundCatalogObjectRef>,
    pub if_exists: bool,
}


