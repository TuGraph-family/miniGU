#![allow(unused)]

use gql_parser::ast::Procedure;
use minigu_catalog::provider::{CatalogRef, SchemaRef};

use crate::error::BindResult;
use crate::program::Binder;
use crate::statement::procedure_spec::BoundProcedure;

mod error;
mod mock_catalog;
mod object_ref;
mod program;
mod resolver;
mod statement;
mod type_checker;
mod types;
mod validator;

pub fn bind(
    procedure: &Procedure,
    catalog: CatalogRef,
    current_schema: Option<SchemaRef>,
) -> BindResult<BoundProcedure> {
    let mut binder = Binder::new(catalog, current_schema);
    binder.bind_procedure(procedure)
}
