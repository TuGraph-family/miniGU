use std::sync::Arc;

use minigu_catalog::memory::graph_type::MemoryGraphTypeCatalog;
use minigu_catalog::named_ref::NamedGraphRef;
use minigu_common::data_type::LogicalType;
use minigu_common::types::{EdgeId, LabelId, VertexId};
use minigu_common::value::ScalarValue;
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::procedure::Procedure;
use minigu_storage::common::{Edge, PropertyRecord, Vertex};
use minigu_storage::tp::MemoryGraph;
use minigu_transaction::IsolationLevel::Serializable;
use minigu_transaction::{GraphTxnManager, Transaction};

/// Creates test graph and data with the given name in the current schema.
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

        let graph = MemoryGraph::new();
        let graph_type = Arc::new(MemoryGraphTypeCatalog::new());
        let container = Arc::new(GraphContainer::new(
            graph_type.clone(),
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

        const PERSON_LABEL_ID: LabelId = LabelId::new(1).unwrap();
        const FRIEND_LABEL_ID: LabelId = LabelId::new(2).unwrap();

        let mut id_map: Vec<u64> = Vec::with_capacity(n);
        for _i in 0..n as u64 {
            let vertex = Vertex::new(
                VertexId::from(_i),
                PERSON_LABEL_ID,
                PropertyRecord::new(vec![ScalarValue::String(Some("per".to_string()))]),
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
                    EdgeId::from((i * n + j) as u64),
                    src,
                    dst,
                    FRIEND_LABEL_ID,
                    PropertyRecord::new(vec![ScalarValue::String(Some("2024-03-01".to_string()))]),
                );
                mem.create_edge(&txn, edge);
                created_edges += 1;
            }
        }
        txn.commit()?;
        Ok(vec![])
    })
}
