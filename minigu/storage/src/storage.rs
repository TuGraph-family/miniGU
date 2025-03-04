use common::datatype::value::PropertyValue;

use crate::error::StorageResult;
use crate::iterators::{EdgeIterator, VertexIterator};
use crate::model::edge::Direction;

/// Storage transaction
pub trait StorageTransaction {
    fn commit(self) -> StorageResult<()>;
    fn abort(self) -> StorageResult<()>;
}

/// Read-only graph structure
pub trait Graph {
    type Transaction: StorageTransaction;

    type VertexID: Copy;
    type EdgeID: Copy;
    type Vertex;
    type Edge;
    type Adjacency;

    type VertexIter<'a>: VertexIterator
    where
        Self: 'a;
    type EdgeIter<'a>: EdgeIterator
    where
        Self: 'a;
    type AdjacencyIter<'a>: EdgeIterator
    where
        Self: 'a;

    fn get_vertex(
        &self,
        txn: &Self::Transaction,
        id: Self::VertexID,
    ) -> StorageResult<Option<Self::Vertex>>;
    fn get_edge(
        &self,
        txn: &Self::Transaction,
        id: Self::EdgeID,
    ) -> StorageResult<Option<Self::Edge>>;
    fn vertices<'a>(&self, txn: &'a Self::Transaction) -> StorageResult<Self::VertexIter<'a>>;
    fn edges<'a>(&self, txn: &'a Self::Transaction) -> StorageResult<Self::EdgeIter<'a>>;
    fn neighbors<'a>(
        &self,
        txn: &'a Self::Transaction,
        id: Self::VertexID,
        direction: Direction,
    ) -> StorageResult<Self::AdjacencyIter<'a>>;
}

/// Mutable graph store
pub trait MutGraph: Graph {
    /// Insert a vertex
    fn create_vertex(&self, txn: &Self::Transaction, vertex: Self::Vertex) -> StorageResult<()>;

    /// Insert an edge
    fn create_edge(&self, txn: &Self::Transaction, edge: Self::Edge) -> StorageResult<()>;

    fn delete_vertices(
        &self,
        txn: &Self::Transaction,
        vertices: Vec<Self::Vertex>,
    ) -> StorageResult<()>;

    fn delete_edges(&self, txn: &Self::Transaction, edges: Vec<Self::Edge>) -> StorageResult<()>;

    fn set_vertex_property(
        &self,
        txn: &Self::Transaction,
        vid: u64,
        indices: Vec<usize>,
        props: Vec<PropertyValue>,
    ) -> StorageResult<()>;

    fn set_edge_property(
        &self,
        txn: &Self::Transaction,
        eid: u64,
        indices: Vec<usize>,
        props: Vec<PropertyValue>,
    ) -> StorageResult<()>;
}
