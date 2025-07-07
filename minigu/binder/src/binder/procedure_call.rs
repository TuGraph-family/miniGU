use std::sync::Arc;

use gql_parser::ast::{CallProcedureStatement, NamedProcedureCall, ProcedureCall};
use itertools::Itertools;
use minigu_common::data_type::{DataField, DataSchema};
use minigu_common::error::not_implemented;
use minigu_ir::bound::{BoundCallProcedureStatement, BoundNamedProcedureCall, BoundProcedureCall};

use super::Binder;
use crate::error::{BindError, BindResult};

impl Binder<'_> {
    pub fn bind_call_procedure_statement(
        &self,
        statement: &CallProcedureStatement,
    ) -> BindResult<BoundCallProcedureStatement> {
        let optional = statement.optional;
        let procedure = self.bind_procedure_call(statement.procedure.value())?;
        Ok(BoundCallProcedureStatement {
            optional,
            procedure,
        })
    }

    pub fn bind_procedure_call(&self, call: &ProcedureCall) -> BindResult<BoundProcedureCall> {
        match call {
            ProcedureCall::Named(call) => self
                .bind_named_procedure_call(call)
                .map(BoundProcedureCall::Named),
            _ => not_implemented("inline procedure call".to_string(), None),
        }
    }

    pub fn bind_named_procedure_call(
        &self,
        call: &NamedProcedureCall,
    ) -> BindResult<BoundNamedProcedureCall> {
        let procedure_ref = self.bind_procedure_ref(call.name.value())?;
        let parameters = procedure_ref.parameters();
        let args: Vec<_> = call
            .args
            .iter()
            .map(|arg| self.bind_value_expression(arg.value()))
            .try_collect()?;
        let args_types = args.iter().map(|a| a.logical_type.clone()).collect_vec();
        if args_types != parameters {
            return Err(BindError::IncorrectArguments {
                procedure: procedure_ref.name().clone(),
                expected: parameters.to_vec(),
                actual: args_types,
            });
        }
        let schema = if let Some(yield_clause) = call.yield_clause.as_ref() {
            let original_schema = procedure_ref.schema();
            let yield_clause = yield_clause.value();
            if let Some(original_schema) = original_schema {
                if yield_clause.len() != original_schema.fields().len() {
                    return Err(BindError::IncorrectNumberOfYieldItems {
                        expected: original_schema.fields().len(),
                        actual: yield_clause.len(),
                    });
                }
                let mut fields = Vec::with_capacity(yield_clause.len());
                for (item, field) in yield_clause.iter().zip(original_schema.fields()) {
                    let item = item.value();
                    let item_name = item.name.value();
                    if item_name != field.name() {
                        return Err(BindError::YieldItemNotFound(item_name.clone()));
                    }
                    let name = item.alias.as_ref().map(|a| a.value()).unwrap_or(item_name);
                    fields.push(DataField::new(
                        name.to_string(),
                        field.ty().clone(),
                        field.is_nullable(),
                    ));
                }
                Some(Arc::new(DataSchema::new(fields)))
            } else {
                return Err(BindError::YieldAfterSchemalessProcedure(
                    procedure_ref.name().clone(),
                ));
            }
        } else {
            procedure_ref.schema().clone()
        };
        Ok(BoundNamedProcedureCall {
            procedure_ref,
            args,
            schema,
        })
    }
}
