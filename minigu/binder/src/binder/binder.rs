use std::collections::HashMap;
use gql_parser::ast::{Procedure, Statement};
use minigu_catalog::provider::{CatalogRef, GraphRef, SchemaRef};
use minigu_common::types::LabelId;
use crate::bound_statement::common::{BoundElementPattern, BoundElementPatternFiller};
use crate::bound_statement::procedure_spec::{BoundNextStatement, BoundProcedure, BoundStatement};
use crate::catalog_ref::Ident;
use crate::error::BindResult;

pub struct Binder {
    pub catalog: Option<CatalogRef>,
    pub schema: Option<SchemaRef>,
    pub graph: Option<GraphRef>,
    pub variable_context: HashMap<Ident, LabelId>
}

impl Binder {
    pub fn new() -> Binder {
        Self {
            catalog: None,
            schema: None,
            graph: None,
            variable_context: HashMap::new()
        }
    }
}

impl Binder {
    pub fn bind_procedure(
        &mut self,
        procedure: &Procedure,
        catalog: CatalogRef,
        current_schema: Option<SchemaRef>,
    ) -> BindResult<BoundProcedure> {
        self.catalog = Some(catalog);
        self.schema = current_schema;
        Ok(BoundProcedure {
            at: None,
            binding_variable_def: procedure.binding_variable_defs.clone(),
            statement: self.bind_statement(procedure.statement.value())?,
            next_statement: procedure
                .next_statements
                .iter()
                .map(|stmt| {
                    Ok(BoundNextStatement {
                        yield_clause: stmt.value().yield_clause.clone(),
                        statement: self.bind_statement(stmt.value().statement.value())?,
                    })
                })
                .collect::<BindResult<Vec<_>>>()?,
        })
    }

    pub fn bind_statement(&mut self, statement: &Statement) -> BindResult<BoundStatement> {
        let mut resolved_statement = self.resolve_statement(&statement)?;
        self.type_check(&resolved_statement)?;
        self.validate(&resolved_statement)?;
        Ok(resolved_statement)
    }
}
