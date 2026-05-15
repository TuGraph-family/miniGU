use std::sync::Arc;

use arrow::array::{ArrayRef, StringArray};
use itertools::Itertools;
use minigu_catalog::memory::MemoryCatalog;
use minigu_catalog::memory::directory::MemoryDirectoryCatalog;
use minigu_catalog::memory::graph_type::MemoryGraphTypeCatalog;
use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_catalog::provider::{DirectoryOrSchema, GraphProvider, SchemaProvider};
use minigu_common::data_chunk;
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_common::value::ScalarValue;
use minigu_context::database::{DatabaseConfig, DatabaseContext};
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::procedure::Procedure;
use minigu_context::runtime::DatabaseRuntime;
use minigu_context::session::SessionContext;
use minigu_storage::tp::MemoryGraph;
use minigu_transaction::{GraphTxnManager, IsolationLevel, LockStrategy, TxnOptions};

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
        let graph = MemoryGraph::in_memory_with_options(context.database().config().txn_options);
        let mut graph_type = MemoryGraphTypeCatalog::new();
        let container = GraphContainer::new(Arc::new(graph_type), GraphStorage::Memory(graph));
        if !schema.add_graph(graph_name.clone(), Arc::new(container)) {
            return Err(anyhow::anyhow!("graph {graph_name} already exists").into());
        }
        Ok(vec![])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_session_with_txn_options(
        txn_options: TxnOptions,
    ) -> (SessionContext, Arc<MemorySchemaCatalog>) {
        let root = Arc::new(MemoryDirectoryCatalog::new(None));
        let parent = Arc::downgrade(&root);
        let schema = Arc::new(MemorySchemaCatalog::new(Some(parent)));
        assert!(root.add_child(
            "default".to_string(),
            DirectoryOrSchema::Schema(schema.clone())
        ));

        let catalog = MemoryCatalog::new(DirectoryOrSchema::Directory(root));
        let runtime = DatabaseRuntime::new(1).unwrap();
        let database = Arc::new(DatabaseContext::new(
            catalog,
            runtime,
            DatabaseConfig {
                txn_options,
                ..DatabaseConfig::default()
            },
        ));

        let mut context = SessionContext::new(database);
        context.current_schema = Some(schema.clone());
        (context, schema)
    }

    #[test]
    fn create_test_graph_uses_database_txn_options_default_lock() {
        let (context, schema) = build_session_with_txn_options(TxnOptions {
            default_lock: LockStrategy::Optimistic,
            ..Default::default()
        });

        let procedure = build_procedure();
        procedure
            .call(
                context,
                vec![ScalarValue::String(Some("g_opt".to_string()))],
            )
            .unwrap();

        let graph_ref = schema
            .get_graph("g_opt")
            .unwrap()
            .expect("graph should exist");
        let container = graph_ref
            .downcast_arc::<GraphContainer>()
            .expect("graph should be GraphContainer");
        let graph = match container.graph_storage() {
            GraphStorage::Memory(graph) => Arc::clone(graph),
        };
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Snapshot)
            .unwrap();
        assert_eq!(txn.lock_strategy(), LockStrategy::Optimistic);
    }
}
