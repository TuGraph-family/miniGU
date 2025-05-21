mod data;
pub mod expr;
pub mod query;
mod resolver;
mod resolve_schema;
mod resolve_catalog;
mod resolve_expr;

use smol_str::ToSmolStr;
use gql_parser::ast::*;
use gql_parser::span::Spanned;
use minigu_catalog::schema::CatalogInstance;
use crate::mock::schema::Schema as MockSchema;
use crate::mock::schema::Catalog as MockCatalog;

use crate::error::{
    Error, GraphNotExistsError, GraphTypeNotExistsError, NotSupportError, SchemaAlreadyExistsError,
    SchemaNotExistsError,
};
use crate::program::bound_statement::catalog::{BoundCreateSchemaStatement, *};
use crate::program::bound_statement::common::{BoundStatement, LabelId};
use crate::program::bound_statement::expr::{BoundGraphExpr, *};
use crate::program::bound_statement::object_ref::{BoundOfGraphType, *};
use crate::program::bound_statement::procedure::*;
use crate::program::{Binder, SessionContext};




impl Binder {
    pub fn resolve_of_graph_type(&self, graph_type: &OfGraphType) -> Result<BoundOfGraphType, Error> {
        match &graph_type {
            OfGraphType::Like(expr) => match expr.value() {
                GraphExpr::Name(ident) => {
                    match CatalogInstance::read().get_schema(self.session_context.current_working_path.segments.as_slice()) {
                        Some(schema) => {
                            let graph = schema.get_graph_id(ident.as_str());
                            if let Some(graph) = graph {
                                Ok(BoundOfGraphType::Like(BoundGraphExpr::Ref(
                                    BoundCatalogObjectRef {     
                                        schema: self.session_context.current_working_path.clone(),
                                        object: vec![graph]
                                    }
                                )))
                            } else {
                                return Err(Error::GraphNotExists(ident.clone().to_string()));
                            }
                        }
                        None => {
                            return Err(Error::GraphNotExists(ident.clone().to_string()));
                        }
                    }
                },
                GraphExpr::Ref(graph_ref) => {
                    let gra_ref = self.resolve_graph_ref(graph_ref)?;
                    Ok(BoundOfGraphType::Ref(BoundGraphTypeRef::Ref(
                        gra_ref.path,
                    )))
                }
                _ => {
                    // TODO: Support Parameter.
                    return Err(Error::NotSupported("object".to_string()));
                }
            },
            
            OfGraphType::Ref(ref_path) => {
                let type_ref = self.resolve_graph_type_ref(ref_path.value())?;
                match type_ref {
                    BoundGraphTypeRef::Ref(graph_type) => {
                        Ok(BoundOfGraphType::Ref(
                            BoundGraphTypeRef::Ref(
                                BoundCatalogObjectRef {
                                    schema: graph_type.schema.clone(),
                                    object: graph_type.object.clone(),
                                }
                            )
                        ))
                    },
                    BoundGraphTypeRef::Parameter(ident) => {
                        return Err(Error::NotSupported("Graph type parameter".to_string()));
                    }
                }
            }
            OfGraphType::Any => Ok(BoundOfGraphType::Any),
            OfGraphType::Nested(nested) =>  Ok(BoundOfGraphType::Nested(
                nested
                    .into_iter()
                    .map(|element| element.value().clone())
                    .collect(),
            ))
        }
    }

    pub fn resolve_schema_path(&self, schema_path: &SchemaPath) -> CanonicalSchemaPath {
        normalize_path(&schema_path)
    }
    
    pub fn resolve_schema_ref(&self, schema_ref: &Option<Spanned<SchemaRef>>) -> CanonicalSchemaPath {
        if let Some(schema) = schema_ref {
            let bound_schema_ref = resolve_schema_ref(&self.session_context, &schema.value());
            bound_schema_ref
                .as_canonical()
                .expect("Expected canonical schema path")
                .clone()
        } else {
            self.session_context.current_working_path.clone()
        }
    }

