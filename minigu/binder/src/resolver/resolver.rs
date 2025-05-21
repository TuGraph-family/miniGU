use gql_parser::ast::{CatalogModifyingStatement, Statement};
use crate::binder::Binder;
use crate::bound_statement::catalog::BoundCatalogModifyingStatement;
use crate::bound_statement::procedure_spec::BoundStatement;
use crate::error::BindResult;

impl Binder {
    pub(crate) fn resolve_statement(&mut self, statement: &Statement) -> BindResult<BoundStatement> {
        match statement {
            Statement::Catalog(stmt) => {
                Ok(BoundStatement::Catalog(
                    stmt.iter().flat_map(
                        |catalog_stmt| {
                            match catalog_stmt.value() {
                                CatalogModifyingStatement::Call(call)=> {
                                    Some(Ok(BoundCatalogModifyingStatement::Call(
                                        self.resolve_call_procedure(call)?
                                    )))
                                },
                                
                                
                            }
                        }
                    )
                    
                ))
            }
        }
        
    }
}