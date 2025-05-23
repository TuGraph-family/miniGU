use std::collections::HashMap;
use std::sync::Arc;

use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_common::value::ScalarValue;
use minigu_storage::memory::MemoryGraph;

use crate::database::DatabaseContext;

pub struct SessionContext {
    database: Arc<DatabaseContext>,
    current_schema: Option<Arc<MemorySchemaCatalog>>,
    current_graph: Option<Arc<MemoryGraph>>,
    parameters: HashMap<String, ScalarValue>,
}

impl SessionContext {
    #[inline]
    pub fn new(database: Arc<DatabaseContext>) -> Self {
        Self {
            database,
            current_schema: None,
            current_graph: None,
            parameters: HashMap::new(),
        }
    }

    #[inline]
    pub fn database(&self) -> &Arc<DatabaseContext> {
        &self.database
    }

    #[inline]
    pub fn current_schema(&self) -> Option<&Arc<MemorySchemaCatalog>> {
        self.current_schema.as_ref()
    }

    #[inline]
    pub fn current_schema_mut(&mut self, schema: Option<Arc<MemorySchemaCatalog>>) {
        self.current_schema = schema;
    }

    #[inline]
    pub fn current_graph(&self) -> Option<&Arc<MemoryGraph>> {
        self.current_graph.as_ref()
    }

    #[inline]
    pub fn current_graph_mut(&mut self, graph: Option<Arc<MemoryGraph>>) {
        self.current_graph = graph;
    }

    #[inline]
    pub fn get_parameter(&self, name: &str) -> Option<&ScalarValue> {
        self.parameters.get(name)
    }

    #[inline]
    pub fn set_parameter(&mut self, name: String, value: ScalarValue) {
        self.parameters.insert(name, value);
    }
}
