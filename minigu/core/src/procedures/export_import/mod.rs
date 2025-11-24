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
use minigu_catalog::txn::catalog_txn::CatalogTxn;
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
    // Precomputed property schemas to avoid holding a txn beyond construction.
    vertex_props: HashMap<LabelId, Vec<Property>>, // vertex label -> props
    edge_props: HashMap<LabelId, Vec<Property>>,   // edge label -> props
}

impl SchemaMetadata {
    fn from_schema(
        graph_type: Arc<dyn GraphTypeProvider>,
        catalog_txn: &CatalogTxn,
    ) -> Result<Self> {
        // Build a label map LabelId -> String
        let label_names = graph_type.label_names(catalog_txn);
        let mut label_map = HashMap::with_capacity(label_names.len());
        for name in label_names {
            let label_id = graph_type
                .get_label_id(&name, catalog_txn)?
                .expect("label id not found");
            label_map.insert(label_id, name);
        }

        let mut vertex_labels = HashSet::new();
        let mut v_lset_to_label = HashMap::new();
        let mut edge_infos = HashMap::new();
        for (&id, _) in label_map.iter() {
            let label_set = LabelSet::from_iter(vec![id]);

            if let Some(edge_type) = graph_type
                .get_edge_type(&label_set, catalog_txn)
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

        let edge_infos: HashMap<LabelId, (LabelId, LabelId)> = edge_infos
            .iter()
            .map(|(&id, (src, dst))| {
                let src_id = *v_lset_to_label.get(src).expect("label set not found");
                let dst_id = *v_lset_to_label.get(dst).expect("label set not found");

                (id, (src_id, dst_id))
            })
            .collect();

        // Precompute properties for vertices and edges.
        let mut vertex_props: HashMap<LabelId, Vec<Property>> = HashMap::new();
        for &id in vertex_labels.iter() {
            let props = graph_type
                .get_vertex_type(&LabelSet::from_iter(vec![id]), catalog_txn)
                .expect("vertex type fetch failed")
                .expect("vertex type not found")
                .properties()
                .into_iter()
                .map(|(_, p)| p)
                .collect::<Vec<_>>();
            vertex_props.insert(id, props);
        }
        let mut edge_props: HashMap<LabelId, Vec<Property>> = HashMap::new();
        for (&edge_id, _) in edge_infos.iter() {
            let props = graph_type
                .get_edge_type(&LabelSet::from_iter(vec![edge_id]), catalog_txn)
                .expect("edge type fetch failed")
                .expect("edge type not found")
                .properties()
                .into_iter()
                .map(|(_, p)| p)
                .collect::<Vec<_>>();
            edge_props.insert(edge_id, props);
        }

        Ok(Self {
            label_map,
            vertex_labels,
            edge_infos,
            vertex_props,
            edge_props,
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
            let name = metadata
                .label_map
                .get(&id)
                .cloned()
                .unwrap_or_else(|| format!("vertex_{}", id.get()));
            let path = format!("{}.csv", name);
            let props_schema = metadata.vertex_props.get(&id).cloned().unwrap_or_default();

            vertex_specs.push(VertexSpec::new(
                name,
                FileSpec::new(path, "csv".to_string()),
                props_schema,
            ))
        }

        let edge_infos = &metadata.edge_infos;
        let mut edge_specs = Vec::with_capacity(edge_infos.len());

        for (&id, (src_id, dst_id)) in edge_infos {
            let name = metadata
                .label_map
                .get(&id)
                .cloned()
                .unwrap_or_else(|| format!("edge_{}", id.get()));
            let path = format!("{}.csv", name);
            let props_schema = metadata.edge_props.get(&id).cloned().unwrap_or_default();

            let src_label = metadata
                .label_map
                .get(src_id)
                .cloned()
                .unwrap_or_else(|| format!("vertex_{}", src_id.get()));
            let dst_label = metadata
                .label_map
                .get(dst_id)
                .cloned()
                .unwrap_or_else(|| format!("vertex_{}", dst_id.get()));

            edge_specs.push(EdgeSpec::new(
                name,
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
    use minigu_catalog::txn::manager::CatalogTxnManager;
    use minigu_common::data_type::LogicalType;
    use minigu_common::types::{EdgeId, VertexId};
    use minigu_common::value::ScalarValue;
    use minigu_storage::common::{Edge, PropertyRecord, Vertex};
    use minigu_storage::tp::MemoryGraph;
    use minigu_storage::tp::checkpoint::CheckpointManagerConfig;
    use minigu_storage::wal::graph_wal::WalManagerConfig;
    use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};
    use walkdir::WalkDir;

    use super::*;
    use crate::procedures::export_import::export::export;
    use crate::procedures::export_import::import::import;

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

    fn mock_graph(person: LabelId, friend: LabelId, follow: LabelId) -> Arc<MemoryGraph> {
        let graph = MemoryGraph::with_config_fresh(mock_checkpoint_config(), mock_wal_config());

        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        let alice = create_vertex(1, person, vec![
            ScalarValue::String(Some("Alice".to_string())),
            ScalarValue::Int32(Some(25)),
        ]);

        let bob = create_vertex(2, person, vec![
            ScalarValue::String(Some("Bob".to_string())),
            ScalarValue::Int32(Some(28)),
        ]);

        let carol = create_vertex(3, person, vec![
            ScalarValue::String(Some("Carol".to_string())),
            ScalarValue::Int32(Some(24)),
        ]);

        let david = create_vertex(4, person, vec![
            ScalarValue::String(Some("David".to_string())),
            ScalarValue::Int32(Some(27)),
        ]);

        // Add vertices to the graph
        graph.create_vertex(&txn, alice).unwrap();
        graph.create_vertex(&txn, bob).unwrap();
        graph.create_vertex(&txn, carol).unwrap();
        graph.create_vertex(&txn, david).unwrap();

        // Create friend edges
        let friend1 = create_edge(1, 1, 2, friend, vec![ScalarValue::String(Some(
            "2020-01-01".to_string(),
        ))]);

        let friend2 = create_edge(2, 2, 3, friend, vec![ScalarValue::String(Some(
            "2021-03-15".to_string(),
        ))]);

        // Create follow edges
        let follow1 = create_edge(3, 1, 3, follow, vec![ScalarValue::String(Some(
            "2022-06-01".to_string(),
        ))]);

        let follow2 = create_edge(4, 4, 1, follow, vec![ScalarValue::String(Some(
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

    fn mock_graph_type(
        mgr: &CatalogTxnManager,
    ) -> (MemoryGraphTypeCatalog, LabelId, LabelId, LabelId) {
        use minigu_catalog::txn::manager::CatalogTxnManager;
        use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};
        let graph_type = MemoryGraphTypeCatalog::new();

        // Txn 1: Create labels and vertex types, and commit
        let txn1 = mgr.begin_transaction(IsolationLevel::Snapshot).unwrap();
        let person_id = graph_type
            .add_label("person".to_string(), txn1.as_ref())
            .unwrap();
        let friend_id = graph_type
            .add_label("friend".to_string(), txn1.as_ref())
            .unwrap();
        let follow_id = graph_type
            .add_label("follow".to_string(), txn1.as_ref())
            .unwrap();

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

        graph_type
            .add_vertex_type(person_label_set.clone(), vertex_type.clone(), txn1.as_ref())
            .unwrap();
        txn1.commit().unwrap();

        // Txn 2: Create edge types based on committed vertex types, and commit
        let txn2 = mgr.begin_transaction(IsolationLevel::Snapshot).unwrap();
        graph_type
            .add_edge_type(
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
                txn2.as_ref(),
            )
            .unwrap();
        graph_type
            .add_edge_type(
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
                txn2.as_ref(),
            )
            .unwrap();
        txn2.commit().unwrap();

        // Verification - check all labels are visible
        let txn3 = mgr.begin_transaction(IsolationLevel::Snapshot).unwrap();

        let fetched_person_id = graph_type.get_label_id("person", txn3.as_ref()).unwrap();
        let fetched_friend_id = graph_type.get_label_id("friend", txn3.as_ref()).unwrap();
        let fetched_follow_id = graph_type.get_label_id("follow", txn3.as_ref()).unwrap();

        assert_eq!(fetched_person_id, Some(person_id), "person label mismatch");
        assert_eq!(fetched_friend_id, Some(friend_id), "friend label mismatch");
        assert_eq!(fetched_follow_id, Some(follow_id), "follow label mismatch");

        txn3.commit().unwrap();

        (graph_type, person_id, friend_id, follow_id)
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
    fn test_mock_graph_type() {
        let mgr = CatalogTxnManager::new();
        let (gt, person_id, friend_id, follow_id) = mock_graph_type(&mgr);
        assert_eq!(
            gt.get_label_id(
                "person",
                mgr.begin_transaction(IsolationLevel::Snapshot)
                    .unwrap()
                    .as_ref()
            )
            .unwrap(),
            Some(person_id)
        );
        assert_eq!(
            gt.get_label_id(
                "friend",
                mgr.begin_transaction(IsolationLevel::Snapshot)
                    .unwrap()
                    .as_ref()
            )
            .unwrap(),
            Some(friend_id)
        );
        assert_eq!(
            gt.get_label_id(
                "follow",
                mgr.begin_transaction(IsolationLevel::Snapshot)
                    .unwrap()
                    .as_ref()
            )
            .unwrap(),
            Some(follow_id)
        );
    }

    #[test]
    fn test_export_and_import() {
        let export_dir1 = tempfile::tempdir().unwrap();
        let export_dir2 = tempfile::tempdir().unwrap();

        let export_dir1 = export_dir1.path();
        let export_dir2 = export_dir2.path();

        let manifest_rel_path = "manifest.json";

        let mgr = CatalogTxnManager::new();
        let (gt, person_id, friend_id, follow_id) = mock_graph_type(&mgr);
        let graph_type: Arc<dyn GraphTypeProvider> = Arc::new(gt);
        {
            let txn = mgr.begin_transaction(IsolationLevel::Snapshot).unwrap();
            let graph = mock_graph(person_id, friend_id, follow_id);

            export(
                graph,
                export_dir1,
                manifest_rel_path.as_ref(),
                Arc::clone(&graph_type),
                txn.as_ref(),
            )
            .unwrap();
            let _ = txn.commit().unwrap();
        }

        {
            let manifest_path = export_dir1.join(manifest_rel_path);
            let (graph, graph_type) = import(manifest_path, &mgr).unwrap();
            let txn = mgr.begin_transaction(IsolationLevel::Snapshot).unwrap();

            export(
                graph,
                export_dir2,
                manifest_rel_path.as_ref(),
                graph_type.clone(),
                txn.as_ref(),
            )
            .unwrap();
            let _ = txn.commit().unwrap();
        }

        assert!(export_dirs_equal_semantically(export_dir1, export_dir2));
    }
}
