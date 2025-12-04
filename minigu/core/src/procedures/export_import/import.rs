//! call import(<graph_name>, <dir_path>, <manifest_relative_path>);
//!
//! Import a graph from CSV files plus a JSON `manifest.json` on disk into an in-memory graph,
//! then register it in the current schema under `<graph_name>`.
//!
//! ## Inputs
//! * `<graph_name>` – Name to register the imported graph under in the current schema.
//! * `<dir_path>` – Directory that contains the CSV files and the manifest.
//! * `<manifest_relative_path>` – File name or relative path (inside `dir_path`) to
//!   `manifest.json`.
//!
//! ## Output
//! * Returns nothing. On success the graph is added to the current schema. Errors (missing files,
//!   schema mismatch, duplicate graph name, etc.) are surfaced via `Result`.

use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use csv::ReaderBuilder;
use minigu_catalog::label_set::LabelSet;
use minigu_catalog::memory::graph_type::{
    MemoryEdgeTypeCatalog, MemoryGraphTypeCatalog, MemoryVertexTypeCatalog,
};
use minigu_catalog::property::Property;
use minigu_catalog::provider::GraphTypeProvider;
use minigu_common::data_type::LogicalType;
use minigu_common::error::not_implemented;
use minigu_common::types::{LabelId, VertexId};
use minigu_common::value::ScalarValue;
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::procedure::Procedure;
use minigu_storage::common::{Edge, PropertyRecord, Vertex};
use minigu_storage::tp::MemoryGraph;
use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

use crate::procedures::export_import::{Manifest, Result};

fn build_manifest<P: AsRef<Path>>(manifest_path: P) -> Result<Manifest> {
    let data = std::fs::read(manifest_path)?;

    let data_str = std::str::from_utf8(&data)?;
    Manifest::from_str(data_str)
}

