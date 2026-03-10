use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use minigu_catalog::label_set::LabelSet;
use minigu_catalog::memory::graph_type::{
    MemoryEdgeTypeCatalog, MemoryGraphTypeCatalog, MemoryVertexTypeCatalog,
};
use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_catalog::property::Property;
use minigu_catalog::provider::{
    EdgeTypeProvider, GraphTypeProvider, PropertiesProvider, SchemaProvider, VertexTypeProvider,
};
use minigu_common::types::LabelId;
use minigu_context::graph::GraphContainer;
use serde::{Deserialize, Serialize};

use crate::error::Result;

const CATALOG_FILE: &str = "catalog.json";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DatabaseCatalog {
    pub graphs: HashMap<String, GraphCatalogEntry>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphCatalogEntry {
    pub labels: Vec<LabelDef>,
    pub vertex_types: Vec<VertexTypeDef>,
    pub edge_types: Vec<EdgeTypeDef>,
}

/// A label name with its original ID, so we can restore the exact same mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelDef {
    pub id: u32,
    pub name: String,
}

/// Vertex type definition (label + properties).
#[derive(Debug, Serialize, Deserialize)]
pub struct VertexTypeDef {
    pub label: String,
    pub properties: Vec<Property>,
}

/// Edge type definition (label + src/dst labels + properties).
#[derive(Debug, Serialize, Deserialize)]
pub struct EdgeTypeDef {
    pub label: String,
    pub src_label: String,
    pub dst_label: String,
    pub properties: Vec<Property>,
}

/// Save the database catalog to `<db_path>/catalog.json`.
pub fn save_catalog(db_path: &Path, schema: &MemorySchemaCatalog) -> Result<()> {
    let mut catalog = DatabaseCatalog::default();

    for graph_name in schema.graph_names() {
        if let Ok(Some(graph_ref)) = schema.get_graph(&graph_name) {
            let container = minigu_catalog::provider::GraphProvider::as_any(graph_ref.as_ref())
                .downcast_ref::<GraphContainer>()
                .expect("graph should be a GraphContainer");
            let graph_type = container.graph_type();
            let entry = graph_type_to_catalog_entry(&graph_type);
            catalog.graphs.insert(graph_name, entry);
        }
    }

    let json = serde_json::to_string_pretty(&catalog)?;
    std::fs::write(db_path.join(CATALOG_FILE), json)?;
    Ok(())
}

/// Load the database catalog from `<db_path>/catalog.json`.
/// Returns `None` if the file does not exist.
pub fn load_catalog(db_path: &Path) -> Result<Option<DatabaseCatalog>> {
    let path = db_path.join(CATALOG_FILE);
    if !path.exists() {
        return Ok(None);
    }
    let data = std::fs::read_to_string(path)?;
    let catalog: DatabaseCatalog = serde_json::from_str(&data)?;
    Ok(Some(catalog))
}

