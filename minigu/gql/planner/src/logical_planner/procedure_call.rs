use std::sync::Arc;

use minigu_common::error::not_implemented;

use crate::bound::{BoundCallProcedureStatement, BoundNamedProcedureCall, BoundProcedureCall};
use crate::error::PlanResult;
use crate::logical_planner::LogicalPlanner;
use crate::plan::PlanNode;
use crate::plan::call::Call;

impl LogicalPlanner {
    pub fn plan_call_procedure_statement(
        &self,
        statement: BoundCallProcedureStatement,
    ) -> PlanResult<PlanNode> {
        if statement.optional {
            return not_implemented("optional procedure call", None);
        }
        self.plan_procedure_call(statement.procedure)
    }

    pub fn plan_procedure_call(&self, statement: BoundProcedureCall) -> PlanResult<PlanNode> {
        match statement {
            BoundProcedureCall::Inline(_) => not_implemented("inline procedure call", None),
            BoundProcedureCall::Named(call) => self.plan_named_procedure_call(call),
        }
    }

    pub fn plan_named_procedure_call(&self, call: BoundNamedProcedureCall) -> PlanResult<PlanNode> {
        // The binder guarantees that the arguments are evaluable.
        let args = call
            .args
            .into_iter()
            .map(|arg| arg.evaluate_scalar().expect("arguments must be evaluable"))
            .collect();
        let schema = call.schema.clone();
        let call = Call::new(call.procedure_ref, args, schema);
        Ok(PlanNode::LogicalCall(Arc::new(call)))
    }
}
