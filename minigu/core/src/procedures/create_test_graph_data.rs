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

/// Creates a test graph with multiple vertex types (PERSON, COMPANY, CITY) and edge types (FRIEND,
/// WORKS_AT, LOCATED_IN) with sample data.
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

        // Add labels
        let person_label_id = graph_type.add_label("PERSON".to_string()).unwrap();
        let company_label_id = graph_type.add_label("COMPANY".to_string()).unwrap();
        let city_label_id = graph_type.add_label("CITY".to_string()).unwrap();
        let friend_label_id = graph_type.add_label("FRIEND".to_string()).unwrap();
        let works_at_label_id = graph_type.add_label("WORKS_AT".to_string()).unwrap();
        let located_in_label_id = graph_type.add_label("LOCATED_IN".to_string()).unwrap();

        // Create vertex types
        let person_label_set: LabelSet = vec![person_label_id].into_iter().collect();
        let person = Arc::new(MemoryVertexTypeCatalog::new(
            person_label_set.clone(),
            vec![
                Property::new("name".to_string(), LogicalType::String, false),
                Property::new("age".to_string(), LogicalType::Int8, false),
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

        let city_label_set: LabelSet = vec![city_label_id].into_iter().collect();
        let city = Arc::new(MemoryVertexTypeCatalog::new(
            city_label_set.clone(),
            vec![
                Property::new("name".to_string(), LogicalType::String, false),
                Property::new("population".to_string(), LogicalType::Int32, false),
            ],
        ));

        // Create edge types
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

        let works_at_label_set: LabelSet = vec![works_at_label_id].into_iter().collect();
        let works_at = Arc::new(MemoryEdgeTypeCatalog::new(
            works_at_label_set.clone(),
            person.clone(),
            company.clone(),
            vec![Property::new(
                "since".to_string(),
                LogicalType::Int32,
                false,
            )],
        ));

        let located_in_label_set: LabelSet = vec![located_in_label_id].into_iter().collect();
        let located_in = Arc::new(MemoryEdgeTypeCatalog::new(
            located_in_label_set.clone(),
            company.clone(),
            city.clone(),
            vec![Property::new(
                "address".to_string(),
                LogicalType::String,
                false,
            )],
        ));

        graph_type.add_vertex_type(person_label_set, person);
        graph_type.add_vertex_type(company_label_set, company);
        graph_type.add_vertex_type(city_label_set, city);
        graph_type.add_edge_type(friend_label_set, friend);
        graph_type.add_edge_type(works_at_label_set, works_at);
        graph_type.add_edge_type(located_in_label_set, located_in);
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

        // Create vertices - reduce total number
        // Example when n=5:
        //   - num_persons = 2 (person0, person1)
        //   - num_companies = 1 (company0)
        //   - num_cities = 1 (city0)
        let num_persons = (n / 2).max(1);
        let num_companies = (n / 4).max(1);
        let num_cities = (n / 4).max(1);

        let mut person_ids: Vec<u64> = Vec::with_capacity(num_persons);
        let mut company_ids: Vec<u64> = Vec::with_capacity(num_companies);
        let mut city_ids: Vec<u64> = Vec::with_capacity(num_cities);

        // Create PERSON vertices
        for i in 0..num_persons {
            let vid = i as u64;
            let vertex = Vertex::new(
                VertexId::from(vid),
                person_label_id,
                PropertyRecord::new(vec![
                    ScalarValue::String(Some(format!("person{}", i))),
                    ScalarValue::Int8(Some(20 + i as i8)),
                ]),
            );
            mem.create_vertex(&txn, vertex)?;
            person_ids.push(vid);
        }

        // Create COMPANY vertices
        let company_start_id = num_persons as u64;
        for i in 0..num_companies {
            let vid = company_start_id + i as u64;
            let vertex = Vertex::new(
                VertexId::from(vid),
                company_label_id,
                PropertyRecord::new(vec![
                    ScalarValue::String(Some(format!("company{}", i))),
                    ScalarValue::Int64(Some(1000000 * (i + 1) as i64)),
                ]),
            );
            mem.create_vertex(&txn, vertex)?;
            company_ids.push(vid);
        }

        // Create CITY vertices
        let city_start_id = company_start_id + num_companies as u64;
        for i in 0..num_cities {
            let vid = city_start_id + i as u64;
            let vertex = Vertex::new(
                VertexId::from(vid),
                city_label_id,
                PropertyRecord::new(vec![
                    ScalarValue::String(Some(format!("city{}", i))),
                    ScalarValue::Int32(Some(100000 * (i + 1) as i32)),
                ]),
            );
            mem.create_vertex(&txn, vertex)?;
            city_ids.push(vid);
        }

        // Create edges - reduce total number
        //
        // Example when n=5:
        //   Vertices: person0, person1, company0, city0
        //
        //   FRIEND edges (fully connected):
        //     - person0 <-> person1 (1 edge, bidirectional representation)
        //     Total: 1 FRIEND edge
        //
        //   WORKS_AT edges (partial connection - only 60% employed):
        //     - person0 -> company0 (person1 has no job)
        //     Total: 1 WORKS_AT edge
        //
        //   LOCATED_IN edges:
        //     - company0 -> city0
        //     Total: 1 LOCATED_IN edge
        //
        //   Graph structure:
        //     person0 <--FRIEND--> person1
        //       |                    |
        //    WORKS_AT            (no job)
        //       |
        //    company0
        //       |
        //   LOCATED_IN
        //       |
        //     city0
        //
        let mut edge_id_counter = 0u64;

        // Create FRIEND edges - fully connected graph between all persons
        // Every person is friends with every other person (complete graph)
        // This creates a complete subgraph: PERSON-FRIEND-PERSON yields all persons
        for i in 0..num_persons {
            for j in 0..num_persons {
                let edge = Edge::new(
                    EdgeId::from(edge_id_counter),
                    person_ids[i],
                    person_ids[j],
                    friend_label_id,
                    PropertyRecord::new(vec![ScalarValue::Int32(Some((i + j) as i32))]),
                );
                mem.create_edge(&txn, edge)?;
                edge_id_counter += 1;
            }
        }

        // Create WORKS_AT edges - NOT all persons have jobs (partial connection)
        // This means PERSON-WORKS_AT-COMPANY does NOT yield all persons (some are unemployed)
        let num_employed = (num_persons * 3 / 5).max(1); // 60% of persons have jobs
        for (i, _) in person_ids.iter().enumerate().take(num_employed) {
            let company_idx = i % num_companies;
            let edge = Edge::new(
                EdgeId::from(edge_id_counter),
                person_ids[i],
                company_ids[company_idx],
                works_at_label_id,
                PropertyRecord::new(vec![ScalarValue::Int32(Some(2020 + (i % 5) as i32))]),
            );
            mem.create_edge(&txn, edge)?;
            edge_id_counter += 1;
        }

        // Create LOCATED_IN edges (each company is located in one city)
        for (i, _) in company_ids.iter().enumerate().take(num_companies) {
            let city_idx = i % num_cities;
            let edge = Edge::new(
                EdgeId::from(edge_id_counter),
                company_ids[i],
                city_ids[city_idx],
                located_in_label_id,
                PropertyRecord::new(vec![ScalarValue::String(Some(format!("address{}", i)))]),
            );
            mem.create_edge(&txn, edge)?;
            edge_id_counter += 1;
        }

        txn.commit()?;
        Ok(vec![])
    })
}
