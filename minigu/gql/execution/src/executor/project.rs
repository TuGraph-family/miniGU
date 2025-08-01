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
    use minigu_common::data_chunk;

    use super::*;
    use crate::evaluator::Evaluator;
    use crate::evaluator::column_ref::ColumnRef;
    use crate::evaluator::constant::Constant;

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
}
