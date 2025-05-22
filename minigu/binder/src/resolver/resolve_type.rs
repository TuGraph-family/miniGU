use gql_parser::ast::GraphTypeRef;
use crate::binder::binder::Binder;
use crate::bound_statement::catalog::BoundGraphTypeRef;
use crate::catalog_ref::GraphTypeCatalogRef;
use crate::error::{BindError, BindResult};

impl Binder {
    pub(crate) fn resolve_graph_type_ref(
        &self,
        graph_type_ref: &GraphTypeRef,
    ) -> BindResult<BoundGraphTypeRef> {
        let bound_graph_type_ref = match graph_type_ref {
            GraphTypeRef::Ref(type_ref) => {
                let schema = type_ref
                    .schema
                    .as_ref()
                    .map(|s| self.resolve_schema_ref(s.value()))
                    .transpose()?
                    .unwrap_or_else(|| self.schema.clone());

                if type_ref.objects.len() != 1 || type_ref.objects[0].value().is_empty() {
                    return Err(BindError::NotSupported(
                        "Can only ref one graph type".to_string(),
                    ));
                }

                let name = type_ref.objects[0].value().clone();

                let graph_type_catalog = schema
                    .get_graph_type(name.as_str())
                    .map_err(|e| BindError::External(Box::new(e)))?
                    .ok_or_else(|| BindError::GraphTypeNotExists(name.to_string()))?;
                Ok(BoundGraphTypeRef::Ref(GraphTypeCatalogRef {
                    name,
                    graph_type_ref: graph_type_catalog,
                }))
            }
            GraphTypeRef::Parameter(param) => Ok(BoundGraphTypeRef::Parameter(param.clone())),
        };
        bound_graph_type_ref
    }
}