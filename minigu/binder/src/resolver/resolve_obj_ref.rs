use gql_parser::ast::ProcedureRef;
use smol_str::ToSmolStr;

use crate::binder::binder::Binder;
use crate::bound_statement::object_ref::BoundProcedureRef;
use crate::catalog_ref::CallProcedureCatalogRef;
use crate::error::BindError;

impl Binder {
    pub fn resolve_procedure_ref(
        &self,
        call: &ProcedureRef,
    ) -> Result<BoundProcedureRef, BindError> {
        match call {
            ProcedureRef::Ref(catalog) => {
                let schema = match &catalog.schema {
                    Some(schema_ref) => self.resolve_schema_ref(schema_ref.value())?,
                    None => self.schema.clone(),
                };
                if catalog.objects.len() != 1 {
                    return Err(BindError::NotSupported(
                        "can only  resolve one object".to_string(),
                    ));
                }
                let procedure_name = catalog.objects[0].value();

                let procedure = schema
                    .get_procedure(procedure_name)
                    .map_err(|e| Err(BindError::External(Box::new(e))))?;

                match procedure {
                    Some(procedure) => Ok(BoundProcedureRef::Ref(CallProcedureCatalogRef {
                        name: procedure_name.to_smolstr(),
                        procedure_ref: procedure.clone(),
                    })),
                    None => Err(BindError::ProcedureNotExists(procedure_name.to_string())),
                }
            }
            // TODO: Handle Parameter in procedure ref.
            ProcedureRef::Parameter(param) => Ok(BoundProcedureRef::Parameter(param.clone())),
        }
    }
}
