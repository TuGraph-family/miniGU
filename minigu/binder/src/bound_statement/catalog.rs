use macro_rules_attribute::apply;
use serde::Serialize;
use gql_parser::ast::{CreateGraphOrGraphTypeStatementKind, GraphElementType, Ident};
use crate::bound_statement::expr::BoundGraphExpr;
use crate::bound_statement::object_ref::{BoundOfGraphType, CanonicalSchemaPath};
use crate::bound_statement::procedure::BoundCallProcedureStatement;
use crate::catalog_ref::{GraphTypeCatalogRef, SchemaCatalogRef};
use crate::macros::base;

pub type LinearBoundCatalogModifyingStatement = Vec<BoundCatalogModifyingStatement>;

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct BoundCreateGraphStatement {
    pub schema: SchemaCatalogRef,
    pub name: Ident,
    pub kind: CreateGraphOrGraphTypeStatementKind,
    pub type_ref: BoundOfGraphType,
    pub source: Option<BoundGraphExpr>,
}

#[derive(Debug, Serialize)]
pub struct BoundCreateGraphTypeAndGraphStatement {
    // When creating a graph, the corresponding graph type might not have been created yet,
    // so two statements need to be returned simultaneously.
    pub create_graph_type: Option<BoundCreateGraphTypeStatement>,
    pub create_graph: Option<BoundCreateGraphStatement>,
}

#[derive(Debug, Serialize)]
pub struct BoundDropGraphStatement {
    pub schema: SchemaCatalogRef,
    pub graph_list: Vec<Ident>,
    pub if_exists: bool,
}

#[derive(Debug, Serialize)]
pub struct BoundCreateGraphTypeStatement {
    pub schema: SchemaCatalogRef,
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
    Ref(GraphTypeCatalogRef),
    // When creating a graph, the corresponding graph type may needs to be created at the same time. 
    // Since the reference does not exist yet, the name must be recorded instead.
    Name(Ident),
    Parameter(Ident),
}


#[derive(Debug, Serialize)]
pub struct BoundDropGraphTypeStatement {
    pub schema: SchemaCatalogRef,
    pub type_list: Vec<Ident>,
    pub if_exists: bool,
}


