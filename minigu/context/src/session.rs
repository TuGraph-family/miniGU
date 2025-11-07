use std::sync::Arc;

use gql_parser::ast::{Ident, SchemaPathSegment, SchemaRef};
use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_catalog::named_ref::NamedGraphRef;
use minigu_catalog::provider::{CatalogProvider, SchemaProvider};

use crate::database::DatabaseContext;
use crate::error::{Error, SessionResult};

#[derive(Clone, Debug)]
pub struct SessionContext {
    database: Arc<DatabaseContext>,
    pub home_schema: Option<Arc<MemorySchemaCatalog>>,
    pub current_schema: Option<Arc<MemorySchemaCatalog>>,
    // currently home_graph and current_graph is same.
    // In the future, home_graph is a default graph named default.
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

    pub fn set_current_schema(&mut self, schema: SchemaRef) -> SessionResult<()> {
        match schema {
            SchemaRef::Absolute(schema_path) => {
                let mut current = self.database.catalog().get_root()?;
                let mut current_path = vec![];
                for segment in schema_path {
                    let name = match segment.value() {
                        SchemaPathSegment::Name(name) => name,
                        SchemaPathSegment::Parent => unreachable!(),
                    };
                    let current_dir = current
                        .into_directory()
                        .ok_or_else(|| Error::SchemaPathInvalid)?;
                    current_path.push(segment.value().clone());
                    let child = current_dir
                        .get_child(name)?
                        .ok_or_else(|| Error::SchemaPathInvalid)?;
                    current = child;
                }
                let schema_arc: minigu_catalog::provider::SchemaRef = current
                    .into_schema()
                    .ok_or_else(|| Error::SchemaPathInvalid)?;

                let msc: Arc<MemorySchemaCatalog> = schema_arc
                    .downcast_arc::<MemorySchemaCatalog>()
                    .map_err(|_| Error::SchemaPathInvalid)?;
                self.current_schema = Some(msc);
                Ok(())
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn set_current_graph(&mut self, graph_name: String) -> SessionResult<()> {
        if self.current_schema.is_none() {
            return Err(Error::CurrentSchemaNotSet);
        };
        let schema = self
            .current_schema
            .as_ref()
            .ok_or_else(|| Error::CurrentSchemaNotSet)?
            .as_ref();
        let graph = schema
            .get_graph(graph_name.as_str())?
            .ok_or_else(|| Error::GraphNotExists(graph_name.clone()))?;
        self.current_graph = Some(NamedGraphRef::new(Ident::new(graph_name), graph));
        Ok(())
    }

    pub fn reset_current_graph(&mut self) {
        self.current_graph = self.home_graph.clone();
    }

    pub fn reset_current_schema(&mut self) {
        self.current_schema = self.home_schema.clone();
    }
}
