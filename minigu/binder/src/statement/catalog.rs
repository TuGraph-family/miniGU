use serde::Serialize;

use crate::object_ref::{GraphRef, SchemaRef};
use crate::statement::object_ref::BoundGraphType;
use crate::statement::procedure::BoundCallProcedureStatement;
use crate::types::Ident;

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

#[derive(Debug, Serialize)]
pub struct BoundCreateSchemaStatement {
    pub schema_path: Vec<Ident>,
    pub if_not_exists: bool,
}

#[derive(Debug, Serialize)]
pub struct BoundDropSchemaStatement {
    pub schema_path: Vec<Ident>,
    pub if_exists: bool,
}

#[derive(Debug, Serialize)]
pub struct BoundCreateGraphStatement {
    pub schema: SchemaRef,
    pub name: Ident,
    pub kind: CreateKind,
    pub type_ref: BoundGraphType,
    pub source: Option<GraphRef>,
}

#[derive(Debug, Serialize)]
pub enum CreateKind {
    Create,
    CreateIfNotExists,
    CreateOrReplace,
}

#[derive(Debug, Serialize)]
pub struct BoundDropGraphStatement {
    pub schema: SchemaRef,
    pub graph: Ident,
    pub if_exists: bool,
}

#[derive(Debug, Serialize)]
pub struct BoundCreateGraphTypeStatement {
    pub schema: SchemaRef,
    pub name: Ident,
    pub kind: CreateKind,
    pub source: BoundGraphType,
}

#[derive(Debug, Serialize)]
pub struct BoundDropGraphTypeStatement {
    pub schema: SchemaRef,
    pub graph_type: Ident,
    pub if_exists: bool,
}