    /// Resolves a CatalogObjectRef into a BoundCatalogObjectRef based on the specified object type
    /// (obj_type), by obtaining a canonical schema path and converting object names to their
    /// corresponding ID identifiers. This function handles various types of catalog objects
    /// (procedures, graph types, graphs, etc.), mapping them from name strings to internal numeric
    /// IDs and returns an error if Schema or any object doesn't exist.
    pub fn resolve_catalog_obj(
        &self,
        catalog_object_ref: &CatalogObjectRef,
        obj_type: CatalogObjRefType,
    ) -> Result<BoundCatalogObjectRef, Error> {
        let canonical_path = self.resolve_schema_ref(&catalog_object_ref.schema);

        let mut obj_vec = vec![];
        // TODO: Get Schema.
        let mut catalog = MockCatalog::default();
        let schema = catalog.get_schema_mut(canonical_path.segments.as_slice()).unwrap();
        
        for obj in &catalog_object_ref.objects {
            let obj_id = match obj_type {
                CatalogObjRefType::ProcedureRef => schema.get_procedure_id(obj.value().as_str()),
                CatalogObjRefType::GraphTypeRef => schema.get_graph_type_id(obj.value().as_str()),
                CatalogObjRefType::GraphRef => schema.get_graph_id(obj.value().as_str()),
                _ => None
            };
            match obj_id {
                Some(id) => obj_vec.push(id),
                None => return Err(
                    match obj_type {
                        CatalogObjRefType::ProcedureRef => {Error::ProcedureNotExists(obj.value().to_string())},
                        CatalogObjRefType::GraphTypeRef => {Error::GraphTypeNotExists(obj.value().to_string())},
                        CatalogObjRefType::GraphRef => {Error::GraphNotExists(obj.value().to_string())},
                        CatalogObjRefType::BindingTableRef => {Error::NotSupported("BindingTableRef".to_string())},
                    }
                ),
            }
        }
        
        Ok(BoundCatalogObjectRef {
            schema: schema.get_schema_id(),
            object: obj_vec.clone(),
        })
    }

    pub fn resolve_create_schema(
        &self,
        create_schema_statement: &CreateSchemaStatement,
    ) -> Result<BoundCreateSchemaStatement, Error> {
        let path = self.resolve_schema_path(create_schema_statement.path.value());
        // TODO: Catalog.
        let schema_exists = CatalogInstance::read()
            .get_schema(&path.segments)
            .is_some();
        if schema_exists && !create_schema_statement.if_not_exists {
            return Err(Error::SchemaAlreadyExists(path.to_string()));
        }
        Ok(BoundCreateSchemaStatement {
            path: path.clone(),
            if_not_exists: create_schema_statement.if_not_exists,
        })
    }

    pub fn resolve_drop_schema(
        &self,
        drop_schema_statement: &DropSchemaStatement,
    ) -> Result<BoundDropSchemaStatement, Error> {
        let path = self.resolve_schema_path(drop_schema_statement.path.value());
        // TODO: Get Schema and get Schema id.
        let schema_exists = CatalogInstance::read()
            .get_schema(&path.segments)
            .is_some();

        if !schema_exists && !drop_schema_statement.if_exists {
            return Err(Error::SchemaNotExists(path.to_string()));
        };
        
        // TODO: Add Get Schema ID;
        Ok(BoundDropSchemaStatement {
            schema_id: 1,
            if_exists: drop_schema_statement.if_exists,
        })
    }

