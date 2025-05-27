use gql_parser::ast::{Procedure, Statement};
use minigu_catalog::provider::{CatalogRef, SchemaRef};

use crate::error::BindResult;
use crate::statement::procedure_spec::{BoundNextStatement, BoundProcedure, BoundStatement};
pub struct Binder {
    pub catalog: CatalogRef,
    pub schema: Option<SchemaRef>,
}

impl Binder {
    pub fn new(catalog_ref: CatalogRef, schema_ref: Option<SchemaRef>) -> Self {
        Binder {
            catalog: catalog_ref,
            schema: schema_ref,
        }
    }
}

impl Binder {
    pub fn bind_procedure(&mut self, procedure: &Procedure) -> BindResult<BoundProcedure> {
        if let Some(at_schema) = &procedure.at {
            let schema_ref = self.resolve_schema_ref(at_schema.value())?;
            self.schema = Some(schema_ref);
        }

        Ok(BoundProcedure {
            statement: self.bind_statement(procedure.statement.value())?,
            next_statement: procedure
                .next_statements
                .iter()
                .map(|stmt| {
                    Ok(BoundNextStatement {
                        yield_clause: None,
                        statement: self.bind_statement(stmt.value().statement.value())?,
                    })
                })
                .collect::<BindResult<Vec<_>>>()?,
        })
    }

    pub fn bind_statement(&mut self, statement: &Statement) -> BindResult<BoundStatement> {
        let mut resolved_statement = self.resolve_statement(statement)?;
        // self.type_check(&resolved_statement)?;
        // self.validate(&resolved_statement)?;
        Ok(resolved_statement)
    }
}
