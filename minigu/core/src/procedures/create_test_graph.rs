use std::sync::Arc;

use arrow::array::{ArrayRef, StringArray};
use itertools::Itertools;
use minigu_catalog::memory::graph_type::MemoryGraphTypeCatalog;
use minigu_common::data_chunk;
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::procedure::Procedure;
use minigu_storage::tp::MemoryGraph;

use crate::procedures::common::{create_ckpt_config, create_wal_config};

/// Create a test graph with the given name in the current schema.
pub fn build_procedure() -> Procedure {
    let parameters = vec![LogicalType::String];
    Procedure::new(parameters, None, move |context, args| {
        let graph_name = args[0]
            .try_as_string()
            .expect("arg must be a string")
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("graph name cannot be null"))?;
        let schema = context
            .current_schema
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("current schema not set"))?;

        let ckpt_dir = context.database().config().checkpoint_dir.as_path();
        let wal_path = context.database().config().wal_path.as_path();
        let graph = MemoryGraph::with_config_fresh(
            create_ckpt_config(ckpt_dir),
            create_wal_config(wal_path),
        );
        let mut graph_type = MemoryGraphTypeCatalog::new();
        let container = GraphContainer::new(Arc::new(graph_type), GraphStorage::Memory(graph));
        if !schema.add_graph(graph_name.clone(), Arc::new(container)) {
            return Err(anyhow::anyhow!("graph {graph_name} already exists").into());
        }
        Ok(vec![])
    })
}
