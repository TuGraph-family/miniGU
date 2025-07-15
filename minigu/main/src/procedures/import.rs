//! call import(<graph_name>, <dir_path>, <manifest_relative_path>) return *;

use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use minigu_catalog::label_set::LabelSet;
use minigu_catalog::memory::graph_type::{
    self, MemoryEdgeTypeCatalog, MemoryGraphTypeCatalog, MemoryVertexTypeCatalog,
};
use minigu_catalog::property::Property;
use minigu_catalog::provider::{GraphProvider, SchemaProvider};
use minigu_common::data_type::{DataSchema, LogicalType};
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::procedure::Procedure;
use minigu_storage::tp::export_import::Manifest;
use minigu_storage::tp::{IsolationLevel, MemoryGraph};

fn get_graph_type_from_manifest<P: AsRef<Path>>(
    manifest_path: P,
) -> Result<MemoryGraphTypeCatalog, Box<dyn Error + Send + Sync + 'static>> {
    let data = std::fs::read(manifest_path)?;
    let data_str = std::str::from_utf8(&data)?;
    let manifest = Manifest::from_str(data_str)?;
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

    Ok(graph_type)
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
            .expect("arg[1] can't be empty");
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

        let graph_type = Arc::new(get_graph_type_from_manifest(manifest_path.as_path())?);
        let graph = MemoryGraph::new();
        let container = GraphContainer::new(
            Arc::clone(&graph_type),
            GraphStorage::Memory(Arc::clone(&graph)),
        );
        if !schema.add_graph(graph_name.clone(), Arc::new(container)) {
            return Err(anyhow::anyhow!("graph {graph_name} already exists").into());
        }

        let txn = graph.begin_transaction(IsolationLevel::Serializable);
        graph.import_graph(&txn, manifest_path, graph_type);
        txn.commit()?;

        Ok(vec![])
    })
}
