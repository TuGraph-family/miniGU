mod catalog;
mod object_ref;
mod procedure_call;
mod procedure_spec;
mod query;
mod value_expr;

use std::mem;

use gql_parser::ast::Procedure;
use minigu_catalog::provider::{CatalogRef, SchemaRef};
use minigu_ir::bound::BoundProcedure;
use minigu_ir::named_ref::NamedGraphRef;

use crate::context::BindContext;
use crate::error::BindResult;

#[derive(Debug)]
pub struct Binder {
    catalog: CatalogRef,
    current_schema: Option<SchemaRef>,
    home_schema: Option<SchemaRef>,
    current_graph: Option<NamedGraphRef>,
    home_graph: Option<NamedGraphRef>,
    context: BindContext,
}

impl Binder {
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
            context: BindContext::new(),
        }
    }

    pub fn bind(mut self, procedure: &Procedure) -> BindResult<BoundProcedure> {
        self.bind_procedure(procedure)
    }

    fn take_context(&mut self) -> BindContext {
        mem::take(&mut self.context)
    }

    fn set_context(&mut self, context: BindContext) {
        self.context = context;
    }
}
