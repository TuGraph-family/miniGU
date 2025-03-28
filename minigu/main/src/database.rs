use std::path::Path;

use crate::error::Result;
use crate::session::Session;

#[derive(Debug)]
pub struct Database {}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        todo!()
    }

    pub fn open_in_memory() -> Result<Self> {
        Ok(Self {})
    }

    pub fn session(&self) -> Result<Session> {
        Ok(Session {})
    }
}
