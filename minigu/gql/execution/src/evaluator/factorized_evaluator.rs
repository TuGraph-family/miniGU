use std::fmt::Debug;
use std::sync::Arc;

use arrow::array::{Array, ArrayRef, AsArray};
use arrow::compute::kernels::{boolean, cmp, numeric};
use minigu_common::data_chunk::DataChunk;
use minigu_common::result_set::{DataPos, ResultSet};
use minigu_common::value::ScalarValue;

use crate::error::ExecutionResult;
use crate::evaluator::DatumRef;
use crate::evaluator::binary::BinaryOp;
use crate::evaluator::unary::UnaryOp;

pub trait FactorizedEvaluator: Debug {
    /// Evaluate and return a chunk based on the ResultSet
    fn evaluate(&self, result_set: &ResultSet) -> ExecutionResult<DataChunk>;

    fn get_data_pos(&self) -> Option<&DataPos> {
        None
    }

    fn add<E>(self, other: E) -> FactorizedBinary<Self, E>
    where
        Self: Sized,
        E: FactorizedEvaluator,
    {
        FactorizedBinary::new(BinaryOp::Add, self, other)
    }

    fn sub<E>(self, other: E) -> FactorizedBinary<Self, E>
    where
        Self: Sized,
        E: FactorizedEvaluator,
    {
        FactorizedBinary::new(BinaryOp::Sub, self, other)
    }

    fn mul<E>(self, other: E) -> FactorizedBinary<Self, E>
    where
        Self: Sized,
        E: FactorizedEvaluator,
    {
        FactorizedBinary::new(BinaryOp::Mul, self, other)
    }

    fn div<E>(self, other: E) -> FactorizedBinary<Self, E>
    where
        Self: Sized,
        E: FactorizedEvaluator,
    {
        FactorizedBinary::new(BinaryOp::Div, self, other)
    }

    fn neg(self) -> FactorizedUnary<Self>
    where
        Self: Sized,
    {
        FactorizedUnary::new(UnaryOp::Neg, self)
    }

    fn not(self) -> FactorizedUnary<Self>
    where
        Self: Sized,
    {
        FactorizedUnary::new(UnaryOp::Not, self)
    }
}

pub type BoxedFactorizedEvaluator = Box<dyn FactorizedEvaluator>;

/// Reference to a column in a ResultSet
#[derive(Debug, Clone)]
pub struct FactorizedDataRef {
    data_pos: DataPos,
}

impl FactorizedDataRef {
    pub fn new(data_pos: DataPos) -> Self {
        Self { data_pos }
    }

    pub fn data_pos(&self) -> &DataPos {
        &self.data_pos
    }
}

#[macro_export]
macro_rules! data_ref {
    ($chunk_idx:expr, $col_idx:expr) => {
        $crate::evaluator::factorized_evaluator::FactorizedDataRef::new(minigu_common::data_pos!(
            $chunk_idx, $col_idx
        ))
    };
}

impl FactorizedEvaluator for FactorizedDataRef {
    fn evaluate(&self, result_set: &ResultSet) -> ExecutionResult<DataChunk> {
        let column = result_set.get_column(&self.data_pos);
        let mut chunk = DataChunk::new(vec![column.clone()]);

        // set flat/unflat
        let source_chunk = result_set
            .get_data_chunk(self.data_pos.data_chunk_pos)
            .unwrap();
        chunk.set_cur_idx(source_chunk.cur_idx());

        Ok(chunk)
    }

    fn get_data_pos(&self) -> Option<&DataPos> {
        Some(&self.data_pos)
    }
}

/// Constant value in factorized execution
#[derive(Debug, Clone)]
pub struct FactorizedConstant {
    value: ArrayRef,
}

impl FactorizedConstant {
    pub fn new(value: ScalarValue) -> Self {
        Self {
            value: value.to_scalar_array(),
        }
    }
}

impl FactorizedEvaluator for FactorizedConstant {
    fn evaluate(&self, _result_set: &ResultSet) -> ExecutionResult<DataChunk> {
        let mut chunk = DataChunk::new(vec![self.value.clone()]);
        chunk.set_cur_idx(Some(0));
        Ok(chunk)
    }
}

