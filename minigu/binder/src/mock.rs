use std::collections::HashMap;
use std::sync::{Arc, Weak};

use minigu_catalog::error::CatalogResult;
use minigu_catalog::label_set::LabelSet;
use minigu_catalog::property::Property;
use minigu_catalog::provider::*;
use minigu_catalog::types::{EdgeTypeId, GraphTypeId, SchemaId, VertexTypeId};
use minigu_common::logical_type::LogicalType;
use minigu_common::types::{GraphId, LabelId, PropertyId};
use serde::Serialize;
use smol_str::ToSmolStr;
use minigu_catalog::provider::DirectoryOrSchema::Directory;
use crate::catalog_ref::Ident;

/// This file defines a mock Catalog metadata structure designed for testing the Binder. The Catalog
/// primarily includes the following components.

/// Schema Directory:
/// /default/a/b
/// In the schema b, there has :
/// {
///   Graph { id: 3, graph_type_ref: graph_type_test }
///   GraphType {
///      id: 2,
///      name: graph_type_test,
///      labelId: label1:2, label2: 3.
///      vertex_type { id: 1, LabelSet: 2, {1 pro_f32, float32, false; 2 pro_i32, int32,
/// true}}              
///      edge_type { id: 1, LabelSet:3, src:1, dst:1, {1 pro_f32, float32, false; 2
/// pro_i32, int32, true}}
/// }
/// }

#[derive(Debug, Serialize)]
pub struct MockDirectoryCatalog {
    name: Ident,
    parent: Option<Weak<MockDirectoryCatalog>>,
    children: HashMap<Ident, DirectoryOrSchema>,
}

#[derive(Debug)]
pub struct MockSchemaCatalog {}

impl DirectoryProvider for MockDirectoryCatalog {
    #[inline]
    fn id(&self) -> SchemaId {
        SchemaId::new(1).unwrap()
    }

    fn parent(&self) -> CatalogResult<Option<Weak<dyn DirectoryProvider>>> {
        Ok(self.parent.clone())
    }

    fn get_directory_or_schema(&self, name: &str) -> CatalogResult<Option<DirectoryOrSchema>> {
        Ok(self.children.get(name).cloned())
    }
}

#[derive(Debug, Serialize)]
pub struct MockCatalog {
    root: Arc<MockDirectoryCatalog>,
}

impl MockCatalog {
    pub fn default() -> Self {
        let schema_b = Arc::new(MockSchemaCatalog {});
        let schema_node  = DirectoryOrSchema::Schema(schema_b.clone());
        
        let mut dir_a = Arc::new(MockDirectoryCatalog {
            name: "a".to_smolstr(),
            parent: None,
            children: HashMap::new(),
        });
        Arc::get_mut(&mut dir_a).unwrap().children.insert(
            "b".to_smolstr(), schema_node,
        );
        
        let mut default = Arc::new(MockDirectoryCatalog {
            name: "default".to_smolstr(),
            parent: None,
            children: HashMap::new(),
        });
        
        Arc::get_mut(&mut default).unwrap().children.insert(
            "a".to_smolstr(), DirectoryOrSchema::Directory(dir_a.clone()),
        );
        
        let root = Arc::new(MockDirectoryCatalog {
            name: "".to_smolstr(),
            parent:None,
            children: HashMap::new(),
        });
        Arc::get_mut(&mut Arc::clone(&root))
            .unwrap()
            .children
            .insert("default".to_smolstr(), DirectoryOrSchema::Directory(default.clone()));
        
        let root_weak = Arc::downgrade(&root);
        Arc::get_mut(&mut default).unwrap().parent = Some(root_weak.clone());
        Arc::get_mut(&mut dir_a).unwrap().parent = Some(Arc::downgrade(&default));
        
        Self  {root}
        
    }
}

impl CatalogProvider for MockCatalog {
    fn get_root(&self) -> CatalogResult<DirectoryOrSchema> {
        Ok(DirectoryOrSchema::Directory(self.root.clone()))
    }

    fn get_directory_or_schema_by_id(
        &self,
        id: SchemaId,
    ) -> CatalogResult<Option<DirectoryOrSchema>> {
        todo!()
    }
}

impl SchemaProvider for MockSchemaCatalog {
    fn id(&self) -> SchemaId {
        SchemaId::new(1).unwrap()
    }

    fn parent(&self) -> Option<SchemaId> {
        Option::from(SchemaId::new(2).unwrap())
    }

    fn get_graph(&self, name: &str) -> CatalogResult<Option<GraphRef>> {
        if name.eq("graph_test") {
            Ok(Some(MockGraph::default().into()))
        } else {
            Ok(None)
        }
    }

