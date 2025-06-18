use super::{Executor, IntoExecutor};
use crate::evaluator::BoxedEvaluator;

#[derive(Debug)]
pub struct JoinBuilder<L, R> {
    left: L,
    right: R,
    conds: Vec<JoinCond>,
}

#[derive(Debug)]
#[allow(unused)]
pub struct JoinCond {
    left_key: BoxedEvaluator,
    right_key: BoxedEvaluator,
}

impl JoinCond {
    pub fn new(left_key: BoxedEvaluator, right_key: BoxedEvaluator) -> Self {
        Self {
            left_key,
            right_key,
        }
    }
}

impl<L, R> JoinBuilder<L, R> {
    pub fn new(left: L, right: R, conds: Vec<JoinCond>) -> Self {
        Self { left, right, conds }
    }
}

impl<L, R> IntoExecutor for JoinBuilder<L, R>
where
    L: Executor,
    R: Executor,
{
    type IntoExecutor = impl Executor;

    // TODO: Implement this.
    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            #[allow(unused)]
            let JoinBuilder { left, right, conds } = self;
        }
        .into_executor()
    }
}
