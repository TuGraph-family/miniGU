mod catalog;
mod common;
mod object_expr;
mod object_ref;
mod procedure_call;
mod procedure_spec;
mod query;
mod value_expr;

use gql_parser::ast::Procedure;
use minigu_catalog::provider::{CatalogProvider, SchemaRef};
use minigu_common::data_type::DataSchema;
use minigu_ir::bound::BoundProcedure;
use minigu_ir::named_ref::NamedGraphRef;

use crate::error::BindResult;

#[derive(Debug)]
pub struct Binder<'a> {
    catalog: &'a dyn CatalogProvider,

    current_schema: Option<SchemaRef>,
    home_schema: Option<SchemaRef>,
    current_graph: Option<NamedGraphRef>,
    home_graph: Option<NamedGraphRef>,

    active_data_schema: Option<DataSchema>,
}

impl<'a> Binder<'a> {
    pub fn new(
        catalog: &'a dyn CatalogProvider,
        current_schema: Option<SchemaRef>,
        home_schema: Option<SchemaRef>,
        current_graph: Option<NamedGraphRef>,
        home_graph: Option<NamedGraphRef>,
    ) -> Self {
        Binder {
            catalog,
            current_schema,
            home_schema,
            current_graph,
            home_graph,
            active_data_schema: None,
        }
    }

    pub fn bind(mut self, procedure: &Procedure) -> BindResult<BoundProcedure> {
        self.bind_procedure(procedure)
    }
}
