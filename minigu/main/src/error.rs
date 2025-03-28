use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;
