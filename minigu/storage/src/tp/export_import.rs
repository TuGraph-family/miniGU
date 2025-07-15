//! Graph import/export utilities for `MemoryGraph`
//! # File layout produced by `export_graph`
//!
//! ```text
//! <output‑dir>/
//! ├── person.csv        #  vertex records labelled "person"
//! ├── friend.csv        #  edge records labelled "friend"
//! ├── follow.csv        #  edge records labelled "follow"
//! └── manifest.json       #  manifest generated from `Manifest`
//! ```
//!
//! Each vertex CSV row encodes
//!
//! ```csv
//! <vid>,<prop‑1>,<prop‑2>, ...
//! ```
//!
//! while edges are encoded as
//!
//! ```csv
//! <eid>,<src‑vid>,<dst‑vid>,<prop‑1>,<prop‑2>, ...
//! ```

use core::error::Error;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use csv::{ReaderBuilder, Writer, WriterBuilder};
use minigu_catalog::label_set::LabelSet;
use minigu_catalog::property::Property;
use minigu_catalog::provider::GraphTypeProvider;
use minigu_common::data_type::LogicalType;
use minigu_common::types::{EdgeId, LabelId, VertexId};
use minigu_common::value::ScalarValue;
use serde::{Deserialize, Serialize};

use crate::common::{Edge, PropertyRecord, Vertex};
use crate::tp::{MemoryGraph, TransactionHandle};

type RecordType = Vec<String>;

/// Cached lookup information derived from `GraphTypeProvider`.
#[derive(Debug)]
struct SchemaMetadata {
    label_map: HashMap<LabelId, String>,
    vertex_labels: HashSet<LabelId>,
    edge_infos: HashMap<LabelId, (LabelId, LabelId)>,
    schema: Arc<dyn GraphTypeProvider>,
}

