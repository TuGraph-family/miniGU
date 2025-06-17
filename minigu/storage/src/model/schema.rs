use std::collections::HashMap;
use std::sync::Arc;

use minigu_common::data_type::DataField;
use minigu_common::types::LabelId;

use crate::error::{SchemaError, StorageError, StorageResult};

pub type Identifier = String;

#[derive(Debug, Clone)]
pub struct VertexSchema {
    pub schema: Vec<DataField>, // propertyName -> DataField
}

impl VertexSchema {
    pub fn new(schema: Vec<DataField>) -> Self {
        VertexSchema { schema }
    }
}

#[derive(Debug, Clone)]
pub struct EdgeSchema {
    pub source_label_id: LabelId,
    pub target_label_id: LabelId,
    pub schema: Vec<DataField>,
}

impl EdgeSchema {
    pub fn new(source_label_id: LabelId, target_label_id: LabelId, schema: Vec<DataField>) -> Self {
        EdgeSchema {
            source_label_id,
            target_label_id,
            schema,
        }
    }
}

#[derive(Debug)]
pub struct SchemaManager {
    pub vertex_schemas: HashMap<Identifier, Arc<VertexSchema>>, /* vertex_label_name ->
                                                                 * VertexSchema */
    pub edge_schemas: HashMap<Identifier, Arc<EdgeSchema>>, // edge_label_name -> EdgeSchema
    pub id_to_vertex_schema_map: HashMap<LabelId, Identifier>, // label_id -> vertex_schema_name
    pub id_to_edge_schema_map: HashMap<LabelId, Identifier>, // label_id -> edge_schema_name
    pub vertex_label_id: LabelId,
    pub edge_label_id: LabelId,
}

impl Default for SchemaManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaManager {
    pub fn new() -> Self {
        SchemaManager {
            vertex_schemas: HashMap::new(),
            edge_schemas: HashMap::new(),
            id_to_vertex_schema_map: HashMap::new(),
            id_to_edge_schema_map: HashMap::new(),
            vertex_label_id: LabelId::new(1).unwrap(),
            edge_label_id: LabelId::new(1).unwrap(),
        }
    }

    pub fn create_vertex_schema(
        &mut self,
        schema_name: &str,
        schema: VertexSchema,
    ) -> StorageResult<()> {
        if self.vertex_schemas.contains_key(schema_name) {
            return Err(StorageError::Schema(SchemaError::VertexSchemaAlreadyExists));
        }

        self.vertex_schemas
            .insert(schema_name.to_string(), Arc::new(schema));
        self.id_to_vertex_schema_map
            .insert(self.vertex_label_id, schema_name.to_string());
        self.vertex_label_id = LabelId::new(self.vertex_label_id.get() + 1).unwrap();
        Ok(())
    }

    pub fn get_vertex_schema(&self, schema_name: &str) -> Option<Arc<VertexSchema>> {
        self.vertex_schemas.get(schema_name).cloned()
    }

    pub fn create_edge_schema(
        &mut self,
        schema_name: &str,
        schema: EdgeSchema,
    ) -> StorageResult<()> {
        if self.edge_schemas.contains_key(schema_name) {
            return Err(StorageError::Schema(SchemaError::EdgeSchemaAlreadyExists));
        }

        self.edge_schemas
            .insert(schema_name.to_string(), Arc::new(schema));
        self.id_to_edge_schema_map
            .insert(self.edge_label_id, schema_name.to_string());
        self.edge_label_id = LabelId::new(self.edge_label_id.get() + 1).unwrap();
        Ok(())
    }

    pub fn get_edge_schema(&self, schema_name: &str) -> Option<Arc<EdgeSchema>> {
        self.edge_schemas.get(schema_name).cloned()
    }

    pub fn get_vertex_schema_by_label_id(&self, label_id: LabelId) -> Option<Arc<VertexSchema>> {
        let schema_name = self.id_to_vertex_schema_map.get(&label_id)?;
        self.vertex_schemas.get(schema_name).cloned()
    }

    pub fn get_edge_schema_by_label_id(&self, label_id: LabelId) -> Option<Arc<EdgeSchema>> {
        let schema_name = self.id_to_edge_schema_map.get(&label_id)?;
        self.edge_schemas.get(schema_name).cloned()
    }

    pub fn get_label_id_by_vertex_schema_name(&self, schema_name: &str) -> Option<LabelId> {
        for (label_id, name) in &self.id_to_vertex_schema_map {
            if name == schema_name {
                return Some(*label_id);
            }
        }
        None
    }

    pub fn get_label_id_by_edge_schema_name(&self, schema_name: &str) -> Option<LabelId> {
        for (label_id, name) in &self.id_to_edge_schema_map {
            if name == schema_name {
                return Some(*label_id);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use minigu_common::data_type::{DataField, LogicalType};

    use super::*;

    fn create_vertex_schema() -> VertexSchema {
        VertexSchema::new(vec![
            DataField::new("name".to_string(), LogicalType::String, false),
            DataField::new("age".to_string(), LogicalType::Int32, false),
        ])
    }

    fn create_edge_schema() -> EdgeSchema {
        EdgeSchema::new(LabelId::new(1).unwrap(), LabelId::new(2).unwrap(), vec![
            DataField::new("from".to_string(), LogicalType::Int32, false),
        ])
    }

    #[test]
    fn test_vertex_schema_creation() {
        let mut schema_manager = SchemaManager::new();
        let vertex_schema = create_vertex_schema();

        assert!(
            schema_manager
                .create_vertex_schema("Person", vertex_schema.clone())
                .is_ok()
        );

        let retrieved_schema = schema_manager.get_vertex_schema("Person");
        assert!(retrieved_schema.is_some());
        assert_eq!(retrieved_schema.unwrap().schema.len(), 2);
    }

    #[test]
    fn test_edge_schema_creation() {
        let mut schema_manager = SchemaManager::new();
        let edge_schema = create_edge_schema();

        assert!(
            schema_manager
                .create_edge_schema("Knows", edge_schema.clone())
                .is_ok()
        );

        let retrieved_schema = schema_manager.get_edge_schema("Knows");
        assert!(retrieved_schema.is_some());
        assert_eq!(retrieved_schema.unwrap().schema.len(), 1);
    }

    #[test]
    fn test_schema_name_mapping() {
        let mut schema_manager = SchemaManager::new();
        let vertex_schema = create_vertex_schema();

        schema_manager
            .create_vertex_schema("Person", vertex_schema)
            .unwrap();

        let label_id = schema_manager.get_label_id_by_vertex_schema_name("Person");
        assert!(label_id.is_some());

        let retrieved_schema = schema_manager.get_vertex_schema_by_label_id(label_id.unwrap());
        assert!(retrieved_schema.is_some());
    }
}
