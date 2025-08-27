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

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;

use minigu_catalog::label_set::LabelSet;
use minigu_catalog::property::Property;
use minigu_catalog::provider::GraphTypeProvider;
use minigu_common::types::LabelId;
use serde::{Deserialize, Serialize};

pub mod export;
pub mod import;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync + 'static>>;
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
    fn from_schema(graph_type: Arc<dyn GraphTypeProvider>) -> Result<Self> {
        // Build a label map LabelId -> String
        let label_names = graph_type.label_names();
        let mut label_map = HashMap::with_capacity(label_names.len());
        for name in label_names {
            let label_id = graph_type.get_label_id(&name)?.expect("label id not found");
            label_map.insert(label_id, name);
        }

        let mut vertex_labels = HashSet::new();
        let mut v_lset_to_label = HashMap::new();
        let mut edge_infos = HashMap::new();
        for (&id, _) in label_map.iter() {
            let label_set = LabelSet::from_iter(vec![id]);

            if let Some(edge_type) = graph_type
                .get_edge_type(&label_set)
                .expect("edge type not found")
            {
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
                let src_id = *v_lset_to_label.get(src).expect("label set not found");
                let dst_id = *v_lset_to_label.get(dst).expect("label set not found");

                (id, (src_id, dst_id))
            })
            .collect();

        Ok(Self {
            label_map,
            vertex_labels,
            edge_infos,
            schema: Arc::clone(&graph_type),
        })
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
struct VertexSpec {
    label: String,
    file: FileSpec,
    properties: Vec<Property>,
}

impl VertexSpec {
    fn label_name(&self) -> &String {
        &self.label
    }

    fn properties(&self) -> &Vec<Property> {
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

    fn src_label(&self) -> &String {
        &self.src_label
    }

    fn dst_label(&self) -> &String {
        &self.dst_label
    }

    fn label_name(&self) -> &String {
        &self.label
    }

    fn properties(&self) -> &Vec<Property> {
        &self.properties
    }
}

// Top-level manifest written to disk.
#[derive(Deserialize, Serialize, Default, Debug)]
struct Manifest {
    vertices: Vec<VertexSpec>,
    edges: Vec<EdgeSpec>,
}

impl Manifest {
    fn from_schema(metadata: SchemaMetadata) -> Result<Self> {
        let vertex_labels = &metadata.vertex_labels;
        let mut vertex_specs = Vec::with_capacity(vertex_labels.len());

        for &id in vertex_labels {
            let name = metadata.label_map.get(&id).expect("label id not found");
            let path = format!("{}.csv", name);
            let props_schema = metadata
                .schema
                .get_vertex_type(&LabelSet::from_iter(vec![id]))? // will return None for vertex (inverse call later)
                .expect("vertex type not found")
                .properties()
                .into_iter()
                .map(|prop| prop.1) // drop index key
                .collect::<Vec<_>>();

            vertex_specs.push(VertexSpec::new(
                name.clone(),
                FileSpec::new(path, "csv".to_string()),
                props_schema,
            ))
        }

        let edge_infos = &metadata.edge_infos;
        let mut edge_specs = Vec::with_capacity(edge_infos.len());

        for (&id, (src_id, dst_id)) in edge_infos {
            let name = metadata.label_map.get(&id).expect("label id not found");
            let path = format!("{}.csv", name);
            let props_schema = metadata
                .schema
                .get_edge_type(&LabelSet::from_iter(vec![id]))? // will return None for vertex (inverse call later)
                .expect("edge type not found")
                .properties()
                .into_iter()
                .map(|prop| prop.1) // drop index key
                .collect::<Vec<_>>();

            let src_label = metadata.label_map.get(src_id).unwrap().clone();
            let dst_label = metadata.label_map.get(dst_id).unwrap().clone();

            edge_specs.push(EdgeSpec::new(
                name.clone(),
                src_label,
                dst_label,
                FileSpec::new(path, "csv".to_string()),
                props_schema,
            ));
        }

        Ok(Self {
            vertices: vertex_specs,
            edges: edge_specs,
        })
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

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::path::Path;

    use minigu_catalog::memory::graph_type::{
        MemoryEdgeTypeCatalog, MemoryGraphTypeCatalog, MemoryVertexTypeCatalog,
    };
    use minigu_common::data_type::LogicalType;
    use minigu_common::types::{EdgeId, VertexId};
    use minigu_common::value::ScalarValue;
    use minigu_storage::common::{Edge, PropertyRecord, Vertex};
    use minigu_storage::tp::checkpoint::CheckpointManagerConfig;
    use minigu_storage::tp::{IsolationLevel, MemoryGraph};
    use minigu_storage::wal::graph_wal::WalManagerConfig;
    use walkdir::WalkDir;

    use super::*;
    use crate::procedures::export_import::export::export;
    use crate::procedures::export_import::import::import;

    const PERSON: LabelId = LabelId::new(1).unwrap();
    const FRIEND: LabelId = LabelId::new(2).unwrap();
    const FOLLOW: LabelId = LabelId::new(3).unwrap();

    fn create_vertex(vid: VertexId, label_id: LabelId, properties: Vec<ScalarValue>) -> Vertex {
        Vertex::new(vid, label_id, PropertyRecord::new(properties))
    }

    fn create_edge(
        eid: EdgeId,
        src_id: VertexId,
        dst_id: VertexId,
        label_id: LabelId,
        properties: Vec<ScalarValue>,
    ) -> Edge {
        Edge::new(
            eid,
            src_id,
            dst_id,
            label_id,
            PropertyRecord::new(properties),
        )
    }

    fn mock_checkpoint_config() -> CheckpointManagerConfig {
        let dir = tempfile::tempdir().unwrap();
        let checkpoint_dir = dir.as_ref().join(format!(
            "checkpoint_{}",
            chrono::Utc::now().format("%Y%m%d%H%M")
        ));

        CheckpointManagerConfig {
            checkpoint_dir,
            ..Default::default()
        }
    }

    fn mock_wal_config() -> WalManagerConfig {
        let dir = tempfile::tempdir().unwrap();
        let filename = format!("wal_{}.log", chrono::Utc::now().format("%Y%m%d%H%M"));
        let wal_path = dir.as_ref().join(filename);

        WalManagerConfig { wal_path }
    }

    fn mock_graph() -> Arc<MemoryGraph> {
        let graph = MemoryGraph::with_config_fresh(mock_checkpoint_config(), mock_wal_config());

        let txn = graph.begin_transaction(IsolationLevel::Serializable);

        let alice = create_vertex(1, PERSON, vec![
            ScalarValue::String(Some("Alice".to_string())),
            ScalarValue::Int32(Some(25)),
        ]);

        let bob = create_vertex(2, PERSON, vec![
            ScalarValue::String(Some("Bob".to_string())),
            ScalarValue::Int32(Some(28)),
        ]);

        let carol = create_vertex(3, PERSON, vec![
            ScalarValue::String(Some("Carol".to_string())),
            ScalarValue::Int32(Some(24)),
        ]);

        let david = create_vertex(4, PERSON, vec![
            ScalarValue::String(Some("David".to_string())),
            ScalarValue::Int32(Some(27)),
        ]);

        // Add vertices to the graph
        graph.create_vertex(&txn, alice).unwrap();
        graph.create_vertex(&txn, bob).unwrap();
        graph.create_vertex(&txn, carol).unwrap();
        graph.create_vertex(&txn, david).unwrap();

        // Create friend edges
        let friend1 = create_edge(1, 1, 2, FRIEND, vec![ScalarValue::String(Some(
            "2020-01-01".to_string(),
        ))]);

        let friend2 = create_edge(2, 2, 3, FRIEND, vec![ScalarValue::String(Some(
            "2021-03-15".to_string(),
        ))]);

        // Create follow edges
        let follow1 = create_edge(3, 1, 3, FOLLOW, vec![ScalarValue::String(Some(
            "2022-06-01".to_string(),
        ))]);

        let follow2 = create_edge(4, 4, 1, FOLLOW, vec![ScalarValue::String(Some(
            "2022-07-15".to_string(),
        ))]);

        // Add edges to the graph
        graph.create_edge(&txn, friend1).unwrap();
        graph.create_edge(&txn, friend2).unwrap();
        graph.create_edge(&txn, follow1).unwrap();
        graph.create_edge(&txn, follow2).unwrap();

        txn.commit().unwrap();

        graph
    }

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
            let graph = mock_graph();

            export(
                graph,
                export_dir1,
                manifest_rel_path.as_ref(),
                Arc::clone(&graph_type),
            )
            .unwrap();
        }

        {
            let manifest_path = export_dir1.join(manifest_rel_path);
            let (graph, graph_type) = import(manifest_path).unwrap();

            export(
                graph,
                export_dir2,
                manifest_rel_path.as_ref(),
                graph_type.clone(),
            )
            .unwrap();
        }

        assert!(export_dirs_equal_semantically(export_dir1, export_dir2));
    }
}
