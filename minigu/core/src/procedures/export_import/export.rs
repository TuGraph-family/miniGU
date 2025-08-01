//! call export(<graph_name>, <dir_path>, <manifest_relative_path>);
//!
//! Export the in-memory graph `<graph_name>` to CSV files plus a JSON `manifest.json` on disk.
//!
//! ## Inputs
//! * `<graph_name>` – Name of the graph in the current schema to export.
//! * `<dir_path>` – Target directory for all output files; it will be created if it doesn't exist.
//! * `<manifest_relative_path>` – Relative path (under `dir_path`) of the manifest file (e.g.
//!   `manifest.json`).
//!
//! ## Output
//! * Returns nothing. On success the files are written; errors (I/O failure, unknown graph, etc.)
//!   are returned via `Result`.

use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use csv::{Writer, WriterBuilder};
use minigu_catalog::provider::{GraphProvider, GraphTypeProvider, SchemaProvider};
use minigu_common::data_type::LogicalType;
use minigu_common::error::not_implemented;
use minigu_common::types::{EdgeId, LabelId, VertexId};
use minigu_common::value::ScalarValue;
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::procedure::Procedure;
use minigu_storage::common::{Edge, Vertex};
use minigu_storage::tp::{IsolationLevel, MemoryGraph};

use crate::procedures::export_import::{Manifest, RecordType, Result, SchemaMetadata};

/// Convert a [`ScalarValue`] back into a *CSV‑ready* string. `NULL` becomes an
/// empty string.
fn scalar_value_to_string(scalar_value: &ScalarValue) -> Result<String> {
    match scalar_value {
        ScalarValue::Int8(value) => Ok(value.map_or(String::new(), |inner| inner.to_string())),
        ScalarValue::Int16(value) => Ok(value.map_or(String::new(), |inner| inner.to_string())),
        ScalarValue::Int32(value) => Ok(value.map_or(String::new(), |inner| inner.to_string())),
        ScalarValue::Int64(value) => Ok(value.map_or(String::new(), |inner| inner.to_string())),
        ScalarValue::UInt8(value) => Ok(value.map_or(String::new(), |inner| inner.to_string())),
        ScalarValue::UInt16(value) => Ok(value.map_or(String::new(), |inner| inner.to_string())),
        ScalarValue::UInt32(value) => Ok(value.map_or(String::new(), |inner| inner.to_string())),
        ScalarValue::UInt64(value) => Ok(value.map_or(String::new(), |inner| inner.to_string())),
        ScalarValue::Boolean(value) => Ok(value.map_or(String::new(), |inner| inner.to_string())),
        ScalarValue::Float32(value) => Ok(value.map_or(String::new(), |inner| inner.to_string())),
        ScalarValue::Float64(value) => Ok(value.map_or(String::new(), |inner| inner.to_string())),
        ScalarValue::String(value) => Ok(value.clone().unwrap_or_default()),
        ScalarValue::Null => Ok(String::new()),
        _ => not_implemented(
            "convert `ScalarValue::Vertex`/`ScalarValue::Edge` to string",
            None,
        ),
    }
}

fn get_graph_from_graph_container(container: Arc<dyn GraphProvider>) -> Result<Arc<MemoryGraph>> {
    let container = container
        .as_any()
        .downcast_ref::<GraphContainer>()
        .ok_or_else(|| anyhow::anyhow!("downcast failed"))?;

    match container.graph_storage() {
        GraphStorage::Memory(graph) => Ok(Arc::clone(graph)),
    }
}

#[derive(Debug)]
struct VerticesBuilder {
    records: HashMap<LabelId, BTreeMap<VertexId, RecordType>>,
    writers: HashMap<LabelId, Writer<File>>,
}

impl VerticesBuilder {
    fn new<P: AsRef<Path>>(dir: P, map: &HashMap<LabelId, String>) -> Result<Self> {
        let mut writers = HashMap::with_capacity(map.len());

        for (&id, label) in map {
            let filename = format!("{}.csv", label);
            let path = dir.as_ref().join(filename);

            writers.insert(id, WriterBuilder::new().from_path(path)?);
        }

        Ok(Self {
            records: HashMap::new(),
            writers,
        })
    }

