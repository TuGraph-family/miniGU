use std::sync::Arc;
use crate::graph_type::GraphTypeCatalog;
use crate::types::GraphId;

#[derive(Debug, Clone)]
pub struct GraphCatalog {
    name: String,
    type_catalog_ref:Arc<GraphTypeCatalog>
}

impl GraphCatalog {
    pub fn new(name: String, type_catalog_ref: Arc<GraphTypeCatalog>) -> Self {
        GraphCatalog {
            name,
            type_catalog_ref
        }
    }
}