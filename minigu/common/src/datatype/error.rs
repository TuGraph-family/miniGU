use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("Data conversion error: {0}")]
    Conversion(#[from] super::value::ConversionError),

    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid data type: {0}")]
    InvalidDataType(String),
}

impl PartialEq for DataError {
    fn eq(&self, other: &Self) -> bool {
        use DataError::*;
        match (self, other) {
            (Conversion(a), Conversion(b)) => a == b,
            (ConstraintViolation(a), ConstraintViolation(b)) => a == b,
            _ => false,
        }
    }
}
