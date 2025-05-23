use std::path::Path;
use std::sync::RwLock;

use minigu_catalog::memory::MemoryCatalog;

use crate::error::Result;
use crate::graph::GraphRegistry;
use crate::procedure::registry::ProcedureRegistry;
use crate::session::Session;

#[derive(Debug)]
pub struct Database {}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        todo!("on-disk database is not implemented yet")
    }

    pub fn open_in_memory() -> Result<Self> {
        Ok(Self {})
    }

    pub fn session(&self) -> Result<Session> {
        Ok(Session {})
    }
}

pub struct DatabaseContext {
    catalog: MemoryCatalog,
    procedure_registry: ProcedureRegistry,
    graph_registry: GraphRegistry,
}

impl DatabaseContext {
    pub fn read_catalog(&self) {
    }
}
