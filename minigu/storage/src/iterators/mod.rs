mod adjacency_iter;
mod edge_iter;
mod vertex_iter;
use std::sync::Arc;

pub use adjacency_iter::{AdjacencyIteratorTrait, Direction};
use common::datatype::value::PropertyValue;
pub use edge_iter::EdgeIteratorTrait;
pub use vertex_iter::VertexIteratorTrait;

// Only used for dev
pub type ArrayRef = Arc<Vec<PropertyValue>>;
pub type ChunkData = Vec<ArrayRef>;
