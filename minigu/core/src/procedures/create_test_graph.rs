use std::sync::Arc;

use arrow::array::{ArrayRef, StringArray};
use itertools::Itertools;
use minigu_catalog::memory::graph_type::MemoryGraphTypeCatalog;
use minigu_catalog::provider::SchemaProvider;
use minigu_catalog::txn::manager::CatalogTxnManager;
use minigu_common::data_chunk;
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::procedure::Procedure;
use minigu_storage::tp::MemoryGraph;
use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

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
            .ok_or_else(|| anyhow::anyhow!("current schema not set"))?;
        let graph = MemoryGraph::new();
        let graph_type = MemoryGraphTypeCatalog::new();
        let container = GraphContainer::new(Arc::new(graph_type), GraphStorage::Memory(graph));
        // Record the graph in a explicit transaction in the current schema
        let txn = context
            .catalog_txn_mgr
            .begin_transaction(IsolationLevel::Snapshot)?;
        // Existence check
        let exists = schema.get_graph(graph_name, txn.as_ref())?.is_some();
        if exists {
            txn.abort()?;
            return Err(anyhow::anyhow!(format!("graph {} already exists", graph_name)).into());
        }
        schema.add_graph_txn(graph_name.clone(), Arc::new(container), txn.as_ref())?;
        let _ = txn.commit();
        Ok(vec![])
    })
}
