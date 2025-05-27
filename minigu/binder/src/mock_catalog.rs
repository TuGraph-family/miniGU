use std::collections::HashMap;
use std::sync::{Arc, RwLock, Weak};

use minigu_catalog::error::CatalogResult;
use minigu_catalog::label_set::LabelSet;
use minigu_catalog::property::Property;
use minigu_catalog::provider::{
    CatalogProvider, DirectoryOrSchema, DirectoryProvider, DirectoryRef, EdgeTypeProvider,
    EdgeTypeRef, GraphProvider, GraphRef, GraphTypeProvider, GraphTypeRef, ProcedureProvider,
    ProcedureRef, PropertiesProvider, SchemaProvider, VertexTypeProvider, VertexTypeRef,
};
use minigu_common::data_type::{DataField, DataSchema, DataSchemaRef, LogicalType};
use minigu_common::types::{LabelId, PropertyId};
use serde::Serialize;

use crate::types::Ident;

/// This file defines a mock Catalog metadata structure designed for testing the Binder. The Catalog
/// primarily includes the following components.
/// Schema Directory:
/// /default/a/b
/// In the schema b, there has :
/// {
///   Graph { graph_type_ref: graph_type_test }
///   GraphType {
///      name: graph_type_test,
///      labelId: label1:2, label2: 3.
///      vertex_type { LabelSet: 2}
///      edge_type { LabelSet:3, src:2, dst:2,}
/// },
///  Procedure {
///     name: procedure_test
///     parameters {float32, float64}
///     schema { (time, int32, false), (value, float32, false)}
/// }

#[derive(Debug)]
pub struct MockDirectory {
    parent: Option<Weak<dyn DirectoryProvider>>,
    children: RwLock<HashMap<Ident, DirectoryOrSchema>>,
}

#[derive(Debug)]
pub struct MockCatalog {
    root: Arc<MockDirectory>,
}

#[derive(Debug)]
pub struct MockSchema {
    parent: Option<Weak<dyn DirectoryProvider>>,
}

impl MockSchema {
    pub fn new(parent: Option<Weak<dyn DirectoryProvider>>) -> Self {
        Self { parent }
    }
}

#[derive(Debug)]
pub struct MockProcedure {}

impl MockDirectory {
    #[inline]
    pub fn new(parent: Option<std::sync::Weak<dyn DirectoryProvider>>) -> Self {
        Self {
            parent,
            children: HashMap::new().into(),
        }
    }
}

impl DirectoryProvider for MockDirectory {
    fn parent(&self) -> Option<DirectoryRef> {
        self.parent.clone().and_then(|p| p.upgrade())
    }

    fn get_child(&self, name: &str) -> CatalogResult<Option<DirectoryOrSchema>> {
        Ok(self.children.read().unwrap().get(name).cloned())
    }

    fn children_names(&self) -> Vec<String> {
        self.children
            .read()
            .unwrap()
            .keys()
            .cloned()
            .map(|k| k.to_string())
            .collect()
    }
}

fn weak_parent<T: DirectoryProvider + 'static>(
    arc: &Arc<T>,
) -> Option<Weak<dyn DirectoryProvider>> {
    Some(Arc::downgrade(&(arc.clone() as Arc<dyn DirectoryProvider>)))
}

pub fn build_mock_catalog() -> Arc<MockDirectory> {
    let root = Arc::new(MockDirectory::new(None));
    let root_dyn: Arc<dyn DirectoryProvider> = root.clone();

    let default = Arc::new(MockDirectory::new(weak_parent(&root)));
    root.children.write().unwrap().insert(
        "default".into(),
        DirectoryOrSchema::Directory(default.clone()),
    );

    let a = Arc::new(MockDirectory::new(weak_parent(&default)));
    default
        .children
        .write()
        .unwrap()
        .insert("a".into(), DirectoryOrSchema::Directory(a.clone()));

    let b = Arc::new(MockSchema::new(weak_parent(&a)));
    a.children
        .write()
        .unwrap()
        .insert("b".into(), DirectoryOrSchema::Schema(b.clone()));

    root
}
impl MockCatalog {
    pub fn default() -> Self {
        let root = build_mock_catalog();
        MockCatalog { root }
    }
}

