use std::sync::Arc;
use smol_str::ToSmolStr;
use gql_parser::ast::{CallProcedureStatement, ProcedureCall, ProcedureRef};

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
                
                Ok(BoundCallProcedureStatement {
                    optional: call_procedure_statement.optional,
                    procedure: BoundProcedureCall::Named(
                        BoundNamedProcedureCall {
                            procedure_ref: ObjProcedureRef {
                                name: procedure_name.to_smolstr(),
                                object: procedure_obj,
                            },
                            yield_index: vec![],
                        }
                    )
                })
            }
        }
    }
}