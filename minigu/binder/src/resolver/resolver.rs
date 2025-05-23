use std::error::Error;
use crate::binder::binder::Binder;
use crate::bound_statement::catalog::BoundCatalogModifyingStatement;
use crate::bound_statement::procedure_spec::BoundStatement;
use crate::error::{BindError, BindResult};
use gql_parser::ast::{CatalogModifyingStatement, Statement};

impl Binder {
    pub fn resolve_statement(&mut self, statement: &Statement) -> Result<BoundStatement, BindError> {
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
                            resolved_stmts.push(BoundCatalogModifyingStatement::CreateGraphType(stmt));
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
                return Err(BindError::NotSupported("Data".to_string()));
            }
        }
    }
}

mod tests {
    use gql_parser::ast::ProgramActivity;
    use crate::binder::binder::Binder;
    use crate::bound_statement::procedure_spec::BoundProcedure;
    use crate::error::{BindError, BindResult};
    use crate::mock::MockCatalog;

    fn get_bound_procedure(gql: &str) -> BindResult<BoundProcedure> {
        let parsed = gql_parser::parse_gql(gql);
        let program_activity = parsed.unwrap().value().clone()
            .activity.unwrap().value().clone();
        let trans_activity = match program_activity {
            ProgramActivity::Session(session) => {
                return Err(BindError::NotSupported("Session".to_string()));
            }
            ProgramActivity::Transaction(transaction) => Some(transaction),
        };
        let procedure = trans_activity.unwrap().procedure.unwrap().value();
        let catalog = MockCatalog::default();
        let mut binder = Binder::new();
        binder.bind_procedure(procedure, MockCatalog::default().into(), None)
        
    }
    
    
    #[test]
    fn test_schema_create_and_drop() {
        let stmt = get_bound_procedure("create schema /root");
    }
    
}