use std::error::Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CatalogError {
    #[error(transparent)]
    External(#[from] Box<dyn Error + Send + Sync>),
}

pub type CatalogResult<T> = Result<T, CatalogError>;