impl CatalogProvider for MockCatalog {
    fn get_root(&self) -> CatalogResult<DirectoryOrSchema> {
        Ok(DirectoryOrSchema::Directory(self.root.clone()))
    }
}

impl SchemaProvider for MockSchema {
    fn parent(&self) -> Option<DirectoryRef> {
        self.parent.clone().and_then(|p| p.upgrade())
    }

    fn graph_names(&self) -> Vec<String> {
        todo!()
    }

    fn get_graph(&self, name: &str) -> CatalogResult<Option<GraphRef>> {
        if name.eq("graph_test") {
            let graph: GraphRef = Arc::new(MockGraph {});
            Ok(Some(graph))
        } else {
            Ok(None)
        }
    }

    fn graph_type_names(&self) -> Vec<String> {
        todo!()
    }

    fn get_graph_type(&self, name: &str) -> CatalogResult<Option<GraphTypeRef>> {
        if name.eq("graph_type_test") {
            let graph_type: GraphTypeRef = Arc::new(MockGraphType {});
            Ok(Some(graph_type))
        } else {
            Ok(None)
        }
    }

    fn procedure_names(&self) -> Vec<String> {
        todo!()
    }

    fn get_procedure(&self, name: &str) -> CatalogResult<Option<ProcedureRef>> {
        if name.eq("procedure_test") {
            Ok(Some(Arc::new(MockProcedure {})))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct MockGraphType;

impl GraphTypeProvider for MockGraphType {
    fn get_label_id(&self, name: &str) -> CatalogResult<Option<LabelId>> {
        if name.eq("label1") {
            Ok(Some(LabelId::new(2).unwrap()))
        } else if name.eq("label2") {
            Ok(Some(LabelId::new(3).unwrap()))
        } else {
            Ok(None)
        }
    }

    fn label_names(&self) -> Vec<String> {
        todo!()
    }

    fn get_vertex_type(&self, key: &LabelSet) -> CatalogResult<Option<VertexTypeRef>> {
        Ok(Some(Arc::new(MockVertexType {})))
    }

    fn vertex_type_keys(&self) -> Vec<LabelSet> {
        todo!()
    }

    fn get_edge_type(&self, key: &LabelSet) -> CatalogResult<Option<EdgeTypeRef>> {
        Ok(Some(Arc::new(MockEdgeType {})))
    }

    fn edge_type_keys(&self) -> Vec<LabelSet> {
        todo!()
    }
}

#[derive(Debug, Serialize, Default)]
pub struct MockGraph;

impl GraphProvider for MockGraph {
    fn graph_type(&self) -> GraphTypeRef {
        Arc::new(MockGraphType {})
    }
}

#[derive(Debug, Serialize, Default)]
pub struct MockVertexType;

impl VertexTypeProvider for MockVertexType {
    fn label_set(&self) -> LabelSet {
        LabelSet::from_iter([LabelId::new(2).unwrap()])
    }
}

#[derive(Debug, Serialize, Default)]
pub struct MockEdgeType;

impl EdgeTypeProvider for MockEdgeType {
    fn label_set(&self) -> LabelSet {
        LabelSet::from_iter([LabelId::new(2).unwrap()])
    }

    fn src(&self) -> VertexTypeRef {
        Arc::new(MockVertexType {})
    }

    fn dst(&self) -> VertexTypeRef {
        Arc::new(MockVertexType {})
    }
}

impl PropertiesProvider for MockEdgeType {
    fn get_property(&self, name: &str) -> CatalogResult<Option<(PropertyId, &Property)>> {
        todo!()
    }

    fn properties(&self) -> Vec<(PropertyId, Property)> {
        todo!()
    }
}

impl PropertiesProvider for MockVertexType {
    fn get_property(&self, name: &str) -> CatalogResult<Option<(PropertyId, &Property)>> {
        todo!()
    }

    fn properties(&self) -> Vec<(PropertyId, Property)> {
        todo!()
    }
}

impl ProcedureProvider for MockProcedure {
    fn parameters(&self) -> &[LogicalType] {
        &[LogicalType::Float32, LogicalType::Float64]
    }

    fn schema(&self) -> Option<DataSchemaRef> {
        Some(Arc::new(DataSchema::new(vec![
            DataField::new("t1".to_string(), LogicalType::Int32, false),
            DataField::new("t2".to_string(), LogicalType::Float32, false),
        ])))
    }
}
