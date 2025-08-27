use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::NonZeroU32;
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::time::Instant;

use bitvec::order::Lsb0;
use bitvec::prelude::BitVec;
use dashmap::DashMap;
use minigu_common::types::{LabelId, VertexId};
use minigu_common::value::{F32, F64, ScalarValue};
use minigu_storage::ap::olap_graph::{
    BLOCK_CAPACITY, CompressedEdgeBlock, CompressedPropertyBlock, CompressedPropertyColumn,
    EdgeBlock, OlapEdge, OlapPropertyStore, OlapStorage, OlapStorageEdge, OlapVertex,
    PropertyBlock, PropertyColumn,
};
use minigu_storage::ap::{MutOlapGraph, OlapGraph};
use minigu_storage::model::properties::PropertyRecord;

const PATH: &str = "";

fn mock_olap_graph(property_cnt: u64) -> OlapStorage {
    let storage = OlapStorage {
        logic_id_counter: AtomicU64::new(0),
        dense_id_map: DashMap::new(),
        vertices: RwLock::new(Vec::new()),
        edges: RwLock::new(Vec::new()),
        property_columns: RwLock::new(Vec::new()),
        is_edge_compressed: AtomicBool::new(false),
        compressed_edges: RwLock::new(Vec::new()),
        is_property_compressed: AtomicBool::new(false),
        compressed_properties: RwLock::new(vec![]),
    };

    {
        let mut ref_columns = storage.property_columns.write().unwrap();
        for _i in 0..property_cnt {
            ref_columns.push(PropertyColumn { blocks: Vec::new() })
        }
    }
    storage
}

#[test]
fn create_vertex_test() {
    let storage = mock_olap_graph(0);
    for i in 1..=289 {
        let _result = storage.create_vertex(&(), OlapVertex {
            vid: (i + 30) as VertexId,
            properties: PropertyRecord::new(vec![
                ScalarValue::Int32(Some(i + 100)),
                ScalarValue::String(Some("hello".to_string())),
            ]),
            block_offset: 0,
        });
    }

    let vertices = storage.vertices.read().unwrap();

    assert_eq!(vertices.get(128).unwrap().vid, 159);
    assert_eq!(
        vertices.get(128).unwrap().properties.get(0),
        Some(&ScalarValue::Int32(Some(229)))
    );
    assert_eq!(
        vertices.get(128).unwrap().properties.get(1),
        Some(&ScalarValue::String(Some("hello".to_string())))
    );

    let vertices_len = vertices.len();
    let id_map_len = storage.dense_id_map.len();
    assert_eq!(vertices_len, 289);
    assert_eq!(id_map_len, 289);

    let id = *storage.dense_id_map.get(&129).unwrap();
    assert_eq!(id, 98);
}

#[test]
fn create_edge_test() {
    let storage = mock_olap_graph(1);
    // Insert vertex
    for i in 1u32..=5 {
        let _result = storage.create_vertex(&(), OlapVertex {
            vid: i as VertexId,
            properties: PropertyRecord::default(),
            block_offset: 0,
        });

        for j in 1u32..=(400 - (i - 1) * 10) {
            let _result1 = storage.create_edge(&(), OlapEdge {
                label_id: NonZeroU32::new(i * 10000 + j),
                src_id: i as u64,
                dst_id: ((j - 1) * i) as u64,
                properties: OlapPropertyStore::new(vec![Some(ScalarValue::String(Some(
                    "hello".to_string(),
                )))]),
            });
        }
    }

    let edges = storage.edges.read().unwrap();
    assert_eq!(edges.len(), 5 * 2);
    assert_eq!(edges.get(5).unwrap().edges[0].dst_id, 3 * 256);
    assert_eq!(
        edges.get(4).unwrap().edges[0].label_id,
        NonZeroU32::new(30001)
    );
    assert_eq!(edges.get(3).unwrap().pre_block_index.unwrap(), 2);
    assert_eq!(edges.get(2).unwrap().pre_block_index, None);
    assert_eq!(edges.get(1).unwrap().edge_counter, 144);
    assert_eq!(edges.first().unwrap().src_id, 1);
}

#[test]
fn get_vertex_test() {
    let storage = mock_olap_graph(0);
    for i in 0..289 {
        let _result = storage.create_vertex(&(), OlapVertex {
            vid: (i + 30) as VertexId,
            properties: PropertyRecord::new(vec![
                ScalarValue::Int32(Some(i + 100)),
                ScalarValue::String(Some("hello".to_string())),
            ]),
            block_offset: 0,
        });
    }

    let result1 = storage.get_vertex(&(), 33);
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap().vid, 33);

    let result2 = storage.get_vertex(&(), 63);
    assert!(result2.is_ok());
    assert_eq!(
        result2
            .unwrap()
            .properties
            .get(0)
            .unwrap()
            .get_int32()
            .unwrap(),
        133
    );
}

#[test]
fn get_edge_test() {
    let storage = mock_olap_graph(1);
    // Insert vertex
    for i in 1..=5 {
        let _result = storage.create_vertex(&(), OlapVertex {
            vid: i as VertexId,
            properties: PropertyRecord::default(),
            block_offset: 0,
        });

        for j in 1..=(400 - i * 10) {
            let _result1 = storage.create_edge(&(), OlapEdge {
                label_id: NonZeroU32::new(i * 10000 + j),
                src_id: i as u64,
                dst_id: (j * (i + 1)) as u64,
                properties: OlapPropertyStore::new(vec![Some(ScalarValue::String(Some(
                    "hello".to_string(),
                )))]),
            });
        }
    }

    let result1 = storage.get_edge(&(), NonZeroU32::new(30099));
    println!("{result1:?}");
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap().dst_id, 396);

    let result2 = storage.get_edge(&(), NonZeroU32::new(20333));
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap().label_id, NonZeroU32::new(20333));
}