/// Convert a *string* coming from CSV into an owned [`ScalarValue`] according
/// to a given property definition.
fn property_to_scalar_value(property: &Property, value: &str) -> Result<ScalarValue> {
    // 如果值为空字符串，根据属性是否可空来处理
    if value.is_empty() {
        if property.nullable() {
            return match property.logical_type() {
                LogicalType::Int8 => Ok(ScalarValue::Int8(None)),
                LogicalType::Int16 => Ok(ScalarValue::Int16(None)),
                LogicalType::Int32 => Ok(ScalarValue::Int32(None)),
                LogicalType::Int64 => Ok(ScalarValue::Int64(None)),
                LogicalType::UInt8 => Ok(ScalarValue::UInt8(None)),
                LogicalType::UInt16 => Ok(ScalarValue::UInt16(None)),
                LogicalType::UInt32 => Ok(ScalarValue::UInt32(None)),
                LogicalType::UInt64 => Ok(ScalarValue::UInt64(None)),
                LogicalType::Boolean => Ok(ScalarValue::Boolean(None)),
                LogicalType::Float32 => Ok(ScalarValue::Float32(None)),
                LogicalType::Float64 => Ok(ScalarValue::Float64(None)),
                LogicalType::String => Ok(ScalarValue::String(None)),
                LogicalType::Null => Ok(ScalarValue::Null),
                _ => not_implemented("", None),
            };
        } else {
            // 属性不可为空，但值为空，返回错误
            return Err(anyhow::anyhow!(
                "Cannot parse empty string for non-nullable property '{}' of type {:?}",
                property.name(),
                property.logical_type()
            ).into());
        }
    }

    // 值不为空，尝试解析
    match property.logical_type() {
        LogicalType::Int8 => Ok(ScalarValue::Int8(Some(value.parse().map_err(|e| {
            anyhow::anyhow!("Cannot parse Int8 from '{}' for property '{}': {}", value, property.name(), e)
        })?))),
        LogicalType::Int16 => Ok(ScalarValue::Int16(Some(value.parse().map_err(|e| {
            anyhow::anyhow!("Cannot parse Int16 from '{}' for property '{}': {}", value, property.name(), e)
        })?))),
        LogicalType::Int32 => Ok(ScalarValue::Int32(Some(value.parse().map_err(|e| {
            anyhow::anyhow!("Cannot parse Int32 from '{}' for property '{}': {}", value, property.name(), e)
        })?))),
        LogicalType::Int64 => Ok(ScalarValue::Int64(Some(value.parse().map_err(|e| {
            anyhow::anyhow!("Cannot parse Int64 from '{}' for property '{}': {}", value, property.name(), e)
        })?))),
        LogicalType::UInt8 => Ok(ScalarValue::UInt8(Some(value.parse().map_err(|e| {
            anyhow::anyhow!("Cannot parse UInt8 from '{}' for property '{}': {}", value, property.name(), e)
        })?))),
        LogicalType::UInt16 => Ok(ScalarValue::UInt16(Some(value.parse().map_err(|e| {
            anyhow::anyhow!("Cannot parse UInt16 from '{}' for property '{}': {}", value, property.name(), e)
        })?))),
        LogicalType::UInt32 => Ok(ScalarValue::UInt32(Some(value.parse().map_err(|e| {
            anyhow::anyhow!("Cannot parse UInt32 from '{}' for property '{}': {}", value, property.name(), e)
        })?))),
        LogicalType::UInt64 => Ok(ScalarValue::UInt64(Some(value.parse().map_err(|e| {
            anyhow::anyhow!("Cannot parse UInt64 from '{}' for property '{}': {}", value, property.name(), e)
        })?))),
        LogicalType::Boolean => Ok(ScalarValue::Boolean(Some(value.parse().map_err(|e| {
            anyhow::anyhow!("Cannot parse Boolean from '{}' for property '{}': {}", value, property.name(), e)
        })?))),
        LogicalType::Float32 => Ok(ScalarValue::Float32(Some(value.parse().map_err(|e| {
            anyhow::anyhow!("Cannot parse Float32 from '{}' for property '{}': {}", value, property.name(), e)
        })?))),
        LogicalType::Float64 => Ok(ScalarValue::Float64(Some(value.parse().map_err(|e| {
            anyhow::anyhow!("Cannot parse Float64 from '{}' for property '{}': {}", value, property.name(), e)
        })?))),
        LogicalType::String => Ok(ScalarValue::String(Some(value.to_string()))),
        LogicalType::Null => Err(anyhow::anyhow!("str isn't empty").into()),
        _ => not_implemented("", None),
    }
}

fn build_properties<'a>(
    props_schema: Vec<(u32, Property)>,
    record_iter: impl Iterator<Item = &'a str>,
) -> Result<Vec<ScalarValue>> {
    let mut props = Vec::with_capacity(props_schema.len());

    for ((_, property), value) in props_schema.iter().zip(record_iter) {
        props.push(property_to_scalar_value(property, value)?);
    }

    Ok(props)
}

