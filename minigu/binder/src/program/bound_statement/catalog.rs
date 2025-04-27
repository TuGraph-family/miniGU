use gql_parser::ast::{CreateGraphOrGraphTypeStatementKind, Ident};
use macro_rules_attribute::apply;
use minigu_catalog::schema::SchemaId;
use crate::macros::base;
use crate::program::bound_statement::common::{GraphId, LabelId};
use crate::program::bound_statement::expr::BoundGraphExpr;
use crate::program::bound_statement::object_ref::{BoundCatalogObjectRef, BoundCatalogObjectStrRef, BoundGraphTypeSource, BoundSchemaRef, CanonicalSchemaPath};
use crate::program::bound_statement::procedure::BoundCallProcedureStatement;

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
    pub path: CanonicalSchemaPath,
    pub if_not_exists: bool,
}

#[apply(base)]
pub struct BoundDropSchemaStatement {
    pub schema_id: SchemaId,
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

#[apply(base)]
pub struct BoundCreateGraphTypeStatement {
    pub path: BoundCatalogObjectStrRef,
    pub kind: CreateGraphOrGraphTypeStatementKind,
    pub source: BoundGraphTypeSource,
}

#[apply(base)]
pub struct BoundDropGraphTypeStatement {
    // If path is none, just skip this drop stmt.
    pub path: Option<BoundCatalogObjectRef>,
    pub if_exists: bool,
}

pub type LabelIdSet = Vec<LabelId>;