#[test]
fn vertex_iterator_test() {
    let storage = mock_olap_graph(0);
    for i in 0..500 {
        let _result = storage.create_vertex(&(), OlapVertex {
            vid: (i + 30) as VertexId,
            properties: PropertyRecord::new(vec![
                ScalarValue::Int32(Some(i + 100)),
                ScalarValue::String(Some("hello".to_string())),
            ]),
            block_offset: 0,
        });
    }

    let mut vertex_iter = storage.iter_vertices(&()).unwrap();
    let vertex1 = vertex_iter.next().unwrap().unwrap();
    let vertex2 = vertex_iter.next().unwrap().unwrap();

    assert_eq!(vertex1.vid, 30);
    assert_eq!(vertex2.vid, 31);
}

#[test]
fn edge_iterator_test() {
    let storage = mock_olap_graph(1);
    for i in 1i32..=4 {
        let _result = storage.create_vertex(&(), OlapVertex {
            vid: i as VertexId,
            properties: PropertyRecord::new(vec![
                ScalarValue::Int32(Some(i + 100)),
                ScalarValue::String(Some("hello".to_string())),
            ]),
            block_offset: 0,
        });

        for j in 1i32..=(i * 10) {
            let _result1 = storage.create_edge(&(), OlapEdge {
                label_id: NonZeroU32::new((i * 10000 + j) as u32),
                src_id: i as VertexId,
                dst_id: (j * (i + 1)) as VertexId,
                properties: OlapPropertyStore::new(vec![Option::from(ScalarValue::String(Some(
                    "hello".to_string(),
                )))]),
            });
        }
    }

    let edge_iter = storage.iter_edges(&()).unwrap();
    let mut cnt: usize = 0;

    for next in edge_iter {
        // Check properties
        // unwrap unwrap unwrap unwrap ??
        let edge = next.unwrap();
        assert_eq!(
            edge.properties.get(0).unwrap().get_string().unwrap(),
            "hello".to_string()
        );
        cnt += 1;
    }
    // Vertex 0 has 0 edges
    assert_eq!(cnt, 10 + 20 + 30 + 40);
}

#[test]
fn adjacency_iterator_test() {
    let storage = mock_olap_graph(1);

    for i in 0..10 {
        let _result = storage.create_vertex(&(), OlapVertex {
            vid: i as VertexId,
            properties: PropertyRecord::default(),
            block_offset: 0,
        });

        for j in 0..(i * 100) {
            let _result1 = storage.create_edge(&(), OlapEdge {
                label_id: NonZeroU32::new(i * 10000 + j),
                src_id: i as VertexId,
                dst_id: (j * (i + 1)) as VertexId,
                properties: OlapPropertyStore::new(vec![Option::from(ScalarValue::String(Some(
                    "hello".to_string(),
                )))]),
            });
        }
    }

    let mut adjacency = storage.iter_adjacency(&(), 8).unwrap();
    // Should be the 759th edge
    assert_eq!(
        adjacency.next().unwrap().unwrap().dst_id,
        (256 * 3) * (8 + 1)
    );
    assert_eq!(
        adjacency.next().unwrap().unwrap().dst_id,
        (256 * 3) * (8 + 1) + 9
    );
    // Has 30 edges left
    for _i in 0..30 {
        adjacency.next();
    }
    // Should move to next block
    assert_eq!(
        adjacency.next().unwrap().unwrap().dst_id,
        (256 * 2) * (8 + 1)
    );
    for _i in 0..255 {
        adjacency.next();
    }
    assert_eq!(adjacency.next().unwrap().unwrap().dst_id, (256) * (8 + 1));
    for _i in 0..255 + 256 {
        adjacency.next();
    }
    // Should be None
    println!("{:?}", adjacency.next());
}

#[test]
fn set_vertex_properties_test() {
    let storage = mock_olap_graph(0);
    for i in 0..100 {
        let _result = storage.create_vertex(&(), OlapVertex {
            vid: (i + 30) as VertexId,
            properties: PropertyRecord::new(vec![
                ScalarValue::Int32(Some(i + 100)),
                ScalarValue::String(Some("hello".to_string())),
            ]),
            block_offset: 0,
        });
    }

    let result1 = storage.set_vertex_property(&(), 30, vec![0], vec![ScalarValue::Int32(Some(1))]);
    let result2 = storage.set_vertex_property(&(), 50, vec![1], vec![ScalarValue::String(Some(
        "No hello".to_string(),
    ))]);
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert_eq!(
        storage
            .vertices
            .read()
            .unwrap()
            .first()
            .unwrap()
            .properties
            .get(0)
            .unwrap()
            .get_int32()
            .unwrap(),
        1
    );
    assert_eq!(
        storage
            .vertices
            .read()
            .unwrap()
            .get(20)
            .unwrap()
            .properties
            .get(1)
            .unwrap()
            .get_string()
            .unwrap(),
        "No hello".to_string()
    );
}

