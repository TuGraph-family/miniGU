use std::sync::atomic::{AtomicU64, Ordering};

use common::datatype::types::{EdgeId, LabelId, VertexId};
use common::datatype::value::PropertyValue;

use crate::model::edge::Edge;
use crate::model::vertex::Vertex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// Represents a commit timestamp used for multi-version concurrency control (MVCC).
pub struct Timestamp(pub u64);

impl Timestamp {
    pub(super) const TXN_ID_START: u64 = 1 << 63;
    pub(super) const TXN_START_TS: Self = Self(Self::TXN_ID_START);

    /// Generates a new transaction ID, ensuring atomicity using an atomic counter.
    pub fn new_txn_id() -> Self {
        // Static counter initialized once, persists between calls.
        static COUNTER: AtomicU64 = AtomicU64::new(Timestamp::TXN_ID_START + 1);
        // Transaction ID only needs to be atomically incremented
        // and does not require strict memory order.
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    /// Generates a new commit timestamp using an atomic counter.
    pub fn new_commit_ts() -> Self {
        // Static counter initialized once, persists between calls.
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        // Only one transaction can commit at a time.
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    /// Create timestamp by a given commit ts
    pub fn with_commit_ts(commit_ts: u64) -> Self {
        Self(commit_ts)
    }

    /// Returns the maximum possible commit timestamp.
    pub fn max_commit_ts() -> Self {
        Self(u64::MAX & !Self::TXN_ID_START)
    }

    /// Returns true if the timestamp is a transaction ID.
    pub fn is_txn_id(&self) -> bool {
        self.0 & Self::TXN_ID_START != 0
    }

    /// Returns true if the timestamp is a commit timestamp.
    pub fn is_commit_ts(&self) -> bool {
        self.0 & Self::TXN_ID_START == 0
    }
}

/// Represents a pointer to an undo entry in the undo buffer.
#[derive(Debug, Clone, Copy)]
pub struct UndoPtr {
    /// The transaction id of the undo ptr.
    txn_id: Timestamp,
    /// The entry offset of the undo ptr, points to the undo entry in the undo buffer.
    entry_offset: usize,
}

impl UndoPtr {
    /// Create a UndoPtr
    pub fn new(txn_id: Timestamp, entry_offset: usize) -> Self {
        Self {
            txn_id,
            entry_offset,
        }
    }

    /// Get the transaction id of the undo ptr.
    pub fn txn_id(&self) -> Timestamp {
        self.txn_id
    }

    /// Get the entry offset of the undo ptr.
    pub fn entry_offset(&self) -> usize {
        self.entry_offset
    }
}

#[derive(Debug, Clone)]
/// Represents an undo log entry for multi-version concurrency control.
pub struct UndoEntry {
    /// The delta operation of the undo entry.
    delta: DeltaOp,
    /// The timestamp when this version is committed.
    timestamp: Timestamp,
    /// The next undo entry in the undo buffer.
    next: Option<UndoPtr>,
}

impl UndoEntry {
    /// Create a UndoEntry
    pub(super) fn new(delta: DeltaOp, timestamp: Timestamp, next: Option<UndoPtr>) -> Self {
        Self {
            delta,
            timestamp,
            next,
        }
    }

    /// Get the data of the undo entry.
    pub(super) fn delta(&self) -> &DeltaOp {
        &self.delta
    }

    /// Get the end timestamp of the undo entry.
    pub(super) fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    /// Get the next undo ptr of the undo entry.
    pub(super) fn next(&self) -> Option<UndoPtr> {
        self.next
    }
}

#[derive(Debug, Clone)]
pub struct SetPropsOp {
    pub indices: Vec<usize>,
    pub props: Vec<PropertyValue>,
}

#[derive(Debug, Clone)]
pub enum DeltaOp {
    DelVertex(VertexId),
    DelEdge(EdgeId),
    CreateVertex(Vertex),
    CreateEdge(Edge),
    SetVertexProps(VertexId, SetPropsOp),
    SetEdgeProps(EdgeId, SetPropsOp),
    /// Used only for Vertex
    AddLabel(LabelId),
    RemoveLabel(LabelId),
    // AddInEdge(Edge),
    // AddOutEdge(Edge),
    // RemoveInEdge(Adjacency),
    // RemoveOutEdge(Adjacency),
}

pub enum IsolationLevel {
    Snapshot,
    Serializable,
}
