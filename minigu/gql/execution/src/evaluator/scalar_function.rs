use std::fmt::{self, Debug};

use itertools::Itertools;
use minigu_common::data_chunk::DataChunk;

use super::{BoxedEvaluator, DatumRef, Evaluator};
use crate::error::ExecutionResult;

pub struct ScalarFunction<F> {
    func: F,
    args: Vec<BoxedEvaluator>,
}

impl<F> Debug for ScalarFunction<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ScalarFunction({:?})", self.args)
    }
}

impl<F> ScalarFunction<F> {
    pub fn new(func: F, args: Vec<BoxedEvaluator>) -> Self {
        Self { func, args }
    }
}

impl<F> Evaluator for ScalarFunction<F>
where
    F: Fn(Vec<DatumRef>) -> ExecutionResult<DatumRef>,
{
    fn evaluate(&self, chunk: &DataChunk) -> ExecutionResult<DatumRef> {
        let args: Vec<_> = self
            .args
            .iter()
            .map(|arg| arg.evaluate(chunk))
            .try_collect()?;
        (self.func)(args)
    }
}

#[cfg(test)]
mod tests {
    use arrow::array::create_array;
    use arrow::compute;
    use minigu_common::data_chunk;
    use thiserror::Error;

    use super::*;
    use crate::error::ExecutionError;
    use crate::evaluator::column_ref::ColumnRef;

    #[derive(Debug, Error)]
    #[error("{0}")]
    struct SimpleError(String);

    #[test]
    fn test_scalar_function() {
        let chunk = data_chunk!((Int32, [1, 2, 3]), (Int32, [4, 5, 6]));
        let add = |args: Vec<DatumRef>| -> ExecutionResult<DatumRef> {
            if args.len() != 2 {
                return Err(ExecutionError::Custom(Box::new(SimpleError(
                    "expected 2 arguments".to_string(),
                ))));
            }
            let array = compute::kernels::numeric::add(&args[0], &args[1])?;
            Ok(DatumRef::new(array, false))
        };
        let e1 = ColumnRef::new(0);
        let e2 = ColumnRef::new(1);
        let evaluator = ScalarFunction::new(add, vec![Box::new(e1), Box::new(e2)]);
        let result = evaluator.evaluate(&chunk).unwrap();
        let expected = create_array!(Int32, [5, 7, 9]);
        assert_eq!(result.into_array().as_ref(), expected.as_ref());
    }
}
