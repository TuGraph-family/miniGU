//! call import(<graph_name>, <dir_path>, <manifest_relative_path>) return *;

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
use minigu_common::data_type::{DataSchema, LogicalType};
use minigu_common::types::VertexId;
use minigu_common::value::ScalarValue;
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::procedure::Procedure;
use minigu_storage::common::{Edge, PropertyRecord, Vertex};
use minigu_storage::tp::{IsolationLevel, MemoryGraph};

use crate::procedures::export_import::{Manifest, Result};

fn build_manifest<P: AsRef<Path>>(manifest_path: P) -> Result<Manifest> {
    let data = std::fs::read(manifest_path)?;
    let data_str = std::str::from_utf8(&data)?;
    Manifest::from_str(data_str)
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
        LogicalType::Null => ScalarValue::Null,
        _ => todo!(),
    }
}

pub(crate) fn import<P: AsRef<Path>>(
    manifest_path: P,
) -> Result<(Arc<MemoryGraph>, Arc<MemoryGraphTypeCatalog>)> {
    // Graph type
    let manifest = build_manifest(&manifest_path)?;
    let graph_type = get_graph_type_from_manifest(&manifest)?;

    // Graph
    let graph = MemoryGraph::new();
    let txn = graph.begin_transaction(IsolationLevel::Serializable);

    let manifest_parent_dir = manifest_path.as_ref().parent().unwrap();
    // Map each original vertex ID to it's newly assigned ID.
    let mut vid_mapping = HashMap::new();

    // 1. Vertices
    let mut vid = 1;
    for vertex_spec in manifest.vertices.iter() {
        let path = manifest_parent_dir.join(&vertex_spec.file.path);
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(path)
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
            let old_vid: VertexId = record.get(0).unwrap().parse().unwrap();

            let props = props_schema
                .iter()
                .zip(record.iter().skip(1)) // skip vid
                .map(|((_, p), value)| property_to_scalar_value(p, value))
                .collect();
            let vertex = Vertex::new(vid, label_id, PropertyRecord::new(props));

            graph.create_vertex(&txn, vertex).unwrap();
            // Update vid mapping
            vid_mapping.insert(old_vid, vid);
            vid += 1;
        }
    }

    // 2. Edges
    let mut eid = 1;
    for edge_spec in manifest.edges.iter() {
        let path = manifest_parent_dir.join(&edge_spec.file.path);
        let label_id = graph_type.get_label_id(&edge_spec.label).unwrap().unwrap();

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(path)
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
            let old_src_id = record.get(1).unwrap().parse().unwrap();
            let old_dst_id = record.get(2).unwrap().parse().unwrap();
            let src_id = vid_mapping.get(&old_src_id).unwrap();
            let dst_id = vid_mapping.get(&old_dst_id).unwrap();

            let props = props
                .iter()
                .zip(record.iter().skip(3)) // skip eid, src and dst
                .map(|((_, p), value)| property_to_scalar_value(p, value))
                .collect();

            let edge = Edge::new(eid, *src_id, *dst_id, label_id, PropertyRecord::new(props));
            graph.create_edge(&txn, edge).unwrap();
            eid += 1;
        }
    }

    txn.commit().unwrap();

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
        let src_type = label_vertex_type.get(es.src_label()).unwrap();
        let dst_type = label_vertex_type.get(es.dst_label()).unwrap();

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

    let schema = Arc::new(DataSchema::new(vec![]));

    Procedure::new(parameters, Some(schema), |context, args| {
        assert_eq!(args.len(), 3);
        let graph_name = args[0]
            .try_as_string()
            .expect("arg[0] must be a string")
            .clone()
            .expect("arg[0] can't be empty");
        let dir_path = args[1]
            .try_as_string()
            .expect("arg[1] must be a string")
            .clone()
            .expect("arg[1] can't be empty");
        let manifest_rel_path = args[2]
            .try_as_string()
            .expect("arg[2] must be a string")
            .clone()
            .expect("arg[2] can't be empty");

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
