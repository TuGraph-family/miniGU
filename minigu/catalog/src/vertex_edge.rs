use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::Arc;
use crate::types::{Ident, LabelId, PropertyId};
use minigu_common::logical_type::LogicalType;
use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property {
    data_type: LogicalType,
    is_optional: bool,
    is_unique: bool,
    // support default value?
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EdgeTypeCatalog {
    name: Ident,
    property_id_map: HashMap<Ident, PropertyId>,
    properties: HashMap<PropertyId, Property>,
    src: Arc<VertexTypeCatalog>,
    dist: Arc<VertexTypeCatalog>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VertexTypeCatalog {
    name: Ident,
    property_id_map: HashMap<Ident, PropertyId>,
    properties: HashMap<PropertyId, Property>
}

pub trait PropertyCatalog {
    fn property_id_map(&self) -> &HashMap<Ident, PropertyId>;
    fn property_id_map_mut(&mut self) -> &mut HashMap<Ident, PropertyId>;
    fn properties(&self) -> &HashMap<PropertyId, Property>;
    fn properties_mut(&mut self) -> &mut HashMap<PropertyId, Property>;

    fn add_property(
        &mut self,
        name: Ident,
        prop: Property,
    ) -> Result<(), Error> {
        if self.property_id_map().contains_key(&name) {
            return Err(Error::PropertyAlreadyExists(name.to_string()));
        }
        let id = NonZeroU32::new(self.property_id_map().len() as u32 + 1)
            .unwrap();
        self.property_id_map_mut().insert(name.clone(), id);
        self.properties_mut().insert(id, prop);
        Ok(())
    }

    fn get_property_by_name(&self, name: &Ident) -> Result<&Property, Error> {
        self.property_id_map()
            .get(name)
            .and_then(|id| self.properties().get(id))
            .ok_or_else(|| Error::PropertyNotExists(name.to_string()))
    }

    fn get_property_by_id(&self, id: PropertyId) -> Result<&Property, Error> {
        self.properties()
            .get(&id)
            .ok_or_else(|| Error::PropertyNotExists(format!("id={}", id)))
    }

    fn update_property(&mut self, id: PropertyId, new_prop: Property) -> Result<(), Error> {
        if !self.properties().contains_key(&id) {
            return Err(Error::PropertyNotExists(format!("id={}", id)));
        }
        self.properties_mut().insert(id, new_prop);
        Ok(())
    }

    fn remove_property_by_name(&mut self, name: &Ident) -> Result<(), Error> {
        let id = self
            .property_id_map_mut()
            .remove(name)
            .ok_or_else(|| Error::PropertyNotExists(name.to_string()))?;

        self.properties_mut().remove(&id);
        Ok(())
    }

    fn remove_property_by_id(&mut self, id: PropertyId) -> Result<(), Error> {
        if self.properties_mut().remove(&id).is_none() {
            return Err(Error::PropertyNotExists(format!("id={}", id)));
        }
        self.property_id_map_mut().retain(|_, v| *v != id);
        Ok(())
    }
}

impl PropertyCatalog for VertexTypeCatalog {
    fn property_id_map(&self) -> &HashMap<Ident, PropertyId> {
        &self.property_id_map
    }

    fn property_id_map_mut(&mut self) -> &mut HashMap<Ident, PropertyId> {
        &mut self.property_id_map
    }

    fn properties(&self) -> &HashMap<PropertyId, Property> {
        &self.properties
    }

    fn properties_mut(&mut self) -> &mut HashMap<PropertyId, Property> {
        &mut self.properties
    }
}

impl PropertyCatalog for EdgeTypeCatalog {
    fn property_id_map(&self) -> &HashMap<Ident, PropertyId> {
        &self.property_id_map
    }

    fn property_id_map_mut(&mut self) -> &mut HashMap<Ident, PropertyId> {
        &mut self.property_id_map
    }

    fn properties(&self) -> &HashMap<PropertyId, Property> {
        &self.properties
    }

    fn properties_mut(&mut self) -> &mut HashMap<PropertyId, Property> {
        &mut self.properties
    }
}


impl VertexTypeCatalog {
    pub fn new(name: Ident) -> Self {
        Self {
            name,
            property_id_map: HashMap::new(),
            properties: HashMap::new(),
        }
    }
}

impl EdgeTypeCatalog {
    pub fn new(
        name: Ident,
        src: Arc<VertexTypeCatalog>,
        dist: Arc<VertexTypeCatalog>,
    ) -> Self {
        Self {
            name,
            property_id_map: HashMap::new(),
            properties: HashMap::new(),
            src,
            dist,
        }
    }
    pub fn src_types(&self) -> &VertexTypeCatalog {
        &self.src
    }

    pub fn dist_type(&self) -> &VertexTypeCatalog {
        &self.dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use minigu_common::logical_type::LogicalType;

    fn dummy_property(dt: LogicalType) -> Property {
        Property {
            data_type: dt,
            is_optional: false,
            is_unique: false,
        }
    }

    fn dummy_ident(name: &str) -> Ident {
        name.into()
    }

    fn dummy_vertex(name: &str) -> VertexTypeCatalog {
        VertexTypeCatalog::new(dummy_ident(name))
    }

    #[test]
    fn test_vertex_property_crud() {
        let mut vertex = dummy_vertex( "Person");
        let pid = NonZeroU32::new(1).unwrap();
        let pname = dummy_ident("age");
        let prop = dummy_property(LogicalType::Int8);

        // add
        vertex.add_property(pname.clone(), prop).unwrap();

        // get by name
        let p = vertex.get_property_by_name(&pname).unwrap();
        assert_eq!(p.data_type, LogicalType::Int8);

        // get by id
        let p2 = vertex.get_property_by_id(pid).unwrap();
        assert_eq!(p2.data_type, LogicalType::Int8);

        // update
        vertex
            .update_property(pid, dummy_property(LogicalType::Float32))
            .unwrap();
        let updated = vertex.get_property_by_id(pid).unwrap();
        assert_eq!(updated.data_type, LogicalType::Float32);

        // remove by name
        vertex.remove_property_by_name(&pname).unwrap();
        assert!(vertex.get_property_by_name(&pname).is_err());
    }

    #[test]
    fn test_vertex_duplicate_property_error() {
        let mut vertex = dummy_vertex( "Book");
        let pid = 200;
        let pname = dummy_ident("title");
        let prop = dummy_property(LogicalType::String);

        vertex.add_property(pname.clone(), prop.clone()).unwrap();
        let err = vertex.add_property(pname.clone(), prop).unwrap_err();
        assert_eq!(err, Error::PropertyAlreadyExists("title".to_string()));
    }

    #[test]
    fn test_edge_property_crud() {
        let src = Arc::new(dummy_vertex("Person"));
        let dst = Arc::new(dummy_vertex("Movie"));
        let mut edge = EdgeTypeCatalog::new(dummy_ident("LIKES"), src, dst);

        let pid = NonZeroU32::new(1).unwrap();
        let pname = dummy_ident("weight");
        let prop = dummy_property(LogicalType::Float32);

        edge.add_property(pname.clone(), prop).unwrap();
        assert_eq!(
            edge.get_property_by_name(&pname).unwrap().data_type,
            LogicalType::Float32
        );

        edge.remove_property_by_id(pid).unwrap();
        assert!(edge.get_property_by_id(pid).is_err());
    }

    #[test]
    fn test_edge_src_and_dst() {
        let src = Arc::new(dummy_vertex( "User"));
        let dst = Arc::new(dummy_vertex( "Product"));
        let edge = EdgeTypeCatalog::new( dummy_ident("BUYS"), src.clone(), dst.clone());

        assert_eq!(edge.dist_type().name, dst.name);
    }
}



