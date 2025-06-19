use arrow::array::AsArray;
use itertools::Itertools;
use minigu_context::session::SessionContext;
use minigu_ir::bound::{BoundExpr, BoundExprKind};
use minigu_ir::plan::PlanNode;

use crate::evaluator::BoxedEvaluator;
use crate::evaluator::column_ref::ColumnRef;
use crate::evaluator::constant::Constant;
use crate::executor::procedure_call::ProcedureCallBuilder;
use crate::executor::{BoxedExecutor, Executor, IntoExecutor};

pub struct ExecutorBuilder {
    session: SessionContext,
}

impl ExecutorBuilder {
    pub fn new(session: SessionContext) -> Self {
        Self { session }
    }

    pub fn build(self, physical_plan: &PlanNode) -> BoxedExecutor {
        self.build_executor(physical_plan)
    }

    fn build_executor(&self, physical_plan: &PlanNode) -> BoxedExecutor {
        let mut children = physical_plan
            .children()
            .iter()
            .map(|child| self.build_executor(child));
        match physical_plan {
            PlanNode::PhysicalFilter(filter) => {
                let (child,) = children
                    .collect_tuple()
                    .expect("filter should have exactly one child");
                let predicate = self.build_evaluator(&filter.predicate);
                let filter = child.filter(move |c| {
                    predicate
                        .evaluate(c)
                        .map(|a| a.into_array().as_boolean().clone())
                });
                Box::new(filter)
            }
            PlanNode::PhysicalProject(project) => {
                let (child,) = children
                    .collect_tuple()
                    .expect("project should have exactly one child");
                let evaluators = project
                    .exprs
                    .iter()
                    .map(|e| self.build_evaluator(e))
                    .collect();
                let project = child.project(evaluators);
                Box::new(project)
            }
            PlanNode::PhysicalCall(call) => {
                assert!(children.next().is_none());
                let procedure = call.procedure.object().clone();
                let session = self.session.clone();
                let args = call.args.clone();
                Box::new(ProcedureCallBuilder::new(procedure, session, args).into_executor())
            }
            _ => unreachable!(),
        }
    }

    fn build_evaluator(&self, expr: &BoundExpr) -> BoxedEvaluator {
        match &expr.kind {
            BoundExprKind::Value(value) => Box::new(Constant::new(value.clone())),
            BoundExprKind::ColumnRef(index) => Box::new(ColumnRef::new(*index)),
            BoundExprKind::Path(_) => todo!(),
            BoundExprKind::Subpath(_) => todo!(),
            BoundExprKind::Vertex(_) => todo!(),
            BoundExprKind::Edge(_) => todo!(),
        }
    }
}
