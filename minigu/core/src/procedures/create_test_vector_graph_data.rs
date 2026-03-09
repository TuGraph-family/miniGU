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

const EMBEDDING_DIM: usize = 104;
const EMBEDDING105_DIM: usize = 105;

/// Creates a test graph with vector properties and sample data.
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

        let graph = MemoryGraph::in_memory();
        let mut graph_type = MemoryGraphTypeCatalog::new();

        let person_label_id = graph_type.add_label("PERSON".to_string()).unwrap();
        let person_label_set: LabelSet = vec![person_label_id].into_iter().collect();
        let person = Arc::new(MemoryVertexTypeCatalog::new(
            person_label_set.clone(),
            vec![
                Property::new("name".to_string(), LogicalType::String, false),
                Property::new(
                    "embedding".to_string(),
                    LogicalType::Vector(EMBEDDING_DIM),
                    false,
                ),
                Property::new(
                    "embedding105".to_string(),
                    LogicalType::Vector(EMBEDDING105_DIM),
                    false,
                ),
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

        for i in 0..n {
            let vector = build_vector(i, EMBEDDING_DIM);
            let vector105 = build_vector(i, EMBEDDING105_DIM);
            let vertex = Vertex::new(
                VertexId::from(i as u64),
                person_label_id,
                PropertyRecord::new(vec![
                    ScalarValue::String(Some(format!("person{}", i))),
                    ScalarValue::new_vector(EMBEDDING_DIM, Some(vector)),
                    ScalarValue::new_vector(EMBEDDING105_DIM, Some(vector105)),
                ]),
            );
            mem.create_vertex(&txn, vertex)?;
        }

        txn.commit()?;
        Ok(vec![])
    })
}

fn build_vector(seed: usize, dimension: usize) -> VectorValue {
    let mut data = vec![F32::from(0.0); dimension];
    for (idx, item) in data.iter_mut().enumerate() {
        *item = F32::from((seed as f32 + idx as f32) / 100.0);
    }
    VectorValue::new(data, dimension).expect("vector should be constructable")
}
