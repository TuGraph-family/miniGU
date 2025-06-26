use gql_parser::ast::{Procedure, Statement};
use itertools::Itertools;
use minigu_common::error::not_implemented;
use minigu_ir::bound::{BoundProcedure, BoundStatement};

use super::Binder;
use crate::error::BindResult;

impl Binder<'_> {
    pub fn bind_procedure(&mut self, procedure: &Procedure) -> BindResult<BoundProcedure> {
        if let Some(schema) = &procedure.at {
            let schema = self.bind_schema_ref(schema.value())?;
            self.current_schema = Some(schema);
        }
        if !procedure.binding_variable_defs.is_empty() {
            return not_implemented("binding variable definitions".to_string(), None);
        }
        let statement = self.bind_statement(procedure.statement.value())?;
        if !procedure.next_statements.is_empty() {
            return not_implemented("next statement".to_string(), None);
        }
        Ok(BoundProcedure {
            statement,
            next_statements: Vec::new(),
        })
    }

    pub fn bind_statement(&mut self, statement: &Statement) -> BindResult<BoundStatement> {
        match statement {
            Statement::Catalog(statements) => statements
                .iter()
                .map(|s| self.bind_catalog_modifying_statement(s.value()))
                .try_collect()
                .map(BoundStatement::Catalog),
            Statement::Query(statement) => self
                .bind_composite_query_statement(statement)
                .map(BoundStatement::Query),
            Statement::Data(_) => not_implemented("data-modifying statement".to_string(), None),
        }
    }
}
