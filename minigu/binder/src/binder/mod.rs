mod catalog;
mod object_ref;
mod procedure_call;
mod procedure_spec;
mod query;
mod value_expr;

use gql_parser::ast::Procedure;
use minigu_catalog::provider::{CatalogRef, SchemaRef};

use crate::error::BindResult;
use crate::named_ref::NamedGraphRef;
use crate::procedure::procedure_spec::BoundProcedure;

#[derive(Debug)]
pub struct Binder {
    catalog: CatalogRef,
    current_schema: Option<SchemaRef>,
    home_schema: Option<SchemaRef>,
    current_graph: Option<NamedGraphRef>,
    home_graph: Option<NamedGraphRef>,
}

impl Binder {
    #[inline]
    pub fn new(
        catalog: CatalogRef,
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
        }
    }

    #[inline]
    pub fn bind(mut self, procedure: &Procedure) -> BindResult<BoundProcedure> {
        self.bind_procedure(procedure)
    }
}
