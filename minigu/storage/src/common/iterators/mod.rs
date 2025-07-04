/// Iterator traits for graph storage operations.
///
/// This module provides common iterator traits for vertex, edge, and adjacency
/// list iteration across different storage backends (TP and AP storage).
///
/// The traits are designed to be:
/// - Generic over storage implementations
/// - Support both single-item and batch operations
/// - Provide efficient filtering and seeking capabilities
mod adjacency_iter;
mod edge_iter;
mod vertex_iter;

pub use adjacency_iter::{AdjacencyIteratorTrait, Direction};
pub use edge_iter::EdgeIteratorTrait;
pub use vertex_iter::VertexIteratorTrait;

pub const DEFAULT_BATCH_SIZE: usize = 1024;
