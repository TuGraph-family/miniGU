use std::sync::Arc;

use arrow::array::{Array, AsArray, ListArray};
use arrow::compute::kernels::{boolean, numeric};
use arrow::datatypes::{DataType, Field};
use minigu_common::data_chunk::DataChunk;

use super::unary::UnaryOp;
use super::{DatumRef, Evaluator};
use crate::error::ExecutionResult;

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

impl<E: Evaluator> Evaluator for FactorizedUnary<E> {
    fn evaluate(&self, chunk: &DataChunk) -> ExecutionResult<DatumRef> {
        let operand = self.operand.evaluate(chunk)?;

        let unflat_lists: &ListArray = operand.as_array().as_list();
        let values_datum = DatumRef::new(unflat_lists.values().clone(), false);

        let result_values = match self.op {
            UnaryOp::Neg => numeric::neg(&values_datum.as_array())?,
            UnaryOp::Not => {
                let values = values_datum.as_array().as_boolean();
                Arc::new(boolean::not(values)?)
            }
        };

        let DataType::List(field) = unflat_lists.data_type() else {
            return Err(crate::error::ExecutionError::Custom(
                format!(
                    "Expected List data type for ListArray, but found {:?}",
                    unflat_lists.data_type()
                )
                .into(),
            ));
        };
        let new_field = Arc::new(Field::new(
            field.name(),
            result_values.data_type().clone(),
            field.is_nullable(),
        ));

        let result_array = ListArray::new(
            new_field,
            unflat_lists.offsets().clone(),
            result_values,
            unflat_lists.nulls().cloned(),
        );

        Ok(DatumRef::new(Arc::new(result_array), operand.is_scalar()))
    }
}

#[cfg(test)]
mod tests {
    use arrow::array::{ArrayRef, Int32Builder, ListBuilder};
    use arrow::datatypes::{DataType, Field};

    use super::*;
    use crate::evaluator::column_ref::ColumnRef;

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
        let e = ColumnRef::new(0).factorized_neg();
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
        let e = ColumnRef::new(0).factorized_not();
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
