use gql_parser::ast::Statement;
use crate::error::{Error, NotSupportError};
use crate::error::Error::NotSupported;
use crate::program::{Binder, SessionContext};
use crate::program::bound_statement::common::BoundStatement;

impl Binder {
    pub(crate) fn validate_statement(&self,statement: &BoundStatement) ->Result<BoundStatement, Error> {
        // There is no need to validate statement currently.
        Ok(statement.clone())
    }
}

