use common::datatype::types::{EdgeId, VertexId};
use dashmap::mapref::one::Ref;

use super::memory_graph::CommitTimestamp;
use crate::error::StorageResult;
use crate::iterators::AdjacencyIteratorTrait;
use crate::memory::memory_graph::{MemTransaction, VersionedAdjEntry};
use crate::model::edge::{Adjacency, Direction};

/// An adjacency list iterator that supports filtering (for iterating over a single vertex's
/// adjacency list).
pub struct AdjacencyIterator<'a> {
    adjacency_list: Option<Ref<'a, VertexId, Vec<VersionedAdjEntry>>>, /* The adjacency list for
                                                                        * the vertex */
    index: usize,            // Current index in the adjacency list
    txn: &'a MemTransaction, // Reference to the transaction
    filters: Vec<Box<dyn Fn(&Adjacency) -> bool + 'a>>, // List of filtering predicates
    current_entry: Option<Adjacency>, // The currently iterated adjacency entry
}

impl<'a> Iterator for AdjacencyIterator<'a> {
    type Item = StorageResult<Adjacency>;

    /// Retrieves the next visible adjacency entry that satisfies all filters.
    fn next(&mut self) -> Option<Self::Item> {
        let entries = self.adjacency_list.as_deref()?;

        while self.index < entries.len() {
            let adj_entry = &entries[self.index];
            self.index += 1;

            // Perform MVCC visibility check
            if !(adj_entry.begin_ts() <= self.txn.start_ts()
                && self.txn.start_ts() < adj_entry.end_ts()
                && adj_entry.end_ts() == CommitTimestamp::max())
            {
                continue;
            }

            // Apply all filtering conditions
            if self.filters.iter().all(|f| f(adj_entry.inner())) {
                let adjacency = adj_entry.inner().clone(); // 获取 `Adjacency` 对象
                self.current_entry = Some(adjacency.clone());
                return Some(Ok(adjacency));
            }
        }

        self.current_entry = None; // Reset when iteration ends
        None
    }
}

impl<'a> AdjacencyIterator<'a> {
    /// Creates a new `AdjacencyIterator` for a given vertex and direction (incoming or outgoing).
    pub fn new(txn: &'a MemTransaction, vid: VertexId, direction: Direction) -> Self {
        let adjacency_list = match direction {
            Direction::Out => txn.graph().adjacency_out().get(&vid),
            Direction::In => txn.graph().adjacency_in().get(&vid),
        };

        AdjacencyIterator {
            adjacency_list,
            index: 0,
            txn,
            filters: Vec::new(),
            current_entry: None,
        }
    }
}

impl<'a> AdjacencyIteratorTrait<'a> for AdjacencyIterator<'a> {
    /// Adds a filtering predicate to the iterator (supports method chaining).
    fn filter<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&Adjacency) -> bool + 'a,
    {
        self.filters.push(Box::new(predicate));
        self
    }

    /// Advances the iterator to the edge with the specified ID or the next greater edge.
    /// Returns `Ok(true)` if the exact edge is found, `Ok(false)` otherwise.
    fn advance(&mut self, id: EdgeId) -> StorageResult<bool> {
        while let Some(result) = self.next() {
            match result {
                Ok(entry) if entry.edge_id() == id => return Ok(true),
                Ok(entry) if entry.edge_id() > id => return Ok(false),
                _ => continue,
            }
        }
        Ok(false)
    }

    /// Returns a reference to the currently iterated adjacency entry.
    fn current_entry(&self) -> Option<&Adjacency> {
        self.current_entry.as_ref()
    }
}

/// Implementation for `MemTransaction`
impl<'a> MemTransaction {
    /// Returns an iterator over the adjacency list of a given vertex.
    /// Filtering conditions can be applied using the `filter` method.
    pub fn iter_adjacency(&self, vid: VertexId, direction: Direction) -> AdjacencyIterator<'_> {
        AdjacencyIterator::new(self, vid, direction)
    }
}
