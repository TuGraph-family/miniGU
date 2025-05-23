use macro_rules_attribute::apply;
use serde::Serialize;
use gql_parser::ast::{BindingVariableDefBlock, Yield};
use gql_parser::span::OptSpanned;
use crate::bound_statement::catalog::LinearBoundCatalogModifyingStatement;
use crate::bound_statement::data::BoundLinearDataModifyingStatement;
use crate::bound_statement::query::BoundCompositeQueryStatement;
use crate::catalog_ref::SchemaCatalogRef;
use crate::macros::base;

#[derive(Debug, Serialize)]
pub struct BoundProcedure {
    pub at: Option<SchemaCatalogRef>,
    pub binding_variable_def: BindingVariableDefBlock,
    pub statement: BoundStatement,
    pub next_statement: Vec<BoundNextStatement>,
}


#[derive(Debug, Serialize)]
pub enum BoundStatement {
    Catalog(LinearBoundCatalogModifyingStatement),
    Query(BoundCompositeQueryStatement),
    Data(BoundLinearDataModifyingStatement),
}


#[derive(Debug, Serialize)]
pub struct BoundNextStatement {
    pub yield_clause: OptSpanned<Yield>,
    pub statement: BoundStatement,
}
