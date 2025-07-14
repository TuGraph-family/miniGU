use std::error::Error;
use std::sync::Arc;

use minigu_catalog::provider::{GraphProvider, SchemaProvider};
use minigu_common::data_type::{DataSchema, LogicalType};
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::procedure::Procedure;
use minigu_storage::tp::{IsolationLevel, MemoryGraph};

fn get_graph_from_graph_container(
    container: Arc<dyn GraphProvider>,
) -> Result<Arc<MemoryGraph>, Box<dyn Error + Send + Sync + 'static>> {
    let container = container
        .as_any()
        .downcast_ref::<GraphContainer>()
        .ok_or_else(|| anyhow::anyhow!("downcast failed"))?;

    match container.graph_storage() {
        GraphStorage::Memory(graph) => Ok(Arc::clone(graph)),
        _ => unreachable!(),
    }
}

pub fn build_procedure() -> Procedure {
    // Name, directory path, config relative path
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
        let config_rel_path = args[2]
            .try_as_string()
            .expect("arg[2] must be a string")
            .clone()
            .expect("arg[2] can't be empty");

        let schema = context
            .current_schema
            .ok_or_else(|| anyhow::anyhow!("current schema not set"))?;
        let graph_container = schema
            .get_graph(&graph_name)?
            .ok_or_else(|| anyhow::anyhow!("graph type named with {} not found", graph_name))?;
        let graph_type = graph_container.graph_type();
        let graph = get_graph_from_graph_container(graph_container)?;

        let txn = graph.begin_transaction(IsolationLevel::Serializable);
        graph.import_graph(&txn, config_rel_path, graph_type);

        Ok(vec![])
    })
}
