use std::sync::Arc;

use arrow::array::AsArray;
use arrow::compute::kernels::{boolean, numeric};
use minigu_common::data_chunk::DataChunk;

use super::{DatumRef, Evaluator};
use crate::error::ExecutionResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug)]
pub struct Unary<E> {
    op: UnaryOp,
    operand: E,
}

impl<E> Unary<E> {
    pub fn new(op: UnaryOp, operand: E) -> Self {
        Self { op, operand }
    }
}

impl<E: Evaluator> Evaluator for Unary<E> {
    fn evaluate(&self, chunk: &DataChunk) -> ExecutionResult<DatumRef> {
        let operand = self.operand.evaluate(chunk)?;
        let array = match self.op {
            UnaryOp::Neg => numeric::neg(&operand.as_array())?,
            UnaryOp::Not => {
                let operand = operand.as_array().as_boolean();
                Arc::new(boolean::not(operand)?)
            }
        };
        Ok(DatumRef::new(array, operand.is_scalar()))
    }
}

#[cfg(test)]
mod tests {
    use arrow::array::{ArrayRef, create_array};
    use minigu_common::data_chunk;

    use super::*;
    use crate::evaluator::column_ref::ColumnRef;

    #[test]
    fn test_unary_neg() {
        let chunk = data_chunk!((Int32, [Some(1), None, Some(3)]));
        let e = ColumnRef::new(0).neg();
        let result = e.evaluate(&chunk).unwrap();
        let expected: ArrayRef = create_array!(Int32, [Some(-1), None, Some(-3)]);
        assert_eq!(result.as_array(), &expected);
    }

    #[test]
    fn test_unary_not() {
        let chunk = data_chunk!((Boolean, [None, Some(true), Some(false)]));
        let e = ColumnRef::new(0).not();
        let result = e.evaluate(&chunk).unwrap();
        let expected: ArrayRef = create_array!(Boolean, [None, Some(false), Some(true)]);
        assert_eq!(result.as_array(), &expected);
    }
}
