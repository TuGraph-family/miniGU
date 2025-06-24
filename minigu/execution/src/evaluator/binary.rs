use std::sync::Arc;

use arrow::array::AsArray;
use arrow::compute::kernels::{boolean, cmp, numeric};
use minigu_common::data_chunk::DataChunk;

use super::{DatumRef, Evaluator};
use crate::error::ExecutionResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}

#[derive(Debug)]
pub struct Binary<L, R> {
    op: BinaryOp,
    left: L,
    right: R,
}

impl<L, R> Binary<L, R> {
    pub fn new(op: BinaryOp, left: L, right: R) -> Self {
        Self { op, left, right }
    }
}

impl<L: Evaluator, R: Evaluator> Evaluator for Binary<L, R> {
    fn evaluate(&self, chunk: &DataChunk) -> ExecutionResult<DatumRef> {
        let left = self.left.evaluate(chunk)?;
        let right = self.right.evaluate(chunk)?;
        let array = match self.op {
            BinaryOp::Add => numeric::add(&left, &right)?,
            BinaryOp::Sub => numeric::sub(&left, &right)?,
            BinaryOp::Mul => numeric::mul(&left, &right)?,
            BinaryOp::Div => numeric::div(&left, &right)?,
            BinaryOp::Rem => numeric::rem(&left, &right)?,
            BinaryOp::And | BinaryOp::Or => {
                let left = left.as_array().as_boolean();
                let right = right.as_array().as_boolean();
                match self.op {
                    BinaryOp::And => Arc::new(boolean::and_kleene(left, right)?),
                    BinaryOp::Or => Arc::new(boolean::or_kleene(left, right)?),
                    _ => unreachable!(),
                }
            }
            BinaryOp::Eq => Arc::new(cmp::eq(&left, &right)?),
            BinaryOp::Ne => Arc::new(cmp::neq(&left, &right)?),
            BinaryOp::Gt => Arc::new(cmp::gt(&left, &right)?),
            BinaryOp::Ge => Arc::new(cmp::gt_eq(&left, &right)?),
            BinaryOp::Lt => Arc::new(cmp::lt(&left, &right)?),
            BinaryOp::Le => Arc::new(cmp::lt_eq(&left, &right)?),
        };
        Ok(DatumRef::new(array, left.is_scalar() && right.is_scalar()))
    }
}

#[cfg(test)]
mod tests {
    use arrow::array::{ArrayRef, create_array};
    use minigu_common::data_chunk;

    use super::*;
    use crate::evaluator::column_ref::ColumnRef;
    use crate::evaluator::constant::Constant;

    #[test]
    fn test_binary_1() {
        let chunk = data_chunk!((Int32, [1, 2, 3]), (Utf8, ["a", "b", "c"]));
        // c0 + c0
        let c0_add_c0 = ColumnRef::new(0).add(ColumnRef::new(0));
        let result = c0_add_c0.evaluate(&chunk).unwrap();
        let expected: ArrayRef = create_array!(Int32, [2, 4, 6]);
        assert_eq!(result.as_array(), &expected);
    }

    #[test]
    fn test_binary_2() {
        let chunk = data_chunk!((Int32, [Some(1), Some(2), None]), (Utf8, ["a", "b", "c"]));
        // c0 * 3
        let c0_add_3 = ColumnRef::new(0).mul(Constant::new(3i32.into()));
        let result = c0_add_3.evaluate(&chunk).unwrap();
        let expected: ArrayRef = create_array!(Int32, [Some(3), Some(6), None]);
        assert_eq!(result.as_array(), &expected);
    }

    #[test]
    fn test_binary_3() {
        let chunk = data_chunk!((Int32, [1, 2, 3]), (Utf8, ["a", "b", "c"]));
        // c0 + c1
        let c0_add_c1 = ColumnRef::new(0).add(ColumnRef::new(1));
        assert!(c0_add_c1.evaluate(&chunk).is_err());
    }

    #[test]
    fn test_binary_4() {
        let chunk = data_chunk!(
            (Int32, [1, 2, 3]),
            (Int32, [None, Some(4), Some(6)]),
            (Int32, [Some(3), None, Some(8)])
        );
        // c0 + c1 <= c2
        let c0_add_c1_le_c2 = ColumnRef::new(0)
            .add(ColumnRef::new(1))
            .le(ColumnRef::new(2));
        let result = c0_add_c1_le_c2.evaluate(&chunk).unwrap();
        let expected: ArrayRef = create_array!(Boolean, [None, None, Some(false)]);
        assert_eq!(result.as_array(), &expected);
    }

    /// Test three-valued logic.
    #[test]
    fn test_binary_5() {
        let chunk = data_chunk!(
            (Boolean, [Some(true), None, Some(false), None, None]),
            (Boolean, [Some(true), None, None, Some(true), Some(false)]),
            (Boolean, [
                Some(false),
                Some(true),
                None,
                Some(false),
                Some(false)
            ])
        );
        // c0 AND c1 OR c2
        let c0_and_c1_or_c2 = ColumnRef::new(0)
            .and(ColumnRef::new(1))
            .or(ColumnRef::new(2));
        let result = c0_and_c1_or_c2.evaluate(&chunk).unwrap();
        let expected: ArrayRef =
            create_array!(Boolean, [Some(true), Some(true), None, None, Some(false)]);
        assert_eq!(result.as_array(), &expected);
    }

    #[test]
    fn test_binary_6() {
        let chunk = data_chunk!((Int32, [Some(1), Some(2), None]));
        // c0 * 3 + (1 + 1)
        let c0_mul_3_plus_2 = ColumnRef::new(0)
            .mul(Constant::new(3i32.into()))
            .add(Constant::new(1i32.into()).add(Constant::new(1i32.into())));
        let result = c0_mul_3_plus_2.evaluate(&chunk).unwrap();
        let expected: ArrayRef = create_array!(Int32, [Some(5), Some(8), None]);
        assert_eq!(result.as_array(), &expected);
    }
}
