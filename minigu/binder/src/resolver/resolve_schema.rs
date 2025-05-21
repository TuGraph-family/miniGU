use gql_parser::ast::{CreateGraphOrGraphTypeStatementKind, CreateGraphStatement, CreateGraphTypeStatement, CreateSchemaStatement, DropSchemaStatement, GraphTypeRef, GraphTypeSource};
use smol_str::ToSmolStr;

use crate::binder::binder::Binder;
use crate::bound_statement::catalog::{BoundCreateGraphStatement, BoundCreateGraphTypeStatement, BoundCreateSchemaStatement, BoundDropSchemaStatement, BoundGraphTypeRef, BoundGraphTypeSource};
use crate::bound_statement::expr::BoundGraphExpr;
use crate::bound_statement::object_ref::normalize_path;
use crate::catalog_ref::{GraphTypeCatalog, SchemaCatalog};
use crate::error::{BindError, BindResult};

impl Binder {
    pub(crate) fn resolve_create_schema(
        &self,
        create: &CreateSchemaStatement,
    ) -> BindResult<BoundCreateSchemaStatement> {
        let path = normalize_path(create.path.value());
        let schema = self.resolve_canonical_schema_path(&path);
        if schema.is_ok() && !create.if_not_exists {
            return Err(BindError::SchemaAlreadyExists(path.to_string()));
        };
        Ok(BoundCreateSchemaStatement {
            schema_path: path.clone(),
            if_not_exists: create.if_not_exists,
        })
    }

    pub(crate) fn resolve_drop_schema(
        &self,
        drop: &DropSchemaStatement,
    ) -> BindResult<BoundDropSchemaStatement> {
        let path = normalize_path(drop.path.value());
        let schema = self.resolve_canonical_schema_path(&path);
        if schema.is_err() && !drop.if_exists {
            return Err(BindError::SchemaNotExists(path.to_string()));
        };
        Ok(BoundDropSchemaStatement {
            schema_path: path.clone(),
            if_exists: drop.if_exists,
        })
    }

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
                Ok(BoundGraphTypeRef::Ref(GraphTypeCatalog {
                    name,
                    obj_ref: graph_type_catalog,
                }))
            }
            GraphTypeRef::Parameter(param) => Ok(BoundGraphTypeRef::Parameter(param.clone())),
        };
        bound_graph_type_ref
    }

    pub(crate) fn resolve_create_graph_type(
        &self,
        create: &CreateGraphTypeStatement,
    ) -> BindResult<BoundCreateGraphTypeStatement> {
        let mut schema_name = "name".to_smolstr();
        let schema = match &create.path.value().schema {
            Some(path) => self.resolve_schema_ref(path.value())?,
            None => self.schema.clone(),
        };
        // Only Support create one graph type in statement.
        if create.path.value().objects.len() > 1 {
            return Err(BindError::NotSupported(
                "more than one graph type name".to_string(),
            ));
        };

        let graph_name = create.path.value().objects[0].value().clone();

        let graph = schema
            .get_graph(graph_name.as_str())
            .map_err(|e| BindError::External(Box::new(e)))?;
        match create.kind.value() {
            CreateGraphOrGraphTypeStatementKind::Create => {
                if graph.is_some() {
                    return Err(BindError::GraphTypeAlreadyExists(graph_name.to_string()));
                }
            }
            _ => {}
        }

        let bound_graph_type_source = match create.source.value() {
            GraphTypeSource::Copy(copy) => {
                BoundGraphTypeSource::Ref(self.resolve_graph_type_ref(copy.value())?)
            }
            GraphTypeSource::Like(like) => {
                let bound_graph_expr = self.resolve_graph_expr(like.value())?;
                match bound_graph_expr {
                    BoundGraphExpr::Ref(graph_ref) => Ok(BoundGraphTypeSource::Ref(
                        BoundGraphTypeRef::Ref(GraphTypeCatalog {
                            name: graph_ref.name,
                            obj_ref: graph_ref.obj_ref.graph_type(),
                        }),
                    )),
                    _ => Err(BindError::NotSupported("object expr".to_string())),
                }?
            }
            GraphTypeSource::Nested(element_vec) => BoundGraphTypeSource::Nested(
                element_vec
                    .into_iter()
                    .map(|element| element.value().clone())
                    .collect(),
            ),
        };

        Ok(BoundCreateGraphTypeStatement {
            schema: SchemaCatalog {
                name: schema_name,
                obj_ref: schema,
            },
            name: graph_name,
            kind: create.kind.value().clone(),
            source: bound_graph_type_source,
        })
    }
    
    pub(crate) fn resolve_create_graph(&self, create: &CreateGraphStatement) -> BindResult<BoundCreateGraphStatement> {
        let schema = match &create.path.value().schema {
            Some(path) => self.resolve_schema_ref(path.value())?,
            None => self.schema.clone(),
        };
        
        
        
    }
}