    pub fn resolve_graph_ref(&self, graph_ref: &GraphRef) -> Result<BoundGraphRef, Error> {
        match graph_ref {
            GraphRef::Name(ident) =>  {
                let schema_ref = self.get_available_schema()?;
                schema_ref.get_graph_id(ident.as_str())
                    .map(|id| {
                        let schema_path = schema_ref.get_schema_path();
                        BoundGraphRef {
                            path: BoundCatalogObjectRef {
                                schema: CanonicalSchemaPath { segments: schema_path },
                                object: vec![id],
                            }
                        }
                    })
                    .ok_or(Error::ErrorCur)
            }
            GraphRef::Ref(graph_ref) => {
                self.resolve_catalog_obj(graph_ref, CatalogObjRefType::GraphRef)
                    .map(|catalog_ref| BoundGraphRef { path: catalog_ref })
            }
            GraphRef::Home => {
                let schema_ref = self.get_available_schema()?;
                Ok(BoundGraphRef {
                    path: BoundCatalogObjectRef {
                        schema: CanonicalSchemaPath{ segments:schema_ref.get_schema_path()},
                        object: vec![schema_ref.get_home_graph()]
                    }
                })
            }
            GraphRef::Parameter(ident) => {
                return Err(Error::NotSupported( "Parameter".to_string()));
            }
        }
    }

    pub fn resolve_graph_expr(&self, graph_expr: &GraphExpr) -> Result<BoundGraphExpr, Error> {
        match graph_expr {
            GraphExpr::Name(ident) => {
                match CatalogInstance::read().get_schema(self.session_context.current_working_path.segments.as_slice()) {
                    Some(schema) => {
                        let graph = schema.get_graph_id(ident.as_str());
                        if let Some(graph) = graph {
                            Ok(BoundGraphExpr::Ref(
                                BoundCatalogObjectRef {
                                    schema: self.session_context.current_working_path.clone(),
                                    object: vec![graph]
                                }
                            ))
                        } else {
                            return Err(Error::GraphNotExists(ident.clone().to_string()));
                        }
                    }
                    None => {
                        return Err(Error::GraphNotExists(ident.clone().to_string()));
                    }
                }
            },
            GraphExpr::Ref(graph_ref) => {
                let gra_ref = self.resolve_graph_ref(graph_ref)?.path;
                Ok(BoundGraphExpr::Ref(
                    BoundCatalogObjectRef {
                        schema: gra_ref.schema,
                        object: gra_ref.object
                    }
                )
                )
            }
            _ => {
                // TODO: Support Parameter.
                return Err(Error::NotSupported("object".to_string()));
            }
        }
    }

    pub fn resolve_graph_type_ref(
        &self,
        graph_type_ref: &GraphTypeRef,
    ) -> Result<BoundGraphTypeRef, Error> {
        match graph_type_ref {
            GraphTypeRef::Ref(graph_ref) => {
                let catalog_ref = self.resolve_catalog_obj(graph_ref, CatalogObjRefType::GraphTypeRef)?;
                Ok(BoundGraphTypeRef::Ref(catalog_ref))
            }
            // TODO: Handle parameter in graph type ref.
            GraphTypeRef::Parameter(ident) => Ok(BoundGraphTypeRef::Parameter(ident.clone())),
        }
    }

