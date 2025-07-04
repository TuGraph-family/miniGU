use std::sync::Arc;

use crossbeam_skiplist::SkipSet;
use minigu_common::types::{EdgeId, VertexId};

use crate::common::iterators::{AdjacencyIteratorTrait, Direction};
use crate::common::model::edge::Neighbor;
use crate::error::StorageResult;
use crate::tp_storage::transaction::MemTransaction;

type AdjFilter<'a> = Box<dyn Fn(&Neighbor) -> bool + 'a>;

/// An adjacency list iterator that supports filtering (for iterating over a single vertex's
/// adjacency list).
pub struct AdjacencyIterator<'a> {
    adj_list: Option<Arc<SkipSet<Neighbor>>>, // The adjacency list for the vertex
    current_position: Option<Neighbor>,       // Current position in the iteration
    txn: &'a MemTransaction,                  // Reference to the transaction
    filters: Vec<AdjFilter<'a>>,              // List of filtering predicates
    finished: bool,                           // Whether iteration is finished
}

impl Iterator for AdjacencyIterator<'_> {
    type Item = StorageResult<Neighbor>;

    /// Retrieves the next visible adjacency entry that satisfies all filters.
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let adj_list = self.adj_list.as_ref()?;

        // Find the next position to start from
        let mut current_entry = if let Some(current_pos) = self.current_position {
            // Continue from after the current position
            if let Some(current_entry) = adj_list.get(&current_pos) {
                current_entry.next()
            } else {
                // Current position no longer exists, start from beginning
                adj_list.front()
            }
        } else {
            // Start from the beginning
            adj_list.front()
        };

        // Iterate through entries starting from the current position
        while let Some(entry_ref) = current_entry {
            let entry = *entry_ref.value();
            let eid = entry.eid();

            // Update current position for next iteration
            self.current_position = Some(entry);

            // Perform MVCC visibility check
            let is_visible = self
                .txn
                .graph()
                .edges
                .get(&eid)
                .map(|edge| edge.is_visible(self.txn))
                .unwrap_or(false);

            if is_visible && self.filters.iter().all(|f| f(&entry)) {
                return Some(Ok(entry));
            }

            // Move to next entry
            current_entry = entry_ref.next();
        }

        // No more entries found
        self.finished = true;
        self.current_position = None;
        None
    }
}

impl<'a> AdjacencyIterator<'a> {
    /// Creates a new `AdjacencyIterator` for a given vertex and direction (incoming or outgoing).
    pub fn new(txn: &'a MemTransaction, vid: VertexId, direction: Direction) -> Self {
        let adjacency_list = txn.graph().adjacency_list.get(&vid);

        let adj_list = adjacency_list.map(|entry| match direction {
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
        });

        Self {
            adj_list,
            current_position: None,
            txn,
            filters: Vec::new(),
            finished: false,
        }
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
    fn current_neighbor(&self) -> Option<&Neighbor> {
        self.current_position.as_ref()
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
