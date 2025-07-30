use arrow::array::ArrayRef;
use minigu_common::data_chunk::DataChunk;
use minigu_common::value::ScalarValue;

use super::{DatumRef, Evaluator};
use crate::error::ExecutionResult;

#[derive(Debug, Clone)]
pub struct Constant(ArrayRef);

impl Constant {
    pub fn new(value: ScalarValue) -> Self {
        Self(value.to_scalar_array())
    }
}

impl Evaluator for Constant {
    fn evaluate(&self, _chunk: &DataChunk) -> ExecutionResult<DatumRef> {
        Ok(DatumRef::new(self.0.clone(), true))
    }
}

#[cfg(test)]
mod tests {
    use arrow::array::create_array;
    use minigu_common::data_chunk;

    use super::*;

    #[test]
    fn test_constant() {
        let chunk = data_chunk!((Int32, [123]));
        let e = Constant::new(1i32.into());
        let result = e.evaluate(&chunk).unwrap();
        let expected: ArrayRef = create_array!(Int32, [1]);
        assert_eq!(result.as_array(), &expected);
        assert!(result.is_scalar());
    }
}