    pub fn resolve_create_graph(
        &self,
        create: &CreateGraphStatement,
    ) -> Result<BoundCreateGraphTypeAndGraphStatement, Error> {
        let mut schema = None;
        let schema_ref = create.path.value().schema.clone();
        if schema_ref.is_none() {
            schema = CatalogInstance::read().get_schema(self.session_context.current_working_path.segments.as_slice());
        } else {
            schema = self.resolve_schema_ref(schema_ref.unwrap().value())
        }
        let schema = self.resolve_schema_path(create.path.value().schema.unwrap().value())
        let path = self.resolve_catalog_obj(create.path.value(), CatalogObjRefType::GraphRef);
        let kind = &create.kind;
        match (path, kind.value()) {
            (Ok(_), CreateGraphOrGraphTypeStatementKind::Create) => {
                return Err(Error::ErrorCur);
            }
            (Ok(existing_path), CreateGraphOrGraphTypeStatementKind::CreateIfNotExists) => {
                return Ok(BoundCreateGraphTypeAndGraphStatement {
                    create_graph_type: None,
                    create_graph:None,
                })
            }
            (Err(error), _) => match error{
                Error::GraphNotExists(name) => {
                    // Do nothing
                }
                _ => {
                    return Err(error);
                }
            }
            (_,_) => {}
        }
        let mut result = BoundCreateGraphTypeAndGraphStatement{
            create_graph_type: None,
            create_graph: None};
        let graph_obj = self.resolve_of_graph_type(create.graph_type.value())?;
        let label_id: Option<LabelId> = match graph_obj {
            BoundOfGraphType::Ref(graph_ref) => {
                match graph_ref {
                    BoundGraphTypeRef::Ref(gra_ref) => {
                        // TODO: Graph Object id is stored at first.
                        Some(gra_ref.object.first().unwrap().clone())
                    }
                    BoundGraphTypeRef::Parameter(ident) => {
                        return Err(Error::NotSupported("Parameter in graph type".to_string()));
                    }
                }
            },
            BoundOfGraphType::Like(graph_like) => {
                match graph_like {
                    BoundGraphExpr::Ref(grap ) => {
                        Some(grap.object.first().unwrap().clone())
                    },
                    BoundGraphExpr::Current => {
                        // TODO: Return current graph Id.
                        return Err(Error::NotSupported("Current graph".to_string()));
                    }
                }
            },
            // If meet nested, return None and try to create graph type.
            BoundOfGraphType::Nested(nested) => None,
            // Any graph type's Id?
            BoundOfGraphType::Any => Some(0),
        };
        
        let graph_path = BoundCatalogObjectStrRef {
            schema: self.resolve_schema_ref(&create.path.value().schema),
            object: create.path.value().objects.iter().map(|v| v.value().clone()).collect()
        };
        
        result.create_graph =Some(BoundCreateGraphStatement {
            path:graph_path,
            kind: kind.value().clone(),
            label_id,
            label_name: None,
            source:create.clone().source.map(|v| self.resolve_graph_expr(v.value())).transpose()?,
        });
        if let OfGraphType::Nested(nested) = &create.graph_type.value() {
            let temp_create_graph_type = CreateGraphTypeStatement {
                // TODO: Graph Type Name?
                path:create.path.clone(),
                kind:Spanned(CreateGraphOrGraphTypeStatementKind::Create, (0..1)),
                source: Spanned(GraphTypeSource::Nested(nested.clone()), (0..1)),
            };
            let create_graph_type_stmt = self.resolve_create_graph_type(&temp_create_graph_type)?;
            if let BoundCreateGraphTypeStatementOrNot::Process(stmt) = create_graph_type_stmt {
                result.create_graph.clone().unwrap().label_name = Some(create.path.value().objects.first().unwrap().value().clone());
                result.create_graph_type = Some(stmt.clone());
            }
        }
        Ok(result)
    }

    pub fn resolve_drop_graph(
        &self,
        drop: &DropGraphStatement,
    ) -> Result<BoundDropGraphStatement, Error> {
        let schema_path = self.resolve_schema_ref(&drop.path.value().schema);
        let binding = CatalogInstance::write();
        let schema = binding.get_schema(self.session_context.current_working_path.segments.as_slice());
        if schema.is_none() {
            return Err(Error::SchemaNotExists(schema_path.to_string()));
        }
        
        let graph_names = &drop.path.value().objects;
        
        
        let mut graph_id_vec = vec![];
        for graph_name in graph_names.iter() {
            let graph_id = schema.unwrap().get_graph_id(graph_name.value());
            if !drop.if_exists  && graph_id.is_some() {
                return Err(Error::GraphNotExists(graph_name.value().to_string()));
            }
            if graph_id.is_some() {
               graph_id_vec.push(graph_id.unwrap());
            }
 
        }
        
        Ok(BoundDropGraphStatement {
            schema_id: schema.unwrap().id,
            graph_id: graph_id_vec.clone(),
            if_exists: drop.if_exists,
        })
    }

