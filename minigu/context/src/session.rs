use std::sync::Arc;

use minigu_catalog::provider::SchemaRef;
use minigu_ir::named_ref::NamedGraphRef;

use crate::database::DatabaseContext;

#[derive(Clone)]
pub struct SessionContext {
    database: Arc<DatabaseContext>,
    pub home_schema: Option<SchemaRef>,
    pub current_schema: Option<SchemaRef>,
    pub home_graph: Option<NamedGraphRef>,
    pub current_graph: Option<NamedGraphRef>,
}

impl SessionContext {
    pub fn new(database: Arc<DatabaseContext>) -> Self {
        Self {
            database,
            home_schema: None,
            current_schema: None,
            home_graph: None,
            current_graph: None,
        }
    }

    pub fn database(&self) -> &DatabaseContext {
        &self.database
    }
}