#[test]
fn set_edge_properties_test() {
    let storage = mock_olap_graph(3);
    for i in 0..2 {
        let _result = storage.create_vertex(&(), OlapVertex {
            vid: i as VertexId,
            properties: PropertyRecord::default(),
            block_offset: 0,
        });
        for j in 0..3 {
            let _result1 = storage.create_edge(&(), OlapEdge {
                label_id: NonZeroU32::new(i * 10000 + j),
                src_id: i as VertexId,
                dst_id: (j + i) as VertexId,
                properties: OlapPropertyStore::new(vec![
                    Some(ScalarValue::UInt32(Some(j * 10))),
                    Some(ScalarValue::String(Some("hello".to_string()))),
                    Some(ScalarValue::Boolean(Some(true))),
                ]),
            });
        }
    }

    let _ = storage.set_edge_property(&(), NonZeroU32::new(10001), vec![0], vec![
        ScalarValue::Int32(Some(10086)),
    ]);
    let _ = storage.set_edge_property(&(), NonZeroU32::new(10002), vec![1, 2], vec![
        ScalarValue::String(Some("No hello".to_string())),
        ScalarValue::Boolean(Some(false)),
    ]);

    let store1 = storage
        .get_edge(&(), NonZeroU32::new(10001))
        .unwrap()
        .properties;
    let clone1 = store1.properties.first().unwrap().clone();
    assert_eq!(clone1.unwrap(), ScalarValue::Int32(Some(10086)));

    let store2 = storage
        .get_edge(&(), NonZeroU32::new(10002))
        .unwrap()
        .properties;
    let clone2 = store2.properties.get(1).unwrap().clone();
    let clone3 = store2.properties.get(2).unwrap().clone();
    assert_eq!(
        clone2.unwrap(),
        ScalarValue::String(Some("No hello".to_string()))
    );
    assert_eq!(clone3.unwrap(), ScalarValue::Boolean(Some(false)));
}

#[test]
fn delete_vertex_test() {
    let storage = mock_olap_graph(3);

    for i in 0..5 {
        let _result = storage.create_vertex(&(), OlapVertex {
            vid: i as VertexId,
            properties: PropertyRecord::default(),
            block_offset: 0,
        });
        for j in 0..300 {
            let _result1 = storage.create_edge(&(), OlapEdge {
                label_id: NonZeroU32::new(i * 10000 + j),
                src_id: i as VertexId,
                dst_id: (j + i) as VertexId,
                properties: OlapPropertyStore::default(),
            });
        }
    }

    assert_eq!(storage.vertices.read().unwrap().len(), 5);

    let _ = storage.delete_vertex(&(), 3);
    assert_eq!(storage.vertices.read().unwrap().len(), 4);

    assert!(!storage.edges.read().unwrap().get(5).unwrap().is_tombstone);
    assert!(storage.edges.read().unwrap().get(6).unwrap().is_tombstone);
    assert!(storage.edges.read().unwrap().get(7).unwrap().is_tombstone);
    assert!(!storage.edges.read().unwrap().get(8).unwrap().is_tombstone);
}

#[test]
fn delete_property_test() {
    let storage = mock_olap_graph(5);

    let _result = storage.create_vertex(&(), OlapVertex {
        vid: 1 as VertexId,
        properties: PropertyRecord::default(),
        block_offset: 0,
    });

    for i in 1..=5 {
        let _result1 = storage.create_edge(&(), OlapEdge {
            label_id: NonZeroU32::new(i),
            src_id: 1 as VertexId,
            dst_id: (10000 + i) as VertexId,
            properties: OlapPropertyStore::new(vec![
                Some(ScalarValue::UInt32(Some(i * 10))),
                Some(ScalarValue::String(Some("hello".to_string()))),
                Some(ScalarValue::Boolean(Some(true))),
                Some(ScalarValue::Float32(Some(F32::from(0.5) + i as f32))),
                Some(ScalarValue::String(Some("another hello".to_string()))),
            ]),
        });
    }

    let _ = storage.delete_edge(&(), NonZeroU32::new(2));

    {
        let binding = storage.edges.read().unwrap();
        let edge_block = binding.first().unwrap();
        assert_eq!(edge_block.edge_counter, 4);
        assert_eq!(edge_block.edges[0].label_id, NonZeroU32::new(1));
        assert_eq!(edge_block.edges[1].label_id, NonZeroU32::new(3));

        let binding = storage.property_columns.read().unwrap();
        let property_block = binding.first().unwrap().blocks.first().unwrap();
        assert_eq!(
            property_block.values[0],
            Some(ScalarValue::UInt32(Some(10)))
        );
        assert_eq!(
            property_block.values[1],
            Some(ScalarValue::UInt32(Some(30)))
        );
    }

    let _ = storage.delete_edge(&(), NonZeroU32::new(1));
    let _ = storage.delete_edge(&(), NonZeroU32::new(3));
    let _ = storage.delete_edge(&(), NonZeroU32::new(4));
    let _ = storage.delete_edge(&(), NonZeroU32::new(5));

    assert_eq!(
        storage.edges.read().unwrap().first().unwrap().edge_counter,
        0
    );
    assert!(storage.edges.read().unwrap().first().unwrap().is_tombstone);
}

#[test]
fn compress_edge_test() {
    let storage = mock_olap_graph(0);
    // Insert vertex
    for i in 1..=5 {
        let _result = storage.create_vertex(&(), OlapVertex {
            vid: i as VertexId,
            properties: PropertyRecord::default(),
            block_offset: 0,
        });

        for j in 1..=(400 - (i - 1) * 10) {
            let _result1 = storage.create_edge(&(), OlapEdge {
                label_id: NonZeroU32::new(i * 10000 + j),
                src_id: i as u64,
                dst_id: (j + i) as u64,
                properties: Default::default(),
            });
        }
    }

    storage.compress_edge();

    let compaction_borrow = storage.compressed_edges.read().unwrap();
    assert_eq!(compaction_borrow.len(), 10);

    assert_eq!(compaction_borrow.first().unwrap().src_id, 1);
    assert_eq!(compaction_borrow.first().unwrap().first_dst_id, 2);
    assert_eq!(compaction_borrow.first().unwrap().edge_counter, 256);
    assert_eq!(compaction_borrow.first().unwrap().delta_bit_width, 1);

    let bit_ref = compaction_borrow
        .first()
        .unwrap()
        .compressed_dst_ids
        .clone();

    println!("{bit_ref}");
}