    pub fn resolve_create_graph_type(
        &self,
        create: &CreateGraphTypeStatement,
    ) -> Result<BoundCreateGraphTypeStatementOrNot, Error> {
        let path = self.resolve_catalog_obj(create.path.value(), CatalogObjRefType::GraphTypeRef);
        let kind = &create.kind;

        // check if graph type already exists..
        match (path, kind.value()) {
            (Ok(_), CreateGraphOrGraphTypeStatementKind::Create) => {
                return Err(Error::GraphTypeAlreadyExists(create.path.value().objects.iter().map(|v| v.value().to_string()).collect()));
            }
            (Ok(existing_path), CreateGraphOrGraphTypeStatementKind::CreateIfNotExists) => {
                return Ok(BoundCreateGraphTypeStatementOrNot::Skip);
            }
            (Err(error), _) => match error{
                Error::GraphTypeNotExists(name) => {
                    // Do nothing
                }
                _ => {
                    return Err(error);
                }
            }
            (_,_) => {}
        }
        let graph_type_path = BoundCatalogObjectStrRef {
            schema: self.resolve_schema_ref(&create.path.value().schema),
            object: create.path.value().objects.iter().map(|v| v.value().clone()).collect(),
        };
        let bound_graph_type_source = match create.source.value() {
            GraphTypeSource::Copy(copy) => {BoundGraphTypeSource::Copy(self.resolve_graph_type_ref(copy.value())?)}
            GraphTypeSource::Like(like) => {
                BoundGraphTypeSource::Like(self.resolve_graph_expr(like.value())?)
            }
            GraphTypeSource::Nested(nested) => BoundGraphTypeSource::Nested(
                nested
                    .into_iter()
                    .map(|element| element.value().clone())
                    .collect(),
            ),
        };

        Ok(BoundCreateGraphTypeStatementOrNot::Process(BoundCreateGraphTypeStatement {
            path: graph_type_path,
            kind: kind.value().clone(),
            source: bound_graph_type_source,
        }))
    }