impl SchemaMetadata {
    fn from_schema(graph_type: Arc<dyn GraphTypeProvider>) -> Self {
        let label_map = graph_type
            .label_names()
            .iter()
            .map(|label| {
                let label_id = graph_type.get_label_id(label).unwrap().unwrap();
                (label_id, label.clone())
            })
            .collect::<HashMap<_, _>>();

        let mut vertex_labels = HashSet::new();
        let mut v_lset_to_label = HashMap::new();
        let mut edge_infos = HashMap::new();
        for (&id, _) in label_map.iter() {
            let label_set = LabelSet::from_iter(vec![id]);

            if let Some(edge_type) = graph_type.get_edge_type(&label_set).unwrap() {
                let src_label_set = edge_type.src().label_set();
                let dst_label_set = edge_type.dst().label_set();

                edge_infos.insert(id, (src_label_set, dst_label_set));
            } else {
                vertex_labels.insert(id);
                v_lset_to_label.insert(label_set, id);
            }
        }

        let edge_infos = edge_infos
            .iter()
            .map(|(&id, (src, dst))| {
                let src_id = *v_lset_to_label.get(src).unwrap();
                let dst_id = *v_lset_to_label.get(dst).unwrap();

                (id, (src_id, dst_id))
            })
            .collect();

        Self {
            label_map,
            vertex_labels,
            edge_infos,
            schema: Arc::clone(&graph_type),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct FileSpec {
    path: String,   // relative path
    format: String, // currently always "csv"
}

impl FileSpec {
    pub fn new(path: String, format: String) -> Self {
        Self { path, format }
    }
}

/// Common metadata for a vertex or edge collection.
#[derive(Deserialize, Serialize, Debug)]
pub struct VertexSpec {
    label: String,
    file: FileSpec,
    properties: Vec<Property>,
}

impl VertexSpec {
    pub fn label_name(&self) -> &String {
        &self.label
    }

    pub fn properties(&self) -> &Vec<Property> {
        &self.properties
    }

    fn new(label: String, file: FileSpec, properties: Vec<Property>) -> Self {
        Self {
            label,
            file,
            properties,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EdgeSpec {
    label: String,
    src_label: String,
    dst_label: String,
    file: FileSpec,
    properties: Vec<Property>,
}

impl EdgeSpec {
    fn new(
        label: String,
        src_label: String,
        dst_label: String,
        file: FileSpec,
        properties: Vec<Property>,
    ) -> Self {
        Self {
            label,
            src_label,
            dst_label,
            file,
            properties,
        }
    }

    pub fn src_label(&self) -> &String {
        &self.src_label
    }

    pub fn dst_label(&self) -> &String {
        &self.dst_label
    }

    pub fn label_name(&self) -> &String {
        &self.label
    }

    pub fn properties(&self) -> &Vec<Property> {
        &self.properties
    }
}

// Top-level manifest written to disk.
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Manifest {
    vertices: Vec<VertexSpec>,
    edges: Vec<EdgeSpec>,
}

impl Manifest {
    fn from_schema(metadata: SchemaMetadata) -> Self {
        let props_for_vertex = |id: LabelId| {
            let label_set = LabelSet::from_iter(vec![id]);
            metadata
                .schema
                .get_vertex_type(&label_set) // will return None for vertex (inverse call later)
                .unwrap()
                .unwrap()
                .properties()
                .into_iter()
                .map(|prop| prop.1) // drop index key
                .collect::<Vec<_>>()
        };
        let props_for_edge = |id: LabelId| {
            let label_set = LabelSet::from_iter(vec![id]);
            metadata
                .schema
                .get_edge_type(&label_set) // will return None for edges (inverse call later)
                .unwrap()
                .unwrap()
                .properties()
                .into_iter()
                .map(|prop| prop.1) // drop index key
                .collect::<Vec<_>>()
        };

        let vertex_specs = metadata
            .vertex_labels
            .iter()
            .map(|&id| {
                let name = metadata.label_map.get(&id).unwrap();
                let path = format!("{}.csv", name);
                let props_schema = props_for_vertex(id);

                VertexSpec::new(
                    name.clone(),
                    FileSpec::new(path, "csv".to_string()),
                    props_schema,
                )
            })
            .collect();

        let edge_specs = metadata
            .edge_infos
            .iter()
            .map(|(&id, (src_id, dst_id))| {
                let name = metadata.label_map.get(&id).unwrap();
                let path = format!("{}.csv", name);
                let props_schema = props_for_edge(id);
                let src_label = metadata.label_map.get(src_id).unwrap().clone();
                let dst_label = metadata.label_map.get(dst_id).unwrap().clone();

                EdgeSpec::new(
                    name.clone(),
                    src_label,
                    dst_label,
                    FileSpec::new(path, "csv".to_string()),
                    props_schema,
                )
            })
            .collect();

        Self {
            vertices: vertex_specs,
            edges: edge_specs,
        }
    }

    pub fn vertices_spec(&self) -> &Vec<VertexSpec> {
        &self.vertices
    }

    pub fn edges_spec(&self) -> &Vec<EdgeSpec> {
        &self.edges
    }
}

impl FromStr for Manifest {
    type Err = Box<dyn Error + Send + Sync + 'static>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

/// Convert a *string* coming from CSV into an owned [`ScalarValue`] according
/// to a given property definition.
fn property_to_scalar_value(property: &Property, value: &str) -> ScalarValue {
    match property.logical_type() {
        LogicalType::Int8 => ScalarValue::Int8(value.parse().ok()),
        LogicalType::Int16 => ScalarValue::Int16(value.parse().ok()),
        LogicalType::Int32 => ScalarValue::Int32(value.parse().ok()),
        LogicalType::Int64 => ScalarValue::Int64(value.parse().ok()),
        LogicalType::UInt8 => ScalarValue::UInt8(value.parse().ok()),
        LogicalType::UInt16 => ScalarValue::UInt16(value.parse().ok()),
        LogicalType::UInt32 => ScalarValue::UInt32(value.parse().ok()),
        LogicalType::UInt64 => ScalarValue::UInt64(value.parse().ok()),
        LogicalType::Boolean => ScalarValue::Boolean(value.parse().ok()),
        LogicalType::Float32 => ScalarValue::Float32(value.parse().ok()),
        LogicalType::Float64 => ScalarValue::Float64(value.parse().ok()),
        LogicalType::String => ScalarValue::String(Some(value.to_string())),
        _ => todo!(),
    }
}

/// Convert a [`ScalarValue`] back into a *CSV‑ready* string. `NULL` becomes an
/// empty string.
fn scalar_value_to_string(scalar_value: &ScalarValue) -> String {
    match scalar_value {
        ScalarValue::Int8(value) => value.map_or(String::new(), |inner| inner.to_string()),
        ScalarValue::Int16(value) => value.map_or(String::new(), |inner| inner.to_string()),
        ScalarValue::Int32(value) => value.map_or(String::new(), |inner| inner.to_string()),
        ScalarValue::Int64(value) => value.map_or(String::new(), |inner| inner.to_string()),
        ScalarValue::UInt8(value) => value.map_or(String::new(), |inner| inner.to_string()),
        ScalarValue::UInt16(value) => value.map_or(String::new(), |inner| inner.to_string()),
        ScalarValue::UInt32(value) => value.map_or(String::new(), |inner| inner.to_string()),
        ScalarValue::UInt64(value) => value.map_or(String::new(), |inner| inner.to_string()),
        ScalarValue::Boolean(value) => value.map_or(String::new(), |inner| inner.to_string()),
        ScalarValue::Float32(value) => value.map_or(String::new(), |inner| inner.to_string()),
        ScalarValue::Float64(value) => value.map_or(String::new(), |inner| inner.to_string()),
        ScalarValue::String(value) => value.clone().unwrap_or_default(),
        _ => todo!(),
    }
}

#[derive(Debug)]
struct VerticesBuilder {
    records: HashMap<LabelId, BTreeMap<VertexId, RecordType>>,
    writers: HashMap<LabelId, Writer<File>>,
}

impl VerticesBuilder {
    fn new<P: AsRef<Path>>(dir: P, map: &HashMap<LabelId, String>) -> Self {
        let writers = map
            .iter()
            .map(|(id, label)| {
                let filename = format!("{}.csv", label);
                let path = dir.as_ref().join(filename);

                (*id, WriterBuilder::new().from_path(path).unwrap())
            })
            .collect();
        Self {
            records: HashMap::new(),
            writers,
        }
    }

    fn add_vertex(&mut self, v: &Vertex) {
        let record = std::iter::once(v.vid().to_string())
            .chain(v.properties().iter().map(scalar_value_to_string))
            .collect::<Vec<_>>();

        self.records
            .entry(v.label_id)
            .or_default()
            .insert(v.vid(), record);
    }

    fn dump(&mut self) {
        for (label_id, records) in self.records.iter() {
            let w = self.writers.get_mut(label_id).unwrap();

            for (_, record) in records.iter() {
                w.write_record(record).unwrap();
            }
        }
    }
}

#[derive(Debug)]
struct EdgesBuilder {
    records: HashMap<LabelId, BTreeMap<EdgeId, RecordType>>,
    writers: HashMap<LabelId, Writer<File>>,
}

impl EdgesBuilder {
    fn new<P: AsRef<Path>>(dir: P, map: &HashMap<LabelId, String>) -> Self {
        let writers = map
            .iter()
            .map(|(id, label)| {
                let filename = format!("{}.csv", label);
                let path = dir.as_ref().join(filename);

                (*id, WriterBuilder::new().from_path(path).unwrap())
            })
            .collect();

        Self {
            records: HashMap::new(),
            writers,
        }
    }

    pub fn add_edge(&mut self, e: &Edge) {
        let prefix = [
            e.eid().to_string(),
            e.src_id().to_string(),
            e.dst_id().to_string(),
        ];
        let record = prefix
            .into_iter()
            .chain(e.properties().iter().map(scalar_value_to_string))
            .collect::<Vec<_>>();

        self.records
            .entry(e.label_id)
            .or_default()
            .insert(e.eid(), record);
    }

    pub fn dump(&mut self) {
        for (label_id, records) in self.records.iter() {
            let w = self.writers.get_mut(label_id).unwrap();

            for (_, record) in records.iter() {
                w.write_record(record).unwrap();
            }
        }
    }
}

impl MemoryGraph {
    pub fn import_graph<P: AsRef<Path>>(
        &self,
        txn: &TransactionHandle,
        manifest_path: P,
        graph_type: Arc<dyn GraphTypeProvider>,
    ) {
        // 1. Reading manifest
        let manifest_str = std::fs::read_to_string(manifest_path.as_ref()).unwrap();
        let manifest: Manifest = serde_json::from_str(&manifest_str).unwrap();

        // 2. Vertices
        let mut vid = 1;
        for vertex_spec in manifest.vertices.iter() {
            let mut rdr = ReaderBuilder::new()
                .has_headers(false)
                .from_path(
                    manifest_path
                        .as_ref()
                        .parent()
                        .unwrap()
                        .join(&vertex_spec.file.path),
                )
                .unwrap();

            let label_id = graph_type
                .get_label_id(&vertex_spec.label)
                .unwrap()
                .unwrap();

            for record in rdr.records() {
                let label_set = LabelSet::from_iter(vec![label_id]);
                let props_schema = graph_type
                    .get_vertex_type(&label_set)
                    .unwrap()
                    .unwrap()
                    .properties();

                let record = record.unwrap();

                assert_eq!(props_schema.len() + 1, record.len());

                let props = props_schema
                    .iter()
                    .zip(record.iter().skip(1)) // skip vid
                    .map(|((_, p), value)| property_to_scalar_value(p, value))
                    .collect();
                let vertex = Vertex::new(vid, label_id, PropertyRecord::new(props));

                self.create_vertex(txn, vertex).unwrap();
                vid += 1;
            }
        }

        // 3. Edges
        let mut eid = 1;
        for edge_spec in manifest.edges.iter() {
            let label_id = graph_type.get_label_id(&edge_spec.label).unwrap().unwrap();

            let mut rdr = ReaderBuilder::new()
                .has_headers(false)
                .from_path(
                    manifest_path
                        .as_ref()
                        .parent()
                        .unwrap()
                        .join(&edge_spec.file.path),
                )
                .unwrap();

            for record in rdr.records() {
                let label_set = LabelSet::from_iter(vec![label_id]);

                let props = graph_type
                    .get_edge_type(&label_set)
                    .unwrap()
                    .unwrap()
                    .properties();

                let record = record.unwrap();

                assert_eq!(record.len() - 3, props.len());
                let src_id = record.get(1).unwrap().parse().unwrap();
                let dst_id = record.get(2).unwrap().parse().unwrap();

                let props = props
                    .iter()
                    .zip(record.iter().skip(3)) // skip eid, src and dst
                    .map(|((_, p), value)| property_to_scalar_value(p, value))
                    .collect();

                let edge = Edge::new(eid, src_id, dst_id, label_id, PropertyRecord::new(props));
                self.create_edge(txn, edge).unwrap();
                eid += 1;
            }
        }
    }

    pub fn export_graph<P: AsRef<Path>>(
        &self,
        txn: &TransactionHandle,
        dir: P,
        manifest_rel_path: P, // relative path
        graph_type: Arc<dyn GraphTypeProvider>,
    ) {
        // 1. Prepare output paths
        let dir = dir.as_ref();
        std::fs::create_dir_all(dir).unwrap();

        let metadata = SchemaMetadata::from_schema(Arc::clone(&graph_type));

        let mut vertice_builder = VerticesBuilder::new(dir, &metadata.label_map);
        let mut edges_builder = EdgesBuilder::new(dir, &metadata.label_map);

        // 2. Dump vertices
        for v in txn.iter_vertices() {
            vertice_builder.add_vertex(&v.unwrap());
        }
        vertice_builder.dump();

        // 3. Dump edge
        for e in txn.iter_edges() {
            edges_builder.add_edge(&e.unwrap());
        }
        edges_builder.dump();

        // 4. Dump manifest
        let manifest = Manifest::from_schema(metadata);
        std::fs::write(
            dir.join(manifest_rel_path),
            serde_json::to_string(&manifest).unwrap(),
        )
        .unwrap();
    }
}

#[cfg(test)]
mod test {
    use minigu_catalog::memory::graph_type::{
        MemoryEdgeTypeCatalog, MemoryGraphTypeCatalog, MemoryVertexTypeCatalog,
    };
    use walkdir::WalkDir;

    use super::*;
    use crate::tp::IsolationLevel;
    use crate::tp::memory_graph::tests::*;

    fn mock_graph_type() -> MemoryGraphTypeCatalog {
        let mut graph_type = MemoryGraphTypeCatalog::new();
        let person_id = graph_type.add_label("person".to_string()).unwrap();
        let friend_id = graph_type.add_label("friend".to_string()).unwrap();
        let follow_id = graph_type.add_label("follow".to_string()).unwrap();

        let person_label_set = LabelSet::from_iter([person_id]);
        let friend_label_set = LabelSet::from_iter([friend_id]);
        let follow_label_set = LabelSet::from_iter([follow_id]);

        let vertex_type = Arc::new(MemoryVertexTypeCatalog::new(
            person_label_set.clone(),
            vec![
                Property::new("name".to_string(), LogicalType::String, false),
                Property::new("age".to_string(), LogicalType::Int32, false),
            ],
        ));

        graph_type.add_vertex_type(person_label_set, vertex_type.clone());
        graph_type.add_edge_type(
            friend_label_set.clone(),
            Arc::new(MemoryEdgeTypeCatalog::new(
                friend_label_set,
                vertex_type.clone(),
                vertex_type.clone(),
                vec![Property::new(
                    "date".to_string(),
                    LogicalType::String,
                    false,
                )],
            )),
        );
        graph_type.add_edge_type(
            follow_label_set.clone(),
            Arc::new(MemoryEdgeTypeCatalog::new(
                follow_label_set,
                vertex_type.clone(),
                vertex_type.clone(),
                vec![Property::new(
                    "date".to_string(),
                    LogicalType::String,
                    false,
                )],
            )),
        );

        graph_type
    }

    fn export_dirs_equal_semantically<P: AsRef<Path>>(dir1: P, dir2: P) -> bool {
        let dir1 = dir1.as_ref();
        let dir2 = dir2.as_ref();

        assert!(dir1.exists());
        assert!(dir2.exists());
        assert!(dir1.is_dir());
        assert!(dir2.is_dir());

        let index = |root: &Path| {
            WalkDir::new(root)
                .follow_links(true)
                .min_depth(1)
                .into_iter()
                .map(|entry| {
                    let entry = entry.unwrap();
                    (entry.file_name().to_str().unwrap().to_string(), entry)
                })
                .collect::<BTreeMap<_, _>>()
        };

        let index1 = index(dir1);
        let index2 = index(dir2);

        if index1.len() != index2.len() {
            return false;
        }

        index1
            .iter()
            .zip(index2.iter())
            .all(|((filename1, entry1), (filename2, entry2))| {
                // Check if the filename is the same and the file type is the same
                if filename1 != filename2 || entry1.file_type() != entry2.file_type() {
                    return false;
                }

                // If file type is dir, call `dirs_identical`
                assert!(entry1.file_type().is_file());

                let filename1 = dir1.join(filename1);
                let filename2 = dir1.join(filename2);

                // Make sure the manifest file name is ended with ".json"
                if filename1.extension().and_then(|e| e.to_str()) == Some("json") {
                    let v1: serde_json::Value =
                        serde_json::from_slice(&std::fs::read(filename1).unwrap()).unwrap();
                    let v2: serde_json::Value =
                        serde_json::from_slice(&std::fs::read(filename2).unwrap()).unwrap();
                    return v1 == v2;
                }

                // Check if the file size is the same
                if entry1.metadata().unwrap().len() != entry2.metadata().unwrap().len() {
                    return false;
                }

                std::fs::read(filename1).unwrap() == std::fs::read(filename2).unwrap()
            })
    }

    #[test]
    fn test_export_and_import() {
        let export_dir1 = tempfile::tempdir().unwrap();
        let export_dir2 = tempfile::tempdir().unwrap();

        let export_dir1 = export_dir1.path();
        let export_dir2 = export_dir2.path();

        let manifest_rel_path = "manifest.json";

        let graph_type: Arc<dyn GraphTypeProvider> = Arc::new(mock_graph_type());
        {
            let (graph, _cleaner) = mock_graph();
            let txn = graph.begin_transaction(IsolationLevel::Serializable);

            graph.export_graph(
                &txn,
                export_dir1,
                manifest_rel_path.as_ref(),
                Arc::clone(&graph_type),
            );
            txn.commit().unwrap();
        }

        {
            let (graph, _) = mock_empty_graph();
            let txn = graph.begin_transaction(IsolationLevel::Serializable);

            graph.import_graph(
                &txn,
                export_dir1.join(manifest_rel_path),
                Arc::clone(&graph_type),
            );

            graph.export_graph(&txn, export_dir2, manifest_rel_path.as_ref(), graph_type);
            txn.commit().unwrap();
        }

        assert!(export_dirs_equal_semantically(export_dir1, export_dir2))
    }
}
