use itertools::Itertools;
use minigu_common::data_chunk::DataChunk;

use super::utils::gen_try;
use super::{Executor, IntoExecutor};
use crate::evaluator::BoxedEvaluator;

#[derive(Debug)]
pub struct ProjectBuilder<E> {
    child: E,
    evaluators: Vec<BoxedEvaluator>,
}

impl<E> ProjectBuilder<E> {
    pub fn new(child: E, evaluators: Vec<BoxedEvaluator>) -> Self {
        Self { child, evaluators }
    }
}

impl<E> IntoExecutor for ProjectBuilder<E>
where
    E: Executor,
{
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let ProjectBuilder { child, evaluators } = self;
            for chunk in child.into_iter() {
                let chunk = gen_try!(chunk);
                let columns = gen_try!(
                    evaluators
                        .iter()
                        .map(|e| e.evaluate(&chunk).map(|d| d.into_array()))
                        .try_collect()
                );
                let mut new_chunk = DataChunk::new(columns);
                if let Some(filter) = chunk.filter() {
                    new_chunk = new_chunk.with_filter(filter.clone());
                }
                yield Ok(new_chunk);
            }
        }
        .into_executor()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use arrow::array::{BooleanArray, Int32Builder, ListBuilder, create_array};
    use arrow::datatypes::{DataType, Field};
    use minigu_common::data_chunk;

    use super::*;
    use crate::evaluator::column_ref::ColumnRef;
    use crate::evaluator::constant::Constant;
    use crate::evaluator::{Evaluator, UnflatSide};

    #[test]
    fn test_project() {
        let chunk = data_chunk!(
            { true, false, true },
            (Int32, [1, 2, 3]),
            (Utf8, ["a", "b", "c"])
        );
        let e1 = ColumnRef::new(0)
            .mul(ColumnRef::new(0))
            .add(Constant::new(3i32.into()));
        let e2 = ColumnRef::new(1);
        let e3 = ColumnRef::new(0).add(Constant::new(1i32.into()));
        let chunk: DataChunk = [Ok(chunk)]
            .into_executor()
            .project(vec![Box::new(e1), Box::new(e2), Box::new(e3)])
            .into_iter()
            .try_collect()
            .unwrap();
        let expected = data_chunk!((Int32, [4, 12]), (Utf8, ["a", "c"]), (Int32, [2, 4]));
        assert_eq!(chunk, expected);
    }

    #[test]
    fn test_factorized_project() {
        // c1, c2         filter
        // 1, [1]         true
        // 2, [2, 3, 4]   false
        // 3, [5, 6, 7]   true
        // after project(e1 = c2 * c2, e2 = c1 + c2 + 1, e3 = -c2):
        // e1, e2, e3
        // [1], [3], [-1]
        // [25, 36, 49], [9, 10, 11], [-5, -6, -7]
        let c1 = create_array!(Int32, [1, 2, 3]);
        let c2 = {
            let field = Field::new_list_field(DataType::Int32, false);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(1)]);
            builder.append_value([Some(2), Some(3), Some(4)]);
            builder.append_value([Some(5), Some(6), Some(7)]);
            Arc::new(builder.finish())
        };
        let filter = BooleanArray::from(vec![true, false, true]);
        let chunk = DataChunk::new(vec![c1, c2]).with_filter(filter);
        let e1 = ColumnRef::new(1).factorized_mul(ColumnRef::new(1), UnflatSide::Both);
        let e2 = ColumnRef::new(0)
            .factorized_add(ColumnRef::new(1), UnflatSide::Right)
            .factorized_add(Constant::new(1i32.into()), UnflatSide::Left);
        let e3 = ColumnRef::new(1).factorized_neg(); // Test unary operation on unflat column: -c2
        let chunk: DataChunk = [Ok(chunk)]
            .into_executor()
            .project(vec![Box::new(e1), Box::new(e2), Box::new(e3)])
            .into_iter()
            .try_collect()
            .unwrap();
        let expected_c1 = {
            let field = Field::new_list_field(DataType::Int32, false);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(1)]);
            builder.append_value([Some(25), Some(36), Some(49)]);
            Arc::new(builder.finish())
        };
        let expected_c2 = {
            let field = Field::new_list_field(DataType::Int32, false);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(3)]);
            builder.append_value([Some(9), Some(10), Some(11)]);
            Arc::new(builder.finish())
        };
        let expected_c3 = {
            let field = Field::new_list_field(DataType::Int32, false);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(-1)]);
            builder.append_value([Some(-5), Some(-6), Some(-7)]);
            Arc::new(builder.finish())
        };
        let expected = DataChunk::new(vec![expected_c1, expected_c2, expected_c3]);
        assert_eq!(chunk, expected);
    }
}
