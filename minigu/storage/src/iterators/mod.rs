mod edge_iter;
mod vertex_iter;

use std::sync::Arc;

use common::datatype::value::PropertyValue;
pub use edge_iter::EdgeIterator;
pub use vertex_iter::VertexIterator;

// Only used for dev
pub type ArrayRef = Arc<Vec<PropertyValue>>;
pub type ChunkData = Vec<ArrayRef>;
