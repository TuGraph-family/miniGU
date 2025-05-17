use std::sync::Arc;

use crate::graph_type::GraphTypeCatalog;
use crate::types::GraphId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphCatalog {
    pub id: Option<GraphId>,
    pub name: String,
    pub type_catalog_ref: Arc<GraphTypeCatalog>,
}

impl GraphCatalog {
    pub fn new(name: String, type_catalog_ref: Arc<GraphTypeCatalog>) -> Self {
        GraphCatalog {
            id: None,
            name,
            type_catalog_ref,
        }
    }
}