/// Extract type definitions from a `MemoryGraphTypeCatalog`.
fn graph_type_to_catalog_entry(graph_type: &MemoryGraphTypeCatalog) -> GraphCatalogEntry {
    // Build reverse map: LabelId → label name
    let label_names = graph_type.label_names();
    let mut id_to_name: HashMap<LabelId, String> = HashMap::new();
    for name in &label_names {
        if let Ok(Some(id)) = graph_type.get_label_id(name) {
            id_to_name.insert(id, name.clone());
        }
    }

    // Save all labels with their IDs
    let mut labels: Vec<LabelDef> = id_to_name
        .iter()
        .map(|(id, name)| LabelDef {
            id: id.get(),
            name: name.clone(),
        })
        .collect();
    labels.sort_by_key(|l| l.id);

    // Extract vertex types
    let mut vertex_types = Vec::new();
    for label_set in graph_type.vertex_type_keys() {
        if let Ok(Some(vt)) = graph_type.get_vertex_type(&label_set) {
            let label_id = label_set.first().expect("vertex should have a label");
            let label = id_to_name
                .get(&label_id)
                .expect("label name should exist")
                .clone();
            let properties: Vec<Property> = vt.properties().into_iter().map(|(_, p)| p).collect();
            vertex_types.push(VertexTypeDef { label, properties });
        }
    }
    // Sort for deterministic output
    vertex_types.sort_by(|a, b| a.label.cmp(&b.label));

    // Extract edge types
    let mut edge_types = Vec::new();
    for label_set in graph_type.edge_type_keys() {
        if let Ok(Some(et)) = graph_type.get_edge_type(&label_set) {
            let label_id = label_set.first().expect("edge should have a label");
            let label = id_to_name
                .get(&label_id)
                .expect("label name should exist")
                .clone();

            let src_label_set = et.src().label_set();
            let src_label_id = src_label_set.first().expect("src should have a label");
            let src_label = id_to_name
                .get(&src_label_id)
                .expect("src label name should exist")
                .clone();

            let dst_label_set = et.dst().label_set();
            let dst_label_id = dst_label_set.first().expect("dst should have a label");
            let dst_label = id_to_name
                .get(&dst_label_id)
                .expect("dst label name should exist")
                .clone();

            let properties: Vec<Property> = et.properties().into_iter().map(|(_, p)| p).collect();
            edge_types.push(EdgeTypeDef {
                label,
                src_label,
                dst_label,
                properties,
            });
        }
    }
    edge_types.sort_by(|a, b| a.label.cmp(&b.label));

    GraphCatalogEntry {
        labels,
        vertex_types,
        edge_types,
    }
}

/// Reconstruct a `MemoryGraphTypeCatalog` from a `GraphCatalogEntry`.
pub fn catalog_entry_to_graph_type(entry: &GraphCatalogEntry) -> Arc<MemoryGraphTypeCatalog> {
    let mut graph_type = MemoryGraphTypeCatalog::new();

    // Add all labels in ID order so they get the same IDs as the original.
    // `add_label` assigns sequential IDs starting from 1, so adding in sorted
    // ID order reproduces the original mapping.
    let mut sorted_labels = entry.labels.clone();
    sorted_labels.sort_by_key(|l| l.id);
    for label_def in &sorted_labels {
        let label_id = graph_type
            .add_label(label_def.name.clone())
            .expect("add label failed");
        debug_assert_eq!(label_id.get(), label_def.id, "label ID mismatch on restore");
    }

    // Build label name → vertex type map for edge type src/dst references
    let mut label_vertex_type: HashMap<String, Arc<MemoryVertexTypeCatalog>> = HashMap::new();

    // Create vertex types
    for vt_def in &entry.vertex_types {
        let label_id = graph_type
            .get_label_id(&vt_def.label)
            .expect("get label failed")
            .expect("label should exist");
        let label_set = LabelSet::from_iter(vec![label_id]);
        let vertex_type = Arc::new(MemoryVertexTypeCatalog::new(
            label_set.clone(),
            vt_def.properties.clone(),
        ));
        graph_type.add_vertex_type(label_set, Arc::clone(&vertex_type));
        label_vertex_type.insert(vt_def.label.clone(), vertex_type);
    }

    // Create edge types
    for et_def in &entry.edge_types {
        let label_id = graph_type
            .get_label_id(&et_def.label)
            .expect("get label failed")
            .expect("label should exist");
        let label_set = LabelSet::from_iter(vec![label_id]);

        let src_type = label_vertex_type
            .get(&et_def.src_label)
            .expect("src vertex type not found")
            .clone();
        let dst_type = label_vertex_type
            .get(&et_def.dst_label)
            .expect("dst vertex type not found")
            .clone();

        let edge_type = MemoryEdgeTypeCatalog::new(
            label_set.clone(),
            src_type,
            dst_type,
            et_def.properties.clone(),
        );
        graph_type.add_edge_type(label_set, Arc::new(edge_type));
    }

    Arc::new(graph_type)
}

/// Get the `.minigu` file path for a graph.
pub fn graph_data_path(db_path: &Path, graph_name: &str) -> std::path::PathBuf {
    db_path.join(format!("{}.minigu", graph_name))
}
