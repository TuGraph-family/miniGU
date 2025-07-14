use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::sync::Arc;

use csv::{ReaderBuilder, WriterBuilder};
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

#[derive(Deserialize, Serialize)]
struct FileConfig {
    path: String,
    // Current only support CSV, consider using enum
    format: String,
}

impl FileConfig {
    pub fn new(path: String, format: String) -> Self {
        Self { path, format }
    }
}

#[derive(Deserialize, Serialize)]
struct VertexConfig {
    label: String,
    file: FileConfig,
}

impl VertexConfig {
    pub fn new(label: String, file: FileConfig) -> Self {
        Self { label, file }
    }
}

#[derive(Deserialize, Serialize)]
struct EdgeConfig {
    label: String,
    file: FileConfig,
}

impl EdgeConfig {
    pub fn new(label: String, file: FileConfig) -> Self {
        Self { label, file }
    }
}

#[derive(Deserialize, Serialize, Default)]
struct Config {
    vertex_config_list: Vec<VertexConfig>,
    edge_config_list: Vec<EdgeConfig>,
}

impl Config {
    pub fn new(vertex_config_list: Vec<VertexConfig>, edge_config_list: Vec<EdgeConfig>) -> Self {
        Self {
            vertex_config_list,
            edge_config_list,
        }
    }
}

fn property_to_scalar_value(property: &Property, value: &str) -> ScalarValue {
    match property.logical_type() {
        LogicalType::Int8 => ScalarValue::Int8(Some(value.parse().unwrap())),
        LogicalType::Int16 => ScalarValue::Int16(Some(value.parse().unwrap())),
        LogicalType::Int32 => ScalarValue::Int32(Some(value.parse().unwrap())),
        LogicalType::Int64 => ScalarValue::Int64(Some(value.parse().unwrap())),
        LogicalType::UInt8 => ScalarValue::UInt8(Some(value.parse().unwrap())),
        LogicalType::UInt16 => ScalarValue::UInt16(Some(value.parse().unwrap())),
        LogicalType::UInt32 => ScalarValue::UInt32(Some(value.parse().unwrap())),
        LogicalType::UInt64 => ScalarValue::UInt64(Some(value.parse().unwrap())),
        LogicalType::Boolean => ScalarValue::Boolean(Some(value.parse().unwrap())),
        LogicalType::Float32 => ScalarValue::Float32(Some(value.parse().unwrap())),
        LogicalType::Float64 => ScalarValue::Float64(Some(value.parse().unwrap())),
        LogicalType::String => ScalarValue::String(Some(value.to_string())),
        _ => todo!(),
    }
}

fn scalar_value_to_string(scalar_value: &ScalarValue) -> String {
    match scalar_value {
        ScalarValue::Int8(x) => x.unwrap().to_string(),
        ScalarValue::Int16(x) => x.unwrap().to_string(),
        ScalarValue::Int32(x) => x.unwrap().to_string(),
        ScalarValue::Int64(x) => x.unwrap().to_string(),
        ScalarValue::UInt8(x) => x.unwrap().to_string(),
        ScalarValue::UInt16(x) => x.unwrap().to_string(),
        ScalarValue::UInt32(x) => x.unwrap().to_string(),
        ScalarValue::UInt64(x) => x.unwrap().to_string(),
        ScalarValue::Boolean(x) => x.unwrap().to_string(),
        ScalarValue::Float32(x) => x.unwrap().to_string(),
        ScalarValue::Float64(x) => x.unwrap().to_string(),
        ScalarValue::String(x) => x.as_ref().unwrap().clone(),
        _ => todo!(),
    }
}

impl MemoryGraph {
    // TODO: return `Result`
    pub fn import_graph<P: AsRef<Path>>(
        &self,
        txn: &TransactionHandle,
        config_path: P,
        graph_type: Arc<dyn GraphTypeProvider>,
    ) {
        let config_str = std::fs::read_to_string(config_path.as_ref()).unwrap();
        let config: Config = serde_json::from_str(&config_str).unwrap();

        // Import vertex, format: "<vid>,<property-1>,<property-2>,"
        for vertex_config in config.vertex_config_list.iter() {
            let mut rdr = ReaderBuilder::new()
                .has_headers(false)
                .from_path(
                    config_path
                        .as_ref()
                        .parent()
                        .unwrap()
                        .join(&vertex_config.file.path),
                )
                .unwrap();

            let label_id = graph_type
                .get_label_id(&vertex_config.label)
                .unwrap()
                .unwrap();

            for record in rdr.records() {
                let label_set = LabelSet::from_iter(vec![label_id]);
                let props = graph_type
                    .get_vertex_type(&label_set)
                    .unwrap()
                    .unwrap()
                    .properties();

                let record = record.unwrap();

                assert_eq!(props.len() + 1, record.len());

                let vid = record.get(0).unwrap().parse().unwrap();

                let props = props
                    .iter()
                    .zip(record.iter().skip(1))
                    .map(|((_, p), value)| property_to_scalar_value(p, value))
                    .collect();
                let vertex = Vertex::new(vid, label_id, PropertyRecord::new(props));

                self.create_vertex(txn, vertex).unwrap();
            }
        }

        // Import edge, format: "<eid>,<src>,<dst>,<property-1>,<property-2>"
        for edge_config in config.edge_config_list.iter() {
            let label_id = graph_type
                .get_label_id(&edge_config.label)
                .unwrap()
                .unwrap();

            let mut rdr = ReaderBuilder::new()
                .has_headers(false)
                .from_path(
                    config_path
                        .as_ref()
                        .parent()
                        .unwrap()
                        .join(&edge_config.file.path),
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
                let eid = record.get(0).unwrap().parse().unwrap();
                let src_id = record.get(1).unwrap().parse().unwrap();
                let dst_id = record.get(2).unwrap().parse().unwrap();

                let props = props
                    .iter()
                    .zip(record.iter().skip(3))
                    .map(|((_, p), value)| property_to_scalar_value(p, value))
                    .collect();

                let edge = Edge::new(eid, src_id, dst_id, label_id, PropertyRecord::new(props));
                self.create_edge(txn, edge).unwrap();
            }
        }
    }