    fn add_vertex(&mut self, v: &Vertex) -> Result<()> {
        let mut record = Vec::with_capacity(v.properties().len() + 1);
        record.push(v.vid().to_string());

        for prop in v.properties() {
            record.push(scalar_value_to_string(prop)?);
        }

        self.records
            .entry(v.label_id)
            .or_default()
            .insert(v.vid(), record);

        Ok(())
    }

    fn dump(&mut self) -> Result<()> {
        for (label_id, records) in self.records.iter() {
            let w = self.writers.get_mut(label_id).expect("writer not found");

            for (_, record) in records.iter() {
                w.write_record(record)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct EdgesBuilder {
    records: HashMap<LabelId, BTreeMap<EdgeId, RecordType>>,
    writers: HashMap<LabelId, Writer<File>>,
}

impl EdgesBuilder {
    fn new<P: AsRef<Path>>(dir: P, map: &HashMap<LabelId, String>) -> Result<Self> {
        let mut writers = HashMap::with_capacity(map.len());

        for (&id, label) in map {
            let filename = format!("{}.csv", label);
            let path = dir.as_ref().join(filename);

            writers.insert(id, WriterBuilder::new().from_path(path)?);
        }

        Ok(Self {
            records: HashMap::new(),
            writers,
        })
    }

    fn add_edge(&mut self, e: &Edge) -> Result<()> {
        let mut record = Vec::with_capacity(e.properties().len() + 3);
        record.extend_from_slice(&[
            e.eid().to_string(),
            e.src_id().to_string(),
            e.dst_id().to_string(),
        ]);

        for prop in e.properties() {
            record.push(scalar_value_to_string(prop)?);
        }

        self.records
            .entry(e.label_id)
            .or_default()
            .insert(e.eid(), record);
        Ok(())
    }

    fn dump(&mut self) -> Result<()> {
        for (label_id, records) in self.records.iter() {
            let w = self.writers.get_mut(label_id).expect("writers not found");

            for (_, record) in records.iter() {
                w.write_record(record)?;
            }
        }

        Ok(())
    }
}

pub(crate) fn export<P: AsRef<Path>>(
    graph: Arc<MemoryGraph>,
    dir: P,
    manifest_rel_path: P, // relative path
    graph_type: Arc<dyn GraphTypeProvider>,
) -> Result<()> {
    let txn = graph.begin_transaction(IsolationLevel::Serializable);

    // 1. Prepare output paths
    let dir = dir.as_ref();
    std::fs::create_dir_all(dir)?;

    let metadata = SchemaMetadata::from_schema(Arc::clone(&graph_type))?;

    let mut vertice_builder = VerticesBuilder::new(dir, &metadata.label_map)?;
    let mut edges_builder = EdgesBuilder::new(dir, &metadata.label_map)?;

    // 2. Dump vertices
    for v in txn.iter_vertices() {
        vertice_builder.add_vertex(&v?)?;
    }
    vertice_builder.dump()?;

    // 3. Dump edge
    for e in txn.iter_edges() {
        edges_builder.add_edge(&e?)?;
    }
    edges_builder.dump()?;

    // 4. Dump manifest
    let manifest = Manifest::from_schema(metadata)?;
    std::fs::write(
        dir.join(manifest_rel_path),
        serde_json::to_string(&manifest)?,
    )?;

    txn.commit()?;

    Ok(())
}

pub fn build_procedure() -> Procedure {
    // Name, directory path, manifest relative path
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
            .expect("directory can't be empty");
        let manifest_rel_path = args[2]
            .try_as_string()
            .expect("manifest relative path must be a string")
            .clone()
            .expect("manifest relative path can't be empty");

        let schema = context
            .current_schema
            .ok_or_else(|| anyhow::anyhow!("current schema not set"))?;
        let graph_container = schema
            .get_graph(&graph_name)?
            .ok_or_else(|| anyhow::anyhow!("graph type named with {} not found", graph_name))?;
        let graph_type = graph_container.graph_type();
        let graph = get_graph_from_graph_container(graph_container)?;

        export(graph, dir_path, manifest_rel_path, graph_type)?;

        Ok(vec![])
    })
}
