mod adjacency_iter;
mod edge_iter;
mod vertex_iter;
use std::sync::Arc;

pub use adjacency_iter::{AdjacencyIteratorTrait, Direction};
pub use edge_iter::EdgeIteratorTrait;
use minigu_common::value::ScalarValue;
pub use vertex_iter::VertexIteratorTrait;

// Only used for dev
pub type ArrayRef = Arc<Vec<ScalarValue>>;
pub type ChunkData = Vec<ArrayRef>;
