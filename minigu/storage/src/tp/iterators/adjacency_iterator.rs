use std::sync::Arc;

use crossbeam_skiplist::SkipSet;
use minigu_common::types::{EdgeId, VertexId};

use crate::common::iterators::{AdjacencyIteratorTrait, Direction};
use crate::common::model::edge::Neighbor;
use crate::error::StorageResult;
use crate::tp::transaction::MemTransaction;

type AdjFilter<'a> = Box<dyn Fn(&Neighbor) -> bool + 'a>;

#[allow(dead_code)]
const BATCH_SIZE: usize = 64;

/// An adjacency list iterator that supports filtering (for iterating over a single vertex's
/// adjacency list).
pub struct AdjacencyIterator<'a> {
    adj_list: Option<Arc<SkipSet<Neighbor>>>, // The adjacency list for the vertex
    #[allow(dead_code)]
    current_entries: Vec<Neighbor>, // Store current batch of entries
    #[allow(dead_code)]
    current_index: usize, // Current index in the batch
    #[allow(dead_code)]
    txn: &'a MemTransaction, // Reference to the transaction
    filters: Vec<AdjFilter<'a>>,              // List of filtering predicates
    current_adj: Option<Neighbor>,            // Current adjacency entry
}

impl Iterator for AdjacencyIterator<'_> {
    type Item = StorageResult<Neighbor>;

    /// Retrieves the next visible adjacency entry that satisfies all filters.
    fn next(&mut self) -> Option<Self::Item> {
        // TODO：tp storage courge
        // Implement batch processing logic here
        // 1. Check if current_entries has unprocessed entries
        // 2. If yes, process the next entry, apply filters and visibility checks
        // 3. If no, call load_next_batch() to load the next batch of entries
        // 4. Repeat until an entry is found or no more data is available
        None
    }
}

impl<'a> AdjacencyIterator<'a> {
    fn load_next_batch(&mut self) -> Option<()> {
        // TODO：tp storage courge
        // You need to implement the logic for batch loading
        // load BATCH_SIZE entries from adj_list into current_entries
        // reset current_index to 0
        // if data is loaded return Some(()), else return None
        None
    }

    /// Creates a new `AdjacencyIterator` for a given vertex and direction (incoming or outgoing).
    pub fn new(txn: &'a MemTransaction, vid: VertexId, direction: Direction) -> Self {
        let adjacency_list = txn.graph().adjacency_list.get(&vid);

        let mut result = Self {
            adj_list: adjacency_list.map(|entry| match direction {
                Direction::Incoming => entry.incoming().clone(),
                Direction::Outgoing => entry.outgoing().clone(),
                Direction::Both => {
                    let combined = SkipSet::new();
                    for neighbor in entry.incoming().iter() {
                        combined.insert(*neighbor);
                    }
                    for neighbor in entry.outgoing().iter() {
                        combined.insert(*neighbor);
                    }
                    Arc::new(combined)
                }
            }),
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
        F: Fn(&Neighbor) -> bool + 'a,
    {
        self.filters.push(Box::new(predicate));
        self
    }

    /// Advances the iterator to the edge with the specified ID or the next greater edge.
    /// Returns `Ok(true)` if the exact edge is found, `Ok(false)` otherwise.
    fn seek(&mut self, id: EdgeId) -> StorageResult<bool> {
        for result in self.by_ref() {
            match result {
                Ok(entry) if entry.eid() == id => return Ok(true),
                Ok(entry) if entry.eid() > id => return Ok(false),
                _ => continue,
            }
        }
        Ok(false)
    }

    /// Returns a reference to the currently iterated adjacency entry.
    fn current_entry(&self) -> Option<&Neighbor> {
        self.current_adj.as_ref()
    }
}

/// Implementation for `MemTransaction`
impl MemTransaction {
    /// Returns an iterator over the adjacency list of a given vertex.
    /// Filtering conditions can be applied using the `filter` method.
    pub fn iter_adjacency(&self, vid: VertexId) -> AdjacencyIterator<'_> {
        AdjacencyIterator::new(self, vid, Direction::Both)
    }

    #[allow(dead_code)]
    pub fn iter_adjacency_outgoing(&self, vid: VertexId) -> AdjacencyIterator<'_> {
        AdjacencyIterator::new(self, vid, Direction::Outgoing)
    }

    #[allow(dead_code)]
    pub fn iter_adjacency_incoming(&self, vid: VertexId) -> AdjacencyIterator<'_> {
        AdjacencyIterator::new(self, vid, Direction::Incoming)
    }
}
