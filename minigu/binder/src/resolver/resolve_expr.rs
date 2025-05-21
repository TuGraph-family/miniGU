use smol_str::ToSmolStr;
use gql_parser::ast::{GraphExpr, GraphRef};
use crate::binder::binder::Binder;
use crate::bound_statement::expr::BoundGraphExpr;
use crate::catalog_ref::GraphCatalog;
use crate::error::{BindError, BindResult};

impl Binder {
    pub(crate) fn resolve_graph_expr(&self, expr: &GraphExpr) -> BindResult<BoundGraphExpr> {
        let resolve_graph = |name: &str| {
            self.schema
                .get_graph(name)
                .map_err(|e| BindError::External(Box::new(e)))?
                .ok_or_else(|| BindError::GraphNotExists(name.to_string()))
        };
        match expr {
            GraphExpr::Name(name) => {
                let graph = resolve_graph(name)?;
                Ok(BoundGraphExpr::Ref(GraphCatalog {
                    name:name.clone(),
                    obj_ref: graph,
                }))
            },
            GraphExpr::Ref(graph_expr) => {
                match graph_expr {
                    GraphRef::Name(name) => {
                        let graph = resolve_graph(name)?;
                        Ok(BoundGraphExpr::Ref(GraphCatalog {
                            name:name.clone(),
                            obj_ref: graph,
                        }))
                    },
                    GraphRef::Ref(catalog_ref) => {
                        let schema = catalog_ref
                            .schema
                            .as_ref()
                            .map(|s| self.resolve_schema_ref(s.value()))
                            .transpose()?
                            .unwrap_or_else(|| self.schema.clone());
                        if catalog_ref.objects.len() != 1 {
                            return Err(BindError::NotSupported("Only one graph is allowed".to_string()));
                        };
                        let graph_name = catalog_ref.objects[0].value().clone();
                        let graph =
                            schema.get_graph(graph_name.as_str())
                        .map_err(|e| BindError::External(Box::new(e)))?
                        .ok_or_else(|| BindError::GraphNotExists(graph_name.to_string()))?;
                        Ok(BoundGraphExpr::Ref (
                            GraphCatalog {
                                name: graph_name.clone(),
                                obj_ref: graph,
                            }
                        ))
                    },
                    GraphRef::Parameter(param) => {
                        return Err(BindError::NotSupported("Parameter".to_string()));
                    }
                    GraphRef::Home => {
                        return Err(BindError::NotSupported("Home".to_string()));
                    }
                }
            },
            GraphExpr::Current => {
                Ok(BoundGraphExpr::Ref(GraphCatalog {
                    name: "current".to_smolstr(),
                    obj_ref: self.graph.clone(),
                }))
            }
            GraphExpr::Object(obj) => {
                return Err(BindError::NotSupported("Object expression".to_string()));
            }
        }
    }
}