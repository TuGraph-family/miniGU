use std::result;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {}

pub type Result<T> = result::Result<T, Error>;
