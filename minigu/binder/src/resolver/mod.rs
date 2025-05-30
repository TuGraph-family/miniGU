mod resolve_catalog_ref;
mod resolve_procedure;

use gql_parser::ast::{CatalogModifyingStatement, Statement};

use crate::error::{BindError, BindResult};
use crate::program::Binder;
use crate::statement::catalog::BoundCatalogModifyingStatement;
use crate::statement::procedure_spec::BoundStatement;

impl Binder {
    pub fn resolve_statement(&mut self, statement: &Statement) -> BindResult<BoundStatement> {
        match &statement {
            Statement::Catalog(stmt) => {
                let mut resolved_stmts = Vec::new();
                for catalog_stmt in stmt.iter() {
                    match catalog_stmt.value() {
                        CatalogModifyingStatement::Call(call) => {
                            let stmt = self.resolve_call_procedure(call)?;
                            resolved_stmts.push(BoundCatalogModifyingStatement::Call(stmt));
                        }
                        _ => {
                            return Err(BindError::NotSupported("Catalog operation".to_string()));
                        }
                    }
                }
                Ok(BoundStatement::Catalog(resolved_stmts))
            }
            _ => Err(BindError::NotSupported("Catalog operation".to_string())),
        }
    }
}
