use std::sync::Arc;

use dashmap::iter::Iter;

use super::MemTransaction;
use super::memory_graph::{
    Adjacency, UpdateEdgeOp, UpdateVertexOp, VersionedEdge, VersionedVertex,
};
use crate::error::{StorageError, StorageResult};
use crate::iterators::{EdgeIterator, VertexIterator};
use crate::model::edge::{Direction, Edge};
use crate::model::vertex::Vertex;
use crate::storage::Graph;

pub struct MemVertexIter<'a> {
    txn: &'a MemTransaction,
    vertex_iter: Iter<'a, u64, Arc<VersionedVertex>>,
    current_index: usize,
    current_vertex: Option<Vertex>,
    vertex_inserts: Vec<Vertex>,
    end: bool,
}

impl<'a> MemVertexIter<'a> {
    pub fn new(txn: &'a MemTransaction) -> Self {
        let vertex_iter = txn.storage.vertices.iter();
        Self {
            txn,
            vertex_iter,
            current_index: 0,
            current_vertex: None,
            vertex_inserts: vec![],
            end: false,
        }
    }
}

impl<'a> VertexIterator for MemVertexIter<'a> {
    type AdjIter<'b>
        = MemAdjIter<'b>
    where
        'a: 'b;

    fn next(&mut self) -> StorageResult<()> {
        if self.end {
            return Ok(());
        }

        // First try to get the next vertex from the iterator
        if let Some(v) = self.vertex_iter.next() {
            if let Ok(Some(vertex)) = self.txn.storage.get_vertex(self.txn, *v.key()) {
                self.current_vertex = Some(vertex.clone());
                return Ok(());
            }
        }

        // If we've exhausted the iterator, check for local transaction inserts
        // This is a fallback to ensure we don't miss any vertices added in the current transaction
        match self.current_index.cmp(&0) {
            std::cmp::Ordering::Equal => {
                // Only collect once when we first need it
                self.vertex_inserts = self
                    .txn
                    .vertex_updates
                    .iter()
                    .filter_map(|entry| match entry.value() {
                        UpdateVertexOp::Insert(v) => Some(v.inner().clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>();

                if self.vertex_inserts.is_empty() {
                    self.end = true;
                    self.current_vertex = None;
                    return Ok(());
                }
                if let Some(vertex) = self.vertex_inserts.first() {
                    self.current_index += 1;
                    self.current_vertex = Some(vertex.clone());
                    return Ok(());
                }
            }
            std::cmp::Ordering::Greater => {
                // Continue iterating through local inserts if we've started
                if let Some(vertex) = self.vertex_inserts.get(self.current_index) {
                    self.current_index += 1;
                    self.current_vertex = Some(vertex.clone());
                    return Ok(());
                } else {
                    self.end = true;
                    self.current_vertex = None;
                }
            }
            std::cmp::Ordering::Less => {}
        }

        Ok(())
    }

    fn advance(&mut self, id: u64) -> StorageResult<bool> {
        if self.end {
            return Ok(false);
        }

        // Check the current vertex
        if let Some(v) = self.vertex() {
            match v.vid.cmp(&id) {
                std::cmp::Ordering::Equal => return Ok(true),
                std::cmp::Ordering::Greater => return Ok(false),
                std::cmp::Ordering::Less => {}
            }
        }

        // Call the `next` until reaching the target
        self.next()?;
        while let Some(v) = self.vertex() {
            match v.vid.cmp(&id) {
                std::cmp::Ordering::Equal => return Ok(true),
                std::cmp::Ordering::Greater => return Ok(false),
                std::cmp::Ordering::Less => {}
            }
            self.next()?;
        }

        Ok(false)
    }

    fn vertex(&self) -> Option<&Vertex> {
        self.current_vertex.as_ref()
    }

    fn properties(&self) -> crate::iterators::ChunkData {
        if let Some(vertex) = &self.current_vertex {
            vec![Arc::new(vertex.properties().clone())]
        } else {
            crate::iterators::ChunkData::new()
        }
    }

    fn get_out_edge_iterator(&self) -> StorageResult<Self::AdjIter<'a>> {
        // Create an edge iterator for outgoing edges from the current vertex
        if let Some(vertex) = &self.current_vertex {
            // Create a new edge iterator for the outgoing edges
            Ok(MemAdjIter::new(self.txn, vertex.vid(), Direction::Out)?)
        } else {
            // Return an empty iterator if there's no current vertex
            Err(StorageError::IteratorError(
                "Can't create an iterator for a Null vertex".to_string(),
            ))
        }
    }

    fn get_in_edge_iterator(&self) -> StorageResult<Self::AdjIter<'a>> {
        // Create an edge iterator for incoming edges to the current vertex
        if let Some(vertex) = &self.current_vertex {
            // Create a new adjanceny iterator for the IN edges
            Ok(MemAdjIter::new(self.txn, vertex.vid(), Direction::In)?)
        } else {
            Err(StorageError::IteratorError(
                "Can't create an iterator for a Null vertex".to_string(),
            ))
        }
    }
}

pub struct MemAdjIter<'a> {
    txn: &'a MemTransaction,
    current_edge: Option<Edge>,
    adjacency_iter: std::vec::IntoIter<Adjacency>,
    end: bool,
}

