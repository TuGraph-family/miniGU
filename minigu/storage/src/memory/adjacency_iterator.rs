use std::sync::Arc;

use common::datatype::types::{EdgeId, VertexId};
use crossbeam_skiplist::SkipSet;
use dashmap::mapref::one::Ref;

use super::memory_graph::VersionedAdjContainer;
use super::transaction::MemTransaction;
use crate::error::StorageResult;
use crate::iterators::AdjacencyIteratorTrait;
use crate::model::edge::{Adjacency, Direction};

type AdjFilter<'a> = Box<dyn Fn(&Adjacency) -> bool + 'a>;

/// An adjacency list iterator that supports filtering (for iterating over a single vertex's
/// adjacency list).
pub struct AdjacencyIterator<'a> {
    adj_list: Option<Arc<SkipSet<Adjacency>>>, // The adjacency list for the vertex
    current_entries: Vec<Adjacency>,           // Store current batch of entries
    current_index: usize,                      // Current index in the batch
    txn: &'a MemTransaction,                   // Reference to the transaction
    filters: Vec<AdjFilter<'a>>,               // List of filtering predicates
    current_adj: Option<Adjacency>,            // Current adjacency entry
}

impl Iterator for AdjacencyIterator<'_> {
    type Item = StorageResult<Adjacency>;

    /// Retrieves the next visible adjacency entry that satisfies all filters.
    fn next(&mut self) -> Option<Self::Item> {
        // If current batch is processed, get a new batch
        if self.current_index >= self.current_entries.len() {
            self.load_next_batch()?;
        }

        // Process entries in current batch
        while self.current_index < self.current_entries.len() {
            let entry = &self.current_entries[self.current_index];
            self.current_index += 1;

            let eid = entry.edge_id();

            // Perform MVCC visibility check
            let is_visible = self
                .txn
                .graph()
                .edges
                .get(&eid)
                .map(|edge| edge.is_visible(self.txn))
                .unwrap_or(false);

            if is_visible && self.filters.iter().all(|f| f(entry)) {
                let adj = entry.clone();
                self.current_adj = Some(adj.clone());
                return Some(Ok(adj));
            }
        }

        // If current batch is processed but no match found, try loading next batch
        self.load_next_batch()?;
        self.next()
    }
}

impl<'a> AdjacencyIterator<'a> {
    fn load_next_batch(&mut self) -> Option<()> {
        if let Some(adj_list) = &self.adj_list {
            let iter = if let Some(e) = self.current_entries.last() {
                adj_list.get(e)?.next()?
            } else {
                adj_list.front()?
            };
            // Clear current entry batch
            self.current_entries.clear();
            self.current_index = 0;

            // Load the next batch of entries
            self.current_entries.push(iter.value().clone());
            for _ in 0..64 {
                if let Some(entry) = iter.next() {
                    self.current_entries.push(entry.value().clone());
                } else {
                    break;
                }
            }

            if !self.current_entries.is_empty() {
                return Some(());
            }
        }
        None
    }

    /// Creates a new `AdjacencyIterator` for a given vertex and direction (incoming or outgoing).
    pub fn new(txn: &'a MemTransaction, vid: VertexId, direction: Direction) -> Self {
        let adjacency_list: Option<Ref<'_, u64, VersionedAdjContainer>> = match direction {
            Direction::Out => txn.graph().adjacency_out.get(&vid),
            Direction::In => txn.graph().adjacency_in.get(&vid),
        };

        let mut result = Self {
            adj_list: adjacency_list.map(|entry| entry.inner.clone()),
            current_entries: Vec::new(),
            current_index: 0,
            txn,
            filters: Vec::new(),
            current_adj: None,
        };

        // Preload the first batch of data
        if result.adj_list.is_some() {
            result.load_next_batch();
        }

        result
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
        for result in self.by_ref() {
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
        self.current_adj.as_ref()
    }
}

/// Implementation for `MemTransaction`
impl MemTransaction {
    /// Returns an iterator over the adjacency list of a given vertex.
    /// Filtering conditions can be applied using the `filter` method.
    pub fn iter_adjacency(&self, vid: VertexId, direction: Direction) -> AdjacencyIterator<'_> {
        AdjacencyIterator::new(self, vid, direction)
    }
}
