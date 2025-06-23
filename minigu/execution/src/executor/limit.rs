use super::utils::gen_try;
use super::{Executor, IntoExecutor};

#[derive(Debug)]
pub struct LimitBuilder<E> {
    child: E,
    limit: usize,
}

impl<E> LimitBuilder<E> {
    pub fn new(child: E, limit: usize) -> Self {
        Self { child, limit }
    }
}

impl<E> IntoExecutor for LimitBuilder<E>
where
    E: Executor,
{
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let LimitBuilder { child, limit } = self;
            let mut count = 0;

            for chunk in child.into_iter() {
                let chunk = gen_try!(chunk);
                if count >= limit {
                    break;
                }

                let remaining = limit - count;
                if chunk.len() <= remaining {
                    // If the current chunk has fewer rows than the remaining limit, output the
                    // entire chunk.
                    count += chunk.len();
                    yield Ok(chunk);
                } else {
                    // If the current chunk has more rows than the remaining limit, output the
                    // required number of rows.
                    let limited_chunk = chunk.slice(0, remaining);
                    yield Ok(limited_chunk);
                    break;
                }
            }
        }
        .into_executor()
    }
}

#[cfg(test)]
mod tests {

    use minigu_common::data_chunk;
    use minigu_common::data_chunk::DataChunk;

    use super::*;

    #[test]
    fn test_limit() {
        let chunk1 = data_chunk!((Int32, [1, 2, 3]));
        let chunk2 = data_chunk!((Int32, [4, 5, 6]));
        let chunk3 = data_chunk!((Int32, [7, 8, 9]));

        let result: DataChunk = [Ok(chunk1), Ok(chunk2), Ok(chunk3)]
            .into_executor()
            .limit(5)
            .into_iter()
            .collect::<Result<_, _>>()
            .unwrap();

        let expected = data_chunk!((Int32, [1, 2, 3, 4, 5]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_limit_larger_than_input() {
        let chunk = data_chunk!((Int32, [1, 2, 3]));

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .limit(10)
            .into_iter()
            .collect::<Result<_, _>>()
            .unwrap();

        let expected = data_chunk!((Int32, [1, 2, 3]));
        assert_eq!(result, expected);
    }
}
