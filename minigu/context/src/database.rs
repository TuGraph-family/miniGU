use std::sync::Arc;

use minigu_catalog::memory::MemoryCatalog;
use minigu_common::registry::Registry;
use minigu_common::types::{GraphId, ProcedureId};
use minigu_storage::memory::MemoryGraph;

pub struct DatabaseContext {
    catalog: MemoryCatalog,
    graph_registry: Registry<GraphId, Arc<MemoryGraph>>,
    procedure_registry: Registry<ProcedureId, Arc<()>>,
}

impl DatabaseContext {}
