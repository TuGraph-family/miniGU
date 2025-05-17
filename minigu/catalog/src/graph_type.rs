use std::collections::HashMap;
use std::num::NonZeroU32;

use crate::error::Error;
use crate::types::{GraphTypeId, LabelId};
use crate::vertex_edge::{EdgeTypeCatalog, VertexTypeCatalog};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelIdGenerator {
    next_id: u32,
}

impl LabelIdGenerator {
    pub fn next(&mut self) -> LabelId {
        let id = NonZeroU32::new(self.next_id).expect("LabelId must be non-zero");
        self.next_id += 1;
        id
    }
}

impl Default for LabelIdGenerator {
    fn default() -> Self {
        Self { next_id: 1 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GraphTypeCatalog {
    pub id: Option<GraphTypeId>,
    pub name: String,
    pub vertex_id_map: HashMap<String, LabelId>,
    pub vertex_map: HashMap<LabelId, VertexTypeCatalog>,
    pub edge_id_map: HashMap<String, LabelId>,
    pub edge_map: HashMap<LabelId, EdgeTypeCatalog>,
    pub label_id_generator: LabelIdGenerator,
}

impl GraphTypeCatalog {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            name,
            vertex_id_map: HashMap::new(),
            vertex_map: HashMap::new(),
            edge_id_map: HashMap::new(),
            edge_map: HashMap::new(),
            label_id_generator: LabelIdGenerator::default(),
        }
    }

    pub fn create_vertex_type(
        &mut self,
        name: String,
        mut vertex: VertexTypeCatalog,
    ) -> Result<(), Error> {
        if self.vertex_id_map.contains_key(&name) {
            return Err(Error::GraphTypeAlreadyExists(name));
        }
        let label = self.label_id_generator.next();
        vertex.id = Some(label);
        self.vertex_id_map.insert(name.clone(), label);
        self.vertex_map.insert(label, vertex);
        Ok(())
    }

    pub fn get_vertex_type_by_name(&self, name: &str) -> Result<&VertexTypeCatalog, Error> {
        let label = self
            .vertex_id_map
            .get(name)
            .ok_or_else(|| Error::VertexTypeNotExists(name.to_string()))?;
        self.vertex_map
            .get(label)
            .ok_or_else(|| Error::VertexTypeNotExists(name.to_string()))
    }

    pub fn get_vertex_type_by_id(&self, label: LabelId) -> Result<&VertexTypeCatalog, Error> {
        self.vertex_map
            .get(&label)
            .ok_or_else(|| Error::VertexTypeNotExists(format!("label {}", label)))
    }

    pub fn delete_vertex_type_by_name(&mut self, name: &str) -> Result<(), Error> {
        let label = self
            .vertex_id_map
            .remove(name)
            .ok_or_else(|| Error::VertexTypeNotExists(name.to_string()))?;
        self.vertex_map.remove(&label);
        Ok(())
    }

    pub fn create_edge_type(
        &mut self,
        name: String,
        mut edge: EdgeTypeCatalog,
    ) -> Result<(), Error> {
        if self.edge_id_map.contains_key(&name) {
            return Err(Error::EdgeTypeAlreadyExists(name));
        }
        let label = self.label_id_generator.next();

        edge.id = Some(label);
        self.edge_id_map.insert(name.clone(), label);
        self.edge_map.insert(label, edge);
        Ok(())
    }

    pub fn get_edge_type_by_name(&self, name: &str) -> Result<&EdgeTypeCatalog, Error> {
        let label = self
            .edge_id_map
            .get(name)
            .ok_or_else(|| Error::EdgeTypeNotExists(name.to_string()))?;
        self.edge_map
            .get(label)
            .ok_or_else(|| Error::EdgeTypeNotExists(name.to_string()))
    }

    pub fn get_edge_type_by_id(&self, label: LabelId) -> Result<&EdgeTypeCatalog, Error> {
        self.edge_map
            .get(&label)
            .ok_or_else(|| Error::EdgeTypeNotExists(format!("label {}", label)))
    }

    pub fn delete_edge_by_name(&mut self, name: &str) -> Result<(), Error> {
        let label = self
            .edge_id_map
            .remove(name)
            .ok_or_else(|| Error::EdgeTypeNotExists(name.to_string()))?;
        self.edge_map.remove(&label);
        Ok(())
    }

    pub fn contains_vertex_type(&self, label: &LabelId) -> bool {
        self.vertex_map.contains_key(label)
    }

    pub fn contains_edge_type(&self, label: &LabelId) -> bool {
        self.edge_map.contains_key(label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> GraphTypeCatalog {
        GraphTypeCatalog::new("TestGraph".to_string())
    }

    #[test]
    fn test_new_graph_type_catalog() {
        let catalog = setup();
        assert_eq!(catalog.name, "TestGraph");
        assert!(catalog.vertex_id_map.is_empty());
        assert!(catalog.vertex_map.is_empty());
        assert!(catalog.edge_id_map.is_empty());
        assert!(catalog.edge_map.is_empty());
    }

    #[test]
    fn test_add_vertex_type() {
        let mut catalog = setup();
        let vertex = VertexTypeCatalog::default();
        assert!(
            catalog
                .create_vertex_type("Person".to_string(), vertex.clone())
                .is_ok()
        );
        assert!(
            catalog
                .create_vertex_type("Person".to_string(), vertex)
                .is_err()
        );
    }

    #[test]
    fn test_get_vertex_by_name() {
        let mut catalog = setup();
        let vertex = VertexTypeCatalog::default();
        catalog
            .create_vertex_type("Person".to_string(), vertex)
            .unwrap();

        assert!(catalog.get_vertex_type_by_name("Person").is_ok());
        assert!(catalog.get_vertex_type_by_name("NonExistent").is_err());
    }

    #[test]
    fn test_get_vertex_by_id() {
        let mut catalog = setup();
        let vertex = VertexTypeCatalog::default();
        catalog
            .create_vertex_type("Person".to_string(), vertex)
            .unwrap();
        let label = catalog.vertex_id_map.get("Person").unwrap();

        assert!(catalog.get_vertex_type_by_id(*label).is_ok());
        assert!(
            catalog
                .get_vertex_type_by_id(LabelId::new(999).unwrap())
                .is_err()
        );
    }

    #[test]
    fn test_remove_vertex_by_name() {
        let mut catalog = setup();
        let vertex = VertexTypeCatalog::default();
        catalog
            .create_vertex_type("Person".to_string(), vertex)
            .unwrap();

        assert!(catalog.delete_vertex_type_by_name("Person").is_ok());
        assert!(catalog.delete_vertex_type_by_name("Person").is_err());
    }

    #[test]
    fn test_add_edge_type() {
        let mut catalog = setup();
        let edge = EdgeTypeCatalog::default(); // 假设有一个默认实现
        assert!(
            catalog
                .create_edge_type("Knows".to_string(), edge.clone())
                .is_ok()
        );
        assert!(catalog.create_edge_type("Knows".to_string(), edge).is_err());
    }

    #[test]
    fn test_get_edge_by_name() {
        let mut catalog = setup();
        let edge = EdgeTypeCatalog::default();
        catalog.create_edge_type("Knows".to_string(), edge).unwrap();

        assert!(catalog.get_edge_type_by_name("Knows").is_ok());
        assert!(catalog.get_edge_type_by_name("NonExistent").is_err());
    }
}
