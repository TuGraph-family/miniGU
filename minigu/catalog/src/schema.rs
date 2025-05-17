use std::collections::HashMap;
use std::num::NonZeroU32;

use crate::error::Error;
use crate::graph::GraphCatalog;
use crate::graph_type::GraphTypeCatalog;
use crate::procedure::ProcedureCatalog;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    /// This ID is assigned only when the schema is inserted into the catalog.
    /// Before insertion, it remains `None`.
    pub id: Option<SchemaId>,
    pub name: Ident,

    pub graph_id_map: HashMap<String, GraphId>,
    pub graph_map: HashMap<GraphId, GraphCatalog>,

    pub graph_type_id_map: HashMap<String, GraphTypeId>,
    pub graph_type_map: HashMap<GraphTypeId, GraphTypeCatalog>,

    pub procedure_id_map: HashMap<String, ProcedureId>,
    pub procedure_map: HashMap<ProcedureId, ProcedureCatalog>,
}

impl Schema {
    pub fn new(name: Ident) -> Self {
        Self {
            id: None,
            name,
            graph_id_map: HashMap::new(),
            graph_map: HashMap::new(),
            graph_type_id_map: HashMap::new(),
            graph_type_map: HashMap::new(),
            procedure_id_map: HashMap::new(),
            procedure_map: HashMap::new(),
        }
    }

    // ===== Graph =====
    pub fn create_graph(
        &mut self,
        name: String,
        mut graph: GraphCatalog,
    ) -> Result<GraphId, Error> {
        if self.graph_id_map.contains_key(&name) {
            return Err(Error::GraphAlreadyExists(name));
        }
        let id = NonZeroU32::new(self.graph_id_map.len() as u32 + 1).unwrap();
        graph.id = Some(id);
        self.graph_id_map.insert(name.clone(), id);
        self.graph_map.insert(id, graph);
        Ok(id)
    }

    pub fn get_graph_id(&self, name: &str) -> Result<GraphId, Error> {
        self.graph_id_map
            .get(name)
            .copied()
            .ok_or(Error::GraphNotExists(name.to_string()))
    }

    pub fn get_graph(&self, name: &str) -> Result<&GraphCatalog, Error> {
        let id = self.get_graph_id(name)?;
        self.graph_map
            .get(&id)
            .ok_or(Error::GraphNotExists(name.to_string()))
    }

    pub fn get_graph_by_id(&self, id: GraphId) -> Result<&GraphCatalog, Error> {
        self.graph_map
            .get(&id)
            .ok_or(Error::GraphNotExists(id.to_string()))
    }

    pub fn delete_graph(&mut self, name: &str) -> Result<(), Error> {
        let id = self
            .graph_id_map
            .remove(name)
            .ok_or(Error::GraphNotExists(name.to_string()))?;
        self.graph_map.remove(&id);
        Ok(())
    }

    // ===== Graph Type =====
    pub fn create_graph_type(
        &mut self,
        name: String,
        mut graph_type: GraphTypeCatalog,
    ) -> Result<GraphTypeId, Error> {
        if self.graph_type_id_map.contains_key(&name) {
            return Err(Error::GraphTypeAlreadyExists(name));
        }
        let id = NonZeroU32::new(self.graph_type_id_map.len() as u32 + 1).unwrap();
        graph_type.id = Some(id);
        self.graph_type_id_map.insert(name.clone(), id);
        self.graph_type_map.insert(id, graph_type);
        Ok(id)
    }

    pub fn get_graph_type_id(&self, name: &str) -> Result<GraphTypeId, Error> {
        self.graph_type_id_map
            .get(name)
            .copied()
            .ok_or(Error::GraphTypeNotExists(name.to_string()))
    }

    pub fn get_graph_type(&self, name: &str) -> Result<&GraphTypeCatalog, Error> {
        let id = self.get_graph_type_id(name)?;
        self.graph_type_map
            .get(&id)
            .ok_or(Error::GraphTypeNotExists(name.to_string()))
    }

    pub fn get_graph_type_by_id(&self, id: GraphTypeId) -> Result<&GraphTypeCatalog, Error> {
        self.graph_type_map
            .get(&id)
            .ok_or(Error::GraphNotExists(id.to_string()))
    }

    pub fn delete_graph_type(&mut self, name: &str) -> Result<(), Error> {
        let id = self
            .graph_type_id_map
            .remove(name)
            .ok_or(Error::GraphTypeNotExists(name.to_string()))?;
        self.graph_type_map.remove(&id);
        Ok(())
    }

    // ===== Procedure =====
    pub fn create_procedure(
        &mut self,
        name: String,
        proc: ProcedureCatalog,
    ) -> Result<ProcedureId, Error> {
        if self.procedure_id_map.contains_key(&name) {
            return Err(Error::ProcedureAlreadyExists(name));
        }
        let id = NonZeroU32::new(self.graph_type_id_map.len() as u32 + 1).unwrap();
        self.procedure_id_map.insert(name.clone(), id);
        self.procedure_map.insert(id, proc);
        Ok(id)
    }

