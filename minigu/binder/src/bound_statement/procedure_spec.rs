use macro_rules_attribute::apply;
use serde::Serialize;
use gql_parser::ast::{BindingVariableDefBlock, Yield};
use gql_parser::span::OptSpanned;
use crate::binder::bound_statement::data::BoundLinearDataModifyingStatement;
use crate::binder::bound_statement::query::BoundCompositeQueryStatement;
use crate::bound_statement::catalog::LinearBoundCatalogModifyingStatement;
use crate::catalog_ref::SchemaCatalog;
use crate::macros::base;

#[derive(Debug, Serialize)]
pub struct BoundProcedure {
    pub at: Option<SchemaCatalog>,
    pub binding_variable_def: BindingVariableDefBlock,
    pub statement: BoundStatement,
    pub next_statement: Vec<BoundNextStatement>,
}


#[apply(base)]
pub enum BoundStatement {
    Catalog(LinearBoundCatalogModifyingStatement),
    Query(BoundCompositeQueryStatement),
    Data(BoundLinearDataModifyingStatement),
}


#[apply(base)]
pub struct BoundNextStatement {
    pub yield_clause: OptSpanned<Yield>,
    pub statement: BoundStatement,
}
