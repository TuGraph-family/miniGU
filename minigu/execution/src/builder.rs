use minigu_ir::bound::BoundExpr;
use minigu_ir::plan::PlanNode;

use crate::evaluator::BoxedEvaluator;
use crate::executor::{BoxedExecutor, Executor};

pub struct ExecutorBuilder {}

impl ExecutorBuilder {
    pub fn new() -> Self {
        todo!()
    }

    pub fn build(self, physical_plan: PlanNode) -> BoxedExecutor {
        self.build_executor(physical_plan)
    }

    fn build_executor(&self, physical_plan: PlanNode) -> BoxedExecutor {
        todo!()
    }

    fn build_evaluator(&self, expr: BoundExpr) -> BoxedEvaluator {
        todo!()
    }
}
