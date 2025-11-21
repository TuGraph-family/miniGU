use super::utils::gen_try;
use super::{Executor, IntoExecutor};

#[derive(Debug)]
pub struct OffsetBuilder<E> {
    child: E,
    offset: usize,
}

impl<E> OffsetBuilder<E> {
    pub fn new(child: E, offset: usize) -> Self {
        Self { child, offset }
    }
}

impl<E> IntoExecutor for OffsetBuilder<E>
where
    E: Executor,
{
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let OffsetBuilder { child, offset } = self;
            let mut skipped = 0usize;

            for chunk in child.into_iter() {
                let chunk = gen_try!(chunk);

                if skipped < offset {
                    let remaining = offset - skipped;
                    if chunk.len() <= remaining {
                        skipped += chunk.len();
                        continue;
                    } else {
                        let sliced = chunk.slice(remaining, chunk.len() - remaining);
                        skipped = offset;
                        yield Ok(sliced);
                    }
                } else {
                    yield Ok(chunk);
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
    fn test_offset_only_within_chunk() {
        let chunk = data_chunk!((Int32, [1, 2, 3, 4]));

        let result: DataChunk = [Ok(chunk)]
            .into_executor()
            .offset(2)
            .into_iter()
            .collect::<Result<_, _>>()
            .unwrap();

        let expected = data_chunk!((Int32, [3, 4]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_offset_spanning_chunks() {
        let chunk1 = data_chunk!((Int32, [1, 2]));
        let chunk2 = data_chunk!((Int32, [3, 4, 5]));

        let result: DataChunk = [Ok(chunk1), Ok(chunk2)]
            .into_executor()
            .offset(3)
            .into_iter()
            .collect::<Result<_, _>>()
            .unwrap();

        let expected = data_chunk!((Int32, [4, 5]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_offset_exceeds_input() {
        let chunk = data_chunk!((Int32, [1, 2]));

        let mut iter = [Ok(chunk)].into_executor().offset(5).into_iter();
        assert!(iter.next().is_none());
    }
}