    pub fn resolve_drop_graph_type(
        &self,
        drop: &DropGraphTypeStatement,
    ) -> Result<BoundDropGraphTypeStatement, Error> {
        let path = self.resolve_catalog_obj(drop.path.value(), CatalogObjRefType::GraphTypeRef);
        match path {
            Ok(path) => {
                Ok(BoundDropGraphTypeStatement {
                    path: Some(path.clone()),
                    if_exists: drop.if_exists,
                })
            },
            // TODO: Handle schema or graph type not exist error.
            Err(e) =>  {
                if drop.if_exists {
                    Ok(BoundDropGraphTypeStatement {
                        path: None,
                        if_exists: drop.if_exists,
                    })
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn resolve_procedure_ref(&self, call: &ProcedureRef) -> Result<BoundProcedureRef, Error> {
        match call {
            ProcedureRef::Ref(call) => {
                let obj = self.resolve_catalog_obj(call, CatalogObjRefType::ProcedureRef)?;
                Ok(BoundProcedureRef::Ref(obj))
            }
            // TODO: Handle Parameter in procedure ref.
            ProcedureRef::Parameter(param) => Ok(BoundProcedureRef::Parameter(param.clone())),
        }
    }

    pub fn resolve_call_procedure(
        &self,
        call: &CallProcedureStatement,
    ) -> Result<BoundCallProcedureStatement, Error> {
        let bound_procedure = match call.procedure.value() {
            ProcedureCall::Inline(inline) => {
                return Err(Error::NotSupported("Call inline procedure".to_string()));
            }
            ProcedureCall::Named(named) => {
                let bound_procedure_ref = self.resolve_procedure_ref(named.name.value())?;
                let mut expr_vec = Vec::new();
                for expr in named.args.iter() {
                    expr_vec.push(self.resolve_expr(expr.value())?)
                }
                // TODO: Handle yield_clause
                BoundProcedureCall::Named(BoundNamedProcedureCall {
                    name: bound_procedure_ref,
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

    pub fn resolve_statement(&mut self, statement: &Statement) -> Result<BoundStatement, Error> {
        match &statement {
            Statement::Catalog(stmt) => {
                let mut resolved_stmts = Vec::new();
                for catalog_stmt in stmt.iter() {
                    match catalog_stmt.value() {
                        CatalogModifyingStatement::Call(call) => {
                            let stmt = self.resolve_call_procedure(call)?;
                            resolved_stmts.push(BoundCatalogModifyingStatement::Call(stmt));
                        }

                        CatalogModifyingStatement::CreateSchema(create) => {
                            let stmt = self.resolve_create_schema(create)?;
                            resolved_stmts.push(BoundCatalogModifyingStatement::CreateSchema(stmt));
                        }

                        CatalogModifyingStatement::DropSchema(drop) => {
                            let stmt = self.resolve_drop_schema(drop)?;
                            resolved_stmts.push(BoundCatalogModifyingStatement::DropSchema(stmt));
                        }

                        CatalogModifyingStatement::CreateGraph(create) => {
                            let stmt = self.resolve_create_graph(create)?;
                            if let Some(create_graph_type) = stmt.create_graph_type {
                                resolved_stmts.push(BoundCatalogModifyingStatement::CreateGraphType(create_graph_type));
                            }
                            if let Some(create_graph) = stmt.create_graph {
                                resolved_stmts.push(BoundCatalogModifyingStatement::CreateGraph(create_graph));
                            }
                        }

                        CatalogModifyingStatement::DropGraph(drop) => {
                            let stmt = self.resolve_drop_graph(drop)?;
                            resolved_stmts.push(BoundCatalogModifyingStatement::DropGraph(stmt));
                        }

                        CatalogModifyingStatement::CreateGraphType(create) => {
                            let stmt = self.resolve_create_graph_type(create)?;
                            if let BoundCreateGraphTypeStatementOrNot::Process(bound_stmt) = stmt {
                                resolved_stmts
                                    .push(BoundCatalogModifyingStatement::CreateGraphType(bound_stmt))
                            }
                        }

                        CatalogModifyingStatement::DropGraphType(drop) => {
                            let stmt = self.resolve_drop_graph_type(drop)?;
                            resolved_stmts.push(BoundCatalogModifyingStatement::DropGraphType(stmt))
                        }
                    }
                }
                Ok(BoundStatement::Catalog(resolved_stmts))
            }
            Statement::Query(stmt) => {
                let stmt = self.resolve_composite_query_statement(stmt)?;
                Ok(BoundStatement::Query(stmt))
            }
            Statement::Data(stmt) => {
                return Err(Error::NotSupported("Data".to_string()));
            }
        }
    }
}

mod tests {
    use std::collections::HashMap;
    use smol_str::ToSmolStr;
    use gql_parser::ast::{
        Ident, PredefinedSchemaRef, ProgramActivity, SchemaPathSegment, SchemaRef,
    };
    use gql_parser::span::*;

    use crate::program::bound_statement::object_ref::{BoundSchemaRef, CanonicalSchemaPath};
    use crate::program::{Binder, SessionContext};
    use crate::resolver::resolve_schema_ref;

    #[test]
    fn test_resolve_absolute_schema_ref() {
        let session_context = SessionContext {
            current_working_path: CanonicalSchemaPath {
                segments: vec!["root".to_smolstr()],
            },
            current_param_context: HashMap::new(),
        };

        let schema_ref = SchemaRef::Absolute(vec![
            Spanned(SchemaPathSegment::Name(Ident::from("root")), (0..0)),
            Spanned(SchemaPathSegment::Name(Ident::from("db")), (0..0)),
        ]);
        let result = resolve_schema_ref(&session_context, &schema_ref);
        assert_eq!(
            result,
            BoundSchemaRef::Canonical(CanonicalSchemaPath {
                segments: vec!["root".to_smolstr(), "db".to_smolstr()],
            })
        )
    }

    #[test]
    fn test_resolve_relative_schema_ref() {
        let session_context = SessionContext {
            current_working_path: CanonicalSchemaPath {
                segments: vec!["root".to_smolstr(), "db".to_smolstr()],
            },
            current_param_context: HashMap::new(),
        };

        let schema_ref = SchemaRef::Relative(vec![
            Spanned(SchemaPathSegment::Parent, (0..0)),
            Spanned(SchemaPathSegment::Name(Ident::from("db2")), (0..0)),
            Spanned(SchemaPathSegment::Name(Ident::from("ob3")), (0..0)),
        ]);
        let result = resolve_schema_ref(&session_context, &schema_ref);
        assert_eq!(
            result,
            BoundSchemaRef::Canonical(CanonicalSchemaPath {
                segments: vec!["root".to_smolstr(), "db2".to_smolstr(), "ob3".to_smolstr()],
            })
        )
    }

    #[test]
    fn test_resolve_predefined_schema_ref() {
        let session_context = SessionContext {
            current_working_path: CanonicalSchemaPath {
                segments: vec!["root".to_smolstr(), "db".to_smolstr()],
            },
            current_param_context: HashMap::new(),
        };

        let schema_ref = SchemaRef::Predefined(PredefinedSchemaRef::Home);

        let result = resolve_schema_ref(&session_context, &schema_ref);
        assert_eq!(
            result,
            BoundSchemaRef::Canonical(CanonicalSchemaPath {
                segments: vec!["".to_smolstr()]
            }),
        );

        let schema_ref = SchemaRef::Predefined(PredefinedSchemaRef::Current);

        let result = resolve_schema_ref(&session_context, &schema_ref);
        assert_eq!(
            result,
            BoundSchemaRef::Canonical(CanonicalSchemaPath {
                segments: vec!["root".to_smolstr(), "db".to_smolstr()],
            }),
        )
    }

    #[test]
    fn test_resolve_parameter_bound() {
        let mut param_context = HashMap::new();
        param_context.insert(
            Ident::from("param1"),
            SchemaRef::Relative(vec![
                Spanned(SchemaPathSegment::Parent, (0..0)),
                Spanned(SchemaPathSegment::Name(Ident::from("child")), (0..0)),
            ]),
        );

        let session_context = SessionContext {
            current_working_path: CanonicalSchemaPath {
                segments: vec!["base".to_smolstr(), "dir".to_smolstr()],
            },
            current_param_context: param_context,
        };
        let schema_ref = SchemaRef::Parameter(Ident::from("param1"));

        let result = resolve_schema_ref(&session_context, &schema_ref);

        assert_eq!(
            result,
            BoundSchemaRef::Canonical(CanonicalSchemaPath {
                segments: vec!["base".to_smolstr(), "child".to_smolstr()]
            }),
        );
        let schema_ref = SchemaRef::Parameter(Ident::from("unknown"));

        let result = resolve_schema_ref(&session_context, &schema_ref);

        assert_eq!(
            result,
            BoundSchemaRef::UnResolvedParameter(Ident::from("unknown")),
            "Unbound parameter should produce UnResolvedParameter"
        );
    }

    #[test]
    fn test_schema_create_and_drop() {
        let parsed = gql_parser::parse_gql("create schema if not exists /a/b");
        let program_activity = parsed
            .unwrap()
            .value()
            .clone()
            .activity
            .unwrap()
            .value()
            .clone();
        let trans_activity = match program_activity {
            ProgramActivity::Session(session) => None,
            ProgramActivity::Transaction(transaction) => Some(transaction),
        };
        let statement = trans_activity
            .unwrap()
            .clone()
            .procedure
            .unwrap()
            .clone()
            .value()
            .statement
            .value()
            .clone();
        let session_context = SessionContext {
            current_working_path: CanonicalSchemaPath {
                segments: vec!["root".to_smolstr()],
            },
            current_param_context: HashMap::new(),
        };

        let mut binder = Binder::new(session_context);
        let bound_statement = binder.resolve_statement(&statement);
        println!("{:?}", bound_statement);
    }
}