pub(crate) fn import<P: AsRef<Path>>(
    manifest_path: P,
) -> Result<(Arc<MemoryGraph>, Arc<MemoryGraphTypeCatalog>)> {
    // Graph type
    let manifest = build_manifest(&manifest_path)?;
    let graph_type = get_graph_type_from_manifest(&manifest)?;

    // Graph
    let graph = MemoryGraph::with_config_fresh(Default::default(), Default::default());
    let txn = graph
        .txn_manager()
        .begin_transaction(IsolationLevel::Serializable)?;

    let manifest_parent_dir = manifest_path.as_ref().parent().ok_or_else(|| {
        anyhow::anyhow!(
            "manifest path has no parent directory: {}",
            manifest_path.as_ref().display()
        )
    })?;
    // Map each original vertex ID to it's newly assigned ID.
    let mut vid_mapping: HashMap<LabelId, HashMap<VertexId, VertexId>> = HashMap::new();

    // 1. Vertices
    let mut vid = 1;
    for vertex_spec in manifest.vertices.iter() {
        let path = manifest_parent_dir.join(&vertex_spec.file.path);
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b'|')
            .from_path(path)?;

        let label_id = graph_type
            .get_label_id(&vertex_spec.label)?
            .expect("label id not found");

        for record in rdr.records() {
            let record = record?;
            let label_set = LabelSet::from_iter(vec![label_id]);
            let props_schema = graph_type
                .get_vertex_type(&label_set)?
                .expect("vertex type not found")
                .properties();

            assert_eq!(props_schema.len() + 1, record.len());
            let old_vid_str = record.get(0).expect("record to short");
            if old_vid_str.is_empty() {
                eprintln!("Warning: Empty vertex ID in vertex '{}'. Skipping record.", vertex_spec.label);
                continue;
            }
            let old_vid: VertexId = match old_vid_str.parse() {
                Ok(vid) => vid,
                Err(e) => {
                    eprintln!("Warning: Cannot parse vertex ID '{}' in vertex '{}': {}. Skipping record.", 
                        old_vid_str, vertex_spec.label, e);
                    continue;
                }
            };

            // 构建属性，如果失败则跳过该记录
            let props = match build_properties(props_schema, record.iter().skip(1)) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Warning: Failed to build properties for vertex '{}' with ID '{}': {}. Skipping record.", 
                        vertex_spec.label, old_vid, e);
                    continue;
                }
            };
            let vertex = Vertex::new(vid, label_id, PropertyRecord::new(props));

            graph.create_vertex(&txn, vertex)?;
            // Update vid mapping
            let entry = vid_mapping.entry(label_id).or_default();
            if entry.insert(old_vid, vid).is_some() {
                print!("conflict");
            }
            vid += 1;
        }
    }

    // 2. Edges
    let mut eid = 1;
    for edge_spec in manifest.edges.iter() {
        let path = manifest_parent_dir.join(&edge_spec.file.path);
        let label_id = graph_type
            .get_label_id(&edge_spec.label)?
            .expect("label id not found");

        let mut rdr = ReaderBuilder::new().has_headers(true).delimiter(b'|').from_path(path)?;

        for record in rdr.records() {
            let record = record?;
            let label_set = LabelSet::from_iter(vec![label_id]);

            let props = graph_type
                .get_edge_type(&label_set)?
                .expect("edge type not found")
                .properties();

            //assert_eq!(record.len() - 3, props.len());
            
            if record.len() < 3 {
                eprintln!("Warning: Edge record too short in edge '{}'. Expected at least 3 columns, got {}. Skipping record.", 
                    edge_spec.label, record.len());
                eprintln!("  Record: {:?}", record.iter().collect::<Vec<_>>());
                continue;
            }
            
            let old_src_id_str = record.get(1).unwrap();
            let old_dst_id_str = record.get(2).unwrap();
            
            if old_src_id_str.is_empty() || old_dst_id_str.is_empty() {
                eprintln!("Warning: Empty source or destination ID in edge '{}'. Skipping record.", edge_spec.label);
                eprintln!("  Record: {:?}", record.iter().collect::<Vec<_>>());
                continue;
            }
            
            let old_src_id: VertexId = match old_src_id_str.parse() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Warning: Cannot parse source ID '{}' in edge '{}': {}. Skipping record.", 
                        old_src_id_str, edge_spec.label, e);
                    eprintln!("  Record: {:?}", record.iter().collect::<Vec<_>>());
                    continue;
                }
            };
            
            let old_dst_id: VertexId = match old_dst_id_str.parse() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Warning: Cannot parse destination ID '{}' in edge '{}': {}. Skipping record.", 
                        old_dst_id_str, edge_spec.label, e);
                    eprintln!("  Record: {:?}", record.iter().collect::<Vec<_>>());
                    continue;
                }
            };
            let src_label_id = graph_type.get_label_id(&edge_spec.src_label)?.expect("label id not found");
            let end_label_id = graph_type.get_label_id(&edge_spec.dst_label)?.expect("label id not found");
            
            let src_id = match vid_mapping.get(&src_label_id).expect("should not null").get(&old_src_id) {
                Some(id) => id,
                None => {
                    eprintln!("Warning: Source vertex ID {} not found in mapping for edge '{}'. Skipping record.", 
                        old_src_id, edge_spec.label);
                    continue;
                }
            };

            let dst_id = match vid_mapping.get(&end_label_id).expect("should not null").get(&old_dst_id) {
                Some(id) => id,
                None => {
                    eprintln!("Warning: Destination vertex ID {} not found in mapping for edge '{}'. Skipping record.", 
                        old_dst_id, edge_spec.label);
                    continue;
                }
            };

            let vertex = graph.get_vertex(&txn,*src_id)?;
            let src_label_name = graph_type.get_vertex_type(&LabelSet::from_label(vertex.label_id))?.expect("vertex label not found");

            let vertex2 = graph.get_vertex(&txn,*dst_id)?;
            let dst_label_name = graph_type.get_vertex_type(&LabelSet::from_label(vertex2.label_id))?;

            // 构建属性，如果失败则跳过该记录
            let props = match build_properties(props, record.iter().skip(3)) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Warning: Failed to build properties for edge '{}' (src: {}, dst: {}): {}. Skipping record.", 
                        edge_spec.label, old_src_id, old_dst_id, e);
                    continue;
                }
            };

            let edge = Edge::new(eid, *src_id, *dst_id, label_id, PropertyRecord::new(props));
            graph.create_edge(&txn, edge)?;
            eid += 1;
        }
    }

    let _ = txn.commit()?;

    Ok((graph, graph_type))
}