#[test]
fn compress_property_test() {
    let storage = mock_olap_graph(2);

    for i in 1..=5 {
        let _result = storage.create_vertex(&(), OlapVertex {
            vid: i as VertexId,
            properties: PropertyRecord::default(),
            block_offset: 0,
        });

        for j in 1..=400 {
            let _result1 = storage.create_edge(&(), OlapEdge {
                label_id: NonZeroU32::new(i * 10000 + j),
                src_id: i as u64,
                dst_id: (j * (i + 1)) as u64,
                properties: OlapPropertyStore::new(vec![
                    Option::from(ScalarValue::UInt32(Some(j))),
                    None,
                ]),
            });
        }

        for j in 1..=400 {
            let _result1 = storage.create_edge(&(), OlapEdge {
                label_id: NonZeroU32::new(i * 2 * 10000 + j),
                src_id: i as u64,
                dst_id: (j * (i * 2 + 1)) as u64,
                properties: OlapPropertyStore::new(vec![
                    None,
                    Option::from(ScalarValue::String(Some("hello".to_string()))),
                ]),
            });
        }
    }

    storage.compress_property();

    let compaction_borrow = storage.compressed_properties.read().unwrap();
    assert_eq!(compaction_borrow.len(), 2);
    assert_eq!(compaction_borrow.first().unwrap().blocks.len(), 20);
    let block = compaction_borrow.first().unwrap().blocks.first().unwrap();
    assert_eq!(block.offsets[0], 16);
    assert_eq!(block.values[10].get_uint32().unwrap(), 11);
    assert_eq!(block.values[100].get_uint32().unwrap(), 101);
    println!("{}", block.bitmap);
}

#[test]
#[ignore]
fn dataset1_create_edge_for_storage_test() {
    let storage = mock_olap_graph(1);
    println!("Test for Twitter-Congress dataset");

    // Twitter Congress Dataset
    let file_path = PATH.to_owned() + "congress.edgelist";
    let dataset = parse_twitter_congress_dataset(&file_path);
    let vertices = dataset.0;
    let edges = dataset.1;

    // For current storage test
    let vertices_clone = vertices.clone();
    let edges_clone = edges.clone();

    let start_vertex = Instant::now();
    for olap_vertex in vertices_clone {
        let _result = storage.create_vertex(&(), olap_vertex);
    }
    let _duration_vertex = start_vertex.elapsed();

    let start_edge = Instant::now();
    for olap_edges in edges_clone {
        let _result = storage.create_edge(&(), olap_edges);
    }
    let duration_edge = start_edge.elapsed();

    println!("Storage - create_edge time: {duration_edge:?}");

    create_edge_csr1(vertices.clone(), edges.clone());
    create_edge_csr0(vertices.clone(), edges.clone());
    create_edge_adjacency_list(vertices.clone(), edges.clone());
}

#[test]
#[ignore]
fn dataset2_create_edge_for_storage_test() {
    let storage = mock_olap_graph(0);

    println!("Test for Wiki-Vote dataset");
    println!();

    let file_path = PATH.to_owned() + "Wiki-Vote.txt";
    let dataset = parse_two_column_dataset(&file_path);
    let vertices = dataset.0;
    let edges = dataset.1;

    // For current storage test
    let vertices_clone = vertices.clone();
    let edges_clone = edges.clone();

    let start_vertex = Instant::now();
    for olap_vertex in vertices_clone {
        let _result = storage.create_vertex(&(), olap_vertex);
    }
    let _duration_vertex = start_vertex.elapsed();

    let start_edge = Instant::now();
    for olap_edges in edges_clone {
        let _result = storage.create_edge(&(), olap_edges);
    }
    let duration_edge = start_edge.elapsed();

    println!("Storage - create_edge time: {duration_edge:?}");

    create_edge_csr1(vertices.clone(), edges.clone());
    create_edge_csr0(vertices.clone(), edges.clone());
    create_edge_adjacency_list(vertices.clone(), edges.clone());
}

#[test]
#[ignore]
fn dataset3_create_edge_for_storage_test() {
    let storage = mock_olap_graph(0);

    println!("Test for P2P-Gnutella25 dataset");
    println!();

    let file_path = PATH.to_owned() + "p2p-Gnutella25.txt";
    let dataset = parse_two_column_dataset(&file_path);
    let vertices = dataset.0;
    let edges = dataset.1;

    // For current storage test
    let vertices_clone = vertices.clone();
    let edges_clone = edges.clone();

    let start_vertex = Instant::now();
    for olap_vertex in vertices_clone {
        let _result = storage.create_vertex(&(), olap_vertex);
    }
    let _duration_vertex = start_vertex.elapsed();

    let start_edge = Instant::now();
    for olap_edges in edges_clone {
        let _result = storage.create_edge(&(), olap_edges);
    }
    let duration_edge = start_edge.elapsed();

    println!("Storage - create_edge time: {duration_edge:?}");

    create_edge_csr1(vertices.clone(), edges.clone());
    create_edge_csr0(vertices.clone(), edges.clone());
    create_edge_adjacency_list(vertices.clone(), edges.clone());
}

#[test]
#[ignore]
fn dataset1_edge_compaction_test() {
    compress_storage_two_column_without_property(
        PATH.to_owned() + "artist_edges.csv",
        "F_dataset".to_string(),
    )
}

#[test]
#[ignore]
fn dataset2_edge_compaction_test() {
    compress_storage_two_column_without_property(
        PATH.to_owned() + "Amazon0302.txt",
        "Amazon0302".to_string(),
    )
}

#[test]
#[ignore]
fn dataset3_edge_compaction_test() {
    compress_storage_two_column_without_property(
        PATH.to_owned() + "com-youtube.ungraph.txt",
        "com-youtube.ungraph".to_string(),
    )
}

