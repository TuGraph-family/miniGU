use serde::Serialize;
use smol_str::SmolStr;

use super::object_ref::BoundGraphType;
use super::procedure_call::BoundCallProcedureStatement;
use crate::named_ref::NamedGraphRef;

#[derive(Debug, Clone, Serialize)]
pub enum BoundCatalogModifyingStatement {
    Call(BoundCallProcedureStatement),
    CreateSchema(BoundCreateSchemaStatement),
    DropSchema(BoundDropSchemaStatement),
    CreateGraph(BoundCreateGraphStatement),
    DropGraph(BoundDropGraphStatement),
    CreateGraphType(BoundCreateGraphTypeStatement),
    DropGraphType(BoundDropGraphTypeStatement),
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundCreateSchemaStatement {
    pub schema_path: Vec<SmolStr>,
    pub if_not_exists: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundDropSchemaStatement {
    pub schema_path: Vec<SmolStr>,
    pub if_exists: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundCreateGraphStatement {
    // pub schema: SchemaRef,
    pub name: SmolStr,
    pub kind: CreateKind,
    pub graph_type: BoundGraphType,
    pub source: Option<NamedGraphRef>,
}

#[derive(Debug, Clone, Serialize)]
pub enum CreateKind {
    Create,
    CreateIfNotExists,
    CreateOrReplace,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundDropGraphStatement {
    // pub schema: NamedSchemaRef,
    pub name: SmolStr,
    pub if_exists: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundCreateGraphTypeStatement {
    // pub schema: NamedSchemaRef,
    pub name: SmolStr,
    pub kind: CreateKind,
    pub source: BoundGraphType,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundDropGraphTypeStatement {
    //  pub schema: NamedSchemaRef,
    pub name: SmolStr,
    pub if_exists: bool,
}
