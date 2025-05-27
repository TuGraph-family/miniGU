use std::sync::Arc;

use gql_parser::ast::{CallProcedureStatement, NamedProcedureCall, ProcedureCall, ProcedureRef};
use minigu_catalog::provider::ProcedureProvider;
use smol_str::ToSmolStr;

use crate::error::{BindError, BindResult};
use crate::object_ref::ProcedureRef as ObjProcedureRef;
use crate::program::Binder;
use crate::statement::procedure::{
    BoundCallProcedureStatement, BoundNamedProcedureCall, BoundProcedureCall,
};

impl Binder {
    pub(crate) fn resolve_call_procedure(
        &mut self,
        call_procedure_statement: &CallProcedureStatement,
    ) -> BindResult<BoundCallProcedureStatement> {
        match call_procedure_statement.procedure.value() {
            ProcedureCall::Inline(call) => {
                Err(BindError::NotSupported("Inline procedure".to_string()))
            }
            ProcedureCall::Named(named_procedure_call) => {
                let procedure_ref = named_procedure_call.name.value();
                let (schema_ref, procedure_name) = match procedure_ref {
                    ProcedureRef::Ref(procedure) => {
                        let schema_ref = match &procedure.schema {
                            Some(schema_ast) => self.resolve_schema_ref(schema_ast.value())?,
                            None => self
                                .schema
                                .as_ref()
                                .cloned()
                                .ok_or_else(|| BindError::SchemaNotSpecified)?,
                        };
                        let procedure_name = procedure
                            .objects
                            .iter()
                            .map(|ident| ident.value().to_string())
                            .collect::<Vec<_>>()
                            .join(".");
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

                let bound_yield_index =
                    self.resolve_yield_in_call_procedure(&procedure_obj, named_procedure_call)?;
                Ok(BoundCallProcedureStatement {
                    optional: call_procedure_statement.optional,
                    procedure: BoundProcedureCall::Named(BoundNamedProcedureCall {
                        procedure_ref: ObjProcedureRef::new(
                            procedure_name.to_smolstr(),
                            procedure_obj,
                        ),
                        yield_index: bound_yield_index,
                    }),
                })
            }
        }
    }

    pub(crate) fn resolve_yield_in_call_procedure(
        &mut self,
        procedure_obj: &Arc<dyn ProcedureProvider>,
        call: &NamedProcedureCall,
    ) -> BindResult<Vec<usize>> {
        let yield_clause = call.yield_clause.clone();
        let schema_opt = procedure_obj.schema();
        let mut bound_yield_index = Vec::new();
        if let (Some(schema), Some(yield_clause)) = (schema_opt, yield_clause) {
            for yield_item in yield_clause.value() {
                let target_name = yield_item.value().name.value();
                let index_opt = schema
                    .fields()
                    .iter()
                    .position(|field| field.name() == target_name);
                match index_opt {
                    Some(index) => bound_yield_index.push(index),
                    None => {
                        return Err(BindError::NotSupported(format!(
                            "field `{}` not found in procedure fields",
                            target_name
                        )));
                    }
                }
            }
        }
        Ok(bound_yield_index)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use gql_parser::ast::ProgramActivity;
    use insta::assert_yaml_snapshot;
    use minigu_catalog::provider::CatalogRef;

    use crate::error::{BindError, BindResult};
    use crate::mock_catalog::MockCatalog;
    use crate::program::Binder;
    use crate::statement::procedure_spec::BoundProcedure;

    fn get_bound_procedure(gql: &str) -> BindResult<BoundProcedure> {
        let parsed = gql_parser::parse_gql(gql);
        let program_activity = parsed
            .unwrap()
            .value()
            .clone()
            .activity
            .unwrap()
            .value()
            .clone();
        let trans_activity = match program_activity {
            ProgramActivity::Session(session) => {
                return Err(BindError::NotSupported("Session".to_string()));
            }
            ProgramActivity::Transaction(transaction) => Some(transaction),
        };
        let procedure = trans_activity.unwrap().procedure.unwrap().value().clone();
        let catalog: CatalogRef = Arc::new(MockCatalog::default());
        let mut binder = Binder::new(catalog.clone(), None);
        binder.bind_procedure(&procedure)
    }

    #[test]
    fn test_resolve_procedure() {
        let stmt = get_bound_procedure("optional call /a/b.proc(1, 2, 3) yield a as a1, b as b1");
        assert!(matches!(stmt, Err(BindError::SchemaNotFound(_))));
        let stmt =
            get_bound_procedure("optional call /default/a/b/proc(1, 2, 3) yield a as a1, b as b1");
        assert!(matches!(stmt, Err(BindError::ProcedureNotFound(_))));
        let stmt = get_bound_procedure(
            "optional call /default/a/b/procedure_test(1, 2, 3) yield a as a1, b as b1",
        );
        let err_message = "field `a` not found in procedure fields".to_string();
        assert!(matches!(stmt, Err(BindError::NotSupported(err_message))));
        let stmt = get_bound_procedure(
            "optional call /default/a/b/procedure_test(1, 2, 3) yield t1 as a1, t2 as b1",
        );
        assert!(stmt.is_ok());
        assert_yaml_snapshot!(stmt.unwrap());
        let stmt = get_bound_procedure(
            "at /default/a/b optional call procedure_test(1, 2, 3) yield t1 as a1",
        );
        assert!(stmt.is_ok());
        assert_yaml_snapshot!(stmt.unwrap());
    }
}
