mod abs_graph;
mod block_statistic;
mod catalog;
pub mod compact_update_log;
pub mod create_catalog;
mod degreepiecewise;
mod statistic;
pub mod error;
pub mod stat_quality;
pub mod update_log;

pub use block_statistic::BlockStatistic;
pub use catalog::make_alt_key;
pub use statistic::Statistic;
mod graph;
mod query_graph;
pub mod types;
mod union_find;

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fmt, fs, io};

use minigu_common::data_chunk;
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_context::graph::GraphContainer;
use minigu_context::procedure::Procedure;
use minigu_execution::error::ExecutionError;

use crate::procedures::gcard_query::abs_graph::AbstractGraph;
use crate::procedures::gcard_query::catalog::DegreeSeqGraphCompressed;
use crate::procedures::gcard_query::query_graph::QueryGraph;
use crate::procedures::gcard_query::types::Query;

static GCARD_VERBOSE: AtomicBool = AtomicBool::new(false);
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

impl TryFrom<u8> for PredicateApplyType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PredicateApplyType::INNER),
            1 => Ok(PredicateApplyType::OUTER),
            2 => Ok(PredicateApplyType::IGNORE),
            _ => Err("invalid PredicateApplyType value"),
        }
    }
}

impl fmt::Display for PredicateApplyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PredicateApplyType::INNER => write!(f, "INNER"),
            PredicateApplyType::OUTER => write!(f, "OUTER"),
            PredicateApplyType::IGNORE => write!(f, "IGNORE"),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum PredicateApplyType {
    INNER,
    OUTER,
    IGNORE,
}

