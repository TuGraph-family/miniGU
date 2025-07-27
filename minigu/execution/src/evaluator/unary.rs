use std::sync::Arc;

use arrow::array::{Array, AsArray, ListArray};
use arrow::compute::kernels::{boolean, numeric};
use arrow::datatypes::{DataType, Field};
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

/// Apply unary operation to ListArray by operating on each element
fn apply_unary_to_list(op: UnaryOp, list_datum: &DatumRef) -> ExecutionResult<DatumRef> {
    let list_array: &ListArray = list_datum.as_array().as_list();
    let values_datum = DatumRef::new(list_array.values().clone(), false);

    let result_values = match op {
        UnaryOp::Neg => numeric::neg(&values_datum.as_array())?,
        UnaryOp::Not => {
            let values = values_datum.as_array().as_boolean();
            Arc::new(boolean::not(values)?)
        }
    };

    let DataType::List(field) = list_array.data_type() else {
        unreachable!()
    };
    let new_field = Arc::new(Field::new(
        field.name(),
        result_values.data_type().clone(),
        field.is_nullable(),
    ));

    let result_array = ListArray::new(
        new_field,
        list_array.offsets().clone(),
        result_values,
        list_array.nulls().cloned(),
    );

    Ok(DatumRef::new(
        Arc::new(result_array),
        list_datum.is_scalar(),
    ))
}

impl<E: Evaluator> Evaluator for Unary<E> {
    fn evaluate(&self, chunk: &DataChunk) -> ExecutionResult<DatumRef> {
        let operand = self.operand.evaluate(chunk)?;

        match operand.as_array().data_type() {
            DataType::List(_) => apply_unary_to_list(self.op, &operand),
            _ => {
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
    }
}

#[cfg(test)]
mod tests {
    use arrow::array::{ArrayRef, Int32Builder, ListBuilder, create_array};
    use arrow::datatypes::{DataType, Field};
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

    #[test]
    fn test_factorized_unary_neg() {
        let c1 = {
            let field = Field::new_list_field(DataType::Int32, false);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(1), Some(2), Some(3)]);
            builder.append_value([Some(-4), Some(5)]);
            Arc::new(builder.finish())
        };
        let chunk = DataChunk::new(vec![c1]);
        let e = ColumnRef::new(0).neg();
        let result = e.evaluate(&chunk).unwrap();

        // Expected: [[-1, -2, -3], [4, -5]]
        let expected: ArrayRef = {
            let field = Field::new_list_field(DataType::Int32, false);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(-1), Some(-2), Some(-3)]);
            builder.append_value([Some(4), Some(-5)]);
            Arc::new(builder.finish()) as ArrayRef
        };
        assert_eq!(result.as_array(), &expected);
    }

    #[test]
    fn test_factorized_unary_not() {
        let c1 = {
            let field = Field::new_list_field(DataType::Boolean, false);
            let mut builder =
                ListBuilder::new(arrow::array::BooleanBuilder::new()).with_field(Arc::new(field));
            builder.append_value([Some(true), Some(false), Some(true)]);
            builder.append_value([Some(false), Some(true)]);
            Arc::new(builder.finish())
        };
        let chunk = DataChunk::new(vec![c1]);
        let e = ColumnRef::new(0).not();
        let result = e.evaluate(&chunk).unwrap();

        // Expected: [[false, true, false], [true, false]]
        let expected: ArrayRef = {
            let field = Field::new_list_field(DataType::Boolean, false);
            let mut builder =
                ListBuilder::new(arrow::array::BooleanBuilder::new()).with_field(Arc::new(field));
            builder.append_value([Some(false), Some(true), Some(false)]);
            builder.append_value([Some(true), Some(false)]);
            Arc::new(builder.finish())
        };
        assert_eq!(result.as_array(), &expected);
    }
}