fn get_graph_type_from_manifest(manifest: &Manifest) -> Result<Arc<MemoryGraphTypeCatalog>> {
    let mut graph_type = MemoryGraphTypeCatalog::new();
    let mut label_vertex_type = HashMap::new();

    // Vertex
    for vs in manifest.vertices_spec().iter() {
        let label = vs.label_name();
        let label_id = graph_type
            .add_label(label.clone())
            .expect("add label failed");
        let label_set = LabelSet::from_iter(vec![label_id]);
        let vertex_type = Arc::new(MemoryVertexTypeCatalog::new(
            label_set.clone(),
            vs.properties().clone(),
        ));
        graph_type.add_vertex_type(label_set, Arc::clone(&vertex_type));

        label_vertex_type.insert(label.clone(), vertex_type);
    }

    // Edge
    for es in manifest.edges_spec().iter() {
        let label_id = graph_type
            .add_label(es.label_name().clone())
            .expect("add label failed");
        let label_set = LabelSet::from_iter(vec![label_id]);
        let src_type = label_vertex_type
            .get(es.src_label())
            .expect("vertex type not found");
        let dst_type = label_vertex_type
            .get(es.dst_label())
            .expect("vertex type not found");

        let edge_type = MemoryEdgeTypeCatalog::new(
            label_set.clone(),
            src_type.clone(),
            dst_type.clone(),
            es.properties().clone(),
        );
        graph_type.add_edge_type(label_set, Arc::new(edge_type));
    }

    Ok(Arc::new(graph_type))
}

pub fn build_procedure() -> Procedure {
    // Name, directory path, Manifest relative path
    let parameters = vec![
        LogicalType::String,
        LogicalType::String,
        LogicalType::String,
    ];

    Procedure::new(parameters, None, |context, args| {
        assert_eq!(args.len(), 3);
        let graph_name = args[0]
            .try_as_string()
            .expect("graph name must be a string")
            .clone()
            .expect("graph name can't be empty");
        let dir_path = args[1]
            .try_as_string()
            .expect("directory path must be a string")
            .clone()
            .expect("directory path can't be empty");
        let manifest_rel_path = args[2]
            .try_as_string()
            .expect("manifest relative path must be a string")
            .clone()
            .expect("manifest relative path can't be empty");

        let manifest_path = (dir_path.as_ref() as &Path).join(manifest_rel_path);
        let schema = context
            .current_schema
            .ok_or_else(|| anyhow::anyhow!("current schema not set"))?;

        let (graph, graph_type) = import(manifest_path)?;

        let container = GraphContainer::new(
            Arc::clone(&graph_type),
            GraphStorage::Memory(Arc::clone(&graph)),
        );

        if !schema.add_graph(graph_name.clone(), Arc::new(container)) {
            return Err(anyhow::anyhow!("graph {graph_name} already exists").into());
        }
        Ok(vec![])
    })
}
