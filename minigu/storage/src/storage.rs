use crate::error::StorageResult;

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

    type VertexIter: Iterator<Item = Self::Vertex>;
    type EdgeIter: Iterator<Item = Self::Edge>;
    type AdjacencyIter: Iterator<Item = Self::Adjacency>;

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
    fn vertices(&self, txn: &Self::Transaction) -> StorageResult<Self::VertexIter>;
    fn edges(&self, txn: &Self::Transaction) -> StorageResult<Self::EdgeIter>;
    fn neighbors(
        &self,
        txn: &Self::Transaction,
        id: Self::VertexID,
        direction: Direction,
    ) -> StorageResult<Self::AdjacencyIter>;
}

/// Mutable graph store
pub trait MutGraph: Graph {
    fn create_vertex(&self, txn: &Self::Transaction, vertex: Self::Vertex) -> StorageResult<()>;
    fn create_edge(&self, txn: &Self::Transaction, edge: Self::Edge) -> StorageResult<()>;
    fn delete_vertices(
        &self,
        txn: &Self::Transaction,
        vertices: Vec<Self::Vertex>,
    ) -> StorageResult<()>;
    fn delete_edges(&self, txn: &Self::Transaction, edges: Vec<Self::Edge>) -> StorageResult<()>;
}

/// The neighbor access direction
#[derive(Clone, Copy)]
pub enum Direction {
    Forward,
    Reversed,
}
