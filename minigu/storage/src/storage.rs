use std::fmt::Debug;

use minigu_common::types::{EdgeId, VertexId};
use minigu_common::value::ScalarValue;

use crate::error::StorageResult;
use crate::memory::transaction::TransactionHandle;
use crate::model::edge::{Edge, Neighbor};
use crate::model::vertex::Vertex;

pub trait DynGraph:
    Graph<
        Transaction = TransactionHandle,
        VertexID = VertexId,
        EdgeID = EdgeId,
        Vertex = Vertex,
        Edge = Edge,
        Adjacency = Neighbor,
    > + Debug
    + Send
    + Sync
{
}

impl<T> DynGraph for T where
    T: Graph<
            Transaction = TransactionHandle,
            VertexID = u64,
            EdgeID = u64,
            Vertex = Vertex,
            Edge = Edge,
            Adjacency = Neighbor,
        > + Debug
        + Send
        + Sync
{
}

pub type BoxedGraph = Box<dyn DynGraph>;

/// Trait defining a read-only graph interface
pub trait Graph {
    type Transaction;
    type VertexID: Copy;
    type EdgeID: Copy;
    type Vertex;
    type Edge;
    type Adjacency;

    /// Retrieve a vertex by its ID within a transaction.
    fn get_vertex(
        &self,
        txn: &Self::Transaction,
        id: Self::VertexID,
    ) -> StorageResult<Self::Vertex>;

    /// Retrieve an edge by its ID within a transaction.
    fn get_edge(&self, txn: &Self::Transaction, id: Self::EdgeID) -> StorageResult<Self::Edge>;

    /// Get an iterator over all vertices in the graph within a transaction.
    fn iter_vertices<'a>(
        &'a self,
        txn: &'a Self::Transaction,
    ) -> StorageResult<Box<dyn Iterator<Item = StorageResult<Self::Vertex>> + 'a>>;

    /// Get an iterator over all edges in the graph within a transaction.
    fn iter_edges<'a>(
        &'a self,
        txn: &'a Self::Transaction,
    ) -> StorageResult<Box<dyn Iterator<Item = StorageResult<Self::Edge>> + 'a>>;

    /// Get an iterator over adjacency entries (edges connected to a vertex)
    /// in a given direction (incoming or outgoing) within a transaction.
    fn iter_adjacency<'a>(
        &'a self,
        txn: &'a Self::Transaction,
        vid: Self::VertexID,
    ) -> StorageResult<Box<dyn Iterator<Item = StorageResult<Self::Adjacency>> + 'a>>;
}

/// Trait defining a mutable graph interface (extending `Graph`).
pub trait MutGraph: Graph {
    /// Insert a new vertex into the graph within a transaction.
    fn create_vertex(
        &self,
        txn: &Self::Transaction,
        vertex: Self::Vertex,
    ) -> StorageResult<Self::VertexID>;

    /// Insert a new edge into the graph within a transaction.
    fn create_edge(&self, txn: &Self::Transaction, edge: Self::Edge)
    -> StorageResult<Self::EdgeID>;

    /// Delete a vertex from the graph within a transaction.
    fn delete_vertex(&self, txn: &Self::Transaction, vertice: Self::VertexID) -> StorageResult<()>;

    /// Delete an edge from the graph within a transaction.
    fn delete_edge(&self, txn: &Self::Transaction, edge: Self::EdgeID) -> StorageResult<()>;

    /// Update the properties of a vertex within a transaction.
    fn set_vertex_property(
        &self,
        txn: &Self::Transaction,
        vid: Self::VertexID,
        indices: Vec<usize>,
        props: Vec<ScalarValue>,
    ) -> StorageResult<()>;

    /// Update the properties of an edge within a transaction.
    fn set_edge_property(
        &self,
        txn: &Self::Transaction,
        eid: Self::EdgeID,
        indices: Vec<usize>,
        props: Vec<ScalarValue>,
    ) -> StorageResult<()>;
}

/// Trait defining basic transaction operations.
pub trait StorageTransaction {
    type CommitTimestamp;

    /// Commit the current transaction, returning a commit timestamp on success.
    fn commit(&self) -> StorageResult<Self::CommitTimestamp>;

    /// Abort (rollback) the current transaction, discarding all changes.
    fn abort(&self) -> StorageResult<()>;
}
