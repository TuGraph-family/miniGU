use minigu_common::data_type::DataSchemaRef;
use serde::Serialize;

use super::value_expr::BoundExpr;
use crate::named_ref::NamedProcedureRef;

#[derive(Debug, Serialize)]
pub struct BoundCallProcedureStatement {
    pub optional: bool,
    pub procedure: BoundProcedureCall,
}

impl BoundCallProcedureStatement {
    #[inline]
    pub fn schema(&self) -> Option<&DataSchemaRef> {
        self.procedure.schema()
    }
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct BoundInlineProcedureCall {}

#[derive(Debug, Serialize)]
pub struct BoundNamedProcedureCall {
    /// The procedure reference. This can be a query, catalog-modifying or data-modifying
    /// procedure.
    pub procedure_ref: NamedProcedureRef,
    /// The arguments of the procedure call.
    pub args: Vec<BoundExpr>,
    /// The actual schema of the procedure call (possibly with a yield clause). This is only
    /// available for query procedures.
    pub schema: Option<DataSchemaRef>,
}
