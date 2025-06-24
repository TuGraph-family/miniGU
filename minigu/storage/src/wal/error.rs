use std::{fmt, io};

use postcard;

#[derive(Debug)]
pub enum WalError {
    Io(io::Error),
    Serialize(postcard::Error),
    Deserialize(postcard::Error),
}

impl fmt::Display for WalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalError::Io(err) => write!(f, "I/O error: {err}"),
            WalError::Serialize(err) => write!(f, "Serialization error: {err}"),
            WalError::Deserialize(err) => write!(f, "Deserialization error: {err}"),
        }
    }
}

impl std::error::Error for WalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WalError::Io(err) => Some(err),
            WalError::Serialize(err) => Some(err),
            WalError::Deserialize(err) => Some(err),
        }
    }
}
