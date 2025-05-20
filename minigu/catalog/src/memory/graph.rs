use minigu_common::types::GraphId;

use crate::provider::{GraphProvider, GraphTypeRef};

#[derive(Debug)]
pub struct MemoryGraphCatalog {
    id: GraphId,
    graph_type: GraphTypeRef,
}

impl GraphProvider for MemoryGraphCatalog {
    #[inline]
    fn id(&self) -> GraphId {
        self.id
    }

    #[inline]
    fn graph_type(&self) -> GraphTypeRef {
        self.graph_type.clone()
    }
}
