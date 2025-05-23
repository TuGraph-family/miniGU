use gql_parser::ast::Statement;
use crate::binder::binder::Binder;
use crate::bound_statement::procedure_spec::BoundStatement;
use crate::error::BindResult;

impl Binder {
    pub(crate) fn validate_statement(&self,statement: BoundStatement) ->BindResult<BoundStatement> {
        // There is no need to validate statement currently.
        Ok(statement)
    }
}

