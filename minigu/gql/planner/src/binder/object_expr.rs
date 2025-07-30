use gql_parser::ast::GraphExpr;
use minigu_catalog::named_ref::NamedGraphRef;
use minigu_common::error::not_implemented;

use super::error::{BindError, BindResult};
use crate::binder::Binder;

impl Binder<'_> {
    pub fn bind_graph_expr(&self, expr: &GraphExpr) -> BindResult<NamedGraphRef> {
        match expr {
            GraphExpr::Name(_) => not_implemented("graph expression from name", None),
            GraphExpr::Object(_) => {
                not_implemented("graph expression from object expression", None)
            }
            GraphExpr::Ref(graph_ref) => self.bind_graph_ref(graph_ref),
            GraphExpr::Current => self
                .current_graph
                .clone()
                .ok_or(BindError::CurrentGraphNotSpecified),
        }
    }
}
