use thiserror::Error;

use crate::datatype::value::DataType;

#[derive(Debug, Error, PartialEq)]
pub enum DataError {
    #[error("Data conversion error: {0}")]
    ConversionError(#[from] ConversionError),
}

#[derive(Debug, Error, PartialEq)]
pub enum ConversionError {
    #[error("Type mismatch: expected {expected:?}, got {actual:?}")]
    TypeMismatch {
        expected: DataType,
        actual: DataType,
    },
}
