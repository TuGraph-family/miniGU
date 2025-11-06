use std::sync::Arc;

use gql_parser::ast::{Ident, SchemaPathSegment, SchemaRef};
use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_catalog::named_ref::NamedGraphRef;
use minigu_catalog::provider::{CatalogProvider, SchemaProvider};

use crate::database::DatabaseContext;

#[derive(Clone, Debug)]
pub struct SessionContext {
    database: Arc<DatabaseContext>,
    pub home_schema: Option<Arc<MemorySchemaCatalog>>,
    pub current_schema: Option<Arc<MemorySchemaCatalog>>,
    // currently home_graph and current_graph is same.
    // In feature, home_graph is a default graph named default.
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

    pub fn set_current_schema(&mut self, schema: SchemaRef) -> Result<String, String> {
        match schema {
            SchemaRef::Absolute(schema_path) => {
                let mut current = self
                    .database
                    .catalog()
                    .get_root()
                    .map_err(|_| "error".to_string())?;
                let mut current_path = vec![];
                for segment in schema_path {
                    let name = match segment.value() {
                        SchemaPathSegment::Name(name) => name,
                        SchemaPathSegment::Parent => unreachable!(),
                    };
                    let current_dir = current
                        .into_directory()
                        .ok_or_else(|| "Error".to_string())?;
                    current_path.push(segment.value().clone());
                    let child = current_dir
                        .get_child(name)
                        .map_err(|_| "Error".to_string())?
                        .ok_or_else(|| "Error".to_string())?;
                    current = child;
                }
                let schema_arc: minigu_catalog::provider::SchemaRef = current
                    .into_schema()
                    .ok_or_else(|| "not a schema".to_string())?;

                let msc: Arc<MemorySchemaCatalog> = schema_arc
                    .downcast_arc::<MemorySchemaCatalog>()
                    .map_err(|_| "schema is not MemorySchemaCatalog".to_string())?;
                self.current_schema = Some(msc);
                Ok("SET".to_string())
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn set_current_graph(&mut self, graph_name: String) -> Result<String, String> {
        if self.current_schema.is_none() {
            return Err("Error current schema is none".to_string());
        };
        let schema = self
            .current_schema
            .as_ref()
            .ok_or_else(|| "Error".to_string())?
            .as_ref();
        let graph = schema
            .get_graph(graph_name.as_str())
            .map_err(|err| err.to_string())?
            .ok_or_else(|| "Error".to_string())?;
        self.current_graph = Some(NamedGraphRef::new(Ident::new(graph_name), graph));
        Ok("".to_string())
    }

    pub fn reset_current_graph(&mut self) {
        self.current_graph = self.home_graph.clone();
    }

    pub fn reset_current_schema(&mut self) {
        self.current_schema = self.home_schema.clone();
    }
}