    pub fn export_graph<P: AsRef<Path>>(
        &self,
        txn: &TransactionHandle,
        dir: P,
        config_file_name: String,
        graph_type: Arc<dyn GraphTypeProvider>,
    ) {
        // 1. prepare output paths
        let dir = dir.as_ref();
        std::fs::create_dir_all(dir).unwrap();

        let (label_id_to_label, mut file_map): (HashMap<_, _>, HashMap<_, _>) = graph_type
            .label_names()
            .iter()
            .map(|label| {
                let filename = format!("{label}.csv");
                let path = dir.join(&filename);
                let label_id = graph_type.get_label_id(label).unwrap().unwrap();

                (
                    (label_id, label.to_string()),
                    (label_id, WriterBuilder::new().from_path(path).unwrap()),
                )
            })
            .unzip();

        // 2. export vertices
        let mut vertex_config_map = HashMap::new();
        let mut vertice: HashMap<LabelId, BTreeMap<VertexId, RecordType>> = HashMap::new();
        for v in txn.iter_vertices() {
            let v = v.unwrap();

            // Insert vertex config
            vertex_config_map.entry(v.label_id).or_insert_with(|| {
                let label = label_id_to_label.get(&v.label_id).unwrap();
                VertexConfig::new(
                    label.clone(),
                    FileConfig::new(format!("{}.csv", label), "csv".to_string()),
                )
            });

            let record = std::iter::once(v.vid().to_string())
                .chain(v.properties().iter().map(scalar_value_to_string))
                .collect::<Vec<_>>();

            vertice
                .entry(v.label_id)
                .or_default()
                .insert(v.vid(), record);
        }
        for (label_id, records) in vertice {
            let w = file_map.get_mut(&label_id).unwrap();

            for (_, record) in records {
                w.write_record(record).unwrap();
            }
        }
        let vertex_config_list = vertex_config_map.into_values().collect();

        // 3. export edge
        let mut edge_config_map = HashMap::new();
        let mut edges: HashMap<LabelId, BTreeMap<EdgeId, RecordType>> = Default::default();
        for e in txn.iter_edges() {
            let e = e.unwrap();

            // Insert edge config
            edge_config_map.entry(e.label_id).or_insert_with(|| {
                let label = label_id_to_label.get(&e.label_id).unwrap();
                EdgeConfig::new(
                    label.clone(),
                    FileConfig::new(format!("{}.csv", label), "csv".to_string()),
                )
            });

            let prefix = [
                e.eid().to_string(),
                e.src_id().to_string(),
                e.dst_id().to_string(),
            ];
            let record = prefix
                .into_iter()
                .chain(e.properties().iter().map(scalar_value_to_string))
                .collect::<Vec<_>>();

            edges.entry(e.label_id).or_default().insert(e.eid(), record);
        }

        // Dump
        for (label_id, records) in edges {
            let w = file_map.get_mut(&label_id).unwrap();

            for (_, record) in records {
                w.write_record(record).unwrap();
            }
        }

        let edge_config_list = edge_config_map.into_values().collect();

        // 4. write config file
        let config = Config::new(vertex_config_list, edge_config_list);
        std::fs::write(
            dir.join(config_file_name),
            serde_json::to_string(&config).unwrap(),
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

                // Make sure the config file name is ended with ".json"
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

        let config_filename = "config.json";

        let graph_type: Arc<dyn GraphTypeProvider> = Arc::new(mock_graph_type());
        {
            let (graph, _cleaner) = mock_graph();
            let txn = graph.begin_transaction(IsolationLevel::Serializable);

            graph.export_graph(
                &txn,
                export_dir1,
                config_filename.to_string(),
                Arc::clone(&graph_type),
            );
        }

        {
            let (graph, _) = mock_empty_graph();
            let txn = graph.begin_transaction(IsolationLevel::Serializable);

            graph.import_graph(
                &txn,
                export_dir1.join(config_filename),
                Arc::clone(&graph_type),
            );

            graph.export_graph(&txn, export_dir2, config_filename.to_string(), graph_type);
        }

        assert!(export_dirs_equal_semantically(export_dir1, export_dir2))
    }
}