/// Unary operations on factorized data
#[derive(Debug)]
pub struct FactorizedUnary<E> {
    op: UnaryOp,
    operand: E,
}

impl<E> FactorizedUnary<E> {
    pub fn new(op: UnaryOp, operand: E) -> Self {
        Self { op, operand }
    }
}

impl<E: FactorizedEvaluator> FactorizedEvaluator for FactorizedUnary<E> {
    fn evaluate(&self, result_set: &ResultSet) -> ExecutionResult<DataChunk> {
        let operand_chunk = self.operand.evaluate(result_set)?;
        let operand_col = &operand_chunk.columns()[0];

        let result_array = match self.op {
            UnaryOp::Neg => numeric::neg(operand_col)?,
            UnaryOp::Not => {
                let operand = operand_col.as_boolean();
                Arc::new(boolean::not(operand)?)
            }
        };

        let mut result_chunk = DataChunk::new(vec![result_array]);
        result_chunk.set_cur_idx(operand_chunk.cur_idx());
        Ok(result_chunk)
    }
}

/// Binary operations on factorized data
#[derive(Debug)]
pub struct FactorizedBinary<L, R> {
    op: BinaryOp,
    left: L,
    right: R,
}

impl<L, R> FactorizedBinary<L, R> {
    pub fn new(op: BinaryOp, left: L, right: R) -> Self {
        Self { op, left, right }
    }

    /// Check if both operands are DataRef pointing to the same chunk
    fn check_same_chunk(&self) -> bool
    where
        L: FactorizedEvaluator,
        R: FactorizedEvaluator,
    {
        if let (Some(left_pos), Some(right_pos)) =
            (self.left.get_data_pos(), self.right.get_data_pos())
        {
            left_pos.data_chunk_pos == right_pos.data_chunk_pos
        } else {
            false
        }
    }
}

impl<L: FactorizedEvaluator, R: FactorizedEvaluator> FactorizedEvaluator
    for FactorizedBinary<L, R>
{
    fn evaluate(&self, result_set: &ResultSet) -> ExecutionResult<DataChunk> {
        let left_chunk = self.left.evaluate(result_set)?;
        let right_chunk = self.right.evaluate(result_set)?;

        let left_col = &left_chunk.columns()[0];
        let right_col = &right_chunk.columns()[0];

        let left_is_flat = !left_chunk.is_unflat();
        let right_is_flat = !right_chunk.is_unflat();

        let (result_array, result_is_flat) = match (left_is_flat, right_is_flat) {
            // flat + unflat -> unflat
            (true, false) => {
                let cur_idx = left_chunk.cur_idx().unwrap();
                // lefy is_scalar=true
                let left_scalar = DatumRef::new(left_col.slice(cur_idx, 1), true);
                let right_datum = DatumRef::new(right_col.clone(), false);
                let result = self.apply_op(&left_scalar, &right_datum)?;
                (result, false)
            }
            // unflat + flat -> unflat
            (false, true) => {
                let cur_idx = right_chunk.cur_idx().unwrap();
                // right is_scalar=true
                let left_datum = DatumRef::new(left_col.clone(), false);
                let right_scalar = DatumRef::new(right_col.slice(cur_idx, 1), true);
                let result = self.apply_op(&left_datum, &right_scalar)?;
                (result, false)
            }
            // flat + flat -> flat
            (true, true) => {
                let left_idx = left_chunk.cur_idx().unwrap();
                let right_idx = right_chunk.cur_idx().unwrap();
                let left_scalar = DatumRef::new(left_col.slice(left_idx, 1), false);
                let right_scalar = DatumRef::new(right_col.slice(right_idx, 1), false);
                let result = self.apply_op(&left_scalar, &right_scalar)?;
                (result, true)
            }
            // unflat + unflat -> unflat
            (false, false) => {
                // Check if they are from the same chunk
                if self.check_same_chunk() {
                    let left_datum = DatumRef::new(left_col.clone(), false);
                    let right_datum = DatumRef::new(right_col.clone(), false);
                    let result = self.apply_op(&left_datum, &right_datum)?;
                    (result, false)
                } else {
                    // Different chunks -> cartesian product
                    let result = self.apply_cartesian_product(left_col, right_col)?;
                    (result, false)
                }
            }
        };

        let mut result_chunk = DataChunk::new(vec![result_array]);
        if result_is_flat {
            result_chunk.set_cur_idx(Some(0));
        } else {
            result_chunk.set_unflat();
        }

        Ok(result_chunk)
    }
}

