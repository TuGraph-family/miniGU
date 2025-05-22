use minigu_common::types::GraphId;

use crate::provider::{GraphProvider, GraphTypeRef};

#[derive(Debug)]
pub struct MemoryGraphCatalog {
    graph_id: GraphId,
    graph_type: GraphTypeRef,
}

impl MemoryGraphCatalog {
    #[inline]
    pub fn new(graph_id: GraphId, graph_type: GraphTypeRef) -> Self {
        Self {
            graph_id,
            graph_type,
        }
    }
}

impl GraphProvider for MemoryGraphCatalog {
    #[inline]
    fn id(&self) -> GraphId {
        self.graph_id
    }

    #[inline]
    fn graph_type(&self) -> GraphTypeRef {
        self.graph_type.clone()
    }
}
