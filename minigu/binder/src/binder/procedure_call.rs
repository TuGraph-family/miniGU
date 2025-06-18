use std::sync::Arc;

use gql_parser::ast::{CallProcedureStatement, NamedProcedureCall, ProcedureCall, YieldItem};
use itertools::Itertools;
use minigu_common::data_type::{DataField, DataSchema};
use minigu_common::error::not_implemented;
use minigu_ir::bound::{
    BoundCallProcedureStatement, BoundExpr, BoundNamedProcedureCall, BoundProcedureCall,
};

use super::Binder;
use crate::error::{BindError, BindResult};

impl Binder<'_> {
    pub fn bind_call_procedure_statement(
        &mut self,
        statement: &CallProcedureStatement,
    ) -> BindResult<BoundCallProcedureStatement> {
        let optional = statement.optional;
        let procedure = self.bind_procedure_call(statement.procedure.value())?;
        Ok(BoundCallProcedureStatement {
            optional,
            procedure,
        })
    }

    pub fn bind_procedure_call(&mut self, call: &ProcedureCall) -> BindResult<BoundProcedureCall> {
        match call {
            ProcedureCall::Named(call) => self
                .bind_named_procedure_call(call)
                .map(BoundProcedureCall::Named),
            _ => not_implemented("inline procedure call".to_string(), None),
        }
    }

    pub fn bind_named_procedure_call(
        &mut self,
        call: &NamedProcedureCall,
    ) -> BindResult<BoundNamedProcedureCall> {
        let procedure_ref = self.bind_procedure_ref(call.name.value())?;
        let args = call
            .args
            .iter()
            .map(|arg| self.bind_value_expression(arg.value()))
            .try_collect()?;
        let schema = if let Some(yield_clause) = call.yield_clause.as_ref() {
            let yield_clause = yield_clause.value();
            assert!(!yield_clause.is_empty());
            let schema = procedure_ref
                .schema()
                .ok_or_else(|| BindError::ProcedureWithoutSchema(procedure_ref.name().clone()))?;
            let mut fields = Vec::with_capacity(yield_clause.len());
            for item in yield_clause {
                let YieldItem { name, alias } = item.value();
                // Find the first field with the given name
                let (index, field) = schema
                    .fields()
                    .iter()
                    .enumerate()
                    .find(|(_, f)| f.name() == name.value())
                    .ok_or_else(|| BindError::YieldItemNotFound(name.value().clone()))?;
                let name = if let Some(alias) = alias.as_ref() {
                    alias.value()
                } else {
                    name.value()
                };
                fields.push(DataField::new(
                    name.to_string(),
                    field.ty().clone(),
                    field.is_nullable(),
                ));
                // If the name is already in the context, the original expression will be overridden
                self.context.insert(
                    name.clone(),
                    BoundExpr::column_ref(index, field.ty().clone()),
                );
            }
            Some(Arc::new(DataSchema::new(fields)))
        } else {
            let schema = procedure_ref.schema().clone();
            if let Some(schema) = &schema {
                for (i, field) in schema.fields().iter().enumerate() {
                    self.context.insert(
                        field.name().to_string().into(),
                        BoundExpr::column_ref(i, field.ty().clone()),
                    );
                }
            }
            schema
        };
        Ok(BoundNamedProcedureCall {
            procedure_ref,
            args,
            schema,
        })
    }
}
