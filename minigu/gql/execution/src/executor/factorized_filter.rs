use arrow::array::ListArray;
use minigu_common::data_chunk::DataChunk;

use super::utils::gen_try;
use super::{Executor, IntoExecutor};
use crate::error::ExecutionResult;

#[derive(Debug)]
pub struct FactorizedFilterBuilder<E, P> {
    child: E,
    predicate: P,
    unflat_column_indices: Vec<usize>,
}

impl<E, P> FactorizedFilterBuilder<E, P> {
    pub fn new(child: E, predicate: P, unflat_column_indices: Vec<usize>) -> Self {
        Self {
            child,
            predicate,
            unflat_column_indices,
        }
    }
}

impl<E, P> IntoExecutor for FactorizedFilterBuilder<E, P>
where
    E: Executor,
    P: FnMut(&DataChunk) -> ExecutionResult<ListArray>,
{
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let FactorizedFilterBuilder {
                child,
                mut predicate,
                unflat_column_indices,
            } = self;
            for chunk_result in child.into_iter() {
                let mut chunk = gen_try!(chunk_result);
                let filter_list = gen_try!(predicate(&chunk));

                chunk.factorized_compact(&filter_list, &unflat_column_indices);

                yield Ok(chunk);
            }
        }
        .into_executor()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use arrow::array::{BooleanBuilder, Int32Builder, ListBuilder, create_array};
    use arrow::datatypes::{DataType, Field};

    use super::*;

    #[test]
    fn test_factorized_filter_executor() {
        // c1, c2
        // 1, [1, 10, 20]
        // 2, [5, 15, 25]
        // 3, [1, 1, 1]
        let c1 = create_array!(Int32, [1, 2, 3]);
        let c2 = {
            let field = Field::new_list_field(DataType::Int32, false);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(1), Some(10), Some(20)]);
            builder.append_value([Some(5), Some(15), Some(25)]);
            builder.append_value([Some(1), Some(1), Some(1)]);
            Arc::new(builder.finish())
        };
        let chunk = DataChunk::new(vec![c1, c2]);

        // filter
        // [
        //   [true, false, true],
        //   [false, true, false],
        //   [false, false, false],
        // ]
        let result = [Ok(chunk)]
            .into_executor()
            .factorized_filter(
                |_| {
                    let predicate = {
                        let field = Field::new_list_field(DataType::Boolean, false);
                        let mut builder =
                            ListBuilder::new(BooleanBuilder::new()).with_field(Arc::new(field));
                        builder.append_value([Some(true), Some(false), Some(true)]);
                        builder.append_value([Some(false), Some(true), Some(false)]);
                        builder.append_value([Some(false), Some(false), Some(false)]);
                        builder.finish()
                    };
                    Ok(predicate)
                },
                vec![1],
            )
            .next_chunk()
            .unwrap()
            .unwrap();

        // expected
        // c1, c2
        // 1, [1, 20]
        // 2, [15]
        // 3, []
        let expected_c1 = create_array!(Int32, [1, 2, 3]);
        let expected_c2 = {
            let field = Field::new_list_field(DataType::Int32, false);
            let mut builder = ListBuilder::new(Int32Builder::new()).with_field(Arc::new(field));
            builder.append_value([Some(1), Some(20)]);
            builder.append_value([Some(15)]);
            builder.append_value([]);
            Arc::new(builder.finish())
        };
        let expected_chunk = DataChunk::new(vec![expected_c1, expected_c2]);

        assert_eq!(result, expected_chunk);
    }
}
