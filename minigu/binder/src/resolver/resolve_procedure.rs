use std::sync::Arc;
use smol_str::ToSmolStr;
use gql_parser::ast::{CallProcedureStatement, NamedProcedureCall, ProcedureCall, ProcedureRef};

use crate::error::{BindError, BindResult};
use crate::program::Binder;
use crate::statement::procedure::{BoundCallProcedureStatement, BoundNamedProcedureCall, BoundProcedureCall};
use crate::statement::procedure_spec::BoundProcedure;
use crate::object_ref::ProcedureRef as ObjProcedureRef;

impl Binder {
    pub(crate) fn resolve_call_procedure(
        &mut self,
        call_procedure_statement: &CallProcedureStatement,
    ) -> BindResult<BoundCallProcedureStatement> {
        match call_procedure_statement.procedure.value() {
            ProcedureCall::Inline(call) => {
                return Err(BindError::NotSupported("Inline procedure".to_string()));
            }
            ProcedureCall::Named(named_procedure_call) => {
                let procedure_ref = named_procedure_call.name.value();
                let (schema_ref, procedure_name) = match procedure_ref {
                    ProcedureRef::Ref(procedure) => {
                        let schema_ref = match &procedure.schema {
                            Some(schema_ast) => self.resolve_schema_ref(schema_ast.value())?,
                            None => self.schema.as_ref().cloned().ok_or_else(||BindError::SchemaNotSpecified)?
                        };
                        let procedure_name = procedure.objects.iter().map(|ident| ident.value().to_string())
                            .collect::<Vec<_>>().join(".");
                        (schema_ref, procedure_name)
                    }
                    ProcedureRef::Parameter(param) => {
                        return Err(BindError::NotSupported("Procedure".to_string()));
                    }
                };

                let procedure_obj = schema_ref
                    .get_procedure(&procedure_name)
                    .map_err(|e| BindError::External(Box::new(e)))?
                    .ok_or_else(|| BindError::ProcedureNotFound(procedure_name.clone()))?;

                let mut bound_yield_index = Vec::new();

                if let (Some(schema), Some(yield_clause)) = (
                    procedure_obj.schema(),
                    named_procedure_call.clone().yield_clause
                    ) {
                    for yield_item in yield_clause.value() {
                        let target_name = yield_item.value().name.value();
                        if let Some(index) = schema.fields().iter().position(
                            |field| field.name() == target_name
                        ) {
                            bound_yield_index.push(index);
                        }
                    }
                }

                Ok(BoundCallProcedureStatement {
                    optional: call_procedure_statement.optional,
                    procedure: BoundProcedureCall::Named(
                        BoundNamedProcedureCall {
                            procedure_ref: ObjProcedureRef {
                                name: procedure_name.to_smolstr(),
                                object: procedure_obj,
                            },
                            yield_index: bound_yield_index,
                        }
                    )
                })
            }
        }
    }

    pub(crate) fn resolve_yield_in_call_procedure(
        &mut self,
        procedure_obj: &ObjProcedureRef,
        call: &NamedProcedureCall,
    ) -> BindResult<Vec<usize>> {
        let yield_clause = call.yield_clause.clone();
        let schema_opt = procedure_obj.schema();
        if yield_clause.is_empty() && schema_opt.is_some() {
            return Err(BindError::NotSupported("procedure has no schema, but yield clause is provided".to_string()))
        };
        let mut bound_yield_index = Vec::new();
        if let (Some(schema),yield_clause) = (schema_opt, yield_clause) {
            for yield_item in yield_clause {
                let target_name = yield_item.value().name.value();
                let index_opt = schema.fields().iter().position(
                    |field| field.name() == target_name
                );
                match index_opt {
                    Some(index) => bound_yield_index.push(index),
                    None => {
                        return Err(BindError::NotSupported(format!(
                            "field `{}` not found in procedure fields",
                            target_name
                        )))
                    }
                }
            }
        }
        Ok(bound_yield_index)
    }
}