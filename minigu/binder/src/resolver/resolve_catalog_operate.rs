use std::error::Error;

use gql_parser::ast::{CallProcedureStatement, CreateGraphOrGraphTypeStatementKind, CreateGraphStatement, CreateGraphTypeStatement, CreateSchemaStatement, DropGraphStatement, DropGraphTypeStatement, DropSchemaStatement, GraphTypeSource, OfGraphType, ProcedureCall, ProcedureRef};
use smol_str::ToSmolStr;

use crate::binder::binder::Binder;
use crate::bound_statement::catalog::{
    BoundCreateGraphStatement, BoundCreateGraphTypeAndGraphStatement,
    BoundCreateGraphTypeStatement, BoundCreateSchemaStatement, BoundDropGraphStatement,
    BoundDropGraphTypeStatement, BoundDropSchemaStatement, BoundGraphTypeRef, BoundGraphTypeSource,
};
use crate::bound_statement::expr::BoundGraphExpr;
use crate::bound_statement::object_ref::{normalize_path, BoundOfGraphType};
use crate::bound_statement::procedure::{BoundCallProcedureStatement, BoundNamedProcedureCall, BoundProcedureCall};
use crate::catalog_ref::{GraphTypeCatalogRef, SchemaCatalogRef};
use crate::error::{BindError, BindResult};

impl Binder {
    pub(crate) fn resolve_call_procedure(
        &self,
        call: &CallProcedureStatement,
    ) -> BindResult<BoundCallProcedureStatement> {
        let bound_procedure = match call.procedure.value() {
            ProcedureCall::Inline(inline) => {
                return Err(BindError::NotSupported("Call inline procedure".to_string()));
            }
            ProcedureCall::Named(named) => {
                let mut expr_vec = vec![];
                let procedure = self.resolve_procedure_ref(named.name.value())?;
                for expr in named.args.iter() {
                    expr_vec.push(self.resolve_expr(expr.value())?)
                }
                // TODO: Handle yield_clause
                BoundProcedureCall::Named(BoundNamedProcedureCall {
                    name: procedure,
                    args: expr_vec,
                    yield_clause: named.yield_clause.as_ref().map(|sp| sp.value().clone()),
                })
            }
        };
        Ok(BoundCallProcedureStatement {
            optional: call.optional,
            procedure: bound_procedure,
        })
    }

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
                        BoundGraphTypeRef::Ref(GraphTypeCatalogRef {
                            name: graph_ref.name,
                            graph_type_ref: graph_ref.graph_ref.graph_type(),
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
            schema: SchemaCatalogRef {
                name: schema_name,
                schema_ref: schema,
            },
            name: graph_name,
            kind: create.kind.value().clone(),
            source: bound_graph_type_source,
        })
    }

    pub(crate) fn resolve_drop_graph_type(
        &self,
        drop: &DropGraphTypeStatement,
    ) -> BindResult<BoundDropGraphTypeStatement> {
        let schema = match &drop.path.value().schema {
            Some(path) => self.resolve_schema_ref(path.value())?,
            None => self.schema.clone(),
        };

        let graph_type_names = drop
            .path
            .value()
            .objects
            .iter()
            .map(|v| v.value().clone())
            .collect();
        Ok(BoundDropGraphTypeStatement {
            schema: SchemaCatalogRef {
                name: "".to_smolstr(),
                schema_ref: schema.clone(),
            },
            type_list: graph_type_names,
            if_exists: drop.if_exists,
        })
    }

    pub(crate) fn resolve_create_graph(
        &self,
        create: &CreateGraphStatement,
    ) -> BindResult<BoundCreateGraphTypeAndGraphStatement> {
        let schema = match &create.path.value().schema {
            Some(path) => self.resolve_schema_ref(path.value())?,
            None => self.schema.clone(),
        };

        if create.path.value().objects.len() != 1 {
            return Err(BindError::NotSupported("more than one graph".to_string()));
        }

        let graph_name = &create.path.value().objects[0].value().clone();

        let graph = schema
            .get_graph(graph_name.as_str())
            .map_err(|e| BindError::External(Box::new(e)))?;

        match &create.kind.value() {
            CreateGraphOrGraphTypeStatementKind::Create => {
                if graph.is_some() {
                    return Err(BindError::GraphAlreadyExists(graph_name.to_string()));
                }
            }
            _ => {}
        }

        let mut result = BoundCreateGraphTypeAndGraphStatement {
            create_graph_type: None,
            create_graph: None,
        };

        // 1. resolve create graph.
        let of_type_ref = match &create.graph_type.value() {
            OfGraphType::Like(like) => {
                BoundOfGraphType::Like(self.resolve_graph_expr(like.value())?)
            }
            OfGraphType::Ref(of_type_ref) => {
                BoundOfGraphType::Ref(self.resolve_graph_type_ref(of_type_ref.value())?)
            }
            OfGraphType::Nested(nested_vec) => {
                // When creating a graph using the Nested method, we need to first create a
                // corresponding GraphType, and only then can we reference that GraphType.
                let graph_type_name = format!("{}_graph_type", graph_name);
                let create_graph_type_stmt = BoundCreateGraphTypeStatement {
                    schema: SchemaCatalogRef {
                        name: "".to_smolstr(),
                        schema_ref: schema.clone(),
                    },
                    name: graph_type_name.to_smolstr(),
                    kind: CreateGraphOrGraphTypeStatementKind::Create,
                    source: BoundGraphTypeSource::Nested(
                        nested_vec.iter().map(|v| v.value().clone()).collect(),
                    ),
                };
                result.create_graph_type = Some(create_graph_type_stmt);
                BoundOfGraphType::Ref(BoundGraphTypeRef::Name(graph_type_name.to_smolstr()))
            }

            OfGraphType::Any => BoundOfGraphType::Any,
        };

        let source = create
            .source
            .as_ref()
            .map(|v| self.resolve_graph_expr(v.value()))
            .transpose()?;
        result.create_graph = Some(BoundCreateGraphStatement {
            schema: SchemaCatalogRef {
                name: "".to_smolstr(),
                schema_ref: schema.clone(),
            },
            name: graph_name.clone(),
            kind: create.kind.value().clone(),
            type_ref: of_type_ref,
            source,
        });

        Ok(result)
    }

    pub(crate) fn resolve_drop_graph(
        &self,
        drop: &DropGraphStatement,
    ) -> BindResult<BoundDropGraphStatement> {
        let schema = match &drop.path.value().schema {
            Some(path) => self.resolve_schema_ref(path.value())?,
            None => self.schema.clone(),
        };
        let graph_type_names = drop
            .path
            .value()
            .objects
            .iter()
            .map(|v| v.value().clone())
            .collect();

        Ok(BoundDropGraphStatement {
            schema: SchemaCatalogRef {
                name: "".to_smolstr(),
                schema_ref: schema.clone(),
            },
            graph_list: graph_type_names,
            if_exists: drop.if_exists,
        })
    }
}