    pub fn get_procedure_id(&self, name: &str) -> Result<ProcedureId, Error> {
        self.procedure_id_map
            .get(name)
            .copied()
            .ok_or(Error::ProcedureNotExists(name.to_string()))
    }

    pub fn get_procedure(&self, name: &str) -> Result<&ProcedureCatalog, Error> {
        let id = self.get_procedure_id(name)?;
        self.procedure_map
            .get(&id)
            .ok_or(Error::ProcedureNotExists(name.to_string()))
    }

    pub fn delete_procedure(&mut self, name: &str) -> Result<(), Error> {
        let id = self
            .procedure_id_map
            .remove(name)
            .ok_or(Error::ProcedureNotExists(name.to_string()))?;
        self.procedure_map.remove(&id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use smol_str::ToSmolStr;

    use super::*;
    use crate::graph::GraphCatalog;
    use crate::procedure::ProcedureCatalog;

    #[test]
    fn test_create_and_get_graph() {
        let mut schema = Schema::new("test".to_smolstr());
        let catalog = GraphTypeCatalog::default();

        let id = schema
            .create_graph(
                "g1".to_string(),
                GraphCatalog::new("g1".to_string(), Arc::new(catalog)),
            )
            .unwrap();
        assert_eq!(id.get(), 1);

        let id2 = schema.get_graph_id("g1").unwrap();
        assert_eq!(id, id2);
        // let g = schema.get_graph("g1").unwrap();
        // assert!(g.is_some() || g.is_none() == false);
    }

    #[test]
    fn test_duplicate_graph_insert() {
        let mut schema = Schema::new("test".to_smolstr());
        let catalog = GraphTypeCatalog::default();
        let _ = schema
            .create_graph(
                "g1".to_string(),
                GraphCatalog::new("g1".to_string(), Arc::new(catalog.clone())),
            )
            .unwrap();
        let err = schema
            .create_graph(
                "g1".to_string(),
                GraphCatalog::new("g1".to_string(), Arc::new(catalog.clone())),
            )
            .unwrap_err();
        assert_eq!(err, Error::GraphAlreadyExists("g1".to_string()));
    }

    #[test]
    fn test_delete_graph() {
        let mut schema = Schema::new("test".to_smolstr());
        let catalog = GraphTypeCatalog::default();
        schema
            .create_graph(
                "g1".to_string(),
                GraphCatalog::new("g1".to_string(), Arc::new(catalog.clone())),
            )
            .unwrap();
        schema.delete_graph("g1").unwrap();

        assert!(schema.get_graph_id("g1").is_err());
    }

    #[test]
    fn test_create_and_get_graph_type() {
        let mut schema = Schema::new("test".to_smolstr());
        let catalog = GraphTypeCatalog::default();
        let id = schema
            .create_graph_type("type1".to_string(), catalog)
            .unwrap();
        assert_eq!(id.get(), 1);

        let id2 = schema.get_graph_type_id("type1").unwrap();
        assert_eq!(id, id2);
    }

    #[test]
    fn test_delete_graph_type() {
        let mut schema = Schema::new("test".to_smolstr());
        let catalog = GraphTypeCatalog::default();
        schema
            .create_graph_type("type1".to_string(), catalog)
            .unwrap();
        schema.delete_graph_type("type1").unwrap();
        assert!(schema.get_graph_type("type1").is_err());
    }

    #[test]
    fn test_create_and_get_procedure() {
        let mut schema = Schema::new("test".to_smolstr());
        let id = schema
            .create_procedure("proc1".to_string(), ProcedureCatalog {})
            .unwrap();
        assert_eq!(id.get(), 1);

        let id2 = schema.get_procedure_id("proc1").unwrap();
        assert_eq!(id, id2);
    }

    #[test]
    fn test_delete_procedure() {
        let mut schema = Schema::new("test".to_smolstr());
        schema
            .create_procedure("proc1".to_string(), ProcedureCatalog {})
            .unwrap();
        schema.delete_procedure("proc1").unwrap();
        assert!(schema.get_procedure("proc1").is_err());
    }

    #[test]
    fn test_get_missing_items() {
        let schema = Schema::new("test".to_smolstr());

        assert_eq!(
            schema.get_graph("none").unwrap_err(),
            Error::GraphNotExists("none".to_string())
        );
        assert_eq!(
            schema.get_graph_type("none").unwrap_err(),
            Error::GraphTypeNotExists("none".to_string())
        );
        assert_eq!(
            schema.get_procedure("none").unwrap_err(),
            Error::ProcedureNotExists("none".to_string())
        );
    }
}
