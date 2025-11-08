use std::sync::Arc;

use minigu_catalog::label_set::LabelSet;
use minigu_catalog::memory::graph_type::{
    MemoryEdgeTypeCatalog, MemoryGraphTypeCatalog, MemoryVertexTypeCatalog,
};
use minigu_catalog::named_ref::NamedGraphRef;
use minigu_catalog::property::Property;
use minigu_common::data_type::LogicalType;
use minigu_common::types::{EdgeId, VertexId};
use minigu_common::value::ScalarValue;
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::procedure::Procedure;
use minigu_storage::common::{Edge, PropertyRecord, Vertex};
use minigu_storage::tp::MemoryGraph;
use minigu_transaction::IsolationLevel::Serializable;
use minigu_transaction::{GraphTxnManager, Transaction};


/// Create a test graph data with the given name in the current schema.
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
        let friend_label_id = graph_type.add_label("FRIEND".to_string()).unwrap();
        let person_label_set: LabelSet = vec![person_label_id].into_iter().collect();
        let person = Arc::new(MemoryVertexTypeCatalog::new(
            person_label_set.clone(),
            vec![
                Property::new("name".to_string(), LogicalType::String, false),
                Property::new("age".to_string(), LogicalType::Int8, false),
            ],
        ));
        let friend_label_set: LabelSet = vec![friend_label_id].into_iter().collect();
        let friend = Arc::new(MemoryEdgeTypeCatalog::new(
            friend_label_set.clone(),
            person.clone(),
            person.clone(),
            vec![Property::new(
                "distance".to_string(),
                LogicalType::Int32,
                false,
            )],
        ));

        graph_type.add_vertex_type(person_label_set, person);
        graph_type.add_edge_type(friend_label_set, friend);
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

        let mut id_map: Vec<u64> = Vec::with_capacity(n);
        for _i in 0..n as u64 {
            let vertex = Vertex::new(
                VertexId::from(_i),
                person_label_id,
                PropertyRecord::new(vec![
                    ScalarValue::String(Some(format!("bob{}", _i).to_string())),
                    ScalarValue::Int8(Some(20 + _i as i8)),
                ]),
            );
            mem.create_vertex(&txn, vertex);
            id_map.push(_i);
        }

        let mut created_edges: usize = 0;
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let src = id_map[i];
                let dst = id_map[j];
                let edge = Edge::new(
                    EdgeId::from((i * j) as u64),
                    src,
                    dst,
                    friend_label_id,
                    PropertyRecord::new(vec![ScalarValue::Int32(Some(i as i32 * j as i32))]),
                );
                mem.create_edge(&txn, edge);
                created_edges += 1;
            }
        }
        txn.commit()?;
        Ok(vec![])
    })
}