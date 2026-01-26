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
//!
//! call import_graph(<graph_name>, <manifest_path>);
//!
//! Import a graph from CSV files plus a JSON `manifest.json` on disk into an in-memory graph,
//! then register it in the current schema under `<graph_name>`.
//!
//! ## Inputs
//! * `<graph_name>` – Name to register the imported graph under in the current schema.
//! * `<manifest_path>` – `manifest.json` path.
//!
//! ## Output
//! * Returns nothing. On success the graph is added to the current schema. Errors (missing files,
//!   schema mismatch, duplicate graph name, etc.) are surfaced via `Result`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

use csv::ReaderBuilder;
use minigu_catalog::label_set::LabelSet;
use minigu_catalog::memory::graph_type::{
    MemoryEdgeTypeCatalog, MemoryGraphTypeCatalog, MemoryVertexTypeCatalog,
};
use minigu_catalog::property::Property;
use minigu_catalog::provider::{GraphTypeProvider, SchemaProvider};
use minigu_common::data_type::{DataSchema, LogicalType};
use minigu_common::error::not_implemented;
use minigu_common::types::VertexId;
use minigu_common::value::ScalarValue;
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::procedure::Procedure;
use minigu_context::session::SessionContext;
use minigu_storage::common::{Edge, PropertyRecord, Vertex};
use minigu_storage::tp::MemoryGraph;
use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

use super::common::{EdgeSpec, FileSpec, Manifest, RecordType, Result, VertexSpec};

// ============================================================================
// Import-specific implementation
// ============================================================================

fn build_manifest<P: AsRef<Path>>(manifest_path: P) -> Result<Manifest> {
    let data = std::fs::read(manifest_path)?;

    let data_str = std::str::from_utf8(&data)?;
    Manifest::from_str(data_str)
}

/// Convert a *string* coming from CSV into an owned [`ScalarValue`] according
/// to a given property definition.
fn property_to_scalar_value(property: &Property, value: &str) -> Result<ScalarValue> {
    if value.is_empty() && property.nullable() {
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
    }

    match property.logical_type() {
        LogicalType::Int8 => Ok(ScalarValue::Int8(Some(value.parse()?))),
        LogicalType::Int16 => Ok(ScalarValue::Int16(Some(value.parse()?))),
        LogicalType::Int32 => Ok(ScalarValue::Int32(Some(value.parse()?))),
        LogicalType::Int64 => Ok(ScalarValue::Int64(Some(value.parse()?))),
        LogicalType::UInt8 => Ok(ScalarValue::UInt8(Some(value.parse()?))),
        LogicalType::UInt16 => Ok(ScalarValue::UInt16(Some(value.parse()?))),
        LogicalType::UInt32 => Ok(ScalarValue::UInt32(Some(value.parse()?))),
        LogicalType::UInt64 => Ok(ScalarValue::UInt64(Some(value.parse()?))),
        LogicalType::Boolean => Ok(ScalarValue::Boolean(Some(value.parse()?))),
        LogicalType::Float32 => Ok(ScalarValue::Float32(Some(value.parse()?))),
        LogicalType::Float64 => Ok(ScalarValue::Float64(Some(value.parse()?))),
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

pub fn import<P: AsRef<Path>>(
    context: SessionContext,
    graph_name: impl Into<String>,
    manifest_path: P,
) -> Result<()> {
    let graph_name = graph_name.into();
    let schema = context
        .current_schema
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("current schema not set"))?;

    if schema.get_graph(&graph_name)?.is_some() {
        return Err(anyhow::anyhow!("graph {graph_name} already exists").into());
    }

    let (graph, graph_type) = import_internal(manifest_path.as_ref())?;

    let container = GraphContainer::new(
        Arc::clone(&graph_type),
        GraphStorage::Memory(Arc::clone(&graph)),
    );

    if !schema.add_graph(graph_name.clone(), Arc::new(container)) {
        return Err(anyhow::anyhow!("graph {graph_name} already exists").into());
    }

    Ok(())
}

pub(crate) fn import_internal<P: AsRef<Path>>(
    manifest_path: P,
) -> Result<(Arc<MemoryGraph>, Arc<MemoryGraphTypeCatalog>)> {
    // Graph type
    let manifest = build_manifest(&manifest_path)?;
    let graph_type = get_graph_type_from_manifest(&manifest)?;

    // Graph
    let graph = MemoryGraph::in_memory();
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
    let mut vid_mapping = HashMap::new();

    // 1. Vertices
    let mut vid = 1;
    for vertex_spec in manifest.vertices.iter() {
        let path = manifest_parent_dir.join(&vertex_spec.file.path);
        let mut rdr = ReaderBuilder::new().has_headers(false).from_path(path)?;

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
            let old_vid: VertexId = record.get(0).expect("record to short").parse()?;

            let props = build_properties(props_schema, record.iter().skip(1))?;
            let vertex = Vertex::new(vid, label_id, PropertyRecord::new(props));

            graph.create_vertex(&txn, vertex)?;
            // Update vid mapping
            vid_mapping.insert(old_vid, vid);
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

        let mut rdr = ReaderBuilder::new().has_headers(false).from_path(path)?;

        for record in rdr.records() {
            let record = record?;
            let label_set = LabelSet::from_iter(vec![label_id]);

            let props = graph_type
                .get_edge_type(&label_set)?
                .expect("edge type not found")
                .properties();

            assert_eq!(record.len() - 3, props.len());
            let old_src_id = record.get(1).expect("record to short").parse()?;
            let old_dst_id = record.get(2).expect("record to short").parse()?;
            let src_id = vid_mapping.get(&old_src_id).expect("vid mapping not found");
            let dst_id = vid_mapping.get(&old_dst_id).expect("vid mapping not found");

            let props = build_properties(props, record.iter().skip(3))?;

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
    let parameters = vec![LogicalType::String, LogicalType::String];

    Procedure::new(parameters, None, |context, args| {
        assert_eq!(args.len(), 2);
        let graph_name = args[0]
            .try_as_string()
            .expect("graph name must be a string")
            .clone()
            .expect("graph name can't be empty");
        let manifest_path = args[1]
            .try_as_string()
            .expect("manifest path must be a string")
            .clone()
            .expect("manifest path can't be empty");

        import(context, graph_name, manifest_path)?;

        Ok(vec![])
    })
}
