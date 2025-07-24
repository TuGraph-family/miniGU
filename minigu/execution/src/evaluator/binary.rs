use std::sync::Arc;

use arrow::array::{Array, AsArray, ListArray};
use arrow::buffer::OffsetBuffer;
use arrow::compute;
use arrow::compute::kernels::{boolean, cmp, numeric};
use arrow::datatypes::{DataType, Field};
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

fn apply_kernel(op: BinaryOp, left: &DatumRef, right: &DatumRef) -> ExecutionResult<DatumRef> {
    let array = match op {
        BinaryOp::Add => numeric::add(left, right)?,
        BinaryOp::Sub => numeric::sub(left, right)?,
        BinaryOp::Mul => numeric::mul(left, right)?,
        BinaryOp::Div => numeric::div(left, right)?,
        BinaryOp::Rem => numeric::rem(left, right)?,
        BinaryOp::And | BinaryOp::Or => {
            let left = left.as_array().as_boolean();
            let right = right.as_array().as_boolean();
            match op {
                BinaryOp::And => Arc::new(boolean::and_kleene(left, right)?),
                BinaryOp::Or => Arc::new(boolean::or_kleene(left, right)?),
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
    Ok(DatumRef::new(array, left.is_scalar() && right.is_scalar()))
}

/// The result is constructed by computing the result per list element,
/// then concat the computed values and reconstructing a new `ListArray`
/// using the original structure.
fn compute_array_op_list(
    op: BinaryOp,
    array_datum: &DatumRef,
    list_datum: &DatumRef,
) -> ExecutionResult<DatumRef> {
    let array = array_datum.as_array();
    let list_array: &ListArray = list_datum.as_array().as_list();
    let offsets = list_array.offsets().clone();
    let mut result_values = Vec::with_capacity(list_array.len());
    for i in 0..list_array.len() {
        let scalar_val = array.slice(i, 1);
        let scalar_datum = DatumRef::new(scalar_val, true);
        let sub_list_val = list_array.value(i);
        let sub_list_datum = DatumRef::new(sub_list_val, false);
        let result_datum = apply_kernel(op, &scalar_datum, &sub_list_datum)?;
        let result_array = result_datum.into_array();
        result_values.push(result_array);
    }
    let value_refs: Vec<&dyn Array> = result_values.iter().map(|a| a.as_ref()).collect();
    let concatenated_values = compute::concat(&value_refs)?;
    let DataType::List(field) = list_array.data_type() else {
        unreachable!()
    };
    let list_field = field.clone();
    let new_field = Arc::new(Field::new(
        list_field.name(),
        concatenated_values.data_type().clone(),
        list_field.is_nullable(),
    ));
    let result_list_array = ListArray::new(
        new_field,
        OffsetBuffer::new(offsets.into()),
        concatenated_values,
        list_array.nulls().cloned(),
    );
    Ok(DatumRef::new(Arc::new(result_list_array), false))
}

/// Dispatches binary computation based on the types of input operands
fn compute(op: BinaryOp, left: &DatumRef, right: &DatumRef) -> ExecutionResult<DatumRef> {
    match (left.as_array().data_type(), right.as_array().data_type()) {
        // ListArray op ListArray
        (DataType::List(left_field), DataType::List(_)) => {
            let left_list: &ListArray = left.as_array().as_list();
            let right_list: &ListArray = right.as_array().as_list();
            let left_values_datum = DatumRef::new(left_list.values().clone(), false);
            let right_values_datum = DatumRef::new(right_list.values().clone(), false);
            let result_values_datum = apply_kernel(op, &left_values_datum, &right_values_datum)?;
            let new_values = result_values_datum.into_array();
            let new_field = Arc::new(Field::new(
                left_field.name(),
                new_values.data_type().clone(),
                left_field.is_nullable(),
            ));
            let result_array = Arc::new(ListArray::new(
                new_field,
                left_list.offsets().clone(),
                new_values,
                left_list.nulls().cloned(),
            ));
            Ok(DatumRef::new(
                result_array,
                left.is_scalar() && right.is_scalar(),
            ))
        }

        (DataType::List(left_field), _) => {
            // ListArray op scalar
            if right.is_scalar() {
                let left_list: &ListArray = left.as_array().as_list();
                let left_values_datum = DatumRef::new(left_list.values().clone(), false);
                let result_values_datum = apply_kernel(op, &left_values_datum, right)?;
                let new_values = result_values_datum.into_array();
                let new_field = Arc::new(Field::new(
                    left_field.name(),
                    new_values.data_type().clone(),
                    left_field.is_nullable(),
                ));
                let result_array = Arc::new(ListArray::new(
                    new_field,
                    left_list.offsets().clone(),
                    new_values,
                    left_list.nulls().cloned(),
                ));
                Ok(DatumRef::new(result_array, left.is_scalar()))
            } else {
                // ListArray op Array
                compute_array_op_list(op, right, left)
            }
        }

        (_, DataType::List(right_field)) => {
            if left.is_scalar() {
                let right_list: &ListArray = right.as_array().as_list();
                let right_values_datum = DatumRef::new(right_list.values().clone(), false);
                let result_values_datum = apply_kernel(op, left, &right_values_datum)?;
                let new_values = result_values_datum.into_array();
                let new_field = Arc::new(Field::new(
                    right_field.name(),
                    new_values.data_type().clone(),
                    right_field.is_nullable(),
                ));
                let result_array = Arc::new(ListArray::new(
                    new_field,
                    right_list.offsets().clone(),
                    new_values,
                    right_list.nulls().cloned(),
                ));
                Ok(DatumRef::new(result_array, right.is_scalar()))
            } else {
                compute_array_op_list(op, left, right)
            }
        }

        _ => {
            // Neither operand is a ListArray
            apply_kernel(op, left, right)
        }
    }
}

impl<L: Evaluator, R: Evaluator> Evaluator for Binary<L, R> {
    fn evaluate(&self, chunk: &DataChunk) -> ExecutionResult<DatumRef> {
        let left_datum = self.left.evaluate(chunk)?;
        let right_datum = self.right.evaluate(chunk)?;
        compute(self.op, &left_datum, &right_datum)
    }
}

#[cfg(test)]
mod tests {
    use arrow::array::{ArrayRef, Int32Builder, ListBuilder, create_array};
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

    #[test]
    fn test_list_op_scalar() {
        let c0 = {
            let field = Field::new_list_field(DataType::Int32, true);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(1), Some(2), Some(3), None]);
            Arc::new(builder.finish())
        };
        let chunk = DataChunk::new(vec![c0]);
        // c0 * 3 + (1 + 1)
        let c0_mul_3_plus_2 = ColumnRef::new(0)
            .mul(Constant::new(3i32.into()))
            .add(Constant::new(1i32.into()).add(Constant::new(1i32.into())));
        let result = c0_mul_3_plus_2.evaluate(&chunk).unwrap();
        let expected: ArrayRef = {
            let field = Field::new_list_field(DataType::Int32, true);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(5), Some(8), Some(11), None]);
            Arc::new(builder.finish())
        };
        assert_eq!(result.as_array(), &expected);
    }

    #[test]
    fn test_list_op_array() {
        let c0 = {
            let field = Field::new_list_field(DataType::Int32, true);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(1), Some(2), Some(3), None]);
            builder.append_value([Some(4), Some(5), Some(6), None]);
            Arc::new(builder.finish())
        };
        let c1 = create_array!(Int32, [Some(2), None]);
        let chunk = DataChunk::new(vec![c0, c1]);
        // c0 + c1
        let c0_mul_c1 = ColumnRef::new(0).add(ColumnRef::new(1));
        let result = c0_mul_c1.evaluate(&chunk).unwrap();
        let expected: ArrayRef = {
            let field = Field::new_list_field(DataType::Int32, true);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(3), Some(4), Some(5), None]);
            builder.append_value([None, None, None, None]);
            Arc::new(builder.finish())
        };
        assert_eq!(result.as_array(), &expected);
    }

    #[test]
    fn test_list_op_list() {
        let c0 = {
            let field = Field::new_list_field(DataType::Int32, true);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(1), Some(2), Some(3), None]);
            Arc::new(builder.finish())
        };
        let c1 = {
            let field = Field::new_list_field(DataType::Int32, true);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(1), Some(2), None, Some(3)]);
            Arc::new(builder.finish())
        };
        let chunk = DataChunk::new(vec![c0, c1]);
        // c0 * c1
        let c0_mul_c1 = ColumnRef::new(0).mul(ColumnRef::new(1));
        let result = c0_mul_c1.evaluate(&chunk).unwrap();
        let expected: ArrayRef = {
            let field = Field::new_list_field(DataType::Int32, true);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(1), Some(4), None, None]);
            Arc::new(builder.finish())
        };
        assert_eq!(result.as_array(), &expected);
    }
}
