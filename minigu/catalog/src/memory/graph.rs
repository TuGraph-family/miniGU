use crate::provider::{GraphProvider, GraphTypeRef};

#[derive(Debug)]
pub struct MemoryGraphCatalog {
    graph_type: GraphTypeRef,
}

impl MemoryGraphCatalog {
    #[inline]
    pub fn new(graph_type: GraphTypeRef) -> Self {
        Self { graph_type }
    }
}

impl GraphProvider for MemoryGraphCatalog {
    #[inline]
    fn graph_type(&self) -> GraphTypeRef {
        self.graph_type.clone()
    }
}