    fn get_graph_type(&self, name: &str) -> CatalogResult<Option<GraphTypeRef>> {
        if name.eq("graph_type_test") {
            Ok(Some(MockGraphType::default().into()))
        } else {
            Ok(None)
        }
    }

    // Not used
    fn get_graph_type_by_id(&self, id: GraphTypeId) -> CatalogResult<Option<GraphTypeRef>> {
        Ok(None)
    }

    // Not Used
    fn get_graph_by_id(&self, id: GraphId) -> CatalogResult<Option<GraphRef>> {
        Ok(None)
    }

    fn get_procedure(&self, name: &str) -> CatalogResult<Option<ProcedureRef>> {
        Ok(None)
    }
}

#[derive(Debug, Serialize, Default)]
pub struct MockGraphType;

impl GraphTypeProvider for MockGraphType {
    fn id(&self) -> GraphId {
        GraphId::new(2).unwrap()
    }

    fn get_label_id(&self, name: &str) -> CatalogResult<Option<LabelId>> {
        if name.eq("label1") {
            Ok(Some(LabelId::new(2).unwrap()))
        } else if name.eq("label2") {
            Ok(Some(LabelId::new(3).unwrap()))
        } else {
            Ok(None)
        }
    }

    fn get_vertex_type(&self, key: &LabelSet) -> CatalogResult<Option<VertexTypeRef>> {
        Ok(Some(MockVertexType::default().into()))
    }

    fn get_edge_type(&self, key: &LabelSet) -> CatalogResult<Option<EdgeTypeRef>> {
        Ok(Some(MockEdgeType::default().into()))
    }

    fn get_vertex_type_by_id(&self, id: VertexTypeId) -> CatalogResult<Option<VertexTypeRef>> {
        Ok(None)
    }

    fn get_edge_type_by_id(&self, id: EdgeTypeId) -> CatalogResult<Option<EdgeTypeRef>> {
        Ok(None)
    }
}

#[derive(Debug, Serialize, Default)]
pub struct MockGraph;

impl GraphProvider for MockGraph {
    fn id(&self) -> GraphId {
        GraphId::new(3).unwrap()
    }

    fn graph_type(&self) -> GraphTypeRef {
        MockGraphType::default().into()
    }
}

#[derive(Debug, Serialize, Default)]
pub struct MockVertexType;

impl VertexTypeProvider for MockVertexType {
    fn id(&self) -> VertexTypeId {
        VertexTypeId::new(1).unwrap()
    }

    fn label_set(&self) -> &LabelSet {
        &LabelSet::new((2).map(|i| LabelId::new(i).unwrap()))
    }
}

#[derive(Debug, Serialize, Default)]
pub struct MockEdgeType;

impl EdgeTypeProvider for MockEdgeType {
    fn id(&self) -> EdgeTypeId {
        EdgeTypeId::new(2).unwrap()
    }

    fn label_set(&self) -> &LabelSet {
        &LabelSet::new((3).map(|i| EdgeTypeId::new(i).unwrap()))
    }

    fn src(&self) -> VertexTypeRef {
        MockVertexType::default().into()
    }

    fn dst(&self) -> VertexTypeRef {
        MockVertexType::default().into()
    }
}

fn mock_get_property(name: &str) -> CatalogResult<Option<PropertyRef>> {
    if name.eq("pro_f32") {
        Ok(Some(Arc::new(Property {
            id: 1,
            logical_type: LogicalType::Float32.clone().into(),
            nullable: false,
        })))
    } else if name.eq("pro_i32") {
        Ok(Some(Arc::new(Property {
            id: 2,
            logical_type: LogicalType::Int32.clone().into(),
            nullable: true,
        })))
    } else {
        Ok(None)
    }
}

fn mock_get_property_by_id(id: PropertyId) -> CatalogResult<Option<PropertyRef>> {
    if id == 1 {
        Ok(Some(Arc::new(Property {
            id: 1,
            logical_type: LogicalType::Float32.clone().into(),
            nullable: false,
        })))
    } else if id == 2 {
        Ok(Some(Arc::new(Property {
            id: 2,
            logical_type: LogicalType::Int32.clone().into(),
            nullable: true,
        })))
    } else {
        Ok(None)
    }
}

impl PropertySetProvider for MockEdgeType {
    fn get_property(&self, name: &str) -> CatalogResult<Option<PropertyRef>> {
        mock_get_property(name)
    }

    fn get_property_by_id(&self, id: PropertyId) -> CatalogResult<Option<PropertyRef>> {
        mock_get_property_by_id(id)
    }
}

impl PropertySetProvider for MockVertexType {
    fn get_property(&self, name: &str) -> CatalogResult<Option<PropertyRef>> {
        mock_get_property(name)
    }

    fn get_property_by_id(&self, id: PropertyId) -> CatalogResult<Option<PropertyRef>> {
        mock_get_property_by_id(id)
    }
}
