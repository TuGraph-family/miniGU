use gql_parser::ast::{CallProcedureStatement, NamedProcedureCall, ProcedureCall};
use itertools::Itertools;

use super::Binder;
use crate::error::{BindResult, not_implemented};
use crate::procedure::procedure_call::{
    BoundCallProcedureStatement, BoundNamedProcedureCall, BoundProcedureCall,
};

impl Binder {
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
        Ok(BoundNamedProcedureCall {
            procedure_ref,
            args,
            yield_column_indices: vec![],
        })
    }
}