impl<L, R> FactorizedBinary<L, R> {
    fn apply_op(&self, left: &DatumRef, right: &DatumRef) -> ExecutionResult<ArrayRef> {
        let result = match self.op {
            BinaryOp::Add => numeric::add(left, right)?,
            BinaryOp::Sub => numeric::sub(left, right)?,
            BinaryOp::Mul => numeric::mul(left, right)?,
            BinaryOp::Div => numeric::div(left, right)?,
            BinaryOp::Rem => numeric::rem(left, right)?,
            BinaryOp::And | BinaryOp::Or => {
                let left_arr = left.as_array().as_boolean();
                let right_arr = right.as_array().as_boolean();
                match self.op {
                    BinaryOp::And => Arc::new(boolean::and_kleene(left_arr, right_arr)?),
                    BinaryOp::Or => Arc::new(boolean::or_kleene(left_arr, right_arr)?),
                    _ => unreachable!(),
                }
            }
            BinaryOp::Eq => Arc::new(cmp::eq(left, right)?),
            BinaryOp::Ne => Arc::new(cmp::neq(left, right)?),
            BinaryOp::Gt => Arc::new(cmp::gt(left, right)?),
            BinaryOp::Ge => Arc::new(cmp::gt_eq(left, right)?),
            BinaryOp::Lt => Arc::new(cmp::lt(left, right)?),
            BinaryOp::Le => Arc::new(cmp::lt_eq(left, right)?),
        };
        Ok(result)
    }

    /// Apply cartesian product for two unflat arrays from different chunks
    fn apply_cartesian_product(
        &self,
        left_col: &ArrayRef,
        right_col: &ArrayRef,
    ) -> ExecutionResult<ArrayRef> {
        let left_len = left_col.len();

        // For each element in left, compute op with all elements in right
        let mut all_results = Vec::new();
        for i in 0..left_len {
            let left_scalar = DatumRef::new(left_col.slice(i, 1), true);
            let right_datum = DatumRef::new(right_col.clone(), false);
            let result = self.apply_op(&left_scalar, &right_datum)?;
            all_results.push(result);
        }

        // Concatenate all results
        use arrow::compute::concat;
        let all_results_refs: Vec<&dyn Array> =
            all_results.iter().map(|arr| arr.as_ref()).collect();
        let concatenated = concat(&all_results_refs)?;

        Ok(concatenated)
    }
}

impl<E> FactorizedEvaluator for Box<E>
where
    E: FactorizedEvaluator + ?Sized,
{
    fn evaluate(&self, result_set: &ResultSet) -> ExecutionResult<DataChunk> {
        (**self).evaluate(result_set)
    }
}

#[cfg(test)]
mod tests {
    use arrow::array::create_array;
    use minigu_common::result_set::{DataChunkPos, DataPos};
    use minigu_common::{data_chunk, data_pos, result_set};

    use super::*;

    #[test]
    fn test_flat_plus_flat() {
        let chunk1 = data_chunk!((Int32, [1, 2, 3]));
        let chunk2 = data_chunk!((Int32, [10, 20, 30]));
        let result_set = result_set!(chunk1, chunk2);

        // flat + flat: cur_idx=0 -> 1 + 10 = 11
        let evaluator =
            FactorizedDataRef::new(data_pos!(0, 0)).add(FactorizedDataRef::new(data_pos!(1, 0)));

        let result = evaluator.evaluate(&result_set).unwrap();
        assert!(result.cur_idx().is_some());
        let expected = create_array!(Int32, [11]);
        assert_eq!(result.columns()[0].as_ref(), expected.as_ref());
    }

