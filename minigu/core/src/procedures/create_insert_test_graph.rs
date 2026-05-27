//! Test fixture procedure for the INSERT statement test suite.
//!
//! Builds a small graph with only primitive-typed (String / Int) properties so
//! INSERT can populate it from GQL literals (no vector literal syntax exists yet).
//!
//! Schema:
//!   PERSON   (name: String, age: Int32)
//!   COMPANY  (name: String, revenue: Int64)
//!   KNOWS    PERSON  -> PERSON   (since: Int32)
//!   WORKS_AT PERSON  -> COMPANY  (role:  String)
//!
//! The procedure registers the graph in the current schema and sets it as the
//! session's current graph, but inserts no data — INSERT test cases are
//! responsible for populating their own data.

use std::sync::Arc;

use minigu_catalog::label_set::LabelSet;
use minigu_catalog::memory::graph_type::{
    MemoryEdgeTypeCatalog, MemoryGraphTypeCatalog, MemoryVertexTypeCatalog,
};
use minigu_catalog::named_ref::NamedGraphRef;
use minigu_catalog::property::Property;
use minigu_common::data_type::LogicalType;
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::procedure::Procedure;
use minigu_storage::tp::MemoryGraph;

pub fn build_procedure() -> Procedure {
    let parameters = vec![LogicalType::String];

    Procedure::new(parameters, None, move |mut context, args| {
        if args.len() != 1 {
            return Err(anyhow::anyhow!(
                "create_insert_test_graph expects 1 argument, got {}",
                args.len()
            )
            .into());
        }
        let graph_name = args[0]
            .try_as_string()
            .ok_or_else(|| anyhow::anyhow!("graph name must be a string"))?
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("graph name cannot be null"))?
            .to_string();

        let schema = context
            .current_schema
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("current schema not set"))?;

        let graph = MemoryGraph::in_memory_with_options(context.database().config().txn_options);
        let mut graph_type = MemoryGraphTypeCatalog::new();

        // Labels
        let person_label_id = graph_type
            .add_label("PERSON".to_string())
            .ok_or_else(|| anyhow::anyhow!("failed to register PERSON label"))?;
        let company_label_id = graph_type
            .add_label("COMPANY".to_string())
            .ok_or_else(|| anyhow::anyhow!("failed to register COMPANY label"))?;
        let knows_label_id = graph_type
            .add_label("KNOWS".to_string())
            .ok_or_else(|| anyhow::anyhow!("failed to register KNOWS label"))?;
        let works_at_label_id = graph_type
            .add_label("WORKS_AT".to_string())
            .ok_or_else(|| anyhow::anyhow!("failed to register WORKS_AT label"))?;

        // Vertex types
        let person_label_set: LabelSet = vec![person_label_id].into_iter().collect();
        let person = Arc::new(MemoryVertexTypeCatalog::new(
            person_label_set.clone(),
            vec![
                Property::new("name".to_string(), LogicalType::String, false),
                Property::new("age".to_string(), LogicalType::Int32, false),
            ],
        ));

        let company_label_set: LabelSet = vec![company_label_id].into_iter().collect();
        let company = Arc::new(MemoryVertexTypeCatalog::new(
            company_label_set.clone(),
            vec![
                Property::new("name".to_string(), LogicalType::String, false),
                Property::new("revenue".to_string(), LogicalType::Int64, false),
            ],
        ));

        // Edge types
        let knows_label_set: LabelSet = vec![knows_label_id].into_iter().collect();
        let knows = Arc::new(MemoryEdgeTypeCatalog::new(
            knows_label_set.clone(),
            person.clone(),
            person.clone(),
            vec![Property::new(
                "since".to_string(),
                LogicalType::Int32,
                false,
            )],
        ));

        let works_at_label_set: LabelSet = vec![works_at_label_id].into_iter().collect();
        let works_at = Arc::new(MemoryEdgeTypeCatalog::new(
            works_at_label_set.clone(),
            person.clone(),
            company.clone(),
            vec![Property::new(
                "role".to_string(),
                LogicalType::String,
                false,
            )],
        ));

        graph_type.add_vertex_type(person_label_set, person);
        graph_type.add_vertex_type(company_label_set, company);
        graph_type.add_edge_type(knows_label_set, knows);
        graph_type.add_edge_type(works_at_label_set, works_at);

        let container = Arc::new(GraphContainer::new(
            Arc::new(graph_type),
            GraphStorage::Memory(graph.clone()),
        ));

        if !schema.add_graph(graph_name.clone(), container.clone()) {
            return Err(anyhow::anyhow!("graph `{graph_name}` already exists").into());
        }

        context.current_graph = Some(NamedGraphRef::new(graph_name.into(), container.clone()));

        Ok(vec![])
    })
}
