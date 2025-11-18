//! GCard stat-quality procedure.
//!
//! Reports the fraction of [`BlockStatistic`] entries whose bucket's `res_vec` has reached
//! the maximum possible remainder — a measure of how "loose" the stored degree upper bounds
//! have become after incremental updates.  A high ratio indicates that `GCard_build` should
//! be re-run to tighten the statistics.
//!
//! Accepts an optional path pattern argument (comma-separated vertex/edge labels, e.g.
//! `"Person,knows,Person"`) to check the quality of a specific path's statistics only.

use std::sync::Arc;

use minigu_catalog::provider::GraphProvider;
use minigu_common::data_chunk;
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_context::graph::GraphContainer;
use minigu_context::procedure::Procedure;
use minigu_execution::error::ExecutionError;

use crate::procedures::gcard_query::catalog::AltKey;
use crate::procedures::gcard_query::Statistic;

/// Build the `GCard_stat_quality` procedure.
///
/// **Arguments** (all optional)
///
/// | index | type   | description                                                    |
/// |-------|--------|----------------------------------------------------------------|
/// | 0     | String | path pattern: comma-separated vertex/edge labels, e.g.        |
/// |       |        | `"Person,knows,Person"`. When omitted, reports global quality. |
///
/// **Output**
///
/// | column              | type    | description                                    |
/// |---------------------|---------|------------------------------------------------|
/// | `upper_limit_ratio` | Float64 | fraction of entries whose bucket upper bound   |
/// |                     |         | is maximally loose (0.0 = tight, 1.0 = loose)  |
pub fn build_procedure() -> Procedure {
    let parameters = vec![
        LogicalType::String, // optional: path pattern (comma-separated)
    ];

    let schema = Arc::new(DataSchema::new(vec![DataField::new(
        "upper_limit_ratio".into(),
        LogicalType::Float64,
        false,
    )]));

    Procedure::new(parameters, Some(schema), move |context, args| {
        let graph_ref = context.current_graph.clone().ok_or_else(|| {
            ExecutionError::Custom(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "current graph is not selected",
            )))
        })?;

        let provider = graph_ref.object().clone();
        let container = provider
            .downcast_ref::<GraphContainer>()
            .ok_or_else(|| anyhow::anyhow!("graph container type mismatch"))?;

        let stat_arc = container
            .statistic()
            .ok_or_else(|| anyhow::anyhow!("statistic not set (run GCard_build first)"))?;
        let statistic = stat_arc
            .downcast_ref::<Statistic>()
            .ok_or_else(|| anyhow::anyhow!("statistic type mismatch"))?;

        // Parse optional path pattern argument
        let path_pattern: Option<String> = args
            .first()
            .and_then(|a| a.try_as_string())
            .and_then(|opt| opt.as_ref())
            .map(|s| s.to_string());

        let ratio = if let Some(pattern_str) = path_pattern {
            let labels: Vec<String> = pattern_str
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            let alt_key = AltKey(labels);
            match statistic.upper_limit_ratio_for_path(&alt_key) {
                Some(r) => {
                    println!(
                        "GCard stat quality for path [{}] — upper_limit_ratio: {:.4}",
                        alt_key, r
                    );
                    r
                }
                None => {
                    println!(
                        "GCard stat quality — path [{}] not found in statistics",
                        alt_key
                    );
                    -1.0
                }
            }
        } else {
            let r = statistic.upper_limit_ratio();
            println!("GCard stat quality — upper_limit_ratio: {:.4}", r);
            r
        };

        Ok(vec![data_chunk!((Float64, [Some(ratio)]))])
    })
}
