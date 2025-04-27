use std::collections::HashMap;
use minigu_catalog::schema::SchemaId;
use crate::program::bound_statement::common::{GraphId, GraphTypeId, ProcedureId};

use smol_str::SmolStr;
use crate::program::bound_statement::object_ref::CanonicalSchemaPath;

pub type Ident = SmolStr;
#[derive(Debug, Default)]
pub struct Schema;

impl Schema {
    pub fn get_procedure_id(&self, name: &str) -> Option<ProcedureId> {
        Some(1)
    }

    pub fn get_graph_type_id(&self, name: &str) -> Option<GraphTypeId> {
        Some(1)
    }

    pub fn get_graph_id(&self, name: &str) -> Option<GraphId> {
        Some(1)
    }
    
    pub fn get_schema_id(&self) -> SchemaId {
        1
    }

}

#[derive(Debug, Default)]
pub struct Catalog {
    schema: Schema
}
impl Catalog {
    pub fn get_schema_id(&self, path:&[Ident]) -> Option<SchemaId> {
        Some(1)
    }

    pub fn get_schema(&self, path:&[Ident]) -> Option<&Schema> {
        Some(&self.schema)
    }
    pub fn get_schema_mut(&mut self, path:&[Ident]) -> Option<&mut Schema> {
        Some(&mut self.schema)
    }
    
}