    #[test]
    fn test_flat_plus_unflat() {
        let chunk1 = data_chunk!((Int32, [1, 2, 3]));
        let mut chunk2 = data_chunk!((Int32, [10, 20, 30]));
        chunk2.set_unflat();
        let result_set = result_set!(chunk1, chunk2);

        // flat + unflat: 1 + [10, 20, 30] = [11, 21, 31]
        let evaluator =
            FactorizedDataRef::new(data_pos!(0, 0)).add(FactorizedDataRef::new(data_pos!(1, 0)));

        let result = evaluator.evaluate(&result_set).unwrap();
        assert!(result.is_unflat());
        let expected = create_array!(Int32, [11, 21, 31]);
        assert_eq!(result.columns()[0].as_ref(), expected.as_ref());
    }

    #[test]
    fn test_unflat_in_same_chunk() {
        // chunk (unflat): [1, 2, 3], [10, 20, 30]
        // input_total_num_tuples = 3 * factor = 3 * 1 = 3
        let c1 = create_array!(Int32, [1, 2, 3]);
        let c2 = create_array!(Int32, [10, 20, 30]);
        let mut chunk = DataChunk::new(vec![c1, c2]);
        chunk.set_unflat();
        let input_rs = result_set!(chunk);

        // unflat + unflat(in the same chunk): [1, 2, 3] * [10, 20, 30] = [10, 40, 90]
        let evaluator =
            FactorizedDataRef::new(data_pos!(0, 0)).mul(FactorizedDataRef::new(data_pos!(0, 1)));

        let result = evaluator.evaluate(&input_rs).unwrap();
        assert!(result.is_unflat());
        let expected = create_array!(Int32, [10, 40, 90]);
        assert_eq!(result.columns()[0].as_ref(), expected.as_ref());
    }

    #[test]
    fn test_unflat_in_different_chunks() {
        // chunk1 (unflat): [1, 2, 3]
        let mut chunk1 = data_chunk!((Int32, [1, 2, 3]));
        chunk1.set_unflat();

        // chunk2 (unflat): [10, 20]
        let mut chunk2 = data_chunk!((Int32, [10, 20]));
        chunk2.set_unflat();

        let result_set = result_set!(chunk1, chunk2);

        // [1, 2, 3] + [10, 20] = [11, 21, 12, 22, 13, 23]
        let evaluator =
            FactorizedDataRef::new(data_pos!(0, 0)).add(FactorizedDataRef::new(data_pos!(1, 0)));

        let result = evaluator.evaluate(&result_set).unwrap();
        assert!(result.is_unflat());
        let expected = create_array!(Int32, [11, 21, 12, 22, 13, 23]);
        assert_eq!(result.columns()[0].as_ref(), expected.as_ref());
    }

    #[test]
    fn test_unary_neg() {
        let mut chunk = data_chunk!((Int32, [1, 2, 3]));
        chunk.set_unflat();
        let result_set = result_set!(chunk);

        let evaluator = FactorizedDataRef::new(data_pos!(0, 0)).neg();

        let result = evaluator.evaluate(&result_set).unwrap();
        assert!(result.is_unflat());
        let expected = create_array!(Int32, [-1, -2, -3]);
        assert_eq!(result.columns()[0].as_ref(), expected.as_ref());
    }

    #[test]
    fn test_constant_mul_unflat() {
        let mut chunk = data_chunk!((Int32, [5, 10, 15]));
        chunk.set_unflat();
        let result_set = result_set!(chunk);

        // 2 * [5, 10, 15] = [10, 20, 30]
        let evaluator =
            FactorizedConstant::new(2i32.into()).mul(FactorizedDataRef::new(data_pos!(0, 0)));

        let result = evaluator.evaluate(&result_set).unwrap();
        assert!(result.is_unflat());
        let expected = create_array!(Int32, [10, 20, 30]);
        assert_eq!(result.columns()[0].as_ref(), expected.as_ref());
    }
}
