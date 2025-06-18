use arrow::array::BooleanArray;
use arrow::compute::kernels::boolean;
use minigu_common::data_chunk::DataChunk;

use super::utils::gen_try;
use super::{Executor, IntoExecutor};
use crate::error::ExecutionResult;

#[derive(Debug)]
pub struct FilterBuilder<E, P> {
    child: E,
    predicate: P,
}

impl<E, P> FilterBuilder<E, P> {
    pub fn new(child: E, predicate: P) -> Self {
        Self { child, predicate }
    }
}

impl<E, P> IntoExecutor for FilterBuilder<E, P>
where
    E: Executor,
    P: FnMut(&DataChunk) -> ExecutionResult<BooleanArray>,
{
    type IntoExecutor = impl Executor;

    // TODO: Optimize the implementation.
    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let FilterBuilder {
                child,
                mut predicate,
            } = self;
            for chunk in child.into_iter() {
                let chunk = gen_try!(chunk);
                let mut filter = gen_try!(predicate(&chunk));
                if let Some(old_filter) = chunk.filter() {
                    filter = gen_try!(boolean::and(old_filter, &filter));
                }
                match filter.true_count() {
                    0 => (),
                    true_count if true_count == chunk.len() => yield Ok(chunk.unfiltered()),
                    _ => yield Ok(chunk.with_filter(filter)),
                }
            }
        }
        .into_executor()
    }
}

#[cfg(test)]
mod tests {
    use minigu_common::data_chunk;

    use super::*;

    #[test]
    fn test_filter() {
        let chunk = data_chunk!((Int32, [Some(1), Some(2), Some(3), None, None, None]));
        let mut result = [Ok(chunk)]
            .into_executor()
            .filter(|_| {
                Ok(BooleanArray::from_iter([
                    Some(true),
                    Some(false),
                    None,
                    Some(true),
                    Some(false),
                    None,
                ]))
            })
            .next_chunk()
            .unwrap()
            .unwrap();
        result.compact();
        let expected = data_chunk!((Int32, [Some(1), None]));
        assert_eq!(result, expected);
    }
}