#[test]
#[ignore]
fn dataset1_property_compaction_test() {
    let storage = mock_olap_graph(2);

    let file_path = PATH.to_owned() + "title.episode.tsv";
    let dataset = parse_title_episode_dataset(&file_path);
    let vertices = dataset.0;
    let edges = dataset.1;

    let vertices_clone = vertices.clone();
    let edges_clone = edges.clone();

    let start_vertex = Instant::now();
    for olap_vertex in vertices_clone {
        let _result = storage.create_vertex(&(), olap_vertex);
    }
    let _duration_vertex = start_vertex.elapsed();

    let start_edge = Instant::now();
    for olap_edges in edges_clone {
        let _result = storage.create_edge(&(), olap_edges);
    }
    let duration_edge = start_edge.elapsed();

    println!("Storage - create_edge time: {duration_edge:?}");

    let property_size = measure_memory_column(&storage.property_columns);
    println!();
    println!("{property_size}");

    storage.compress_property();

    let compressed_property_size = measure_memory_compressed_col(&storage.compressed_properties);
    println!("{compressed_property_size}");
}

#[test]
#[ignore]
fn dataset2_property_compaction_test() {
    let storage = mock_olap_graph(2);

    let file_path = PATH.to_owned() + "title.crew.tsv";
    let dataset = parse_title_crew_dataset(&file_path);
    let vertices = dataset.0;
    let edges = dataset.1;

    let vertices_clone = vertices.clone();
    let edges_clone = edges.clone();

    let start_vertex = Instant::now();
    for olap_vertex in vertices_clone {
        let _result = storage.create_vertex(&(), olap_vertex);
    }
    let _duration_vertex = start_vertex.elapsed();

    let start_edge = Instant::now();
    for olap_edges in edges_clone {
        let _result = storage.create_edge(&(), olap_edges);
    }
    let duration_edge = start_edge.elapsed();

    println!("Storage - create_edge time: {duration_edge:?}");

    let property_size = measure_memory_column(&storage.property_columns);
    println!();
    println!("{property_size}");

    storage.compress_property();

    let compressed_property_size = measure_memory_compressed_col(&storage.compressed_properties);
    println!("{compressed_property_size}");
}

#[test]
#[ignore]
fn dataset1_col_storage_analysis() {
    let storage = mock_olap_graph(6);
    let edge_path = PATH.to_owned() + "mooc_actions.tsv";
    let property_path = PATH.to_owned() + "mooc_action_features.tsv";

    let dataset = parse_mooc_actions_dataset(&edge_path, &property_path);
    let vertices = dataset.0;
    let edges = dataset.1;

    let vertices_clone = vertices.clone();
    let edges_clone = edges.clone();

    let start_vertex = Instant::now();
    for olap_vertex in vertices_clone {
        let _result = storage.create_vertex(&(), olap_vertex);
    }
    let _duration_vertex = start_vertex.elapsed();

    let start_edge = Instant::now();
    for olap_edges in edges_clone {
        let _result = storage.create_edge(&(), olap_edges);
    }
    let duration_edge = start_edge.elapsed();

    println!("Storage - create_edge time: {duration_edge:?}");

    // Mock row storage
    let mut row_properties: Vec<Vec<Option<ScalarValue>>> = Vec::new();
    let edges_clone2 = edges.clone();
    for edge in edges_clone2 {
        row_properties.push(edge.properties.properties);
    }

    let x = storage.property_columns.read().unwrap();

    // Analysis 1 - Sum
    let mut _total1: f64 = 0.0;
    let mut _total2: f64 = 0.0;

    let start_col_analysis1 = Instant::now();
    for block in &x.get(2).unwrap().blocks {
        for option in &block.values {
            if option.is_none() {
                break;
            }
            _total1 += <Option<ScalarValue> as Clone>::clone(option)
                .unwrap()
                .get_float64()
                .unwrap();
        }
    }

    let duration_col_analysis1 = start_col_analysis1.elapsed();

    let row_clone1 = row_properties.clone();
    let start_row_analysis1 = Instant::now();
    for vec in row_clone1 {
        let value = vec.get(2).unwrap();
        _total2 += <Option<ScalarValue> as Clone>::clone(value)
            .unwrap()
            .get_float64()
            .unwrap();
    }

    let duration_row_analysis1 = start_row_analysis1.elapsed();

    println!("Column analysis 1: {duration_col_analysis1:?}");
    println!("Row analysis 1: {duration_row_analysis1:?}");

    // Analysis 2 - Max
    let mut max1: f64 = -10.0;
    let mut max2: f64 = -10.0;

    let start_col_analysis2 = Instant::now();
    for block in &x.get(3).unwrap().blocks {
        for option in &block.values {
            if option.is_none() {
                break;
            }
            max1 = max1.max(
                option
                    .clone()
                    .map(|s| s.get_float64().unwrap())
                    .unwrap_or_default(),
            );
        }
    }

    let duration_col_analysis2 = start_col_analysis2.elapsed();

    let row_clone2 = row_properties.clone();
    let start_row_analysis2 = Instant::now();
    for vec in row_clone2 {
        let value = vec.get(3).unwrap();
        max2 = max2.max(
            value
                .clone()
                .map(|s| s.get_float64().unwrap())
                .unwrap_or_default(),
        );
    }

    let duration_row_analysis2 = start_row_analysis2.elapsed();

    println!("Column analysis 2: {duration_col_analysis2:?}");
    println!("Row analysis 2: {duration_row_analysis2:?}");

    // Analysis 3 - Min
    let mut min1: f64 = -10.0;
    let mut min2: f64 = -10.0;

    let start_col_analysis3 = Instant::now();
    for block in &x.get(4).unwrap().blocks {
        for option in &block.values {
            if option.is_none() {
                break;
            }
            min1 = min1.min(
                option
                    .clone()
                    .map(|s| s.get_float64().unwrap())
                    .unwrap_or_default(),
            );
        }
    }

    let duration_col_analysis3 = start_col_analysis3.elapsed();

    let row_clone3 = row_properties.clone();
    let start_row_analysis3 = Instant::now();
    for vec in row_clone3 {
        let value = vec.get(4).unwrap();
        min2 = min2.min(
            value
                .clone()
                .map(|s| s.get_float64().unwrap())
                .unwrap_or_default(),
        );
    }

    let duration_row_analysis3 = start_row_analysis3.elapsed();

    println!("Column analysis 3: {duration_col_analysis3:?}");
    println!("Row analysis 3: {duration_row_analysis3:?}");
}

