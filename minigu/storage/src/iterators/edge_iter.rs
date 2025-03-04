use super::ChunkData;
use crate::error::StorageResult;
use crate::model::edge::Edge;

pub trait EdgeIterator {
    /// Get the next edge in the iterator
    fn next(&mut self) -> StorageResult<()>;
    /// Move to the position that has an `id` greater or equal
    /// to the given `id`
    fn advance(&mut self, id: u64) -> StorageResult<bool>;
    /// Get the current edge
    fn edge(&self) -> Option<&Edge>;
    /// Get the properties of the current edge
    fn properties(&self) -> ChunkData;
}