pub fn build_procedure() -> Procedure {
    let parameters = vec![
        LogicalType::String,  // query_json_path
        LogicalType::Int8,    // max_path_length (K)
        LogicalType::Int32,   // simple time
        LogicalType::UInt8,   // predicate apply type
        LogicalType::Boolean, //
    ];

    let schema = Arc::new(DataSchema::new(vec![DataField::new(
        "cardinality".into(),
        LogicalType::Int64,
        false,
    )]));

    Procedure::new(parameters, Some(schema), move |context, args| {
        let graph_ref = context.current_graph.clone().ok_or_else(|| {
            ExecutionError::Custom(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "current graph is not selected",
            )))
        })?;
        let provider = graph_ref.object().clone();
        let container = provider.downcast_ref::<GraphContainer>().ok_or_else(|| {
            ExecutionError::Custom(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "only in-memory graphs support vector scans",
            )))
        })?;
        let metadata: DegreeSeqGraphCompressed = container
            .degree_seq_graph_compressed()
            .as_ref()
            .and_then(|arc| arc.downcast_ref::<DegreeSeqGraphCompressed>().cloned())
            .ok_or_else(|| anyhow::anyhow!("degree_seq_graph_compressed not set (run GCard_build first)"))?;

        let query_json_path = args[0]
            .try_as_string()
            .expect("first arg must be a string")
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("query_json_path cannot be null"))?
            .to_string();
        let simple_size = args[2]
            .to_i32()
            .map_err(|e| anyhow::anyhow!("Failed to convert sample size to i32: {:?}", e))?
            as usize;

        #[allow(unused_variables)]
        let verbose = args
            .get(4)
            .map(|a| a.to_bool().unwrap_or(false))
            .unwrap_or(false);
        GCARD_VERBOSE.store(verbose, Ordering::Relaxed);

        let query_json = fs::read_to_string(&query_json_path).map_err(|e| {
            anyhow::anyhow!("Failed to read query JSON file {}: {}", query_json_path, e)
        })?;

        let query: Query = serde_json::from_str(&query_json)
            .map_err(|e| anyhow::anyhow!("Failed to parse query JSON: {}", e))?;

        let query_graph: QueryGraph = query
            .build_graph()
            .map_err(|e| anyhow::anyhow!("Failed to build query graph: {}", e))?;

        let max_path_length = args[1]
            .to_i32()
            .map_err(|e| anyhow::anyhow!("Failed to convert max_path_length to i32: {:?}", e))?
            as usize;

        if max_path_length == 0 {
            return Err(
                anyhow::anyhow!("max_path_length must be > 0, got {}", max_path_length).into(),
            );
        }

        let predicate_apply_type: PredicateApplyType = args[3]
            .to_u8()
            .map_err(|e| anyhow::anyhow!("Failed to convert predicate apply type to u8: {:?}", e))?
            .try_into()
            .map_err(|e| {
                anyhow::anyhow!(
                    "Failed to convert predicate apply type to PredicateApplyType: {}",
                    e
                )
            })?;

        let cardinality = match query_graph.build_abstract_graph(
            max_path_length,
            50,
            &metadata,
            Some(container),
            simple_size,
            &predicate_apply_type,
        ) {
            Ok(abstract_graphs_with_scores) => {
                let total_count = abstract_graphs_with_scores.len();
                let mut min_nonzero_es = f64::INFINITY;
                let mut score_of_min_es: Option<u32> = None;
                let mut index_of_min_es: Option<usize> = None;
                let mut max_score: u32 = 0;
                #[allow(dead_code)]
                let mut min_es_abstract_graph: Option<AbstractGraph> = None;
                for (idx, (mut abs, score)) in abstract_graphs_with_scores.into_iter().enumerate() {
                    if score > max_score {
                        max_score = score;
                    }
                    let abs_for_debug = abs.clone();

                    if GCARD_VERBOSE.load(Ordering::Relaxed) {
                        println!("use predicate type:{}", predicate_apply_type.to_string());
                        println!("idx:{}", idx);
                        for (edge_id, edge) in &abs_for_debug.edges {
                            println!("edge: {}, selectivity: {}", edge.path_str, edge.selectivity);
                            println!("src : {}", edge.src_pcf);
                            println!("dst : {}", edge.dst_pcf);
                        }
                    }
                    let selectity_outter: f64 = if predicate_apply_type == PredicateApplyType::OUTER
                    {
                        let pred = abs.get_selectivity();
                        if GCARD_VERBOSE.load(Ordering::Relaxed) {
                            println!("pred totally selectivity: {}", pred);
                        }
                        pred
                    } else {
                        1.0
                    };

                    let mut es = abs.get_es().map_err(|e| {
                        ExecutionError::Custom(Box::new(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("GCard get_es: {}", e),
                        )))
                    })?;

                    if GCARD_VERBOSE.load(Ordering::Relaxed) {
                        println!(
                            "es: {}, after pred selective: {}",
                            es,
                            es * selectity_outter
                        );
                        println!("");
                    }
                    es *= selectity_outter;
                    if es > 1.0 {
                        if es < min_nonzero_es {
                            min_nonzero_es = es;
                            score_of_min_es = Some(score);
                            index_of_min_es = Some(idx + 1);
                            min_es_abstract_graph = Some(abs_for_debug);
                        }
                    }
                }
                let cardinality_value = if min_nonzero_es.is_finite() {
                    min_nonzero_es
                } else {
                    0.0
                };
                let is_highest = score_of_min_es.map(|s| s == max_score).unwrap_or(false);
                let display_index = if is_highest { Some(1) } else { index_of_min_es };
                print!(
                    "total: {}, min_es index: {:?}, is highest: {}, ",
                    total_count, display_index, is_highest,
                );
                cardinality_value
            }
            Err(e) => {
                eprintln!(
                    "GCard: unsupported predicate or error, returning 0.0: {}",
                    e
                );
                0.0
            }
        };
        let stem = Path::new(query_json_path.as_str())
            .file_stem()
            .and_then(|n| n.to_str());
        println!("{}, cardinality: {}", stem.unwrap(), cardinality.ceil());

        Ok(vec![data_chunk!((Int64, [
            Some(cardinality.ceil() as i64)
        ]))])
    })
}