fn compress_storage_two_column_without_property(path: String, name: String) {
    let storage = mock_olap_graph(0);

    let file_path = &path.clone();
    let dataset = parse_two_column_dataset(file_path);
    let vertices = dataset.0;
    let edges = dataset.1;

    // For current storage test
    let vertices_clone = vertices.clone();
    let edges_clone = edges.clone();

    let start_vertex = Instant::now();
    for olap_vertex in vertices_clone {
        let _result = storage.create_vertex(&(), olap_vertex);
    }
    let _duration_vertex = start_vertex.elapsed();

    let start_edge = Instant::now();
    for olap_edges in edges_clone {
        let _result = storage.create_edge(&(), olap_edges);
    }
    let duration_edge = start_edge.elapsed();

    println!("Test for {name} dataset");
    println!();
    println!("Storage - create_edge time: {duration_edge:?}");

    let edges_size = measure_edge_memory(&storage.edges);
    println!("Bytes before compaction: {edges_size}b");

    storage.compress_edge();

    let compressed_edges_size = measure_compressed_edge_memory(&storage.compressed_edges);
    println!("Bytes after compaction: {compressed_edges_size}b");
}

// CSR0 - Implemented by array (Not Vec)
fn create_edge_csr0(vertices: Vec<OlapVertex>, edges: Vec<OlapEdge>) {
    let mut vertex_array_capacity = 10;
    let mut vertex_array: Box<[usize]> = vec![0; vertex_array_capacity].into_boxed_slice();

    let mut edge_array_capacity = 10;
    let mut edge_array: Box<[usize]> = vec![0; edge_array_capacity].into_boxed_slice();
    let mut value_array: Box<[f64]> = vec![0.0; edge_array_capacity].into_boxed_slice();

    let mut edge_index: usize = 1;
    let start = Instant::now();

    for (i, vertex) in vertices.iter().enumerate() {
        if i >= vertex_array_capacity {
            vertex_array_capacity += 1;
            let mut new_vertex_array = vec![0; vertex_array_capacity].into_boxed_slice();
            new_vertex_array[..i].copy_from_slice(&vertex_array[..i]);
            vertex_array = new_vertex_array;
        }
        vertex_array[i] = edge_index;
        for edge in edges.iter() {
            if edge.src_id == vertex.vid {
                if edge_index >= edge_array_capacity {
                    edge_array_capacity += 1;
                    let mut new_edge_array = vec![0; edge_array_capacity].into_boxed_slice();
                    let mut new_value_array = vec![0.0; edge_array_capacity].into_boxed_slice();
                    new_edge_array[..edge_index].copy_from_slice(&edge_array[..edge_index]);
                    new_value_array[..edge_index].copy_from_slice(&value_array[..edge_index]);
                    edge_array = new_edge_array;
                    value_array = new_value_array;
                }
                edge_array[edge_index] = edge.dst_id as usize;
                value_array[edge_index] = match edge.properties.get(0) {
                    Some(prop) => prop.get_float64().unwrap(),
                    None => 0.0,
                };
                edge_index += 1;
            }
        }
    }

    if vertices.len() >= vertex_array_capacity {
        vertex_array_capacity += 1;
        let mut new_vertex_array = vec![0; vertex_array_capacity].into_boxed_slice();
        new_vertex_array[..vertices.len()].copy_from_slice(&vertex_array[..vertices.len()]);
        vertex_array = new_vertex_array;
    }

    vertex_array[vertices.len()] = edge_index;
    let duration = start.elapsed();

    println!("CSR0 - create_edge time: {duration:?}");
}

// CSR1 - Implemented by vec
fn create_edge_csr1(vertices: Vec<OlapVertex>, edges: Vec<OlapEdge>) {
    let vertex_count = vertices.len();
    let mut vertex_array = vec![0; vertex_count + 1];
    let mut edge_array = Vec::new();
    let mut value_array = Vec::new();
    let mut edge_index = 0;

    let start = Instant::now();
    for (i, vertex) in vertices.iter().enumerate() {
        vertex_array[i] = edge_index;

        for edge in edges.iter().filter(|e| e.src_id == vertex.vid) {
            edge_array.push(edge.dst_id);
            value_array.push(edge.properties.get(0));
            edge_index += 1;
        }
    }

    vertex_array[vertex_count] = edge_index;

    let duration = start.elapsed();
    println!("CSR1 - create_edge time: {duration:?}");
}

fn create_edge_adjacency_list(vertices: Vec<OlapVertex>, edges: Vec<OlapEdge>) {
    let mut adjacency_list: HashMap<OlapVertex, Vec<OlapEdge>> = HashMap::new();

    let start = Instant::now();

    for vertex in vertices {
        adjacency_list.entry(vertex).or_default();
    }
    for edge in edges {
        if let Some(edge_list) = adjacency_list
            .iter_mut()
            .find(|(vertex, _)| vertex.vid == edge.src_id)
        {
            edge_list.1.push(edge);
        }
    }

    let duration = start.elapsed();
    println!("Adjacency List - create_edge time: {duration:?}");
}

