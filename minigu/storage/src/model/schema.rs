use std::collections::HashMap;
use std::sync::Arc;

use common::datatype::types::LabelId;
use common::datatype::value::PropertyMeta;
use serde::{Deserialize, Serialize};

use crate::error::{SchemaError, StorageError, StorageResult};

pub type Identifier = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VertexSchema {
    pub schema: Vec<PropertyMeta>, // propertyName -> PropertyMeta
}

impl VertexSchema {
    pub fn new(schema: Vec<PropertyMeta>) -> Self {
        VertexSchema { schema }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EdgeSchema {
    pub source_label_id: LabelId,
    pub target_label_id: LabelId,
    pub schema: Vec<PropertyMeta>,
}

impl EdgeSchema {
    pub fn new(
        source_label_id: LabelId,
        target_label_id: LabelId,
        schema: Vec<PropertyMeta>,
    ) -> Self {
        EdgeSchema {
            source_label_id,
            target_label_id,
            schema,
        }
    }
}

#[derive(Debug, Default)]
pub struct SchemaManager {
    pub vertex_schemas: HashMap<Identifier, Arc<VertexSchema>>, /* vertex_label_name ->
                                                                 * VertexSchema */
    pub edge_schemas: HashMap<Identifier, Arc<EdgeSchema>>, // edge_label_name -> EdgeSchema
    pub id_to_vertex_schema_map: HashMap<LabelId, Identifier>, // label_id -> vertex_schema_name
    pub id_to_edge_schema_map: HashMap<LabelId, Identifier>, // label_id -> edge_schema_name
    pub vertex_label_id: LabelId,
    pub edge_label_id: LabelId,
}

impl SchemaManager {
    pub fn new() -> Self {
        SchemaManager {
            vertex_schemas: HashMap::new(),
            edge_schemas: HashMap::new(),
            id_to_vertex_schema_map: HashMap::new(),
            id_to_edge_schema_map: HashMap::new(),
            vertex_label_id: 0,
            edge_label_id: 0,
        }
    }

    // Add a new vertex schema
    pub fn add_vertex_schema(
        &mut self,
        vertex_label: String,
        schema: Arc<VertexSchema>,
    ) -> StorageResult<()> {
        if self.vertex_schemas.contains_key(&vertex_label) {
            return Err(StorageError::Schema(SchemaError::VertexSchemaAlreadyExists));
        }
        self.vertex_schemas.insert(vertex_label.clone(), schema);
        self.id_to_vertex_schema_map
            .insert(self.vertex_label_id, vertex_label);
        self.vertex_label_id += 1;
        Ok(())
    }

    pub fn get_vertex_schema_by_name(&self, name: &str) -> StorageResult<Arc<VertexSchema>> {
        self.vertex_schemas
            .get(name)
            .ok_or(StorageError::Schema(SchemaError::VertexSchemaNotFound))
            .cloned()
    }

    pub fn get_vertex_schema_by_id(&self, id: LabelId) -> StorageResult<Arc<VertexSchema>> {
        let name = self
            .id_to_vertex_schema_map
            .get(&id)
            .ok_or(StorageError::Schema(SchemaError::VertexSchemaNotFound))?;
        self.vertex_schemas
            .get(name)
            .ok_or(StorageError::Schema(SchemaError::VertexSchemaNotFound))
            .cloned()
    }

    // Add a new edge schema
    pub fn add_edge_schema(
        &mut self,
        edge_label: String,
        schema: Arc<EdgeSchema>,
    ) -> StorageResult<()> {
        if self.edge_schemas.contains_key(&edge_label) {
            return Err(StorageError::Schema(SchemaError::EdgeSchemaAlreadyExists));
        }
        self.edge_schemas.insert(edge_label.clone(), schema);
        self.id_to_edge_schema_map
            .insert(self.edge_label_id, edge_label);
        self.edge_label_id += 1;
        Ok(())
    }

    // Get the schema for an edge by label
    pub fn get_edge_schema_by_name(&self, name: &str) -> StorageResult<Arc<EdgeSchema>> {
        self.edge_schemas
            .get(name)
            .ok_or(StorageError::Schema(SchemaError::EdgeSchemaNotFound))
            .cloned()
    }

    // Get the schema for an edge by ID
    pub fn get_edge_schema_by_id(&self, id: LabelId) -> StorageResult<Arc<EdgeSchema>> {
        let name = self
            .id_to_edge_schema_map
            .get(&id)
            .ok_or(StorageError::Schema(SchemaError::EdgeSchemaNotFound))?;
        self.edge_schemas
            .get(name)
            .ok_or(StorageError::Schema(SchemaError::EdgeSchemaNotFound))
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use common::datatype::value::{DataType, PropertyMeta, PropertyValue};

    use super::super::edge::Edge;
    use super::super::properties::PropertyRecord;
    use super::super::vertex::Vertex;
    use super::*;

    fn create_vertex_schema() -> VertexSchema {
        VertexSchema::new(vec![
            PropertyMeta::new("name".to_string(), DataType::String, false, false, None),
            PropertyMeta::new("age".to_string(), DataType::Int, false, false, None),
        ])
    }

    fn create_person_alice() -> Vertex {
        Vertex::new(
            0,
            0,
            PropertyRecord::new(vec![
                PropertyValue::String("Alice".to_string()),
                PropertyValue::Int(30),
            ]),
        )
    }

    fn create_person_bob() -> Vertex {
        Vertex::new(
            1,
            0,
            PropertyRecord::new(vec![
                PropertyValue::String("Bob".to_string()),
                PropertyValue::Int(40),
            ]),
        )
    }

    fn create_edge_schema() -> EdgeSchema {
        EdgeSchema::new(0, 2, vec![PropertyMeta::new(
            "from".to_string(),
            DataType::Int,
            false,
            false,
            None,
        )])
    }

    fn create_edge_alice_knows_bob() -> Edge {
        Edge::new(0, 0, 1, 0, PropertyRecord::new(vec![PropertyValue::Int(5)]))
    }

    #[test]
    fn test_schema_manager() {
        let mut schema_manager = SchemaManager::new();
        let person_vertex_schema = Arc::new(create_vertex_schema());
        let knows_edge_schema = Arc::new(create_edge_schema());

        // Add vertex schema
        schema_manager
            .add_vertex_schema("person".to_string(), person_vertex_schema.to_owned())
            .unwrap();
        let vertex_schema = schema_manager.get_vertex_schema_by_name("person").unwrap();
        assert_eq!(vertex_schema.schema.len(), 2);

        // Add edge schema
        schema_manager
            .add_edge_schema("knows".to_string(), knows_edge_schema.to_owned())
            .unwrap();
        let edge_schema = schema_manager.get_edge_schema_by_name("knows").unwrap();
        assert_eq!(edge_schema.schema.len(), 1);

        // create vertex and verify each property by schema
        let alice = create_person_alice();
        let vertex_schema = schema_manager
            .get_vertex_schema_by_id(alice.label_id)
            .unwrap();
        assert_eq!(
            vertex_schema.schema.first().unwrap().data_type,
            alice.properties.get(0).unwrap().data_type()
        );
        assert_eq!(
            vertex_schema.schema.get(1).unwrap().data_type,
            alice.properties.get(1).unwrap().data_type()
        );

        let bob = create_person_bob();
        let vertex_schema = schema_manager
            .get_vertex_schema_by_id(bob.label_id)
            .unwrap();
        assert_eq!(
            vertex_schema.schema.first().unwrap().data_type,
            bob.properties.get(0).unwrap().data_type()
        );
        assert_eq!(
            vertex_schema.schema.get(1).unwrap().data_type,
            bob.properties.get(1).unwrap().data_type()
        );

        // create edge and verify each property by schema
        let alice_knows_bob = create_edge_alice_knows_bob();
        let edge_schema = schema_manager
            .get_edge_schema_by_id(alice_knows_bob.label_id())
            .unwrap();
        assert_eq!(
            edge_schema.schema.first().unwrap().data_type,
            alice_knows_bob.properties.get(0).unwrap().data_type()
        );
    }
}