impl<'a> MemAdjIter<'a> {
    pub fn new(txn: &'a MemTransaction, vid: u64, direction: Direction) -> StorageResult<Self> {
        let mut adjs = match direction {
            Direction::Out => {
                let adjs = if let Some(adjs) = txn.storage.adjacency_out.get(&vid) {
                    adjs.value().clone()
                } else {
                    vec![]
                };
                let edge_inserts = txn
                    .edge_updates
                    .iter()
                    .filter_map(|entry| match entry.value() {
                        UpdateEdgeOp::Insert(v) if v.inner().source_id == vid => Some(Adjacency {
                            edge_id: v.inner().eid,
                            vertex_id: v.inner().dst_id,
                        }),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                adjs.into_iter().chain(edge_inserts).collect::<Vec<_>>()
            }
            Direction::In => {
                let adjs = if let Some(adjs) = txn.storage.adjacency_in.get(&vid) {
                    adjs.value().clone()
                } else {
                    vec![]
                };
                let edge_inserts = txn
                    .edge_updates
                    .iter()
                    .filter_map(|entry| match entry.value() {
                        UpdateEdgeOp::Insert(v) if v.inner().dst_id == vid => Some(Adjacency {
                            edge_id: v.inner().eid,
                            vertex_id: v.inner().source_id,
                        }),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                adjs.into_iter().chain(edge_inserts).collect::<Vec<_>>()
            }
        };
        adjs.sort_by(|a, b| a.vertex_id.cmp(&b.vertex_id));
        let end = adjs.is_empty();
        Ok(Self {
            txn,
            current_edge: None,
            adjacency_iter: adjs.into_iter(),
            end,
        })
    }
}

impl EdgeIterator for MemAdjIter<'_> {
    fn next(&mut self) -> StorageResult<()> {
        if self.end {
            return Ok(());
        }

        // First check global edges
        loop {
            if let Some(adj) = self.adjacency_iter.next() {
                if let Some(edge) = self.txn.storage.get_edge(self.txn, adj.edge_id)? {
                    self.current_edge = Some(edge);
                    return Ok(());
                } else {
                    continue;
                }
            }
            self.end = true;
            self.current_edge = None;
            break;
        }

        Ok(())
    }

    fn advance(&mut self, id: u64) -> StorageResult<bool> {
        if self.end {
            return Ok(false);
        }

        // Check the current edge
        if let Some(e) = self.edge() {
            match e.eid.cmp(&id) {
                std::cmp::Ordering::Equal => return Ok(true),
                std::cmp::Ordering::Greater => return Ok(false),
                std::cmp::Ordering::Less => {}
            }
        }

        // Call the `next` until reaching the target
        self.next()?;
        while let Some(e) = self.edge() {
            match e.eid.cmp(&id) {
                std::cmp::Ordering::Equal => return Ok(true),
                std::cmp::Ordering::Greater => return Ok(false),
                std::cmp::Ordering::Less => {}
            }
            self.next()?;
        }

        Ok(false)
    }

    fn properties(&self) -> crate::iterators::ChunkData {
        if let Some(edge) = &self.current_edge {
            vec![Arc::new(edge.properties().clone())]
        } else {
            crate::iterators::ChunkData::new()
        }
    }

    fn edge(&self) -> Option<&crate::model::edge::Edge> {
        self.current_edge.as_ref()
    }
}

pub struct MemEdgeIter<'a> {
    txn: &'a MemTransaction,
    current_edge: Option<Edge>,
    edge_iter: Iter<'a, u64, Arc<VersionedEdge>>,
    edge_inserts: Vec<Edge>,
    end: bool,
    current_index: usize,
}

impl<'a> MemEdgeIter<'a> {
    pub fn new(txn: &'a MemTransaction) -> Self {
        let edge_iter = txn.storage.edges.iter();
        Self {
            txn,
            current_edge: None,
            edge_iter,
            edge_inserts: vec![],
            end: false,
            current_index: 0,
        }
    }
}

impl EdgeIterator for MemEdgeIter<'_> {
    fn next(&mut self) -> StorageResult<()> {
        if self.end {
            return Ok(());
        }

        // First try to get the next edge from the iterator
        if let Some(e) = self.edge_iter.next() {
            if let Ok(Some(edge)) = self.txn.storage.get_edge(self.txn, *e.key()) {
                self.current_edge = Some(edge.clone());
                return Ok(());
            }
        }

        match self.current_index.cmp(&0) {
            std::cmp::Ordering::Equal => {
                // Only collect once when we first need it
                self.edge_inserts = self
                    .txn
                    .edge_updates
                    .iter()
                    .filter_map(|entry| match entry.value() {
                        UpdateEdgeOp::Insert(e) => Some(e.inner().clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                if self.edge_inserts.is_empty() {
                    self.end = true;
                    self.current_edge = None;
                    return Ok(());
                }

                if let Some(edge) = self.edge_inserts.first() {
                    self.current_index += 1;
                    self.current_edge = Some(edge.clone());
                    return Ok(());
                }
                return Ok(());
            }
            std::cmp::Ordering::Greater => {
                // Continue iterating through local inserts if we've started
                if let Some(edge) = self.edge_inserts.get(self.current_index) {
                    self.current_index += 1;
                    self.current_edge = Some(edge.clone());
                } else {
                    self.end = true;
                    self.current_edge = None;
                }
            }
            std::cmp::Ordering::Less => {}
        };

        Ok(())
    }

    fn advance(&mut self, id: u64) -> StorageResult<bool> {
        if self.end {
            return Ok(false);
        }

        // Check the current edge
        if let Some(e) = self.edge() {
            match e.eid.cmp(&id) {
                std::cmp::Ordering::Equal => return Ok(true),
                std::cmp::Ordering::Greater => return Ok(false),
                std::cmp::Ordering::Less => {}
            }
        }

        // Call the `next` until reaching the target
        self.next()?;
        while let Some(e) = self.edge() {
            match e.eid.cmp(&id) {
                std::cmp::Ordering::Equal => return Ok(true),
                std::cmp::Ordering::Greater => return Ok(false),
                std::cmp::Ordering::Less => {}
            }
        }

        Ok(false)
    }

    fn edge(&self) -> Option<&Edge> {
        self.current_edge.as_ref()
    }

    fn properties(&self) -> crate::iterators::ChunkData {
        if let Some(edge) = &self.current_edge {
            vec![Arc::new(edge.properties().clone())]
        } else {
            crate::iterators::ChunkData::new()
        }
    }
}
