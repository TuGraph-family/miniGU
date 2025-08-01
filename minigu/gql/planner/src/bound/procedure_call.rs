use minigu_catalog::named_ref::NamedProcedureRef;
use minigu_common::data_type::DataSchemaRef;
use serde::Serialize;
use smol_str::SmolStr;

use super::value_expr::BoundExpr;

#[derive(Debug, Clone, Serialize)]
pub struct BoundCallProcedureStatement {
    pub optional: bool,
    pub procedure: BoundProcedureCall,
}

impl BoundCallProcedureStatement {
    pub fn name(&self) -> SmolStr {
        match &self.procedure {
            BoundProcedureCall::Named(call) => call.procedure_ref.name().clone(),
            _ => todo!(),
        }
    }

    #[inline]
    pub fn schema(&self) -> Option<&DataSchemaRef> {
        self.procedure.schema()
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum BoundProcedureCall {
    Inline(BoundInlineProcedureCall),
    Named(BoundNamedProcedureCall),
}

impl BoundProcedureCall {
    #[inline]
    pub fn schema(&self) -> Option<&DataSchemaRef> {
        match self {
            BoundProcedureCall::Named(call) => call.schema.as_ref(),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundInlineProcedureCall {}

#[derive(Debug, Clone, Serialize)]
pub struct BoundNamedProcedureCall {
    /// The procedure reference. This can be a query, catalog-modifying or data-modifying
    /// procedure.
    pub procedure_ref: NamedProcedureRef,
    /// The arguments of the procedure call.
    pub args: Vec<BoundExpr>,
    /// The actual schema of the procedure call (possibly after a yield clause). This is only
    /// available for query procedures.
    pub schema: Option<DataSchemaRef>,
}