fn measure_edge_memory(vec: &RwLock<Vec<EdgeBlock>>) -> usize {
    let vec_ref = vec.read().unwrap();

    let vec_metadata_size = size_of_val(&*vec_ref);
    let static_block_size = {
        let pre_block_index_size = size_of::<Option<usize>>();
        let cur_block_index_size = size_of::<usize>();
        let max_label_id_size = size_of::<LabelId>();
        let min_label_id_size = size_of::<LabelId>();
        let max_dst_id_size = size_of::<VertexId>();
        let min_dst_id_size = size_of::<VertexId>();
        let src_id_size = size_of::<VertexId>();
        let edge_counter_size = size_of::<usize>();
        let edges_size = BLOCK_CAPACITY * size_of::<OlapStorageEdge>();

        pre_block_index_size
            + cur_block_index_size
            + max_label_id_size
            + min_label_id_size
            + max_dst_id_size
            + min_dst_id_size
            + src_id_size
            + edge_counter_size
            + edges_size
    };

    // Theoretical memory use ver_ref.len
    let total_static_memory = vec_ref.capacity() * static_block_size;
    vec_metadata_size + total_static_memory
}

fn measure_compressed_edge_memory(vec: &RwLock<Vec<CompressedEdgeBlock>>) -> usize {
    let vec_ref = vec.read().unwrap();
    let vec_metadata_size = size_of_val(&*vec_ref);

    let static_block_size = {
        let pre_block_index_size = size_of::<Option<usize>>();
        let cur_block_index_size = size_of::<usize>();
        let max_label_id_size = size_of::<LabelId>();
        let min_label_id_size = size_of::<LabelId>();
        let max_dst_id_size = size_of::<VertexId>();
        let min_dst_id_size = size_of::<VertexId>();
        let src_id_size = size_of::<VertexId>();
        let edge_counter_size = size_of::<usize>();
        let delta_bit_width_size = size_of::<u8>();
        let first_dst_id_size = size_of::<VertexId>();
        let label_ids_size = BLOCK_CAPACITY * size_of::<LabelId>();

        let alignment_padding = 7;

        pre_block_index_size
            + cur_block_index_size
            + max_label_id_size
            + min_label_id_size
            + max_dst_id_size
            + min_dst_id_size
            + src_id_size
            + edge_counter_size
            + delta_bit_width_size
            + alignment_padding
            + first_dst_id_size
            + label_ids_size
    };

    // Theoretical memory use ver_ref.len
    // In one dataset,the number of actual block used is 44139 instead of 65535
    let total_static_memory = vec_ref.capacity() * static_block_size;

    let dynamic_memory: usize = vec_ref
        .iter()
        .map(|block| {
            let compressed_dst_ids_memory = block.compressed_dst_ids.capacity() / 8;

            let bitvec_metadata_size = size_of::<BitVec<u64, Lsb0>>();
            compressed_dst_ids_memory + bitvec_metadata_size
        })
        .sum();

    vec_metadata_size + total_static_memory + dynamic_memory
}

fn measure_memory_column(vec: &RwLock<Vec<PropertyColumn>>) -> usize {
    let mut total_size = 0;
    let vec_borrow = vec.read().unwrap();
    for column in vec_borrow.iter() {
        total_size += size_of::<PropertyColumn>();
        total_size += column.blocks.len() * size_of::<PropertyBlock>();
        let mut single_size = 0;
        for block in &column.blocks {
            total_size += size_of_val(&block.values);
            for value in &block.values {
                if value.is_none() {
                    total_size += single_size;
                } else {
                    let clone = value.clone();
                    let size = match clone.unwrap() {
                        ScalarValue::Int32(_) => size_of::<i32>(),
                        ScalarValue::Int64(_) => size_of::<i64>(),
                        ScalarValue::Float32(_) => size_of::<f32>(),
                        ScalarValue::Float64(_) => size_of::<f64>(),
                        ScalarValue::String(Some(s)) => {
                            size_of::<String>() + s.len() * size_of::<u8>()
                        }
                        ScalarValue::Boolean(_) => size_of::<bool>(),
                        _ => 0,
                    };
                    total_size += size;
                    if single_size == 0 {
                        single_size = size;
                    }
                }
            }
        }
    }
    total_size
}

fn measure_memory_compressed_col(vec: &RwLock<Vec<CompressedPropertyColumn>>) -> usize {
    let mut total_size = 0;
    let vec_borrow = vec.read().unwrap();
    for column in vec_borrow.iter() {
        total_size += size_of::<CompressedPropertyColumn>();
        total_size += column.blocks.len() * size_of::<CompressedPropertyBlock>();
        for block in &column.blocks {
            total_size += block.bitmap.len() / 8; // Convert bitmap size to bytes
            for value in &block.values {
                total_size += match value {
                    ScalarValue::Int32(_) => size_of::<i32>(),
                    ScalarValue::Int64(_) => size_of::<i64>(),
                    ScalarValue::Float32(_) => size_of::<f32>(),
                    ScalarValue::Float64(_) => size_of::<f64>(),
                    ScalarValue::String(Some(s)) => size_of::<String>() + s.len() * size_of::<u8>(),
                    ScalarValue::Boolean(_) => size_of::<bool>(),
                    _ => 0,
                };
            }
        }
    }
    total_size
}

fn parse_two_column_dataset(file_path: &str) -> (Vec<OlapVertex>, Vec<OlapEdge>) {
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let mut current_vertex: usize = 9999999;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if index == 0 {
            continue;
        }
        let mut parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 1 {
            parts = line.split(",").collect()
        }

        let src_id: usize = parts[0].parse().unwrap();
        let dst_id: usize = parts[1].parse().unwrap();

        if current_vertex != src_id {
            vertices.push(OlapVertex {
                vid: src_id as VertexId,
                properties: Default::default(),
                block_offset: 0,
            });
            current_vertex = src_id;
        }

        edges.push(OlapEdge {
            label_id: NonZeroU32::new(1),
            src_id: src_id as VertexId,
            dst_id: dst_id as VertexId,
            properties: OlapPropertyStore::default(),
        });
    }

    (vertices, edges)
}

