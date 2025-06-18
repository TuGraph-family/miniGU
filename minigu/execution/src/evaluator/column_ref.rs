use minigu_common::data_chunk::DataChunk;

use super::{DatumRef, Evaluator};
use crate::error::ExecutionResult;

#[derive(Debug, Clone)]
pub struct ColumnRef(usize);

impl ColumnRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}

impl Evaluator for ColumnRef {
    fn evaluate(&self, chunk: &DataChunk) -> ExecutionResult<DatumRef> {
        let column = chunk.columns().get(self.0).expect("column should exist");
        Ok(DatumRef::new(column.clone(), false))
    }
}

#[cfg(test)]
mod tests {
    use arrow::array::{ArrayRef, create_array};
    use minigu_common::data_chunk;

    use super::*;

    #[test]
    fn test_column_ref() {
        let chunk = data_chunk!((Int32, [1, 2, 3]), (Utf8, ["a", "b", "c"]));
        let e1 = ColumnRef::new(0);
        let result = e1.evaluate(&chunk).unwrap();
        let expected: ArrayRef = create_array!(Int32, [1, 2, 3]);
        assert_eq!(result.as_array(), &expected);

        let e2 = ColumnRef::new(1);
        let result = e2.evaluate(&chunk).unwrap();
        let expected: ArrayRef = create_array!(Utf8, ["a", "b", "c"]);
        assert_eq!(result.as_array(), &expected);
    }
}
