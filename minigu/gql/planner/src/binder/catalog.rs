use gql_parser::ast::{
    CatalogModifyingStatement, CreateGraphOrGraphTypeStatementKind, CreateGraphStatement,
    CreateGraphTypeStatement, CreateSchemaStatement, DropGraphStatement, DropGraphTypeStatement,
    DropSchemaStatement, GraphExpr, GraphRef, GraphTypeSource, OfGraphType, SchemaPathSegment,
};
use minigu_catalog::named_ref::NamedGraphTypeRef;
use minigu_catalog::provider::GraphTypeProvider;
use minigu_common::error::not_implemented;
use smol_str::SmolStr;

use super::Binder;
use super::error::{BindError, BindResult};
use crate::bound::{
    BoundCatalogModifyingStatement, BoundCreateGraphStatement, BoundCreateGraphTypeStatement,
    BoundCreateSchemaStatement, BoundDropGraphStatement, BoundDropGraphTypeStatement,
    BoundDropSchemaStatement, BoundGraphType, CreateKind,
};

impl Binder<'_> {
    pub fn bind_catalog_modifying_statement(
        &mut self,
        statement: &CatalogModifyingStatement,
    ) -> BindResult<BoundCatalogModifyingStatement> {
        match statement {
            CatalogModifyingStatement::Call(statement) => {
                let statement = self.bind_call_procedure_statement(statement)?;
                if statement.optional {
                    return not_implemented("optional catalog modifying statements", None);
                }
                if statement.schema().is_some() {
                    return Err(BindError::NotCatalogProcedure(statement.name()));
                }
                Ok(BoundCatalogModifyingStatement::Call(statement))
            }
            CatalogModifyingStatement::CreateSchema(statement) => self
                .bind_create_schema_statement(statement)
                .map(BoundCatalogModifyingStatement::CreateSchema),
            CatalogModifyingStatement::DropSchema(statement) => self
                .bind_drop_schema_statement(statement)
                .map(BoundCatalogModifyingStatement::DropSchema),
            CatalogModifyingStatement::CreateGraph(statement) => self
                .bind_create_graph_statement(statement)
                .map(BoundCatalogModifyingStatement::CreateGraph),
            CatalogModifyingStatement::DropGraph(statement) => self
                .bind_drop_graph_statement(statement)
                .map(BoundCatalogModifyingStatement::DropGraph),
            CatalogModifyingStatement::CreateGraphType(statement) => self
                .bind_create_graph_type_statement(statement)
                .map(BoundCatalogModifyingStatement::CreateGraphType),
            CatalogModifyingStatement::DropGraphType(statement) => self
                .bind_drop_graph_type_statement(statement)
                .map(BoundCatalogModifyingStatement::DropGraphType),
        }
    }

    pub fn bind_create_schema_statement(
        &mut self,
        statement: &CreateSchemaStatement,
    ) -> BindResult<BoundCreateSchemaStatement> {
        let mut path = Vec::new();
        for seg in statement.path.value().iter() {
            match seg.value() {
                SchemaPathSegment::Name(name) => path.push(name.clone()),
                SchemaPathSegment::Parent => path.push(SmolStr::from("..")),
            }
        }
        Ok(BoundCreateSchemaStatement {
            schema_path: path,
            if_not_exists: statement.if_not_exists,
        })
    }

    pub fn bind_drop_schema_statement(
        &mut self,
        statement: &DropSchemaStatement,
    ) -> BindResult<BoundDropSchemaStatement> {
        let mut path = Vec::new();
        for seg in statement.path.value().iter() {
            match seg.value() {
                SchemaPathSegment::Name(name) => path.push(name.clone()),
                SchemaPathSegment::Parent => path.push(SmolStr::from("..")),
            }
        }
        Ok(BoundDropSchemaStatement {
            schema_path: path,
            if_exists: statement.if_exists,
        })
    }

    pub fn bind_create_graph_statement(
        &mut self,
        statement: &CreateGraphStatement,
    ) -> BindResult<BoundCreateGraphStatement> {
        // Only support single-segment object name and optional schema ref (resolved by executor to
        // current schema).
        let name = match statement.path.value().objects.as_slice() {
            [ident] => ident.value().clone(),
            _ => {
                return Err(BindError::InvalidObjectReference(
                    statement
                        .path
                        .value()
                        .objects
                        .iter()
                        .map(|o| o.value().clone())
                        .collect(),
                ));
            }
        };

        let kind = match statement.kind.value() {
            CreateGraphOrGraphTypeStatementKind::Create => CreateKind::Create,
            CreateGraphOrGraphTypeStatementKind::CreateIfNotExists => CreateKind::CreateIfNotExists,
            CreateGraphOrGraphTypeStatementKind::CreateOrReplace => CreateKind::CreateOrReplace,
        };

        // Bind graph type
        let (graph_type, source_like_graph) = match statement.graph_type.value() {
            OfGraphType::Ref(gt_ref) => {
                let named = self.bind_graph_type_ref(gt_ref.value())?;
                (BoundGraphType::Ref(named), None)
            }
            OfGraphType::Like(graph_expr) => {
                let named_graph = match graph_expr.value() {
                    GraphExpr::Name(id) => self.bind_graph_ref(&GraphRef::Name(id.clone()))?,
                    GraphExpr::Ref(gr) => self.bind_graph_ref(gr)?,
                    GraphExpr::Current => self.bind_graph_ref(&GraphRef::Home)?,
                    GraphExpr::Object(_) => {
                        return not_implemented("object graph expression in LIKE", None);
                    }
                };
                // Wrap its graph type as a NamedGraphTypeRef with the graph's name for display
                let gt =
                    NamedGraphTypeRef::new(named_graph.name().clone(), named_graph.graph_type());
                (BoundGraphType::Ref(gt), Some(named_graph))
            }
            OfGraphType::Nested(_) => {
                // Minimal support: treat as empty graph type
                (BoundGraphType::Nested(vec![]), None)
            }
            OfGraphType::Any => {
                // Minimal support: treat as empty graph type
                (BoundGraphType::Nested(vec![]), None)
            }
        };

        // Bind optional source graph for AS COPY OF
        let source = if let Some(s) = statement.source.as_ref() {
            let g = match s.value() {
                GraphExpr::Name(id) => self.bind_graph_ref(&GraphRef::Name(id.clone()))?,
                GraphExpr::Ref(gr) => self.bind_graph_ref(gr)?,
                GraphExpr::Current => self.bind_graph_ref(&GraphRef::Home)?,
                GraphExpr::Object(_) => {
                    return not_implemented("object graph expression in AS COPY OF", None);
                }
            };
            Some(g)
        } else {
            source_like_graph
        };

        Ok(BoundCreateGraphStatement {
            name,
            kind,
            graph_type,
            source,
        })
    }

    pub fn bind_drop_graph_statement(
        &mut self,
        statement: &DropGraphStatement,
    ) -> BindResult<BoundDropGraphStatement> {
        let name = match statement.path.value().objects.as_slice() {
            [ident] => ident.value().clone(),
            _ => {
                return Err(BindError::InvalidObjectReference(
                    statement
                        .path
                        .value()
                        .objects
                        .iter()
                        .map(|o| o.value().clone())
                        .collect(),
                ));
            }
        };
        Ok(BoundDropGraphStatement {
            name,
            if_exists: statement.if_exists,
        })
    }

    pub fn bind_create_graph_type_statement(
        &mut self,
        statement: &CreateGraphTypeStatement,
    ) -> BindResult<BoundCreateGraphTypeStatement> {
        let name = match statement.path.value().objects.as_slice() {
            [ident] => ident.value().clone(),
            _ => {
                return Err(BindError::InvalidObjectReference(
                    statement
                        .path
                        .value()
                        .objects
                        .iter()
                        .map(|o| o.value().clone())
                        .collect(),
                ));
            }
        };
        let kind = match statement.kind.value() {
            CreateGraphOrGraphTypeStatementKind::Create => CreateKind::Create,
            CreateGraphOrGraphTypeStatementKind::CreateIfNotExists => CreateKind::CreateIfNotExists,
            CreateGraphOrGraphTypeStatementKind::CreateOrReplace => CreateKind::CreateOrReplace,
        };
        let source = match statement.source.value() {
            GraphTypeSource::Copy(gt_ref) => {
                let named = self.bind_graph_type_ref(gt_ref.value())?;
                BoundGraphType::Ref(named)
            }
            GraphTypeSource::Like(ge) => {
                let named_graph = match ge.value() {
                    GraphExpr::Name(id) => self.bind_graph_ref(&GraphRef::Name(id.clone()))?,
                    GraphExpr::Ref(gr) => self.bind_graph_ref(gr)?,
                    GraphExpr::Current => self.bind_graph_ref(&GraphRef::Home)?,
                    GraphExpr::Object(_) => {
                        return not_implemented("object graph expression in LIKE", None);
                    }
                };
                let gt =
                    NamedGraphTypeRef::new(named_graph.name().clone(), named_graph.graph_type());
                BoundGraphType::Ref(gt)
            }
            GraphTypeSource::Nested(_) => {
                // Minimal support: treat as empty graph type
                BoundGraphType::Nested(vec![])
            }
        };
        Ok(BoundCreateGraphTypeStatement { name, kind, source })
    }

    pub fn bind_drop_graph_type_statement(
        &mut self,
        statement: &DropGraphTypeStatement,
    ) -> BindResult<BoundDropGraphTypeStatement> {
        let name = match statement.path.value().objects.as_slice() {
            [ident] => ident.value().clone(),
            _ => {
                return Err(BindError::InvalidObjectReference(
                    statement
                        .path
                        .value()
                        .objects
                        .iter()
                        .map(|o| o.value().clone())
                        .collect(),
                ));
            }
        };
        Ok(BoundDropGraphTypeStatement {
            name,
            if_exists: statement.if_exists,
        })
    }
}
