use gql_parser::ast::Procedure;
use minigu_catalog::provider::{CatalogRef, SchemaRef};

use crate::error::BindResult;
use crate::procedure::procedure_spec::BoundProcedure;
// use crate::program::Binder;

mod binder;
mod error;
mod named_ref;
mod procedure;
// mod resolver;

// #[cfg(test)]
// mod mock;

// pub fn bind(
//     procedure: &Procedure,
//     catalog: CatalogRef,
//     current_schema: Option<SchemaRef>,
// ) -> BindResult<BoundProcedure> {
//     let mut binder = Binder::new(catalog, current_schema);
//     binder.bind_procedure(procedure)
// }
