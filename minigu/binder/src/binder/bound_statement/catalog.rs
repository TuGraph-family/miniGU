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
