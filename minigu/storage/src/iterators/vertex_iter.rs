use super::{ChunkData, EdgeIterator};
use crate::error::StorageResult;
use crate::model::vertex::Vertex;

pub trait VertexIterator {
    type AdjIter<'a>: EdgeIterator
    where
        Self: 'a;

    /// Move to the next position
    fn next(&mut self) -> StorageResult<()>;
    /// Move to the position that has an `id`
    /// greater than or equal to the given `id`
    fn advance(&mut self, id: u64) -> StorageResult<bool>;
    /// Get the current vertex
    fn vertex(&self) -> Option<&Vertex>;
    /// Get the properties of the current vertex
    fn properties(&self) -> ChunkData;
    /// Get the out edge iterator of the current vertex
    fn get_out_edge_iterator(&self) -> StorageResult<Self::AdjIter<'_>>;
    /// Get the in edge iterator of the current vertex
    fn get_in_edge_iterator(&self) -> StorageResult<Self::AdjIter<'_>>;
}
