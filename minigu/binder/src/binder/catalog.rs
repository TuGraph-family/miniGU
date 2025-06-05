use gql_parser::ast::{
    CatalogModifyingStatement, CreateGraphStatement, CreateGraphTypeStatement,
    CreateSchemaStatement, DropGraphStatement, DropGraphTypeStatement, DropSchemaStatement,
};
use minigu_ir::bound::{
    BoundCatalogModifyingStatement, BoundCreateGraphStatement, BoundCreateGraphTypeStatement,
    BoundCreateSchemaStatement, BoundDropGraphStatement, BoundDropGraphTypeStatement,
    BoundDropSchemaStatement,
};

use super::Binder;
use crate::error::BindResult;

impl Binder {
    pub fn bind_catalog_modifying_statement(
        &mut self,
        statement: &CatalogModifyingStatement,
    ) -> BindResult<BoundCatalogModifyingStatement> {
        match statement {
            CatalogModifyingStatement::Call(statement) => self
                .bind_call_procedure_statement(statement)
                .map(BoundCatalogModifyingStatement::Call),
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
        todo!()
    }

    pub fn bind_drop_schema_statement(
        &mut self,
        statement: &DropSchemaStatement,
    ) -> BindResult<BoundDropSchemaStatement> {
        todo!()
    }

    pub fn bind_create_graph_statement(
        &mut self,
        statement: &CreateGraphStatement,
    ) -> BindResult<BoundCreateGraphStatement> {
        todo!()
    }

    pub fn bind_drop_graph_statement(
        &mut self,
        statement: &DropGraphStatement,
    ) -> BindResult<BoundDropGraphStatement> {
        todo!()
    }

    pub fn bind_create_graph_type_statement(
        &mut self,
        statement: &CreateGraphTypeStatement,
    ) -> BindResult<BoundCreateGraphTypeStatement> {
        todo!()
    }

    pub fn bind_drop_graph_type_statement(
        &mut self,
        statement: &DropGraphTypeStatement,
    ) -> BindResult<BoundDropGraphTypeStatement> {
        todo!()
    }
}