fn parse_twitter_congress_dataset(file_path: &str) -> (Vec<OlapVertex>, Vec<OlapEdge>) {
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let mut current_vertex: usize = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }

        let src_id: usize = parts[0].parse().expect("Invalid src_id");
        let dst_id: usize = parts[1].parse().expect("Invalid dst_id");
        let weight: f64 = parts[3]
            .trim_end_matches('}')
            .trim()
            .parse::<f64>()
            .expect("Invalid weight format");

        if src_id != current_vertex {
            vertices.push(OlapVertex {
                vid: src_id as VertexId,
                properties: Default::default(),
                block_offset: 0,
            });
            current_vertex = src_id;
        }

        edges.push(OlapEdge {
            label_id: NonZeroU32::new(1),
            src_id: src_id as VertexId,
            dst_id: dst_id as VertexId,
            properties: OlapPropertyStore::new(vec![Some(ScalarValue::Float64(Some(F64::from(
                weight,
            ))))]),
        })
    }
    (vertices, edges)
}

fn parse_title_episode_dataset(file_path: &str) -> (Vec<OlapVertex>, Vec<OlapEdge>) {
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);
    let mut counter = 0;

    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let mut current_vertex: usize = 100;
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        counter += 1;
        // mock src_id
        let src_id = counter / 10000;
        let dst_id = parts[0].trim_start_matches("tt").parse::<usize>().unwrap();

        let season_number = if parts[2] == r"\N" {
            None
        } else {
            Some(ScalarValue::Int32(Some(parts[2].parse::<i32>().unwrap())))
        };

        let episode_number = if parts[3] == r"\N" {
            None
        } else {
            Some(ScalarValue::Int32(Some(parts[3].parse::<i32>().unwrap())))
        };

        if src_id != current_vertex {
            vertices.push(OlapVertex {
                vid: src_id as VertexId,
                properties: Default::default(),
                block_offset: 0,
            });
            current_vertex = src_id;
        }

        edges.push(OlapEdge {
            label_id: NonZeroU32::new(1),
            src_id: src_id as VertexId,
            dst_id: dst_id as VertexId,
            properties: OlapPropertyStore::new(vec![season_number, episode_number]),
        })
    }
    (vertices, edges)
}

fn parse_title_crew_dataset(file_path: &str) -> (Vec<OlapVertex>, Vec<OlapEdge>) {
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);
    let mut counter = 0;

    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let mut current_vertex: usize = 100;
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        counter += 1;
        let src_id = counter / 10000;
        let dst_id = parts[0].trim_start_matches("tt").parse::<usize>().unwrap();

        let property1 = if parts[1] == r"\N" {
            None
        } else {
            Some(ScalarValue::String(Some(parts[1].to_string())))
        };

        let property2 = if parts[2] == r"\N" {
            None
        } else {
            Some(ScalarValue::String(Some(parts[2].to_string())))
        };

        if src_id != current_vertex {
            vertices.push(OlapVertex {
                vid: src_id as VertexId,
                properties: Default::default(),
                block_offset: 0,
            });
            current_vertex = src_id;
        }

        edges.push(OlapEdge {
            label_id: NonZeroU32::new(1),
            src_id: src_id as VertexId,
            dst_id: dst_id as VertexId,
            properties: OlapPropertyStore::new(vec![property1, property2]),
        })
    }
    (vertices, edges)
}

fn parse_mooc_actions_dataset(
    edge_file_path: &str,
    property_file_path: &str,
) -> (Vec<OlapVertex>, Vec<OlapEdge>) {
    let edge_file = File::open(edge_file_path).unwrap();
    let edge_reader = io::BufReader::new(edge_file);
    let property_file = File::open(property_file_path).unwrap();
    let property_reader = io::BufReader::new(property_file);
    let _counter = 0;

    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let mut current_vertex: usize = 9999999;

    let edge_lines = edge_reader.lines();
    let property_lines = property_reader.lines();

    for (edge_line_result, property_line_result) in edge_lines.zip(property_lines) {
        let edge_line = edge_line_result.unwrap();
        let property_line = property_line_result.unwrap();

        let edge_parts: Vec<&str> = edge_line.split_whitespace().collect();
        let property_parts: Vec<&str> = property_line.split_whitespace().collect();

        let src_id: usize = edge_parts[1].parse().expect("Invalid src_id");
        let dst_id: usize = edge_parts[2].parse().expect("Invalid dst_id");

        let property1 = Some(ScalarValue::Float64(Some(F64::from(
            edge_parts[3].parse::<f64>().unwrap(),
        ))));
        let property2 = Some(ScalarValue::Int32(Some(
            edge_parts[0].parse::<i32>().unwrap(),
        )));
        let property3 = Some(ScalarValue::Float64(Some(F64::from(
            property_parts[1].parse::<f64>().unwrap(),
        ))));
        let property4 = Some(ScalarValue::Float64(Some(F64::from(
            property_parts[2].parse::<f64>().unwrap(),
        ))));
        let property5 = Some(ScalarValue::Float64(Some(F64::from(
            property_parts[3].parse::<f64>().unwrap(),
        ))));
        let property6 = Some(ScalarValue::Float64(Some(F64::from(
            property_parts[4].parse::<f64>().unwrap(),
        ))));

        if src_id != current_vertex {
            vertices.push(OlapVertex {
                vid: src_id as VertexId,
                properties: Default::default(),
                block_offset: 0,
            });
            current_vertex = src_id;
        }

        edges.push(OlapEdge {
            label_id: NonZeroU32::new(1),
            src_id: src_id as VertexId,
            dst_id: dst_id as VertexId,
            properties: OlapPropertyStore::new(vec![
                property1, property2, property3, property4, property5, property6,
            ]),
        })
    }
    (vertices, edges)
}
