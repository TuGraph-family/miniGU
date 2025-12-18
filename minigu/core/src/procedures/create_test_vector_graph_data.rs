use std::sync::Arc;

use minigu_catalog::label_set::LabelSet;
use minigu_catalog::memory::graph_type::{MemoryGraphTypeCatalog, MemoryVertexTypeCatalog};
use minigu_catalog::named_ref::NamedGraphRef;
use minigu_catalog::property::Property;
use minigu_common::data_type::LogicalType;
use minigu_common::types::VertexId;
use minigu_common::value::{F32, ScalarValue, VectorValue};
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::procedure::Procedure;
use minigu_storage::common::{PropertyRecord, Vertex};
use minigu_storage::tp::MemoryGraph;
use minigu_transaction::IsolationLevel::Serializable;
use minigu_transaction::{GraphTxnManager, Transaction};

/// Creates a test graph with a vector property (dimension 104) and sample data.
pub fn build_procedure() -> Procedure {
    let parameters = vec![LogicalType::String, LogicalType::Int8];

    Procedure::new(parameters, None, move |mut context, args| {
        let graph_name = args[0]
            .try_as_string()
            .expect("arg must be a string")
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("graph name cannot be null"))?
            .to_string();

        let num_vertices = args[1]
            .try_as_int8()
            .expect("arg must be a int")
            .ok_or_else(|| anyhow::anyhow!("num_vertices cannot be null"))?;

        if num_vertices < 0 {
            return Err(anyhow::anyhow!("num_vertices must be >= 0").into());
        }
        let n = num_vertices as usize;

        let schema = context
            .current_schema
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("current schema not set"))?;

        let graph = MemoryGraph::with_config_fresh(Default::default(), Default::default());
        let mut graph_type = MemoryGraphTypeCatalog::new();

        let person_label_id = graph_type.add_label("PERSON".to_string()).unwrap();
        let person_label_set: LabelSet = vec![person_label_id].into_iter().collect();
        let person = Arc::new(MemoryVertexTypeCatalog::new(
            person_label_set.clone(),
            vec![
                Property::new("name".to_string(), LogicalType::String, false),
                Property::new("embedding".to_string(), LogicalType::Vector(104), false),
            ],
        ));
        graph_type.add_vertex_type(person_label_set, person);

        let container = Arc::new(GraphContainer::new(
            Arc::new(graph_type),
            GraphStorage::Memory(graph.clone()),
        ));

        if !schema.add_graph(graph_name.clone(), container.clone()) {
            return Err(anyhow::anyhow!("graph `{graph_name}` already exists").into());
        }

        context.current_graph = Some(NamedGraphRef::new(graph_name.into(), container.clone()));

        let mem = match container.graph_storage() {
            GraphStorage::Memory(m) => Arc::clone(m),
        };

        let txn = mem.txn_manager().begin_transaction(Serializable)?;
        let dimension = 104;

        // Create PERSON vertices with deterministic vector embeddings.
        //
        // Example when n=3:
        //   - person0: name="person0", embedding[0..3]=[0.00, 0.01, 0.02, 0.03]...
        //   - person1: name="person1", embedding[0..3]=[0.01, 0.02, 0.03, 0.04]...
        //   - person2: name="person2", embedding[0..3]=[0.02, 0.03, 0.04, 0.05]...
        //
        // Vertex IDs are sequential from 0..n-1. Embeddings are 104-dim and stable for tests.
        for i in 0..n {
            let mut data = vec![F32::from(0.0); dimension];
            for (idx, item) in data.iter_mut().enumerate() {
                *item = F32::from((i as f32 + idx as f32) / 100.0);
            }
            let vector = VectorValue::new(data, dimension).map_err(|err| anyhow::anyhow!(err))?;
            let vertex = Vertex::new(
                VertexId::from(i as u64),
                person_label_id,
                PropertyRecord::new(vec![
                    ScalarValue::String(Some(format!("person{}", i))),
                    ScalarValue::new_vector(dimension, Some(vector)),
                ]),
            );
            mem.create_vertex(&txn, vertex)?;
        }

        txn.commit()?;
        Ok(vec![])
    })
}
