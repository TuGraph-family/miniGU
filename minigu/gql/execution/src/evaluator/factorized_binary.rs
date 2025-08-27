use std::sync::Arc;

use arrow::array::{Array, AsArray, ListArray};
use arrow::compute;
use arrow::compute::kernels::{boolean, cmp, numeric};
use arrow::datatypes::{DataType, Field};
use minigu_common::data_chunk::DataChunk;

use super::binary::BinaryOp;
use super::{DatumRef, Evaluator, UnflatSide};
use crate::error::ExecutionResult;

#[derive(Debug)]
pub struct FactorizedBinary<L, R> {
    op: BinaryOp,
    left: L,
    right: R,
    fact: UnflatSide,
}

impl<L, R> FactorizedBinary<L, R> {
    pub fn new(op: BinaryOp, left: L, right: R, fact: UnflatSide) -> Self {
        Self {
            op,
            left,
            right,
            fact,
        }
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
                _ => {
                    return Err(crate::error::ExecutionError::Custom(
                        format!("Invalid binary operation for boolean operands: {:?}", op).into(),
                    ));
                }
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
fn compute_flat_op_unflat(
    op: BinaryOp,
    flat_datum: &DatumRef,
    unflat_datum: &DatumRef,
    flat_is_left: bool,
) -> ExecutionResult<DatumRef> {
    let flat_array = flat_datum.as_array();
    let unflat_lists: &ListArray = unflat_datum.as_array().as_list();
    let offsets = unflat_lists.offsets().clone();
    let mut result_values = Vec::with_capacity(unflat_lists.len());

    // Iterate over each sub-list and apply the kernel.
    for i in 0..unflat_lists.len() {
        let scalar_val = flat_array.slice(i, 1);
        let scalar_datum = DatumRef::new(scalar_val, true);
        let sub_unflat_val = unflat_lists.value(i);
        let sub_unflat_datum = DatumRef::new(sub_unflat_val, false);

        // Ensure correct order for non-commutative operations.
        let result_datum = if flat_is_left {
            apply_kernel(op, &scalar_datum, &sub_unflat_datum)?
        } else {
            apply_kernel(op, &sub_unflat_datum, &scalar_datum)?
        };
        let result_array = result_datum.into_array();
        result_values.push(result_array);
    }

    // Flatten the results of all sub-lists into a single values array.
    let value_refs: Vec<&dyn Array> = result_values.iter().map(|a| a.as_ref()).collect();
    let concatenated_values = compute::concat(&value_refs)?;
    let DataType::List(field) = unflat_lists.data_type() else {
        return Err(crate::error::ExecutionError::Custom(
            format!(
                "Expected List data type for unflat_lists, found {:?}",
                unflat_lists.data_type()
            )
            .into(),
        ));
    };
    let list_field = field.clone();
    let new_field = Arc::new(Field::new(
        list_field.name(),
        concatenated_values.data_type().clone(),
        list_field.is_nullable(),
    ));

    // Reconstruct the ListArray using original offsets and new values.
    let result_array = ListArray::new(
        new_field,
        offsets.clone(),
        concatenated_values,
        unflat_lists.nulls().cloned(),
    );
    Ok(DatumRef::new(Arc::new(result_array), false))
}

fn compute(
    op: BinaryOp,
    left: &DatumRef,
    right: &DatumRef,
    fact: UnflatSide,
) -> ExecutionResult<DatumRef> {
    match fact {
        UnflatSide::Both => {
            let left_lists: &ListArray = left.as_array().as_list();
            let right_lists: &ListArray = right.as_array().as_list();
            let left_values_datum = DatumRef::new(left_lists.values().clone(), false);
            let right_values_datum = DatumRef::new(right_lists.values().clone(), false);
            let result_values_datum = apply_kernel(op, &left_values_datum, &right_values_datum)?;
            let new_values = result_values_datum.into_array();
            let DataType::List(field) = left.as_array().data_type() else {
                return Err(crate::error::ExecutionError::Custom(
                    format!(
                        "Expected List data type for left operand, found {:?}",
                        left.as_array().data_type()
                    )
                    .into(),
                ));
            };
            let new_field = Arc::new(Field::new(
                field.name(),
                new_values.data_type().clone(),
                field.is_nullable(),
            ));
            let result_array = Arc::new(ListArray::new(
                new_field,
                left_lists.offsets().clone(),
                new_values,
                left_lists.nulls().cloned(),
            ));
            Ok(DatumRef::new(
                result_array,
                left.is_scalar() && right.is_scalar(),
            ))
        }
        UnflatSide::Left => {
            // unflat op scalar
            if right.is_scalar() {
                let left_lists: &ListArray = left.as_array().as_list();
                let left_values_datum = DatumRef::new(left_lists.values().clone(), false);
                let result_values_datum = apply_kernel(op, &left_values_datum, right)?;
                let new_values = result_values_datum.into_array();
                let DataType::List(field) = left.as_array().data_type() else {
                    return Err(crate::error::ExecutionError::Custom(
                        format!(
                            "Expected List data type for left operand, found {:?}",
                            left.as_array().data_type()
                        )
                        .into(),
                    ));
                };
                let new_field = Arc::new(Field::new(
                    field.name(),
                    new_values.data_type().clone(),
                    field.is_nullable(),
                ));
                let result_array = Arc::new(ListArray::new(
                    new_field,
                    left_lists.offsets().clone(),
                    new_values,
                    left_lists.nulls().cloned(),
                ));
                Ok(DatumRef::new(result_array, left.is_scalar()))
            } else {
                // flat op unflat
                compute_flat_op_unflat(op, right, left, false)
            }
        }
        UnflatSide::Right => {
            if left.is_scalar() {
                let right_list: &ListArray = right.as_array().as_list();
                let right_values_datum = DatumRef::new(right_list.values().clone(), false);
                let result_values_datum = apply_kernel(op, left, &right_values_datum)?;
                let new_values = result_values_datum.into_array();
                let DataType::List(field) = right.as_array().data_type() else {
                    return Err(crate::error::ExecutionError::Custom(
                        format!(
                            "Expected List data type for right operand, found {:?}",
                            right.as_array().data_type()
                        )
                        .into(),
                    ));
                };
                let new_field = Arc::new(Field::new(
                    field.name(),
                    new_values.data_type().clone(),
                    field.is_nullable(),
                ));
                let result_array = Arc::new(ListArray::new(
                    new_field,
                    right_list.offsets().clone(),
                    new_values,
                    right_list.nulls().cloned(),
                ));
                Ok(DatumRef::new(result_array, right.is_scalar()))
            } else {
                compute_flat_op_unflat(op, left, right, true)
            }
        }
    }
}

impl<L: Evaluator, R: Evaluator> Evaluator for FactorizedBinary<L, R> {
    fn evaluate(&self, chunk: &DataChunk) -> ExecutionResult<DatumRef> {
        let left_datum = self.left.evaluate(chunk)?;
        let right_datum = self.right.evaluate(chunk)?;
        compute(self.op, &left_datum, &right_datum, self.fact)
    }
}

#[cfg(test)]
mod tests {
    use arrow::array::{ArrayRef, Int32Builder, ListBuilder, create_array};

    use super::*;
    use crate::evaluator::column_ref::ColumnRef;
    use crate::evaluator::constant::Constant;

    #[test]
    fn test_unflat_op_scalar() {
        let c0 = {
            let field = Field::new_list_field(DataType::Int32, true);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(1), Some(2), Some(3), None]);
            Arc::new(builder.finish())
        };
        let chunk = DataChunk::new(vec![c0]);
        // c0 * 3 + 1
        let c0_mul_3_plus_1 = ColumnRef::new(0)
            .factorized_mul(Constant::new(3i32.into()), UnflatSide::Left)
            .factorized_add(Constant::new(1i32.into()), UnflatSide::Left);
        let result = c0_mul_3_plus_1.evaluate(&chunk).unwrap();
        let expected: ArrayRef = {
            let field = Field::new_list_field(DataType::Int32, true);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(4), Some(7), Some(10), None]);
            Arc::new(builder.finish())
        };
        assert_eq!(result.as_array(), &expected);
    }

    #[test]
    fn test_unflat_op_flat() {
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
        let c0_mul_c1 = ColumnRef::new(0).factorized_add(ColumnRef::new(1), UnflatSide::Left);
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
    fn test_unflat_op_unflat() {
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
        let c0_mul_c1 = ColumnRef::new(0).factorized_mul(ColumnRef::new(1), UnflatSide::Both);
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
