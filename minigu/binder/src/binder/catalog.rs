use gql_parser::ast::{
    CatalogModifyingStatement, CreateGraphStatement, CreateGraphTypeStatement,
    CreateSchemaStatement, DropGraphStatement, DropGraphTypeStatement, DropSchemaStatement,
};
use minigu_common::error::not_implemented;
use minigu_ir::bound::{
    BoundCatalogModifyingStatement, BoundCreateGraphStatement, BoundCreateGraphTypeStatement,
    BoundCreateSchemaStatement, BoundDropGraphStatement, BoundDropGraphTypeStatement,
    BoundDropSchemaStatement,
};

use super::Binder;
use crate::error::{BindError, BindResult};

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
        not_implemented("create schema statement", None)
    }

    pub fn bind_drop_schema_statement(
        &mut self,
        statement: &DropSchemaStatement,
    ) -> BindResult<BoundDropSchemaStatement> {
        not_implemented("drop schema statement", None)
    }

    pub fn bind_create_graph_statement(
        &mut self,
        statement: &CreateGraphStatement,
    ) -> BindResult<BoundCreateGraphStatement> {
        not_implemented("create graph statement", None)
    }

    pub fn bind_drop_graph_statement(
        &mut self,
        statement: &DropGraphStatement,
    ) -> BindResult<BoundDropGraphStatement> {
        not_implemented("drop graph statement", None)
    }

    pub fn bind_create_graph_type_statement(
        &mut self,
        statement: &CreateGraphTypeStatement,
    ) -> BindResult<BoundCreateGraphTypeStatement> {
        not_implemented("create graph type statement", None)
    }

    pub fn bind_drop_graph_type_statement(
        &mut self,
        statement: &DropGraphTypeStatement,
    ) -> BindResult<BoundDropGraphTypeStatement> {
        not_implemented("drop graph type statement", None)
    }
}
